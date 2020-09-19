# Day 0

## 21h30 - Search for a simple rust library that let one make a game in the browser

## 22h - found Quicksilver

[quicksilver](https://github.com/ryanisaacg/quicksilver) that seems to fit all requirements

## 22h - debuging dependency to compile quicksilver on Nix, thow up this derivation

```nix
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
```

## 22h30 Making something work in my browser

- Installed cargo-web (`$ cargo install cargo-web`) wich was even bigger than quicksilver (299 creates vs )
- quicksilver wants to use wayland for an unknown reason. Actually it just can't find either wayland or x11
  probablly because I'm on Nix...
- Finally understood why I couldn't buy to the web : 
  I need to pass `--features quicksilver/stdweb` to `cargo web start`... 
  That took my a solid 30 minutes.

## 23h30 Dinner & Brainstorm

So I definetly want to make stuff with particles. I also really like the Chaos theme, but it
is hard to have as a central aspect in a game, as chaos is hardly predictable and the player
needs to be able to somewhat predict what's happening, and to have a sense of control over it

I think the sense of chaos will mostly by in the animations and particle system, probably lots
of explosions should be fun !

# Day 1
## 00:30 Implementing a simple particle system

This was a fail. I went to bed at 2AM after an endless battle with the type checker
and the `rand` create that just did not want to run properly in a web browser.

## 08:00 Making rand work

Finally managed to make rand work using `rand_xorshift` to avoid using any entropy source.

## 09:30 Particles don't die...

Fix the particle system and add text. Now we are ready to do some interesting things !

## 10:00
