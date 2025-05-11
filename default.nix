{ pkgs ? import (builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/e1c2e701296453fe2b46b2824db0a92cb310b311.tar.gz";
  }) {}
}:pkgs.rustPlatform.buildRustPackage rec {
  pname = "nunix";
  version = "0.0.0";
  cargoHash = "sha256-3Iv0QhIXxw2FK+FCWNkm7NnlXF20SfOPeXqdMIRS9OM=";
  src = pkgs.lib.cleanSource ./.;

  nativeBuildInputs = [pkgs.pkg-config pkgs.python3];
  buildInputs =
    [
      pkgs.openssl
      pkgs.zstd
      pkgs.zlib
      pkgs.nghttp2
      pkgs.libgit2
    ];

}