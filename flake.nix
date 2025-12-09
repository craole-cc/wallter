{
  description = "Wallter - Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays self;
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            rust-bin.nightly.latest.default

            # OpenSSL and dependencies
            openssl
            pkg-config
            wayland # Includes the necessary runtime library: libwayland-client.so.0
            libxkbcommon # Essential for keyboard input on Wayland/X11

            # Add X11 libraries for winit fallback, often required
            xorg.libX11

            # Desktop environment tools for color/wallpaper management (Optional but recommended for 'wallter')
            gnome.gsettings-desktop-schemas # For GNOME
            kdePackages.plasma-framework # Provides plasma-apply-colorscheme for KDE

            # Additional useful tools
            cargo-watch
            rust-analyzer
            treefmt
            taplo
            deno
          ];

          # Environment variables for OpenSSL (These are correct and should remain)
          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

          # Optional: Help winit find the libraries at runtime on NixOS
          LD_LIBRARY_PATH = "${pkgs.libxkbcommon}/lib:${pkgs.wayland}/lib:$LD_LIBRARY_PATH";
        };
      }
    );
}
