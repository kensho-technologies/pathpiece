// Copyright 2023-present Kensho Technologies, LLC.

#[cfg(test)]
mod tests {
    use crate::Tokenizer;

    // This test was successful when the code was in rust
    // But now it returns python types so the test doesn't make sense
    // #[test]
    // fn test_decode() {
    //     const NUMBER_OF_DOCUMENTS: usize = 100;
        
    //     let tokenizer = Tokenizer::new("data/vdump_32768.vocab");
    //     let corpus_full = tokenizer.load_corpus("data/corpus_train.csv");
    //     // println!("Corpus Loaded");

    //     let corpus_slice: Vec<Vec<u8>> = corpus_full
    //     .into_iter()
    //     .take(NUMBER_OF_DOCUMENTS)
    //     .collect();
    
    //     let optimal_encoding = tokenizer.encode_batch(corpus_slice.clone());
    //     let decoded_corpus_slice = tokenizer.decode(optimal_encoding);
    //     assert_eq!(corpus_slice, decoded_corpus_slice);
    // }
}