# bevy-moonracer

A little game to try out [bevy](https://bevyengine.org).

Online demo: [https://tristancacqueray.github.io/bevy-moonracer](https://tristancacqueray.github.io/bevy-moonracer).

> Checkout the initial prototype in Haskell: [moonracer](../moonracer).


## Gameplay

- [x] 2d spaceship racing game.
- [x] Reach the goals as fast as possible.
- [ ] Edit the inputs frame by frame to make the perfect score, e.g.: tool assisted speedrun (TAS).


## Features and roadmap

The goal of this project is to implement all the core piece of a classic video games.
This can be used by beginners to get some starting points for their own game.

### Game mechanics

- [x] 2d box collision system.
- [x] Reach goals to increase the score.
- [x] Finish the level by landing back on the launch pad.
- [ ] Save user data (powered by [bevy_pkv](https://github.com/johanhelsing/bevy_pkv)).
- [ ] Difficulty settings (Crash on high velocity, no drag, no gravity).
- [ ] Story line (e.g. a pilot trying to become an astranaut).

### Levels

- [x] Levels data stored in [levels.svg](./src/levels.svg) (editable with inkscape).
- [ ] Minimum completion time.
- [ ] Walls
  - [x] Solid
  - [ ] Bumper
  - [ ] Trap
- [ ] Goals
  - [x] Passing gate
  - [ ] Crate that increases ship weight
  - [ ] Black hole that reverses the gravity
- [ ] Codegen at buildtime the level data to remove the xml parser dependencies from the runtime.

### Inputs

- [x] Keyboard wasd/arrow
- [ ] Gamepad
- [ ] Touchscreen

### Sounds

- [ ] Background music
- [ ] White noise shhhh for the thrust
- [ ] Rewarding bell sample when reaching a goal

### Graphisms

- [ ] Logo
- [ ] Shader background
- [ ] Thrust particles
- [ ] Crash animation
- [ ] 3d models
- [ ] Custom font

### User Interface

- [x] Menus (powered by [ui-navigation](https://github.com/nicopap/ui-navigation)).
- [x] Pause/resume.
- [ ] Level selection screen.
- [ ] Level end screen with current score and a next level button.
- [ ] Tooltips to explain current goal.
- [ ] Background demo to show what needs to be done. (e.g. a bot playing the first level).
- [ ] Settings menu to adjust the sound volume and toggle the bloom effect.
- [ ] About screen with link to the source.
- [ ] HUD for the ship velocity and current time.
- [ ] TAS mode with input editor and stepping through the frames.

### Toolchain

- [x] Reproducible build with nix.
- [x] Run native version with `cargo run` linked with [mold](https://github.com/rui314/mold).
- [x] Build wasm version with `nix build .#web`.
- [ ] Setup GitHub action to update the online demo.
- [ ] Build native version for windows/mac/linux
