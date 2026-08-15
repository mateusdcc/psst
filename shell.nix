{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    rustc
    cargo
    rustfmt
    clippy
  ];

  buildInputs = with pkgs; [
    openssl
  ] ++ pkgs.lib.optionals pkgs.stdenv.hostPlatform.isLinux [
    alsa-lib
    gtk3
    cairo
    glib
    pango
    atk
    gdk-pixbuf
    dbus
  ] ++ pkgs.lib.optionals pkgs.stdenv.hostPlatform.isDarwin [
    apple-sdk
    libiconv
  ];

  shellHook = ''
    export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
  '';
}
