# WPIlib_rs

This is an implementation (or more so an interface) for wpilib in rust.

# Project

Your main robot code will be in `./robot_code/src/main.rs`. this will behaive like a normal rust project.

you can set your team number in `./.cargo/config.toml` using the `FRC_TEAM_NUMBER` environment variable.



# Prerequisites

You will need wpilib 2023 installed on your computer. the default location is HOME/wpilib where HOME is your user directory. this can be changed in `./.cargo/config.toml`

You will also need cargo installed with the `arm-unknown-linux-gnueabi` target installed and its std
```bash 
# add target
$ rustup target add arm-unknown-linux-gnueabi 
# update and install std
$ rustup update
```


When deploying you need to ensure that the shared object files are already installed on the system. (deploying a helloworld cpp project will give you the needed shared objects)


# Building and Deploying

```bash
$ cargo build_native
```
will build the native binary. default to debug build but can be specified with `--release` or `--debug`

```bash
$ cargo build_roborio
```
will build the roborio binary. default to debug build but can be specified with `--release` or `--debug`


```bash
$ cargo deploy
```
will build and deploy the roborio binary.
default to debug build but can be specified with `--release` or `--debug`.
can also use `--nobuild` to skip rebuilding and only deploy (will crash if no binary is found)

# Current State

Currently the 'Core Robot Loop' is complete. There exists a single interface `TimedRobotTrait` equivalent to Cpp and Javas TimedRobot class. All robot initialization and error reporting is handled internally through the library with no unsafe or binding code needed in user robot code. 

# Note

This isn't tested (at all) on windows or mac. I tried as best as I could to add support without having access to either however, we know how that goes...


# Todo/Goals

- Full platform support (linux, windows, mac)
- Roborio debugging
- Proper shared object deploying
- Provide an interface for more wpilib components
- Documentation