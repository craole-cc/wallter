# Wallter Project Plan: Roadmap to Completion

This document outlines a phased approach to building the `wallter` application, focusing on achieving a functional MVP first, then expanding with additional features, including planned GUI and web interfaces, and advanced image alteration capabilities.

## Phase 1: Minimal Viable Product (MVP) - WallHaven Integration & Setting

This phase focuses on getting a working application that can fetch wallpapers from WallHaven and set them as your desktop background.

### Milestone 1.1: Project Setup & WallHaven API Integration (Estimated: 4 hours)

* [ ] **Task 1.1.1: Project Initialization (if not already done)**
    - [ ] Create a new Rust project: `cargo new wallter --bin`
    - [ ] Initialize `jj` repository: `jj init`
    - [ ] Create `.gitignore`

* [ ] **Task 1.1.2: Add Core Dependencies**
    - [ ] Add `wallhaven = "0.2"` to `Cargo.toml`
    - [ ] Add `reqwest = { version = "0.11", features = ["json", "blocking"] }` for HTTP requests
    - [ ] Add `tokio = { version = "1", features = ["full"] }` for async operations
    - [ ] Add `clap = { version = "4.0", features = ["derive"] }` for CLI parsing
    - [ ] Add `serde = { version = "1.0", features = ["derive"] }` and `toml = "0.7"` for config handling
    - [ ] Add `dirs = "5.0"` for platform-agnostic config/data directories
    - [ ] Add `anyhow = "1.0"` for simplified error handling
    - [ ] Add `wallpaper = "4.0"` (or similar cross-platform crate for setting wallpaper)

* [ ] **Task 1.1.3: Basic Configuration Loading**
    - [ ] Define a `Config` struct using `serde` to parse `config.toml`.
    - [ ] Implement logic to load `~/.config/wallter/config.toml` (or create if not exists with defaults).
    - [ ] Ensure `api_key` and other initial configuration paths are correctly read.

* [ ] **Task 1.1.4: Implement WallHaven Search/Fetch Functionality**
    - [ ] Create a function that uses the `wallhaven` crate to search for wallpapers based on parameters (e.g., random, categories).
    - [ ] Return a list of wallpaper URLs.

### Milestone 1.2: Wallpaper Downloading & Saving (Estimated: 3 hours)

* [ ] **Task 1.2.1: Create Download Directory**
    - [ ] Implement logic to ensure the `downloads_dir` (from config) exists. Create it if it doesn't.

* [ ] **Task 1.2.2: Download Wallpapers**
    - [ ] Write a function that takes a wallpaper URL.
    - [ ] Use `reqwest` to download the image data.
    - [ ] Save the image data to a file within the `downloads_dir` with a unique filename (e.g., using the wallpaper ID or a hash).

* [ ] **Task 1.2.3: Integrate into `wallter download` command**
    - [ ] Hook up the download logic to the `wallter download` CLI command.
    - [ ] Allow it to fetch a few (e.g., 5-10) wallpapers and save them.

### Milestone 1.3: Wallpaper Setting (Estimated: 4 hours)

* [ ] **Task 1.3.1: Implement Wallpaper Setter Function**
    - [ ] Write a function that takes a file path to an image.
    - [ ] Use the `wallpaper` crate (or platform-specific commands like `gsettings` for Gnome, AppleScript for macOS, Registry for Windows) to set the given image as the desktop background.

* [ ] **Task 1.3.2: Integrate into `wallter set` command**
    - [ ] Hook up the wallpaper setting logic to the `wallter set` CLI command.
    - [ ] By default, have it select a *random* image from your `downloads_dir` and set it.
    - [ ] Consider an option to specify a particular image file to set.

### Milestone 1.4: Command Line Interface (CLI) Integration (Estimated: 2 hours)

* [ ] **Task 1.4.1: Define CLI Structure with `clap`**
    - [ ] Define subcommands (`init`, `download`, `set`, `slideshow`, `config`) using `clap`.
    - [ ] Add arguments/options for each subcommand (e.g., `download --count <num>`).

* [ ] **Task 1.4.2: Connect Commands to Functions**
    - [ ] Wire up the `clap` parsing to call the appropriate functions implemented in previous tasks (`init` for config setup, `download` for fetching, `set` for setting).

## Phase 2: Feature Enhancements (Post-MVP)

Once the MVP is solid, you can add these features.

### Milestone 2.1: Local Image Directory Support (Estimated: 3 hours)

* [ ] **Task 2.1.1: Update Configuration**
    - [ ] Clarify and update `image_dir` (or `wallpaper_dir`) in `Config` struct for user-provided local image sources distinct from `downloads_dir`.

