{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells = {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              bacon
              cargo-audit
              cargo-tarpaulin
              cargo-watch
              clang
              evcxr
              llvmPackages.bintools
              nodePackages.yaml-language-server
              openssl
              pkg-config
              podman
              podman-compose
              rnix-lsp
              (rust-bin.stable.latest.default.override {
                extensions = [ "rust-analyzer" "rust-src" ];
              })
              sqlfluff
              sqlx-cli
              taplo
            ];

            shellHook = ''
              # Import environment variables
              eval "$(grep -v '^#' ./.env | xargs)"

              export DATABASE_URL="postgres://''${DB_USER}:''${DB_PASSWORD}@localhost:''${DB_PORT}/''${DB_NAME}"
            '';
          };

          nightly = pkgs.mkShell {
            buildInputs = with pkgs; [
              cargo-expand
              (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
            ];
          };
        };
      }
    );
}
