# Intended Use
aa-alias-manager is meant as a software to bridge the gap between apparmor profiles on non-NixOS systems and apparmor on NixOS needing to match profiles against store entries. To achieve that goal, aa-alias-manager can create sets of alias rules to be used with apparmor. The alias rules are automatically generated from the current NixOS generation using a pattern config file to prevent false matches.
I intend to upstream my apparmor experiments to NixOS once things are more stable and 24.11 branch-of is reached. Until then, this project can be included as a flake.
Supported are Linux systems. I do however not have any way to test on any other architectures than x86-64 or aarch64. Feedback and fixes for different architectures are welcome.
More info about motivation can be found on my hedgedoc at https://hedgedoc.grimmauld.de/s/03eJUe0X3#.

# Stability
This software is WIP. API will change, and implementation obviously too. However, I intend the main branch of this repo to always be stable to a degree where using aa-alias-manager or its NixOS package/module will not leave your PC unable to boot. Worst case, your apparmor parser might crash, disabling apparmor protection of your system. This is unintended, but depending on your store contents not entirely impossible. if this happens, please open an issue.
To reach these stability goals, no pull request with failing github action checks will be merged into master. Those checks run `nix flake check`, so you can just check locally too.

# Installing/Making use of aa-alias-manager
aa-alias-manager provides a flake with NixOS module for easy installation.

```nix
inputs.aa-alias-manager.url = "github:LordGrimmauld/aa-alias-manager";
inputs.aa-alias-manager.inputs.nixpkgs.follows = "nixpkgs";
```

The module can be included by adding `aa-alias-manager.nixosModules.default` to your module imports. Non-Flake include (and instructions) of the module might follow in the future. Just loading `src/aa-alias-module.nix` via e.g. niv may work, i did not test this.

The relevant config bits are as follows:
```nix
security.apparmor.aa-alias-manager = {
  enable = <bool>;
  patterns = [
    {
      name = "<name>"; # e.g. bin
      target = "<target>"; # e.g. /bin
      store_suffixes = [ "<suffix>" ];
      individual = <bool>; # whether to list out files individually
      only_exe = <bool>; # whether to only match executables
      disallowed_strings = [ "<disallowed>" ]; # e.g. [ "!" ]
      only_include = <bool>;
    }
  ];
};
```

A default pattern includes `/bin`. `enable` defaults to whatever is defined in `config.security.apparmor.enable`.
