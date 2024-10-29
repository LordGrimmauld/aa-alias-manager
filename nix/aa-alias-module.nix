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
  config = mkIf config.security.apparmor.enable {
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
        ExecStart = "${getExe aa-alias-manager} -o ${alias_dir} -p ${./aa-alias-patterns.json}";
      };
    };
  };
}
