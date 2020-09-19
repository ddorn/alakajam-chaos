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
I choosed to use [this font](https://tinyworlds.itch.io/free-pixel-font-thaleah) because
I like its bold and ambitious feel.

## 10:00 Start the player class and mor interesting particles

## 11:00

Tried to use the create `game-loop` to have fixed updates, but couldn't compile it
(we are startin to see a scheme here...), then found out quicksliver had a built in
thing for this and copied https://github.com/ryanisaacg/quicksilver/blob/master/examples/06_timers.rs

Had some fun tweeking the apearence of the player.

## 11:42 Events and Mouse control

Now I'll try to make my blob go towards the cursor... Time to look at quicksilver's events !
First thing to realise : Event is not what I'm looking for, but `input` is. This was surpisingly
easy to do, so I now have a blob slowly following my cursor !

## 12:00 Fire !

Right know the blob needs two things : 
 - Fire deadly missiles
 - And a proper name

Let's tackle the first one. But first separate the project in different files.

Now it's time to look at events.


## 14:00 Shard shots

Shots would be better in the shape of a shard... Let's look into the meshes...

This went very well, I think I could do lots of things easily with
those meshes. 

## 15:00 Fixing window size.

Now that I like the firing, I don't really know what to do. I'm definetly not going for maps, and idk about ships. Maybe the player will have to bring Chaos to its world, as asked by the Jellymancer. 

Since I don't have any idea, i'll fix what has been anoying me since I opend my first window: wrong game size and white borders. Let's head over
[10_resize.rs](https://docs.rs/crate/quicksilver/0.4.0-alpha0.7/source/examples/10_resize.rs). 
But this was to simple to work, and if you read the description carefully
you will see *"doesn't work on the web"* ! Well, time for some css and html then !