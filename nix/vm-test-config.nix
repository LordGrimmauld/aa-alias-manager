{ modulesPath, ... }:
{
  imports = [
    "${modulesPath}/virtualisation/qemu-vm.nix"
  ];

  security.apparmor = {
    enable = true;

    # test profile to check whether alias is hitting.
    policies.coreutils = {
      enable = true;
      enforce = false;
      profile = ''
        abi <abi/4.0>,
        include <tunables/global>
        profile coreutils /bin/coreutils {
          include <abstractions/base>
        }
      '';
    };
  };

  users.users = {
    alice = {
      isNormalUser = true;
      initialPassword = "test";
      extraGroups = [ "wheel" ];
    };
  };

  boot.loader.grub.devices = [ "/dev/sda" ];
  system.stateVersion = "24.05";
}
