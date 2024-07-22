{ pkgs, ... }:

let
  llvmPackages = pkgs.llvmPackages_14;
in
{
  # Enable devenv shell features
  packages = with pkgs; [
    pkg-config
    udev
    libudev-zero
    lm_sensors
    llvmPackages.libclang
    llvmPackages.clang
    glibc.dev
    gcc
  ];

  languages.rust = {
    enable = true;
    rustflags = "-C link-arg=-lsensors";
  };

  # Set environment variables
  env = {
    LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
      udev
      lm_sensors
      llvmPackages.libclang
      glibc.dev
      gcc.cc.lib
    ];
    PKG_CONFIG_PATH = "${pkgs.pkg-config}/lib/pkgconfig:${pkgs.lm_sensors}/lib/pkgconfig";
    SENSORS_LIB_DIR = "${pkgs.lm_sensors}/lib";
    SENSORS_INCLUDE_DIR = "${pkgs.lm_sensors}/include";
    LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
    BINDGEN_EXTRA_CLANG_ARGS = with pkgs; ''
      -I${llvmPackages.libclang.lib}/lib/clang/${llvmPackages.libclang.version}/include
      -I${glibc.dev}/include
      -I${gcc}/lib/gcc/${stdenv.targetPlatform.config}/${gcc.version}/include
      -I${gcc}/lib/gcc/${stdenv.targetPlatform.config}/${gcc.version}/include-fixed
    '';
    CPATH = with pkgs; lib.makeSearchPathOutput "dev" "include" [
      glibc.dev
      gcc.cc
    ];
    LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
      lm_sensors
    ];
  };

  # You can add more devenv-specific configurations here
}
