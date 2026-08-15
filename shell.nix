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
  ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
    alsa-lib
    gtk3
    cairo
    glib
    pango
    atk
    gdk-pixbuf
    dbus
  ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
    AppKit
    CoreGraphics
    CoreAudio
    AudioUnit
    AudioToolbox
    MediaPlayer
    Security
    SystemConfiguration
  ]);

  shellHook = ''
    export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
  '';
}
