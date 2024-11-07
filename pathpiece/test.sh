# source .venv/bin/activate
maturin develop
python3 -ic "import pathpiece as p; t = p.Tokenizer('data/vdump_32768.vocab'); c = ['hi', 'bye', 'fly', 'pie lie sigh']"
