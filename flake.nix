{
  description = "Simple flake for Rust + Iced on Darwin";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: let
    system = "aarch64-darwin";
    pkgs = import nixpkgs { inherit system; };
  in
  {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = [
        pkgs.rustup
        pkgs.pkg-config
        pkgs.cmake
        pkgs.zlib
        pkgs.nodejs
      ];

      shellHook = ''
        export RUSTUP_HOME=$HOME/.rustup
        export CARGO_HOME=$HOME/.cargo
        export PATH=$CARGO_HOME/bin:$PATH

        if ! command -v iced &>/dev/null; then
          cargo install --locked iced_cli || true
        fi
      '';
    };
  };
}