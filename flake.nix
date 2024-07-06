{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    zig-overlay = {
      url = "github:mitchellh/zig-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    zls-overlay = {
      url = "github:zigtools/zls/master";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = {
    self,
    nixpkgs,
    zig-overlay,
    zls-overlay,
    flake-utils,
    ...
  }: let
    overlays = [
        (final: prev: {
        zigpkgs = zig-overlay.packages.${prev.system};
        zlspkgs = zls-overlay.packages.${prev.system};
        })
    ];
  in

  flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in {
      devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
              go
              gopls
              zigpkgs."0.13.0"
              zls
          ];
      };

      formatter.${system} = pkgs.alejandra;
    }
  );
}
