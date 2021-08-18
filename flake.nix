{
  description = "RandomRamble: Generate stupid things randomly.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, naersk, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        version = "0.3.0";
        name = "random-ramble";
        pname = "rr";
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustc-version = "latest";
        rust = pkgs.rust-bin.nightly.${rustc-version}.default;
        # Override the version used in naersk
        naersk-lib = naersk.lib."${system}".override {
          cargo = rust;
          rustc = rust;
        };
      in rec {

        # `nix build`
        packages."${name}" = naersk-lib.buildPackage {
          inherit pname;
          root = ./.;
        };
        defaultPackage = packages."${name}";

        # docker image
        packages.dockerImage = pkgs.dockerTools.buildLayeredImage {
          inherit name;
          contents = [ packages."${name}" ];
          config.Entrypoint = [ "${pname}" ];
        };

        # `nix run`
        apps."${name}" = flake-utils.lib.mkApp { drv = packages."${name}"; };
        defaultApp = apps."${name}";

        # `nix develop`
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            # dev
            rust-analyzer # rust lsp
            cargo-outdated # show outdated rust deps
            cargo-edit # add, remove deps from the command line
            # build
            act # run github actions locally
            rust
            # nawak
            lolcat
            figlet
          ];

          shellHook = ''
            figlet "${pname}" -f $(showfigfonts | rg '(\w+) :' -r '$1' | shuf -n 1) | lolcat
            [ ! -f ./target/debug/${pname} ] && cargo build ; ln -sf ./target/debug/${pname} rr
          '';
        };

        checks = {

          rust-fmt = with import nixpkgs { inherit system; };
            stdenv.mkDerivation {
              pname = "${name}-cargo-fmt-check";
              inherit version;

              phases = [ "unpackPhase" "buildPhase" ];

              src = self;

              buildInputs = [ pkgs.rustfmt ];

              buildPhase = ''
                ${rust}/bin/cargo fmt -- --check | tee $out
              '';
            };

        };

      });
}
