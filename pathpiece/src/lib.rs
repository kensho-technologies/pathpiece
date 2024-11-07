mod configuration;
mod tests;

use std::cmp;

use configuration::globals::SEED;
use configuration::read;

use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::types::PyDict;
use pyo3::types::PyList;
use pyo3::types::PyString;
use std::collections::HashMap;

use rand::{Rng, SeedableRng};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

#[pymodule]
fn pathpiece(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Tokenizer>()?;
    Ok(())
}

#[pyclass]
// TODO: figure out implement send stuff
pub struct Tokenizer {
    vocab: HashMap<Vec<u8>, usize>, // mapping of tokens to indices
    input_ids: HashMap<usize, Vec<u8>>,   // inverse of vocab, map index to token
    // so t == indices[vocab[t]]
    max_len: usize, // maximum number of bytes in vocab
    greedy: bool,
    random_tiebreaker : bool,  // break ties with random or longest token, doesn't apply to greedy
}

#[pymethods]
impl Tokenizer {
    // create a new Tokenizer by reading the vocab from a file
    // also set if it is greedy or not here so we don't have to change calling code
    #[new]
    #[pyo3(signature = (vocab_path, special="<|endoftext|>", greedy=false, random_tiebreaker=false))]
    pub fn new(vocab_path: &str, special: &str, greedy : bool, random_tiebreaker : bool) -> Self {
        fn flip_vocab(vocab: &HashMap<Vec<u8>, usize>) -> HashMap<usize, Vec<u8>> {
            let mut ids = HashMap::new();
            for (key, value) in vocab.iter() {
                ids.insert(*value, key.clone());
            }
            ids
        }

        let vocab = read::read_vocab(vocab_path, special.as_bytes().to_vec());
        let max_len = vocab.keys().map(|key| key.len()).max().unwrap();
        let input_ids = flip_vocab(&vocab);

        Tokenizer {
            vocab,
            input_ids,
            max_len,
            greedy,
            random_tiebreaker,
        }
    }

