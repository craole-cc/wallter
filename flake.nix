{
  description = "Wallter - Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays self;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          # Reduce the build inputs to strict build-time necessities
          buildInputs = with pkgs; [
            # Rust toolchain
            # rust-bin.nightly.latest.default
            (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)

            # --- Linker Fixes (Clang + Mold) ---
            clang
            mold

            # --- Libraries required for LINKING ---
            openssl
            pkg-config
            wayland # Required for libwayland-client.so
            libxkbcommon # Required for keyboard support
            xorg.libX11 # Required for Winit X11 fallback

            # Dev tools
            cargo-watch
            rust-analyzer
            clippy
          ];

          # --- Environment Variables ---

          # 1. Force Rust to use Clang and Mold (Fixes "Argument list too long" during linking)
          RUSTFLAGS = "-C linker=clang -C link-arg=-fuse-ld=mold";

          # 2. OpenSSL Setup
          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

          # 3. Runtime Library Path (Fixes "NoWaylandLib" errors)
          LD_LIBRARY_PATH = "${pkgs.libxkbcommon}/lib:${pkgs.wayland}/lib:${pkgs.xorg.libX11}/lib:$LD_LIBRARY_PATH";
        };
      }
    );
}
