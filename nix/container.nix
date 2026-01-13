{ config, pkgs, lib, ... }: {
    imports = [ ./module.nix ];

    boot.enableContainers = false;
    boot.isContainer = true;
    boot.isNspawnContainer = true;
    networking.hostName = "savesyncd-container-test";
    nixpkgs.hostPlatform = "x86_64-linux";
    networking.useDHCP = false;
    system.stateVersion = "25.11";

    services.savesyncd = {
        enable = true;
        openFirewall = true;

        settings.port = 8005;
        settings.data_directory = "/root/data";
    };
}