* [ ] **Task 2.1.2: Modify `wallter set` to use `image_dir` (or `wallpaper_dir`)**
    - [ ] When `wallter set` is called, check the user-provided `image_dir` (or `wallpaper_dir` if that's where local images are sourced from) first, or provide a flag (`--local`) to prioritize it.
    - [ ] Read image files from the specified local directory.

### Milestone 2.2: Wallpaper Caching & Slideshow (Estimated: 5 hours)

* [ ] **Task 2.2.1: Caching Strategy Refinement**
    - [ ] Ensure downloaded wallpapers are stored efficiently. You've already got the `downloads_dir`.
    - [ ] Implement a system (e.g., simple file naming, or a lightweight database like `sled`/`rusqlite`) to manage metadata about downloaded wallpapers (URL, local path, last set date, etc.) to prevent re-downloading and enable efficient slideshows.

* [ ] **Task 2.2.2: Implement `wallter slideshow`**
    - [ ] Use a loop to continuously change the wallpaper at `slideshow_interval`.
    - [ ] Select images from either `downloads_dir` or user-defined `wallpaper_dir`/`image_dir`.
    - [ ] Implement error handling if no images are found or if settings fail.

### Milestone 2.3: Multi-Source API & Image Alterations (Estimated: 10 hours)

* [ ] **Task 2.3.1: Implement Multi-Source API Support**
    - [ ] Abstract wallpaper source logic to support multiple APIs (e.g., Wallhaven, Unsplash, Pixabay).
    - [ ] Allow configuration for multiple sources and enable shuffling between them.
    - [ ] Implement command-line options to select specific sources or use a randomized approach.

* [ ] **Task 2.3.2: Implement Basic Image Alterations**
    - [ ] Add a new dependency for image processing (e.g., `image` crate).
    - [ ] Implement functions for basic alterations:
      - [ ] Grayscale conversion
      - [ ] Mosaic effect
      - [ ] Adding text overlay (e.g., file path as watermark/caption)

* [ ] **Task 2.3.3: Integrate Alterations into Wallpaper Setting Flow**
    - [ ] Allow users to specify alteration flags/options via CLI when setting a wallpaper.
    - [ ] Apply the chosen alterations to the image *before* setting it as the background.

* [ ] **Task 2.3.4: API Key Management**
    - [ ] Improve how `api_key`s are handled for multiple sources (e.g., environment variable fallback, secure storage).

### Milestone 2.4: Custom Commands, Monitor Support & UI Preparation (Estimated: 6 hours)

* [ ] **Task 2.4.1: Custom Commands (`custom_commands`)**
    - [ ] Implement logic to execute external commands before and after setting the wallpaper.

* [ ] **Task 2.4.2: Multi-Monitor Support (`monitors`)**
    - [ ] Refine how the `wallpaper` crate or platform-specific methods handle multiple monitors based on your detailed config.
    - [ ] Implement logic to set wallpapers per monitor or span across them, based on configuration.

* [ ] **Task 2.4.3: Lay Groundwork for Future UI (GUI/Web)**
    - [ ] Begin structuring core logic into a library (`lib.rs`) that can be easily consumed by separate UI crates.
    - [ ] Consider abstracting CLI-specific interactions where possible to prepare for other interfaces.

## Phase 3: Future UI and Advanced Features (Beyond MVP)

This phase focuses on expanding `wallter`'s reach and capabilities with graphical interfaces and more advanced functionalities.

### Milestone 3.1: Cross-Platform GUI (Planned)

* [ ] **Task 3.1.1: Choose GUI Framework:** Research and select a suitable Rust GUI framework (e.g., Tauri, egui, iced).

* [ ] **Task 3.1.2: Basic GUI Application:** Create a minimal GUI application that can perform basic `wallter` functions (e.g., set random wallpaper).

* [ ] **Task 3.1.3: Integrate Core Logic:** Connect GUI elements to the core wallpaper management logic.

### Milestone 3.2: Web Interface (Planned)

* [ ] **Task 3.2.1: Choose Web Framework:** Research and select a suitable Rust web framework (e.g., Actix, Axum) or WASM setup.

* [ ] **Task 3.2.2: Basic Web Server:** Create a minimal web server that can serve a simple `wallter` interface.

* [ ] **Task 3.2.3: Implement Web UI:** Develop a web-based user interface for `wallter`'s core features.

### Milestone 3.3: Advanced Alterations & Customization (Planned)

* [ ] **Task 3.3.1: More Image Effects:** Add more complex image alteration capabilities (e.g., blurring, sharpening, color adjustments).

* [ ] **Task 3.3.2: Advanced Source Management:** Implement a more robust system for managing and discovering new wallpaper sources.

* [ ] **Task 3.3.3: User Profiles/Themes:** Allow users to save and switch between different configuration profiles or wallpaper themes.
