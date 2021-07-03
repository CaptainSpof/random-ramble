{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
      pkgBuildInputs = with pkgs; [
        rustc
        cargo
        openssl
        pkg-config
      ];
    in rec {
      # `nix build`
      packages.random-ramble = naersk-lib.buildPackage {
        pname = "random-ramble";
        root = ./.;
        nativeBuildInputs = pkgBuildInputs;
      };
      defaultPackage = packages.random-ramble;

      # `nix run`
      apps.random-ramble = utils.lib.mkApp {
        drv = packages.random-ramble;
      };
      defaultApp = apps.random-ramble;

      # `nix develop`
      devShell = pkgs.mkShell {
        nativeBuildInputs = pkgBuildInputs ++ (with pkgs; [
          rust-analyzer
          cargo-outdated
        ]);
      };
    });
}
