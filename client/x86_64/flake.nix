{
  description = "NixOS ISO Flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: {
    nixosConfigurations.iso = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        "${nixpkgs}/nixos/modules/installer/cd-dvd/installation-cd-minimal.nix"
        ({ pkgs, ... }: {
          isoImage.isoName = "my-nixos-live.iso";
          environment.systemPackages = [
            (pkgs.stdenv.mkDerivation {
              pname = "client";
              version = "0.1.0";
              src = ./x86_64;
              dontUnpack = true;
              dontBuild = true;
              dontConfigure = true;
              installPhase = ''
                mkdir -p $out/bin
                install -m 755 $src $out/bin/client
              '';
              })
            ];
        })
      ];
    };
  };
}
