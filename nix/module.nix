{ config, pkgs, lib, utils, ... }: with lib; let
    cfg = config.services.savesyncd;

    defaultPackage = pkgs.callPackage ./package.nix {};

    defaultPort = 8000;
    defaultDataDirectory = "/var/lib/SaveSyncd";

    settingsFormat = pkgs.formats.json { };
    configFile = settingsFormat.generate "config.json" cfg.settings;
in {
    options.services.savesyncd = with types; {
        enable = mkEnableOption "SaveSyncd, a self-hosted server for backing up & restoring saves on the 3DS";
        package = mkOption {
            type = package;
            default = defaultPackage;
        };

        openFirewall = mkOption {
            type = bool;
            default = false;
            description = ''
                Whether to automatically open ports in the firewall.
            '';
        };

        autoStart = mkOption {
            type = bool;
            default = true;
            description = ''
                Whether savesyncd should be started automatically.
            '';
        };

        settings = mkOption {
            default = {};
            type = submodule (settings: {        
                options = {
                    port = mkOption {
                        description = "Port to listen on.";
                        type = port;
                        default = defaultPort;
                    };
                    
                    data_directory = mkOption {
                        description = "Path for save data to be stored.";
                        type = path;
                        default = defaultDataDirectory;
                    };
                };
            });
        };
    };

    config = mkIf cfg.enable {
        environment.systemPackages = [ cfg.package ];
        networking.firewall.allowedTCPPorts = mkIf cfg.openFirewall [ cfg.settings.port ];

        systemd.services.savesyncd = {
            description = "Self-hosted server for backing up & restoring saves on the 3DS";
            wantedBy = mkIf cfg.autoStart [ "multi-user.target" ];

            startLimitIntervalSec = 500;
            startLimitBurst = 5;

            environment.PATH = lib.mkForce null;
            serviceConfig = {
                ExecStart = utils.escapeSystemdExecArgs (
                    [
                        "${getExe cfg.package}"
                        "${configFile}"
                    ]
                );

                Restart = "on-failure";
                RestartSec = "5s";
            };
        };
    };
}