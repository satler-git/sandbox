{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    aeneas.url = "github:aeneasverif/aeneas";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      aeneas,
      rust-overlay,
    }:
    let
      pkgs = import nixpkgs {
        system = "x86_64-linux";
        overlays = [
          rust-overlay.overlays.default
        ];
      };

      rust-bin = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        buildInputs = [
          aeneas.packages.${pkgs.system}.charon
          aeneas.packages.${pkgs.system}.default

          rust-bin
        ];

        shellHook = ''
          ln -sfn "${aeneas}" "./.aeneas" || true
        '';
      };
    };
}
