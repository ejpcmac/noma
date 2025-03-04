{
  description = "Devices for nomads.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    git-z = {
      url = "github:ejpcmac/git-z";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { flake-parts, ... }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.devshell.flakeModule ];
      systems = [ "x86_64-linux" "x86_64-darwin" "aarch64-darwin" ];

      perSystem = { inputs', system, ... }:
        let
          overlays = [ (import inputs.rust-overlay) ];
          pkgs = import inputs.nixpkgs { inherit system overlays; };
          rust-toolchain =
            pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        {
          ######################################################################
          ##                            Devshells                             ##
          ######################################################################

          devshells =
            let
              git-z = inputs'.git-z.packages.git-z;

              buildToolchain = with pkgs; [
                rust-toolchain
                flip-link
              ] ++ lib.optionals (!stdenv.isDarwin) [
                clang
              ];

              checkToolchain = with pkgs; [
                cargo-hack
                cargo-nextest
                committed
                eclint
                nixpkgs-fmt
                nodePackages.prettier
                taplo
                typos
              ];

              ideToolchain = with pkgs; [
                nixd
                rust-analyzer
              ];

              developmentTools = with pkgs; [
                cargo-bloat
                cargo-outdated
                cargo-watch
                git
                git-z
                gitAndTools.gitflow
                cargo-binutils
                gcc-arm-embedded
                openocd
                probe-rs
              ];

              ideEnv = [
                {
                  name = "NIX_PATH";
                  value = "nixpkgs=${inputs.nixpkgs}";
                }
                {
                  name = "TYPOS_LSP_PATH";
                  value = "${pkgs.typos-lsp}/bin/typos-lsp";
                }
              ];
            in
            {
              default = {
                name = "Noma";

                motd = ''

                  {202}🔨 Welcome to the Noma devshell!{reset}
                '';

                packages =
                  buildToolchain
                  ++ checkToolchain
                  ++ ideToolchain
                  ++ developmentTools;

                env =
                  ideEnv;

                commands = [
                  # Pass-through commands to make some cargo extensions run in
                  # their own devshell.
                  {
                    name = "cargo-udeps";
                    command = "nix develop -L .#udeps -c cargo $@";
                  }
                ];
              };

              ci = {
                name = "Noma CI";

                packages =
                  buildToolchain
                  ++ checkToolchain;
              };

              # NOTE: cargo-udeps needs Rust nightly to run.
              udeps = {
                name = "cargo-udeps";
                packages = with pkgs; [
                  (rust-bin.nightly."2025-03-01".minimal.override {
                    targets = [ "thumbv7em-none-eabihf" ];
                  })
                  clang
                  cargo-hack
                  cargo-udeps
                ];
              };
            };
        };
    };
}
