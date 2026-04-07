# Contributing to ig_dl

Thank you for your interest in contributing to `ig_dl`! We welcome all contributions, including bug reports, feature requests, documentation improvements, and code changes.

## How to Contribute

### Reporting Bugs
If you find a bug or run into issues while using `ig_dl`, please open an issue in the repository. Provide as much context as possible, including:
- Your operating system.
- The version of `ig_dl` and `gallery-dl` you are using.
- The exact command you ran.
- The error output or unexpected behavior.

### Suggesting Enhancements
Have an idea for a new feature? Feel free to open an issue to propose it. Describe how the feature would work and why it would be beneficial to the project.

### Submitting Pull Requests
1. **Fork the repository** on GitHub.
2. **Clone your fork** locally: `git clone https://github.com/<your-username>/ig_dl.git`
3. **Create a new branch** for your feature or bugfix: `git checkout -b feature/my-awesome-feature`
4. **Make your changes**. Ensure your code follows idiomatic Rust style and practices.
5. **Test your changes**: Build the project using `cargo build` and test it against public Instagram accounts.
6. **Commit your changes**: Write clear, concise commit messages. (Sign your commits if possible).
7. **Push your branch** to your fork: `git push origin feature/my-awesome-feature`
8. **Open a Pull Request** against the `main` branch of the upstream repository.

## Development Environment Setup
To set up your development environment, ensure you have Rust installed via `rustup`. `gallery-dl` must also be installed and accessible in your system's PATH.

Run the tests locally (if any) and verify compilation before opening a pull request:
```bash
cargo check
cargo build
```

We appreciate your help in making `ig_dl` a more robust tool!
