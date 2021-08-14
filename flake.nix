{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pname = "rr";
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustc-version = "latest";
        rust-linux = pkgs.rust-bin.stable.${rustc-version}.default;
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            # dev
            rust-analyzer  # rust lsp
            cargo-outdated # show outdated rust deps
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
        };
      });
}
