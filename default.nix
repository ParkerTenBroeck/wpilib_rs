{ pkgs ? import <nixpkgs> {}}:
let
  rustupToolchain = "nightly";

  rustBuildTargetTripple = "arm-unknown-linux-gnueabh";
  rustBuildHostTripple = "x86_64-unknown-linux-gnu";

  arm-cross = import pkgs.path {
    crossSystem = {
      config = "arm-unknown-linux-gnueabh";
    };
  };

  arm_ld = mips-cross.stdenv.cc;

in 

pkgs.mkShell rec {
  buildInputs = with pkgs; [
    rustup
    arm_ld
    git
    #yq
  ];

  RUSTUP_HOME = toString ~/.rustup;
  CARGO_HOME = toString ~/.cargo;
  RUSTUP_TOOLCHAIN = rustupToolchain;


  shellHook = ''
    export PATH=$PATH:${CARGO_HOME}/bin
    export PATH=$PATH:${RUSTUP_HOME}/toolchains/${rustupToolchain}-${rustBuildHostTripple}/bin/
    export PATH=$PATH:${RUSTUP_HOME}/toolchains/${rustupToolchain}-${rustBuildTargetTripple}/bin/
    rustup target add "${rustBuildHostTripple}"
    rustup target add "${rustBuildTargetTripple}"
    rustup component add rust-src --toolchain ${rustupToolchain}-${rustBuildHostTripple}
    rustup component add clippy
    rustup component add rustfmt
    rustup component add llvm-tools-preview
    '';
}