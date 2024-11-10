#!/bin/bash
# Copyright 2023-present Kensho Technologies, LLC.

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
# make a .venv
python -m venv .venv
source .env/bin/activate
pip install -r requirements.txt

#cd pathpiece
# for tar.gz
#maturin develop
# for wheel
# maturin build
#
#cd ..
# python main.py