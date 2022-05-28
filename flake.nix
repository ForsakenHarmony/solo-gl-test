{
  description = "A basic flake with a shell";

  inputs = {
    #nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nixpkgs.url = "github:Gaelan/nixpkgs?ref=e545700e7fcb8eb5116e657b337389f4e8a5ecaa";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }: flake-utils.lib.eachDefaultSystem (system:
  let
    overlays = [
      #( import rust-overlay)
    ];
    pkgs = import nixpkgs {
      inherit overlays system;
      #crossSystem = "armv7l-unknown-linux-gnueabihf";
    };
#    pkgsCrossMusl = import nixpkgs {
#      inherit overlays system;
#      hostSystem = "${system}";
#      crossSystem = "armv7l-unknown-linux-musleabihf";
#    };
     #pkgsCross = pkgs;
    pkgsCross = pkgs.pkgsCross.armv7l-hf-multiplatform;

    #rustTargetTriple = "armv7-unknown-linux-gnueabihf";
    #rustTargetTriple = "arm-unknown-linux-gnueabihf";
  in {
    devShells.default = pkgs.mkShell rec {
      nativeBuildInputs = with pkgs; [
        pkgsCross.stdenv.cc
#        pkg-config
        #(rust-bin.nightly."2022-04-26".default.override {
          #targets = [
            ##rustTargetTriple
            #"arm-unknown-linux-gnueabihf"
            #"armv7-unknown-linux-gnueabihf"
            ##"arm-unknown-linux-musleabihf"
            ##"armv7-unknown-linux-musleabihf"
          #];
          #extensions = [ "rust-src" "clippy" "cargo" "rustfmt-preview" ];
        #})
        # libiconv
        # pkgsCross.stdenv.cc
      ];

      buildInputs = with pkgsCross; [
#        libGLU
#        egl-wayland
#        libGL
#        eglexternalplatform
        #pkgs.darwin.apple_sdk.frameworks.OpenGL
        # stdenv.cc
        #libiconv
        #libgcc
      ];

      # RUSTUP_HOME = "./.nix/.rustup";
      # CARGO_HOME = "./.nix/.cargo";
#      RUSTUP_HOME = "/Users/hrmny/.rustup";
      #CARGO_HOME = "/Users/hrmny/.cargo";

      # RUSTC_VERSION = rustupToolchain;

      #LIBCLANG_PATH= pkgsCross.lib.makeLibraryPath [ pkgsCross.llvmPackages_latest.libclang.lib ];

      # CARGO_BUILD_TARGET = rustTargetTriple;

      # LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
      #shellHook = ''
        #export PATH=$PATH:${CARGO_HOME}/bin
      #'';
#        export PATH=$PATH:${RUSTUP_HOME}/toolchains/${rustupToolchain}-${rustBuildTriple}/bin/
#        rustup component add rust-src
#        rustup target add ${rustTargetTriple}

      CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER =
        "${pkgsCross.stdenv.cc.targetPrefix}ld";
      CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER =
        "${pkgsCross.stdenv.cc.targetPrefix}ld";
    };
  });
}

