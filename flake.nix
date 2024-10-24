{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {
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
        lib = pkgs.lib;
      in {
        devShells.default = pkgs.mkShell {
          nativebuildInputs = with pkgs; [
            pkg-config
          ];
          buildInputs =
            []
            ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs; [
              libiconv
              darwin.apple_sdk_12_3.frameworks.Foundation
              darwin.apple_sdk_12_3.frameworks.Network
              darwin.apple_sdk_12_3.frameworks.CoreWLAN
              darwin.apple_sdk_12_3.frameworks.CoreLocation
            ]);
          packages = with pkgs; [
            dig
          ];
        };
      }
    );
}
