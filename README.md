# NOMORE

> BrainF*ck but with my twist. 

NOMORE is an esoteric programming language interpreter inspired by Brainf*ck, written in Rust. It simplifies the syntax to just two instructions:  `MORE` and `NOMORE`.

## 🎯 Features

- **Minimal Syntax**: Only two instructions to learn
- **Simple Execution**: Converts sequences of `MORE` and `NOMORE` into ASCII characters
- **Fast Performance**: Built with Rust for optimal speed
- **Easy to Use**: Simple command-line interface

## 🚀 How It Works

NOMORE uses a simple counting mechanism: 

- `MORE` increments a counter
- `NOMORE` terminates the current sequence and converts the count to an ASCII character

For example:
```nomore
MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE NOMORE
```

The above counts 72 `MORE` instructions, which is terminated by `NOMORE` and outputs the ASCII character `H` (ASCII value 72).

## 📦 Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 

### Build from Source

```bash
# Clone the repository
git clone https://github.com/thehaidarbahzi/NOMORE.git
cd NOMORE

# Build the project
cargo build --release

# The binary will be located at target/release/test-rust-downloader
```

## 💻 Usage

### Run a NOMORE File

```bash
NOMORE run <filename>
```

Example:
```bash
NOMORE run hello.nomore
```

### Display Help

```bash
NOMORE help
```

## 📝 File Format

NOMORE programs must use the `.nomore` file extension. The interpreter is case-insensitive and ignores whitespace. 

### Example Program

Create a file called `hello.nomore`:

```nomore
MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE NOMORE
MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE MORE NOMORE
```

This outputs `Hi` (ASCII 72 and 105).

## 🛠️ Project Structure

```
NOMORE/
├── src/
│   ├── main.rs           # Entry point and CLI argument handling
│   ├── logic.rs          # Module declarations
│   └── logic/
│       ├── compile.rs    # Core interpreter logic
│       └── help.rs       # Help message handlers
├── Cargo.toml            # Project configuration
├── Cargo.lock            # Dependency lock file
└── .gitignore
```

## 🔍 How the Interpreter Works

1. **File Validation**: Checks if the file has a `.nomore` extension
2. **Tokenization**: Reads the file and extracts `MORE` and `NOMORE` tokens (case-insensitive)
3. **Counting**: Counts consecutive `MORE` instructions until a `NOMORE` is encountered
4. **Conversion**: Converts each count to its corresponding ASCII character
5. **Output**: Prints the resulting string

## 🤝 Contributing

Contributions are welcome! Feel free to: 

- Report bugs
- Suggest new features
- Submit pull requests

## 📄 License

This project is open source and available under no specific license.

## 🔗 Links

- [Repository](https://github.com/thehaidarbahzi/NOMORE)
- [Issues](https://github.com/thehaidarbahzi/NOMORE/issues)

## 👤 Author

**Haidar Bahzi** - [@thehaidarbahzi](https://github.com/thehaidarbahzi)

---

*Created as a fun twist on the classic BrainF*ck esoteric programming language.*
