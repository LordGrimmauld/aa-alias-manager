{ lib, rustPlatform }:
rustPlatform.buildRustPackage {
  pname = "aa-alias-manager";
  version = "unstable-2024-10-23";
  src = lib.cleanSource ../.;

  cargoHash = "sha256-M9TCfJuB1Qhow+yOttj8661DaCicrRQSebnytRoUfng=";

  meta = {
    description = "Tool to generate a file of aliases for apparmor based on current nixos generation";
    homepage = "https://github.com/LordGrimmauld/aa-alias-manager";
    license = lib.licenses.gpl3Only;
    mainProgram = "aa-alias-manager";
    maintainers = with lib.maintainers; [ grimmauld ];
    platforms = lib.platforms.linux;
  };
}
