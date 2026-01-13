{
    description = "Rocket.rs hosted server for 3DS SaveSync";
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11"; 
    };

    outputs = { self, nixpkgs, systems }: let
        forEachSystem = nixpkgs.lib.genAttrs (import systems); 
    in {
        devShells = forEachSystem (system: let
            pkgs = nixpkgs.legacyPackages.${system};
        in {
            default = pkgs.mkShell rec {
                packages = with pkgs; [
                    rustc
                    cargo
                ];

                nativeBuildInputs = with pkgs; [
                    gcc
                    pkg-config
                    redocly
                    act
                ];

                buildInputs = with pkgs; [
                    gtk3
                    libappindicator-gtk3
                    xdotool
                ];

                LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (buildInputs ++ nativeBuildInputs);
                RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            };
        });
        
        nixosConfigurations.container = nixpkgs.lib.nixosSystem {
            modules = [ ./nix/container.nix ];
        };

        nixosModules = rec {
            savesyncd = (import ./nix/module.nix);
            default = savesyncd;
        };

        packages = forEachSystem (system: let
            pkgs = nixpkgs.legacyPackages.${system};
            manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        in rec {
            savesyncd = pkgs.callPackage ./nix/package.nix {};
            default = savesyncd;
        });
    };
}