let
  # rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  # pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  pkgs = import <nixpkgs> {};
  # probe-rs-rules = import (builtins.fetchGit "https://github.com/jneem/probe-rs-rules");
in
with pkgs;
mkShell {
  # imports = [
  #   probe-rs-rules.nixosModules.default
  # ];
  # hardware.probe-rs.enable = true;
  buildInputs = [
    gdb
    picocom
    udev 
    pkg-config
    libusb1
    probe-rs
    # probe-rs-tools
    flip-link
    rust-analyzer
  ];
  shellHook = ''
    export LIBCLANG_PATH="/home/dezash/.rustup/toolchains/esp/xtensa-esp32-elf-clang/esp-19.1.2_20250225/esp-clang/lib"
    export PATH="/home/dezash/.rustup/toolchains/esp/xtensa-esp-elf/esp-14.2.0_20240906/xtensa-esp-elf/bin:$PATH"
  '';
}
