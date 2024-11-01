{
  config,
  lib,
  pkgs,
  ...
}:
let
  inherit (lib) getExe mkIf;
  aa-alias-manager = pkgs.callPackage ./package.nix { };
  alias_dir = "/run/aliases.d";
in
{
  options.security.apparmor.aa-alias-manger = with lib; {
    enable =
      (mkEnableOption ''
        apparmor alias manager to alias store entries with their equivalent on regular linux systems.
        Required to load non-NixOS apparmor profiles
      '')
      // {
        default = config.security.apparmor.enable;
      };

    patterns = mkOption {
      description = "aa-alias-manager patterns to alias";

      type = types.listOf (
        types.submodule {
          options = {
            name = mkOption {
              type = types.nonEmptyStr;
              description = "name of the alias file for this pattern";
            };
            target = mkOption {
              type = types.nonEmptyStr;
              description = "target of the aliases.";
              example = "/bin";
            };
            store_suffixes = mkOption {
              type = types.listOf types.nonEmptyStr;
              description = "suffixes after the store path to alias";
              example = [
                "bin"
                "sbin"
              ];
              default = [ ];
            };
            individual = mkOption {
              type = types.bool;
              description = "Whether to alias contents of directories individually. Can help parser performance to avoid too duplicated aliases pointing to the same target";
              default = false;
            };
            only_exe = mkOption {
              type = types.bool;
              description = "Whether to only alias executable files. Useful when aliasing bin paths.";
              default = false;
            };
            disallowed_strings = mkOption {
              type = types.listOf types.nonEmptyStr;
              description = "apparmor parser does not like some symbols. this can be used to disable certain strings.";
              default = [ "!" ];
            };
            only_include = mkOption {
              type = types.listOf types.nonEmptyStr;
              description = "Only include files matching the listed names. Potentially useful wehn wanting to make more specific aliases.";
              default = [ ];
            };
          };
        }
      );

      default = [
        {
          name = "bin";
          target = "/bin";
          store_suffixes = [
            "bin"
            "libexec"
            "sbin"
            "usr/bin"
            "usr/sbin"
          ];
          individual = true;
          only_exe = true;
        }
      ];

      apply = pkgs.writers.writeJSON "aa-alias-manager-patterns.json";
    };
  };

  config = mkIf config.security.apparmor.aa-alias-manger.enable {
    security.apparmor.includes."tunables/alias.d/store" = ''
      include if exists "${alias_dir}"
    '';

    systemd.services.aa-alias-setup = {
      after = [ "local-fs.target" ];
      before = [ "apparmor.service" ];
      requiredBy = [ "apparmor.service" ];

      path = [ config.nix.package ]; # respect the users choice to use alternative nix implementations

      unitConfig = {
        Description = "Initialize alias rules required for AppArmor policies";
        DefaultDependencies = "no";
        ConditionSecurity = "apparmor";
      };

      serviceConfig = {
        Type = "oneshot";
        ExecStart = "${getExe aa-alias-manager} -o ${alias_dir} -p ${config.security.apparmor.aa-alias-manger.patterns}";
      };
    };
  };
}
