# Copyright 2023-present Kensho Technologies, LLC.

# maturin build --release --sdist  <- note that this produces a giant file, with object files in it
# cargo clean                      <- do this before sdist, to get rid of files
# maturin sdist                    <- just the source
maturin build --release

