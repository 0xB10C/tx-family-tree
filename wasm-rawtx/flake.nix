{
  inputs = { nixpkgs.url = "github:nixos/nixpkgs/23.11"; };

  outputs = { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux.pkgs;
    in {
      devShells.x86_64-linux.default = pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } {
        name = "rawtx-wasm build environment";
        buildInputs = [
          pkgs.rustup
          pkgs.wasm-pack
          pkgs.glibc_multi
        ];
        shellHook = ''
          echo "Welcome in $name"
        '';
        hardeningDisable = [ "fortify" "stackprotector"];
      };
    };
}
