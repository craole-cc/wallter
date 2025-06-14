# Contributing to Wallter

We welcome and appreciate all contributions to the **Wallter** project! Whether you're reporting a bug, suggesting an enhancement, improving documentation, or submitting code, your help is invaluable. By contributing, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

## Getting Started

1. **Fork the Repository:** Start by forking the `wallter` repository on GitHub.
2. **Clone Your Fork:** Clone your forked repository to your local machine:

    ```bash
    git clone [https://github.com/craole-cc/wallter.git](https://github.com/craole-cc/wallter.git)
    cd wallter
    ```

3. **Initialize Jujutsu (jj):** If you prefer using `jj` for version control (as the core developers do), you can initialize `jj` within your cloned Git repository:

    ```bash
    jj git init --colocate
    ```

    *Note: This command initializes `jj` and links it to your existing Git repository's history and configuration. While `git` commands will still work for basic operations like cloning and pushing to GitHub, we encourage using `jj` for local development workflow due to its powerful features for history editing and and collaboration.*
4. **Set up Development Environment:**
    * Ensure you have [Rust](https://www.rust-lang.org/tools/install) and Cargo installed.
    * Run `cargo build` to verify everything is set up correctly.

## How to Contribute

### 1. Reporting Bugs

* Before opening a new bug report, please check the [GitHub Issues page](https://github.com/craole-cc/wallter/issues) to see if the issue has already been reported.
* If not, open a new issue.
* Provide a clear and concise description of the bug.
* Include steps to reproduce the issue.
* Mention your operating system, `wallter` version, and Rust version.
* Attach any relevant error messages, logs, or screenshots.

### 2. Suggesting Enhancements / New Features

* Before suggesting a new feature, please check the [GitHub Issues page](https://github.com/craole-cc/wallter/issues) and the [Roadmap](ROADMAP.md) document to see if similar ideas are already being discussed or planned.
* Open a new issue for your proposal.
* Clearly describe the feature and its purpose.
* Explain why you believe it would be a valuable addition to **Wallter**.
* Provide any mockups or examples if applicable.

### 3. Submitting Code Contributions

We welcome pull requests for bug fixes, new features, and improvements!

* **Review Roadmap:** Refer to the [Roadmap](ROADMAP.md) document for an overview of planned features and current development priorities. This can help you find a suitable task or see where your proposed feature fits.
* **Create a Branch/Changeset:**
  * If using `jj`: `jj new -m "feat: brief description of your change"`
  * If using `git`: `git checkout -b feature/your-feature-name`
* **Implement Your Changes:**
  * Write clean, idiomatic Rust code.
  * Adhere to existing coding styles.
  * Ensure your code is well-commented.
  * Consider adding integration and unit tests for your changes.
* **Test Your Changes:**
  * Run existing tests: `cargo test`
  * Add new tests as appropriate for new functionality or bug fixes.
* **Update Documentation:**
  * If your changes impact how **Wallter** is used (e.g., new CLI arguments, configuration options), update the `README.md`.
  * If you're working on a milestone, update its status in `ROADMAP.md` if relevant to your contribution.
* **Commit Your Work:**
  * Write clear and concise commit messages using conventional commits (e.g., `feat: add new feature`, `fix: resolve bug`).
  * In `jj`, your changes are automatically saved as you work. To finalize a changeset for pushing, ensure your working copy is on a new change or that the current change has a clear, complete description (`jj describe`).
* **Open a Pull Request:**
  * Push your branch/changesets to your fork on GitHub.
    * If using `jj`: `jj git push --all` (or specific branches/bookmarks like `jj git push --branch <your-branch-name>`)
    * If using `git`: `git push origin feature/your-feature-name`
  * Open a pull request from your fork to the `main` branch of the `craole-cc/wallter` repository.
  * Provide a detailed description of your changes in the pull request.
  * Reference any related issues (e.g., `Closes #123`).

## Code of Conduct

Please note that all contributors are expected to adhere to our [Code of Conduct](CODE_OF_CONDUCT.md).

Thank you for contributing to **Wallter**!
