{
  description = "nix dashboard";
  nixConfig.bash-prompt = "\[nix-develop\]$ ";

  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        system = "x86_64-linux";
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        lib = nixpkgs.lib;
      in
      with pkgs;
      {
        devShell = mkShell {
          buildInputs = [
            pkg-config
            jq
            bat
            gcc
            openssl
            postgresql
            tree
            exa
            diesel-cli
            postgresql_14
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = ["rust-src"];
            }))
          ];

          postgresConf =
            writeText "postgresql.conf"
            ''
              # Settings
              log_min_messages = warning
              log_min_error_statement = error
              log_min_duration_statement = 100  # ms
              log_connections = on
              log_disconnections = on
              log_duration = on
              #log_line_prefix = '[] '
              log_timezone = 'UTC'
              log_statement = 'all'
              log_directory = 'pg_log'
              log_filename = 'postgresql-%Y-%m-%d_%H%M%S.log'
              logging_collector = on
              log_min_error_statement = error
            '';

          PGDATA = "/home/req/org/dev/rust/rocketship/.pg";

          shellHook = ''
            echo "Using ${postgresql_14.name}."
            export PGHOST="$PGDATA"
            [ ! -d $PGDATA ] && pg_ctl initdb -o "-U postgres" && cat "$postgresConf" >> $PGDATA/postgresql.conf
            pg_ctl -o "-p 5555 -k $PGDATA" start

            alias fin="pg_ctl stop"
            alias pg="psql -p 5555 -U postgres"
            alias ls='exa --icons'
            alias d='~/.cargo/bin/diesel'
          '';
        };
      }
    );
}


