{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.11";
  };

  outputs = { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      forAllSystems = nixpkgs.lib.genAttrs systems;

      mkPkgs = system:
        nixpkgs.legacyPackages.${system};
    in
    {
      devShells = forAllSystems (system:
        let
          pkgs = mkPkgs system;
        in {
          default =
            pkgs.mkShell.override {
              stdenv = pkgs.clangStdenv;
            } {
              name = "rawtx-wasm build environment";

              buildInputs = [
                pkgs.rustup
                pkgs.cargo
                pkgs.rustc
                pkgs.wasm-pack
              ]
              # glibc_multi does not exist / make sense on Darwin
              ++ pkgs.lib.optional pkgs.stdenv.isLinux pkgs.glibc_multi;

              shellHook = ''
                echo "Welcome in $name"
              '';

              hardeningDisable = [ "all" ];
            };
        });
    };
}
