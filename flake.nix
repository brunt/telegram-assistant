{
  # type `nix develop` in a terminal to use this flake
  ## TODO: compiled binaries don't run on my 3b+ :(
  description = "Cross-compiling a Rust + OpenSSL project to aarch64-unknown-linux-gnu";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        target = "aarch64-unknown-linux-gnu";
        crossPkgs = pkgs.pkgsCross.aarch64-multiplatform;

        rust = pkgs.rust-bin.stable.latest.default.override {
          targets = [ target ];
        };

      in {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            rust
            pkgs.pkg-config
            pkgs.cmake
            crossPkgs.buildPackages.gcc
          ];

          buildInputs = [
            crossPkgs.openssl
          ];

          CARGO_BUILD_TARGET = target;
          PKG_CONFIG_ALLOW_CROSS = "1";
          PKG_CONFIG_PATH = "${crossPkgs.openssl.dev}/lib/pkgconfig";
          PKGCONFIGDIR = "${crossPkgs.openssl.dev}/lib/pkgconfig";
          OPENSSL_DIR = crossPkgs.openssl.dev;
          OPENSSL_LIB_DIR = "${crossPkgs.openssl.out}/lib";
          OPENSSL_INCLUDE_DIR = "${crossPkgs.openssl.dev}/include";
          CMAKECONFIGDIR = "${pkgs.cmake.out}";
          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${crossPkgs.stdenv.cc.targetPrefix}cc";
          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_AR = "${crossPkgs.stdenv.cc.bintools}/bin/${crossPkgs.stdenv.cc.targetPrefix}ar";
          CARGO_TARGET_DIR= "/tmp/target";
        };
      });
}
