// Copyright 2023-present Kensho Technologies, LLC.

pub mod globals {
    pub const SEED: u64 = 42;

    //use num_format::Locale;
    // pub const LOCALE: &Locale = &Locale::en;

    // pub fn get_special() -> Vec<u8>{
    //     "<|endoftext|>".as_bytes().to_vec()
    // }
}

// pub mod test {
//     use std::collections::HashSet;
//
//     pub const MAX_WIDTH: usize = 6;
//
//     pub fn get_vocab() -> HashSet<Vec<u8>> {
//         let test_vocab_array: [&[u8]; 52] = [
//             b"T", b"h", b"e", b" ", b"q", b"u", b"i", b"c", b"k", b"b", b"r", b"o", b"w", b"n", b"f",
//             b"o", b"x", b"Th", b"he", b"e ", b" q", b"qu", b"ui", b"ic", b"ck", b"k ", b" b", b"br", b"ro", b"ow",
//             b"wn", b" f", b"fo", b"ox", b"The", b"he ", b"e ", b"qui", b"ick", b"ck ", b"bro", b"row", b"own",
//             b" fo", b"The ", b"ick ", b" bro", b"rown", b"own ", b"quick", b"rown ", b"quick "
//         ];
//         let test_vocab_vector: Vec<Vec<u8>> = test_vocab_array.iter().map(|bytearray: &&[u8]| bytearray.to_vec()).collect();
//         return HashSet::from_iter(test_vocab_vector.into_iter())
//     }
//
//     pub fn get_corpus() -> Vec<Vec<u8>> {
//         let bytes: &[u8] = b"The quick brown fox";
//         let corpus: Vec<Vec<u8>> = vec![Vec::from(bytes)];
//         return corpus
//
//     }
// }

pub mod read {
    use std::{collections::{HashMap}, fs::File, io::{BufReader, BufRead}};

    pub fn read_vocab(file_path: &str, special: Vec<u8>) -> HashMap<Vec<u8>, usize> {
        let file = File::open(file_path).unwrap_or_else(|_| panic!("Error opening vocab file"));
        let reader = BufReader::new(file);

        let mut vocab = HashMap::new();
        vocab.insert(special.clone(), 0);

        let mut count = 1;

        for line in reader.lines() {
            let line_content = line.unwrap_or_else(|_| panic!("Error reading vocab bytestring"));
            let byte_string = hex_string_to_byte_vec(&line_content);

            // Don't add special as another ID
            if byte_string == special {
                continue;
            }

            vocab.insert(byte_string, count);
            count += 1;
        }

        vocab
    }

    fn hex_string_to_byte_vec(hex_string: &str) -> Vec<u8> {
        hex::decode(hex_string).unwrap_or_else(|e| panic!("Error decoding hexadecimal string: {:?}", e)) // Panic on decode error
    }

    // pub fn read_corpus(filename: &str) -> Vec<Vec<u8>> {
    //     let file = File::open(filename).unwrap();
    //     let mut reader = csv::ReaderBuilder::new()
    //         .has_headers(true)
    //         .from_reader(file);
    //
    //     let mut data: Vec<Vec<u8>> = Vec::new();
    //
    //     for result in reader.records() {
    //         let record = result.unwrap();
    //         let string = record.get(0).unwrap();
    //         let bytes_vector: Vec<u8> = string.as_bytes().to_vec();
    //         data.push(bytes_vector);
    //     }
    //
    //     return data
    // }


}
