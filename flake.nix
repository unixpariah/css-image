{
  description = "CSS image rendering library";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };
        rustEnv = pkgs.mkShell {
          buildInputs = with pkgs; [
            pkg-config
            cairo
            rustfmt
            rust-analyzer
            cargo
            rustup
            rustc
          ];
        };
      in {
        devShell = rustEnv;
        packages = {
          ssb = pkgs.stdenv.mkDerivation {
            name = "css-image";
            src = ./.;
            buildInputs = with pkgs; [rustc cargo];
            buildPhase = ''
              cargo build --release
            '';
            installPhase = ''
              mkdir -p $out/bin
              cp target/release/css-image $out/bin/
            '';
          };
        };
      }
    );
}
