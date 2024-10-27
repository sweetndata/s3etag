# S3 ETag Validator

S3 ETag Validator is a Rust-based tool designed to validate the ETag of a local file against an ETag from Amazon S3. This tool is particularly useful for verifying the integrity of large files that have been uploaded to or downloaded from S3.

## Features

- Supports multi-threaded processing for efficient handling of large files.
- Calculates ETags using MD5 hashing.
- Compatible with common S3 clients like AWS CLI and s3cmd.

## Prerequisites

- Rust (version 1.56 or later)
- Cargo (Rust's package manager)

## Installation

1. **Clone the repository:**

   ```bash
   git clone https://github.com/yourusername/s3-etag-validator.git
   cd s3-etag-validator
   ```

2. **Build the project:**

   ```bash
   cargo build --release
   ```

   This will compile the project and create an executable in the `target/release` directory.

## Usage

To use the S3 ETag Validator, run the following command:

```bash
./target/release/s3etag <inputfile> <etag>
```

- `<inputfile>`: Path to the local file you want to validate.
- `<etag>`: The ETag from S3 that you want to compare against.

## Example

```bash
./target/release/s3etag my_large_file.bin "d41d8cd98f00b204e9800998ecf8427e-3"
```

This command will check if the local file `my_large_file.bin` matches the provided ETag.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any improvements or bug fixes. Make sure to follow the project's coding style and include tests for new features.

## License

This project is open-source and licensed under the BSD 3-Clause License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Rayon](https://github.com/rayon-rs/rayon) for parallel iteration.
- [md5](https://github.com/RustCrypto/hashes) for MD5 hashing.
- [hex](https://github.com/KokaKiwi/rust-hex) for hex encoding.

## Contact

For questions or feedback, please open an issue on the [GitHub repository](https://github.com/yourusername/s3-etag-validator).
