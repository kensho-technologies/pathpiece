
# The PathPiece tokenizer

This repository contains the PathPiece tokenizer described in  the paper [Tokenization Is More Than Compression](https://arxiv.org/abs/2402.18376) by Craig W. Schmidt, Varshini Reddy, Haoran Zhang, Alec Alameddine, Omri Uzan, Yuval Pinter, Chris Tanner.  This paper will be presented at [EMNLP 2024](https://2024.emnlp.org/).

Please direct any questions about this repo to <tokenization-maintainer@kensho.com>.

These files are intended to stimulate academic research, but are not being actively developed at Kensho any longer.  

### Abstract of the paper

Tokenization is a foundational step in natural language processing (NLP) tasks, bridging raw text and language models. Existing tokenization approaches like Byte-Pair Encoding (BPE) originate from the field of data compression, and it has been suggested that the effectiveness of BPE stems from its ability to condense text into a relatively small number of tokens. We test the hypothesis that fewer tokens lead to better downstream performance by introducing PathPiece, a new tokenizer that segments a document's text into the minimum number of tokens for a given vocabulary. Through extensive experimentation we find this hypothesis not to be the case, casting doubt on the understanding of the reasons for effective tokenization. To examine which other factors play a role, we evaluate design decisions across all three phases of tokenization: pre-tokenization, vocabulary construction, and segmentation, offering new insights into the design of effective tokenizers. Specifically, we illustrate the importance of pre-tokenization and the benefits of using BPE to initialize vocabulary construction. We train 64 language models with varying tokenization, ranging in size from 350M to 2.4B parameters, all of which are made publicly available.

### Usage

PathPiece is written in Rust, with a Python interface using the `maturin` package.
You will need to install Rust, and setup a python virtual environment with other python dependencies.

Run `bash run.sh` to setup your machine with Rust and build a python virtual environment.
In subsequent use, you'll want to activate it again with `source .venv/bin/activate`.

Then you will need to build the PathPiece library.  Go to the `pathpiece` directory inside this one, and follow the instructions in `README.md` to do this.  

Then `python main.py` will run some tests, if you have successfully installed PathPiece.

The file `PathPieceUsage.ipynb` runs through how to use PathPiece within python

### Important warning

PathPiece automatically chooses the max token length `L` by looking at the longest token in the vocabulary.
If you pass in a vocabulary from BPE, it will likely have some tokens that are very long runs of spaces.
The complexity of tokenizing is `O(n*L)`, so having a large value of `L` will cause extreme slowdowns.
You sould ensure your vocabulary has a max token length of about 16. 

### License

This repository is licensed under the AI Pubs Open Rail-S license.
To view a copy of this license, visit <https://www.licenses.ai/ai-pubs-open-rails-vz1>

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

© 2023-present Kensho Technologies, LLC. The present date is determined by the timestamp of the most recent commit in the repository.