    #[pyo3(signature = (sample, **kwargs))]
    pub fn __call__(&self, py: Python, sample: &PyAny, kwargs: Option<&PyDict>) -> PyResult<PyObject> {

        // Sample is a string
        if let Ok(sample_str) = sample.downcast::<PyString>() {
            let rust_str = sample_str.to_str()?;
            return self.encode(py, rust_str, kwargs);
        }

        // Sample is a list of strings
        if let Ok(sample_list) = sample.downcast::<PyList>() {
            let mut rust_string_list = Vec::new();

            for item in sample_list.iter() {
                let item_str = item.downcast::<PyString>()?;
                rust_string_list.push(item_str.to_str()?);
            }

            return self.encode_batch(py, rust_string_list, kwargs);
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Expected a string or a list of strings for parameter 'sample'."
        ))
    }

    pub fn get_max_len(&self) -> usize {
        self.max_len
    }

    pub fn get_vocab(&self, py: Python) -> Py<PyDict> {
        let vocab_dict = PyDict::new(py);
        for (k, v) in &self.vocab {
            vocab_dict.set_item(PyBytes::new(py, &k), *v).unwrap();
        }
        vocab_dict.into()
    }

    pub fn get_ids(&self, py: Python) -> Py<PyDict> {
        let ids_dict = PyDict::new(py);
        for (k, v) in &self.input_ids {
            ids_dict.set_item(*k, PyBytes::new(py, &v)).unwrap();
        }
        ids_dict.into()
    }

    #[pyo3(signature = (optimal_ids, **_kwargs))]
    pub fn decode(
        &self,
        py: Python,
        optimal_ids: Vec<usize>,
        _kwargs: Option<&PyDict>,
    ) -> PyResult<PyObject> {
        let bytes: Vec<u8> = optimal_ids
            .iter()
            .flat_map(|&id| self.input_ids[&id].clone())
            .collect();

        let decoded = String::from_utf8_lossy(&bytes).into_owned();

        Ok(PyString::new(py, &decoded).to_object(py))
    }

    #[pyo3(signature = (optimal_ids, **_kwargs))]
    pub fn decode_batch(
        &self,
        py: Python,
        optimal_ids: Vec<Vec<usize>>,
        _kwargs: Option<&PyDict>,
    ) -> PyResult<PyObject> {
        let decoded: Vec<PyObject> = optimal_ids
            .iter()
            .map(|sample| {
                let bytes: Vec<u8> = sample
                    .iter()
                    .flat_map(|&id| self.input_ids[&id].clone())
                    .collect();

                let decoded_sample = String::from_utf8_lossy(&bytes).into_owned();

                PyString::new(py, &decoded_sample).to_object(py)
            })
            .collect();

        Ok(decoded.to_object(py))
    }

    pub fn decode_bytes(&self, py: Python, optimal_ids: Vec<usize>) -> PyResult<PyObject> {
        let bytes: Vec<u8> = optimal_ids
            .iter()
            .flat_map(|&id| self.input_ids[&id].clone())
            .collect();

        let py_bytes = PyBytes::new(py, &bytes);

        Ok(py_bytes.to_object(py))
    }

    pub fn decode_batch_bytes(
        &self,
        py: Python,
        optimal_ids: Vec<Vec<usize>>,
    ) -> PyResult<PyObject> {
        let decoded: Vec<PyObject> = optimal_ids
            .iter()
            .map(|sample| {
                let bytes: Vec<u8> = sample
                    .iter()
                    .flat_map(|&id| self.input_ids[&id].clone())
                    .collect();

                let py_bytes = PyBytes::new(py, &bytes);

                py_bytes.to_object(py)
            })
            .collect();

        Ok(decoded.to_object(py))
    }

    #[pyo3(signature = (sample, **_kwargs))]
    pub fn encode(&self, py: Python, sample: &str, _kwargs: Option<&PyDict>) -> PyResult<PyObject> {

        let sample_bytes: Vec<u8> = sample.as_bytes().to_vec();
        let tokenization = self.encode_rust(&sample_bytes);

        let optimal_indices = tokenization
            .iter()
            .map(|token| self.vocab[token])
            .collect::<Vec<usize>>();

        let result = optimal_indices
            .into_iter()
            .collect::<Vec<usize>>()
            .to_object(py);

        let result_dict = PyDict::new(py);
        result_dict.set_item("input_ids", result.to_object(py))?;

        Ok(result_dict.to_object(py))
    }

    #[pyo3(signature = (corpus, **_kwargs))]
    pub fn encode_batch(
        &self,
        py: Python,
        corpus: Vec<&str>,
        _kwargs: Option<&PyDict>,
    ) -> PyResult<PyObject> {

        let corpus_bytes: Vec<Vec<u8>> =
            corpus.into_iter().map(|s| s.as_bytes().to_vec()).collect();
        let optimal_indices = self.encode_batch_rust(corpus_bytes);

        let result = optimal_indices
            .into_iter()
            .map(|inner| {
                inner
                    .into_iter()
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        let result_dict = PyDict::new(py);
        result_dict.set_item("input_ids", result.to_object(py))?;

        Ok(result_dict.to_object(py))
    }

}

// Functions not exposed to Python
impl Tokenizer {

    fn encode_batch_rust(&self, corpus: Vec<Vec<u8>>) -> Vec<Vec<usize>> {
        let indices: Vec<Vec<usize>> = corpus
            .par_iter()
            .map(|sample| {
                let tokenization = self.encode_rust(sample);
                tokenization
                    .iter()
                    .map(|token| self.vocab[token])
                    .collect::<Vec<usize>>()
            })
            .collect();

        indices
    }

    // call the right encode function
    fn encode_rust(&self, sample: &[u8]) -> Vec<Vec<u8>> {
        return if self.greedy {
            self.greedy_encode_rust(sample)     // greedy, doesn't require a tiebreaker
        } else {
            if self.random_tiebreaker {
                self.opt_encode_rust_random_tiebreaker(sample)    // optimal with optimal tiebreaker
            } else {
                self.opt_encode_rust_longest_tiebreaker(sample)   // optimal with longest token tiebreaker
            }
        }
    }

    // do a greedy encoding of the sample text, always using the longest possible token
    fn greedy_encode_rust(&self, sample: &[u8]) -> Vec<Vec<u8>> {

        let n: usize = sample.len();
        let mut greedy_tokenization: Vec<Vec<u8>> = Vec::new();  // our output
        let mut i : usize = 0;

        while i < n {

            // don't want to go beyond the end
            // ex n = 10, i = 7, max_len = 16
            // can do at most sample[i..i+j] = sample[7..10] since not inclusive
            // want i+maxj == n, so maxj = n-i
            let cur_max_len = cmp::min(self.max_len,n-i);

            // count down from max_len to 1, inclusive
            for j in (1..=cur_max_len).rev() {
                if self.vocab.contains_key(&sample[i..i+j]) {
                    let token: Vec<u8> = sample[i..i+j].to_vec();
                    greedy_tokenization.push(token);
                    i += j;
                    break
                }
            }
        }

        greedy_tokenization
    }

    // do an optimal PathPiece encoding of the sample text,
    // using the smallest possible number of tokens
    fn opt_encode_rust_random_tiebreaker(&self, sample: &[u8]) -> Vec<Vec<u8>> {
        let n: usize = sample.len();
        let mut widths: Vec<usize> = vec![0; n];
        let mut path_length: Vec<usize> = vec![usize::MAX; n];
        let mut solution_count: Vec<usize> = vec![0; n];
        let mut rng = rand::rngs::StdRng::seed_from_u64(SEED);

        for end in 0..n {
            for width in 1..=self.max_len.min(end + 1) {
                let start = 1 + end - width;

                // If vocab contains token
                if self.vocab.contains_key(&sample[start..=end]) {
                    if start == 0 {
                        widths[end] = width;
                        path_length[end] = 1;
                        solution_count[end] = 1;
                    } else {
                        if path_length[start - 1] + 1 == path_length[end] {
                            solution_count[end] += 1;

                            // reservoir sampling:
                            // * 1 has a 1/solution_count chance of being chosen as the result of gen_range
                            // * if there's 2 potential solutions, choose the second one with 1/2 probability
                            // * if there's 3 potential solutions, choose the 3rd one with 1/3 probability, and one of the first two with 2/3 probability (so each of the first 2 has 1/3 chance as well)
                            // * continue by induction: every potential width has an equal chance of being chosen
                            if rng.gen_range(1..=solution_count[end]) == 1 {
                                widths[end] = width
                            }
                        } else if path_length[start - 1] + 1 < path_length[end] {
                            widths[end] = width;
                            path_length[end] = path_length[start - 1] + 1;
                            solution_count[end] = 1;
                        }
                    }
                }
            }
        }

        let mut optimal_tokenization: Vec<Vec<u8>> = Vec::new();
        let mut end = n - 1;

        loop {
            let width = widths[end];
            let start = 1 + end - width;
            let token: Vec<u8> = sample[start..=end].to_vec();

            optimal_tokenization.push(token);
            if end < width {
                break;
            }
            end -= width;
        }
        optimal_tokenization.reverse();

        return optimal_tokenization;
    }

    // Breaks ties between widths in non-unique optimal tokenization by choosing the longest width
    fn opt_encode_rust_longest_tiebreaker(&self, sample: &[u8]) -> Vec<Vec<u8>> {
        let n: usize = sample.len();
        let mut widths: Vec<usize> = vec![0; n];
        let mut path_length: Vec<usize> = vec![usize::MAX; n];
        let mut solution_count: Vec<usize> = vec![0; n];

        for end in 0..n {
            for width in 1..=self.max_len.min(end + 1) {
                let start = 1 + end - width;
                // If vocab contains token
                if self.vocab.contains_key(&sample[start..=end]) {
                    if start == 0 {
                        widths[end] = width;
                        path_length[end] = 1;
                        solution_count[end] = 1;
                    } else {
                        if path_length[start - 1] + 1 == path_length[end] {
                            solution_count[end] += 1;
                            widths[end] = width;
                        } else if path_length[start - 1] + 1 < path_length[end] {
                            widths[end] = width;
                            path_length[end] = path_length[start - 1] + 1;
                            solution_count[end] = 1;
                        }
                    }
                }
            }
        }

        let mut optimal_tokenization: Vec<Vec<u8>> = Vec::new();
        let mut end = n - 1;

        loop {
            let width = widths[end];
            let start = 1 + end - width;
            let token: Vec<u8> = sample[start..=end].to_vec();

            optimal_tokenization.push(token);
            if end < width {
                break;
            }
            end -= width;
        }
        optimal_tokenization.reverse();

        return optimal_tokenization;

    }


}
