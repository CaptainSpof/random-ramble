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
        pname = "rr";
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustc-version = "latest";
        rust-linux = pkgs.rust-bin.nightly.${rustc-version}.default;
        # Override the version used in naersk
        naersk-lib = naersk.lib."${system}".override {
          cargo = rust-linux;
          rustc = rust-linux;
        };
      in rec {

        # `nix build`
        packages.random-ramble = naersk-lib.buildPackage {
          inherit pname;
          root = ./.;
        };
        defaultPackage = packages.random-ramble;

        packages.dockerImage = pkgs.dockerTools.buildLayeredImage {
          name = "${pname}";
          contents = [ packages.random-ramble ];
          config.Entrypoint = [ "${pname}" ];
        };
        # defaultPackage = packages.random-ramble;

        # `nix run`
        apps.random-ramble = flake-utils.lib.mkApp {
          drv = packages.random-ramble;
        };
        defaultApp = apps.random-ramble;


        # `nix develop`
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            # dev
            rust-analyzer  # rust lsp
            cargo-outdated # show outdated rust deps
            cargo-edit     # add, remove deps from the command line
            # build
            act            # run github actions locally
            glibc
            openssl
            pkgconfig
            rust-linux
            # nawak
            lolcat
            figlet
          ];

          shellHook = ''
              figlet "${pname}" -f $(showfigfonts | rg '(\w+) :' -r '$1' | shuf -n 1) | lolcat
              [ ! -f ./target/debug/${pname} ] && cargo build ; ln -sf ./target/debug/${pname} rr
            '';

          # docker = hostPkgs.dockerTools.streamLayeredImage {
          #   name = "${pname}";
          #   contents =
          # };
        };
      });
}
