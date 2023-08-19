{
  description = "TypeSync flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    # Explicitly for pinning rustc/cargo versions (allows nightly)
    rust-overlay.url = "github:oxalica/rust-overlay";

    # Used for eachDefaultSystem function
    flake-utils.url = "github:numtide/flake-utils";

    # 
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }: 
    flake-utils.lib.eachDefaultSystem (system: 
      let 
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in with pkgs; {
        packages.api = pkgs.rustPlatform.buildRustPackage {
          pname = "typesync-api";
          version = "0.1.0";
          cargoLock.lockFile = ./Cargo.lock;
          src = pkgs.lib.cleanSource ./.;
          cargoBuildFlags = "-p api";
          buildInputs = [ ];
          nativeBuildInputs = [
            openssl.dev
            pkgconfig
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
          ];
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
        devShells.default = mkShell {
          name = "typesync-env";
          buildInputs = [ 
            just 
            openssl
            pkgconfig
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
          ];
        };
      }
    );
}
