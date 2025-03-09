let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
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
    (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
  ];
}

