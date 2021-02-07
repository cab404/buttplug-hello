with import <nixpkgs> {}; mkShell {
  buildInputs = [ cargo rustc ] ++ [ pkgconfig libudev.dev libusb1.dev dbus.dev ];
  RUST_SRC_PATH = rustPlatform.rustLibSrc;
}