{ lib, rustPlatform }:
rustPlatform.buildRustPackage {
  pname = "aa-alias-manager";
  version = "unstable-2024-10-23";
  src = lib.cleanSource ../.;

  cargoHash = "sha256-ovl1fLT9yxdunlLYhouoHJIR0ifjBlDFXs2dx0fSoqY=";

  meta = {
    description = "Tool to generate a file of aliases for apparmor based on current nixos generation";
    homepage = "https://github.com/LordGrimmauld/aa-alias-manager";
    license = lib.licenses.gpl3Only;
    mainProgram = "aa-alias-manager";
    maintainers = with lib.maintainers; [ grimmauld ];
    platforms = lib.platforms.linux;
  };
}
