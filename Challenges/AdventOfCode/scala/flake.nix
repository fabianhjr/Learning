{
  description = "Advent of Code Scala Fundamentals";

  inputs.devshell.url    = "github:numtide/devshell";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, devshell, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;

          overlays = [ devshell.overlays.default ];
        };
      in rec {
        config = {
          env = [];
          
          packages = with pkgs; [
            scalafmt
            sbt
          ];
        };

        devShell = pkgs.devshell.mkShell {
          env = config.env;
          packages = config.packages;
        };
      }
    );
}

