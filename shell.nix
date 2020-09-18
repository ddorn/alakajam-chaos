# Thanks to https://duan.ca/2020/05/07/nix-rust-development/

let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> {
    overlays = [ moz_overlay ];
  };
  ruststable = (nixpkgs.latest.rustChannels.stable.rust.override {
    extensions = [ "rust-src" "rust-analysis" ];
  });
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "rust-chaos";
    buildInputs = [ 
      rustup 
      ruststable
      pkg-config
      libudev
      zlib
      alsaLib
    ];
  }
