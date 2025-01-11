{ modulesPath, ... }:
{
  imports = [
    "${modulesPath}/virtualisation/qemu-vm.nix"
  ];

  security.apparmor = {
    enable = true;

    # test profile to check whether alias is hitting.
    policies.coreutils = {
      state = "complain";
      profile = ''
        abi <abi/4.0>,
        include <tunables/global>
        profile coreutils /bin/coreutils {
          include <abstractions/base>
        }
      '';
    };
  };

  boot.loader.grub.devices = [ "/dev/sda" ];
  system.stateVersion = "24.05";
}
