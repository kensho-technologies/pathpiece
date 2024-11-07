# Build

PathPiece is written in Rust with a Python wrapper using the `maturin` package.
You need to build the system into a wheel, and the install that into a python
virtural environment.

Run `bash build.sh` to build a wheel for pathpiece.

You can now install pathpiece into an active `.venv` with

```
pip install target/wheels/pathpiece-0.3.0.tar.gz 
```

or whatever wheel it has producted in `target/wheels`.  
For example for a Mac it might be:

```
pip install target/wheels/pathpiece-0.3.0-cp39-cp39-macosx_11_0_arm64.whl 
```

# Usage
Run `bash test.sh` in this directory to enter an interactive python shell with a tokenizer object called `t` already instantiated.

A tiny test corpus called `c` is included as well:
```
c = ['hi', 'bye', 'fly', 'pie lie sigh']
```

At this point you can use PathPiece from within python.  See the `README.md` in the parent directory for more. 