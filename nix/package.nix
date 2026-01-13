{ pkgs, lib, ... }: let 
    manifest = (lib.importTOML ../Cargo.toml).package;
in pkgs.rustPlatform.buildRustPackage rec {
    pname = manifest.name;
    version = manifest.version;
    
    cargoLock.lockFile = ../Cargo.lock;
    src = lib.cleanSource ../.;

    nativeBuildInputs = with pkgs; [
        makeWrapper
        pkg-config
    ];

    buildInputs = with pkgs; [
        gtk3
        libappindicator-gtk3
        xdotool
    ];

    postFixup = ''
        wrapProgram $out/bin/${pname} \
            --set LD_LIBRARY_PATH "${lib.makeLibraryPath buildInputs}"
    '';

    meta = with lib; {
        mainProgram = pname;
        description = "Server for a 3DS Save Sync program";
        homepage = "https://github.com/coolguy1842/SaveSyncd/";
        license = licenses.gpl3;
    };
}