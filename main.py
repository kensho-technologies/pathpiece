import sys
import time
import pandas as pd

import pathpiece

CORPUS = "corpus_tiny.csv"

tokenizer = pathpiece.Tokenizer("pathpiece/data/vdump_32768.vocab")

if CORPUS == "corpus_tiny.csv":
    corpus_len = 5
elif CORPUS == "corpus_train.csv":
    corpus_len = 10 ** 6


def test_load_corpus():
    start_time = time.time()
    df = pd.read_csv(f'pathpiece/data/{CORPUS}')  # excludes header
    corpus = df.iloc[:, 0].tolist()
    print(f"Corpus loaded [{round(time.time() - start_time)}s]")

    try:
        assert len(corpus) == corpus_len
    except AssertionError:
        print(f'{len(corpus)} ≠ {corpus_len}')
        raise

    return corpus


def test_max_len():
    assert tokenizer.get_max_len() == 16
    print(f"Max Len Passed")



def test_encode_decode_inverse(corpus):
    start_time = time.time()
    for i in range(corpus_len):
        assert corpus[i] == tokenizer.decode(tokenizer(corpus[i])["input_ids"])
    print(f"Encode-Decode Passed [{round(time.time() - start_time)}s]")



def test_encode_decode_batch_inverse(corpus):
    start_time = time.time()
    assert corpus == tokenizer.decode_batch(tokenizer(corpus)["input_ids"])
    print(f"Batch Encode-Decode Passed [{round(time.time() - start_time)}s]")


start_time = time.time()
corpus = test_load_corpus()
test_max_len()
test_encode_decode_inverse(corpus)
test_encode_decode_batch_inverse(corpus)
print(f"All tests passed [{round(time.time() - start_time)}s]")
