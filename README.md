# Efficient File Search (EFS)

[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Cross Platform](https://img.shields.io/badge/cross-platform-orange)




EFS is a high-performance, multithreaded file search utility written in Rust. It's designed to quickly scan large directory structures and locate files based on various criteria. 

This came from Windows search being too slow in our petabyte scale file share. 

## Implementation

This concurrent file search program utilizes several key components:

### Data Structures

1. **VecDeque**: Used for the directory stack, allowing efficient push and pop operations from both ends.
2. **Vec**: Stores the found files matching the target criteria.
3. **PathBuf**: Represents file and directory paths throughout the program.

### SearchState Struct

The `SearchState` struct encapsulates the shared state across threads:

```rust
struct SearchState {
    directory_stack: Arc<Mutex<VecDeque<PathBuf>>>,
    found_files: Arc<Mutex<Vec<PathBuf>>>,
    target_file: String,
}```

## Preformance Benchmarking 

| Dir Size (GB) | Recurrsive (ms) | Concurent (ms) | CI     |
|---------------|-----------------|----------------|--------|
| 3.5           | 273             | 165            | +-40   |
| 113           | 2771            | 1340           | +- 150 |
| 250           | 165890          | 91348          | +-350  |

## Features

- 🚀 **Fast**: Utilizes multithreading for parallel directory traversal
- 🌳 **Efficient**: Implements a tree-based search algorithm
- 💻 **Cross-platform**: Works on Windows, macOS, and Linux

## Installation

Ensure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/).

Then, clone this repository and build the project:

```bash
git clone https://github.com/yen936/file_sys_search.git
cd file-system-search
cargo build --release
```

The compiled binary will be available in target/.

## Usage
Run the program with
```bash
cd file_sys_search
touch search_dirs.txt
echo "<your-target-file>" >> search_dirs.txt
cargo run

```

## Performance

EFS is designed to be significantly faster than traditional single-threaded search utilities, especially on systems with multiple CPU cores and when searching across large directory structures or network-attached storage.

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## License
This project is licensed under the MIT License - see the LICENSE file for details.


## TODO
 - Implement regex support for file name matching
 - Add support for content-based search
 - Create a simple GUI interface


Made with ❤️ and Rust