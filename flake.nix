{
  description = "Development shell for the 'wallter' Rust project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { nixpkgs }:
    {
      devShells.x86_64-linux.default =
        let
          pkgs = nixpkgs.legacyPackages.x86_64-linux;
        in
        pkgs.mkShell {
          # The packages available in the development environment
          buildInputs = with pkgs; [
            # Rust toolchain is usually already installed, but defining it is safe
            rustc
            cargo
            rustfmt
            clippy

            # Essential dependencies for building Rust projects that use system libs
            pkg-config
            openssl
          ];

          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

          # Optional: Set a prompt for the shell for clarity
          shellHook = ''
            echo "Entering Nix-managed development shell. OpenSSL is available."
          '';
        };
    };
}
