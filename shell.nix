{
  mkShell,
  zig,
  zls,
  llvmPackages_latest,
}: let
  llvm = llvmPackages_latest.llvm;
in
mkShell {
  name = "practice";

  buildInputs =
    [
      llvm
      zig
      zls
    ];
}
