{
  description = "👻";

  inputs = {
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    # We want to stay as up to date as possible but need to be careful that the
    # glibc versions used by our dependencies from Nix are compatible with the
    # system glibc that the user is building for.
    nixpkgs-stable.url = "github:nixos/nixpkgs/release-23.05";

    # This is a nixpkgs mirror (based off of master) that contains
    # patches for Zig 0.12 (master/nightly).
    #
    # This gives an up-to-date Zig that contains the nixpkgs patches,
    # specifically the ones relating to NativeTargetInfo
    # (https://github.com/ziglang/zig/issues/15898) in addition to the base
    # hooks. This is used in the package (i.e. packages.ghostty, not the
    # devShell) to build a Zig that can be included in a NixOS configuration.
    nixpkgs-zig-0-12.url = "github:vancluever/nixpkgs/vancluever-zig-0-12";

    zig = {
      url = "github:mitchellh/zig-overlay";
      inputs.nixpkgs.follows = "nixpkgs-stable";
    };

    zls = {
      url = "github:zigtools/zls/master";
      inputs.nixpkgs.follows = "nixpkgs-stable";
    };
  };

  outputs = {
    self,
    nixpkgs-unstable,
    nixpkgs-stable,
    nixpkgs-zig-0-12,
    zig,
    zls,
    ...
  }:
    builtins.foldl' nixpkgs-stable.lib.recursiveUpdate {} (builtins.map (system: let
      pkgs-stable = nixpkgs-stable.legacyPackages.${system};
      pkgs-unstable = nixpkgs-unstable.legacyPackages.${system};
      pkgs-zig-0-12 = nixpkgs-zig-0-12.legacyPackages.${system};
    in {
      devShell.${system} = pkgs-stable.callPackage ./shell.nix {
        inherit (zls.packages.${system}) zls;
        zig = zig.packages.${system}.master;
      };
    }) (builtins.attrNames zig.packages));
}
