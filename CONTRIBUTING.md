# Contributing to Calibrated

First off, thank you for considering contributing.
Whether it’s bug reports, feature requests, or code, your help makes this project better for everyone.

---

## Getting Started

### Recommended Development Setup

This project uses [Rust](https://www.rust-lang.org/) and the [Iced](https://github.com/iced-rs/iced) UI library.  
We **strongly recommend using [Nix](https://nixos.org/) for development** to ensure a consistent environment across all contributors.

1. Install Nix following the instructions on the [NixOS website](https://nixos.org/download.html).
2. Clone the repository:
   ```bash
   git clone https://github.com/playfairs/calibrated.git
   cd calibrated
   ```
3. Enter the development shell:
   ```bash
   nix develop
   ```
   This will provide all the required dependencies, including Rust and Cargo.

4. Build and run the project:
   ```bash
   cargo run
   ```

> Using Nix ensures that everyone is on the same Rust version, library versions, and system dependencies, avoiding the “it works on my machine” issues.

---

## Contributing Guidelines

- **Bug Reports & Feature Requests**  
  Open an [issue](https://github.com/playfairs/calibrated/issues) and provide as much detail as possible, including steps to reproduce, screenshots, and your environment.

- **Pull Requests**  
  - Fork the repository.
  - Create a branch for your feature or bugfix:
    ```bash
    git checkout -b feature/or-bugfix
    ```
  - Make your changes, commit, and push.
  - Open a PR to the `main` branch.
  - Make sure your code builds and passes any tests before submitting.

- **Coding Style**  
  Follow Rust best practices and use `cargo fmt` for formatting.  
  Keep commits small and focused when possible.

---

## Need Help?

If you’re unsure about something, open an issue or reach out in discussions.  
We’re happy to help new contributors get set up.

---

Thanks again for helping improve Calibrated. 💜