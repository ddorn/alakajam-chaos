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
you will see *"doesn't work on the web"* ! Well, time for some css and html then ! I have trouble giving the right size to the canvas, but I'll maybe fix that later.

## 16:00 Basic Enemy

Let's make some enemies that go towards the player (maybe that look like ships ?). I dont think I will make them look like ship, since I probably wont have the time anyway.

Now that a simple enemy is made, I think they could be slime-like enemies,
spawning two when you kill one. It could maybe even be an inifinite slime,
and you'll have to try to kill as many as you can, and it would get strong
instead of weaker when you kill it... :think:

I will have to revise the shoot and aim though, I have the filling that it
is frustrating but in a non-fun way.

## 17:00 Spawn, different enemy types

Time to make some of them apear, have different characteristics and to be able to shoot them !

After some borrow checker hassle, I finally got away by using `swap`. 
The enemies are still not very interesting so it's time to make them interesting !

## 18:00 Make enemies interesting

We'll first try to have them split. And maybe showing the score would be a good idea, along with a pause key !

While coding the pause I've found something funny : when I render the
game, I interpolate the time between two updates, but since the game
is not moving on pause it only make the particles jitter.

Fixed a few things (shot multiple blobs at the same frame, ...)

## 20:00 Player life

This is a very important thing to implement : life. It should be quite easy to do, but I'm a bit too lazy to make a state machine for the game.

Added a restart with R, fought again the double borrow checker when I 
want to do game.player.update(game), and I finially decided to just do
Player::update(game). It's not very clean but *it works*.

## 21:00 Explosions

Let's start some polish, and add bright explosions when a blob dies.


# TODO: Polish
- Pause on dark bg, with yellow particles on each side
- explosion when a blob dies
- bg depending on the score
- Better css for the help
- Keep only important text, put colors and center what is needed
- make the game shake when the player is hit
