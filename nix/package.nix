{
  lib,
  rustPlatform,
  makeWrapper,
  nix,
}:
rustPlatform.buildRustPackage {
  pname = "aa-alias-manager";
  version = "unstable-2024-10-23";
  src = lib.cleanSource ../.;

  cargoHash = "sha256-otmVpAAUiby4bFlAc9RB2M7dWVDxEGfN7GIQm5P46N8=";

  nativeBuildInputs = [ makeWrapper ];
  buildInputs = [ nix ];

  postInstall = ''
    wrapProgram $out/bin/aa-alias-manager \
      --suffix PATH : "${nix}/bin/"
  '';

  meta = {
    description = "Tool to generate a file of aliases for apparmor based on current nixos generation";
    homepage = "https://github.com/LordGrimmauld/aa-alias-manager";
    license = lib.licenses.gpl3Only;
    mainProgram = "aa-alias-manager";
    maintainers = with lib.maintainers; [ grimmauld ];
    platforms = lib.platforms.linux;
  };
}
