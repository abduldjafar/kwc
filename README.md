## Koteka WC (kwc)

**Koteka WC (kwc)** is a high-performance text processing tool inspired by the `wc` command in Linux, created by a developer from Papua. The name "kwc" combines "koteka," a traditional fashion item from Papua, and "wc" from the Linux command, symbolizing the blend of cultural heritage and technological functionality.

## Overview

**kwc** is designed for tasks such as:

- Word count
- Line count
- Word frequency map generation

## Performance

- For a 35 GB file:
  - The `wc` command takes **1 minute 46 seconds** for word count and line count.
  - **kwc** takes **45 seconds** for word count and line count.
  - For word count, map count, and line count, **kwc** takes **104 seconds**.

## Features

- **Word Count:** Quickly counts the number of words in a file.
- **Line Count:** Efficiently counts the number of lines in a file.
- **Word Frequency Map:** Generates a frequency map of all words in a file.

## Machine Details For Testing

- **OS:** MacOS (M2 Pro)
- **CPU Core:** 10
- **RAM:** 16 GB

## Motivation

This tool is designed to provide high performance and efficiency for text processing tasks. It is developed as part of a learning journey with Rust, and is shared in the hope that others may find it useful.

## Installation

To build and run **kwc**, you will need to have Rust installed. Follow these steps:

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/kwc.git
    cd kwc
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

3. Run the tool:
    ```sh
    ./target/release/kwc <options> <file>
    ```

## Usage

**kwc** supports the following options:

- `-w` or `--word-count`: Count the number of words.
- `-l` or `--line-count`: Count the number of lines.
- `-m` or `--map-count`: Generate a word frequency map.

Example usage:

```sh
./kwc --word-count --line-count <file>
```

## Contributing

Feel free to open issues or submit pull requests if you have suggestions or improvements. Contributions are welcome!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Feel free to adjust any sections as needed!

---
*Note: Koteka is a traditional fashion item from Papua, Indonesia, symbolizing the cultural heritage of the Papuan people.*
