{
    description = "Rust template";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
        rust-overlay.url = "github:oxalica/rust-overlay";
    };

    outputs = { self, nixpkgs, rust-overlay }:
    let
        system = "x86_64-linux";
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
            inherit system overlays;
        };
        rust-toolchain = pkgs.rust-bin.stable.latest.default;
        #rust-toolchain = pkgs.rust-bin.nightly.latest.default.override {
        #    extensions = [ "rust-src" ];
        #    targets = [ "wasm32-unknown-unknown" ];
        #};
    in {
        devShells.${system}.default = with pkgs; mkShell {
            buildInputs = [
                rust-toolchain
                rust-analyzer
                rustfmt
                cargo-deny
                cargo-watch
                clippy
            ];
            shellHook = ''
            '';
        };
    };
}
