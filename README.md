# ig_dl

A lightweight, high-performance command-line tool written in Rust to automatically download all unique posts (videos and images) from a public Instagram account.

## How It Works
Standard headless scrapers are often rate-limited or blocked by Instagram's login walls. `ig_dl` works around this by:
1. Identifying the internal numeric User ID of the target account by mimicking an official Instagram mobile client request.
2. Utilizing the powerful [gallery-dl](https://github.com/mikf/gallery-dl) under the hood to bypass standard frontend restrictions and download directly via the `id:` extraction method.

## Prerequisites
- **Rust**: Ensure you have `cargo` and `rustc` installed.
- **gallery-dl**: `ig_dl` requires `gallery-dl` to be installed and available in your system's PATH.

## Installation
Clone the repository and build the binary:
```bash
git clone https://github.com/ajsb85/ig_dl.git
cd ig_dl
cargo build --release
```
The compiled binary will be available in `target/release/ig_dl`.

## Usage
Simply run the binary and pass the target Instagram username as an argument:
```bash
./ig_dl <username>
```

### Example:
```bash
./ig_dl mnfilmandphotos
```
The downloads will automatically be saved into a structured directory created by `gallery-dl` (e.g., `./gallery-dl/instagram/<username>/`).

## License
This project is licensed under the MIT License.
