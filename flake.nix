{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    ...
  }:
    utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
      in rec
      {
        devShells.default = pkgs.mkShell {
          nativebuildInputs = with pkgs; [
            pkg-config
          ];
          packages = with pkgs; [
            dig
          ];
        };
      }
    );
}
