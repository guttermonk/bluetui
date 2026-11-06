{
  lib,
  rustPlatform,
  dbus,
  pkg-config,
}:
let
  cargo = lib.importTOML ./Cargo.toml;
in
rustPlatform.buildRustPackage {
  pname = cargo.package.name;
  version = cargo.package.version;
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  buildInputs = [dbus];
  nativeBuildInputs = [pkg-config];

  # Let taskbars and launchers resolve the window (app_id "bluetui") to a name
  # and icon instead of guessing from the window title.
  postInstall = ''
    install -Dm644 ${./packaging/bluetui.desktop} \
      $out/share/applications/bluetui.desktop
  '';

  meta = {
    description = cargo.package.description;
    homepage = cargo.package.homepage;
    license = lib.licenses.gpl3Only;
    maintainers = with lib.maintainers; [samuel-martineau];
  };
}
