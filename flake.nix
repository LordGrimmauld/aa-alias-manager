{
  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    nix-github-actions = {
      url = "github:nix-community/nix-github-actions";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pre-commit-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      self,
      nix-github-actions,
      pre-commit-hooks,
      rust-overlay,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);
    in
    {
      devShells = forAllSystems (
        system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        {
          default = pkgs.mkShell {
            buildInputs = [
              rustToolchain
            ];
          };
        }
      );

      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          aa-alias-manager-pkg = (pkgs.callPackage ./nix/package.nix { });
        in
        {
          aa-alias-manager = aa-alias-manager-pkg;
          default = aa-alias-manager-pkg;
        }
      );

      apps = forAllSystems (system: rec {
        aa-alias-manager = {
          type = "app";
          program = "${self.packages.${system}.aa-alias-manager}/bin/aa-alias-manager";
        };
        default = aa-alias-manager;
      });

      nixosModules = {
        aa-alias-manager = ./nix/aa-alias-module.nix;
        default = self.nixosModules.aa-alias-manager;
      };

      formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.nixfmt-rfc-style);

      checks = forAllSystems (
        system:
        {
          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks.nixfmt-rfc-style.enable = true;
          };
        }
        // (
          let
            pkgs = nixpkgs.legacyPackages.${system};
            nixos-lib = import (nixpkgs + "/nixos/lib") { };
            inherit (pkgs) lib;
          in
          lib.mapAttrs' (
            n: v:
            let
              name = lib.removeSuffix ".nix" n;
            in
            lib.nameValuePair name (
              (nixos-lib.runTest (
                {
                  hostPkgs = pkgs;
                  imports = [
                    {
                      inherit name;
                      nodes.test =
                        { ... }:
                        {
                          imports = [
                            ./checks/vm-test-config.nix
                            self.nixosModules.default
                          ];
                        };
                    }
                  ];
                }
                // (import ./checks/test/${n})
              ))
            )
          ) (builtins.readDir ./checks/test)
        )
      );

      githubActions = nix-github-actions.lib.mkGithubMatrix {
        checks = nixpkgs.lib.getAttrs [ "x86_64-linux" ] self.checks;
      }; # todo: figure out testing on aarch64-linux
    };
}
