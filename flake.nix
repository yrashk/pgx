{
  description = "Postgres extensions in Rust.";

  inputs = {
    nixpkgs.url = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    gitignore.url = "github:hercules-ci/gitignore.nix";
    gitignore.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay, naersk, gitignore }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);
      supportedPostgresVersions = [ 10 11 12 13 14 ];
      pgxExamples = builtins.attrNames (builtins.removeAttrs (builtins.readDir ./pgx-examples) ["README.md"]);
      nixpkgsWithOverlays = { system, nixpkgs, extraOverlays ? [ ] }: (import nixpkgs {
        inherit system;
        overlays = [
          self.overlay
          rust-overlay.overlay
          (self: super: { inherit (self.rust-bin.stable.latest) rustc cargo rustdoc rust-std; })
        ] ++ extraOverlays;
      });
    in
    {
      lib = {
        inherit supportedSystems supportedPostgresVersions forAllSystems nixpkgsWithOverlays;
        buildPgxExtension =
          { pkgs
          , source ? root
          , root
          , pname
          , pgxPostgresVersion
          , additionalFeatures ? [ ]
          , release ? true
          }: pkgs.callPackage ./nix/extension.nix {
            inherit pname source root pgxPostgresVersion release naersk additionalFeatures;
            inherit (gitignore.lib) gitignoreSource;
          };
        
        buildPgxExtensionAllVariants = { pkgs
          , source ? root
          , root
          , pname
          , additionalFeatures ? [ ]
          }: let 
            cargoToml = (builtins.fromTOML (builtins.readFile (root + /Cargo.toml)));
          in (pkgs.lib.foldl'
            (x: y: x // y)
            { }
            (map
              (pgxPostgresVersion:
                let pgxPostgresVersionString = builtins.toString pgxPostgresVersion; in
                {
                  "${pname}_${pgxPostgresVersionString}" = self.lib.buildPgxExtension { inherit pname pkgs source root additionalFeatures pgxPostgresVersion; };
                  "${pname}_${pgxPostgresVersionString}_debug" = self.lib.buildPgxExtension { inherit pname pkgs source root additionalFeatures pgxPostgresVersion; release = false; };
                })
              supportedPostgresVersions)
          );
      };
      defaultPackage = forAllSystems (system: (nixpkgsWithOverlays { inherit system nixpkgs; }).cargo-pgx);

      packages = forAllSystems (system:
        let
          pkgs = nixpkgsWithOverlays { inherit system nixpkgs; };
        in
        {
          inherit (pkgs) cargo-pgx;
        } // (pkgs.lib.foldl'
            (x: y: x // y)
            { }
            (map
              (pgxExample:
                self.lib.buildPgxExtensionAllVariants { inherit pkgs; root = ./.; pname = pgxExample; })
              pgxExamples)
          ));

      overlay = final: prev: {
        cargo-pgx = final.callPackage ./cargo-pgx {
          inherit naersk;
          gitignoreSource = gitignore.lib.gitignoreSource;
        };
        cargo-pgx_debug = final.callPackage ./cargo-pgx {
          inherit naersk;
          release = false;
          gitignoreSource = gitignore.lib.gitignoreSource;
        };
      };

      devShell = forAllSystems (system:
        let
          pkgs = nixpkgsWithOverlays { inherit system nixpkgs; };
        in
        pkgs.mkShell {
          inputsFrom = with pkgs; [
            postgresql_10
            postgresql_11
            postgresql_12
            postgresql_13
            postgresql_14
          ];
          buildInputs = with pkgs; [
            rustfmt
            nixpkgs-fmt
            cargo-pgx
            rust-bin.stable.latest.minimal
            rust-bin.stable.latest.rustfmt
            rust-bin.stable.latest.clippy
            postgresql
            libiconv
            pkg-config
          ];
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          PGX_PG_SYS_SKIP_BINDING_REWRITE = "1";
          BINDGEN_EXTRA_CLANG_ARGS = [
            ''-I"${pkgs.llvmPackages.libclang.lib}/lib/clang/${pkgs.llvmPackages.libclang.version}/include"''
          ] ++ (if pkgs.stdenv.isLinux then [
            "-I ${pkgs.glibc.dev}/include"
          ] else [ ]);
        });

      checks = forAllSystems (system:
        let
          pkgs = nixpkgsWithOverlays { inherit system nixpkgs; };
        in
        {
          # Not currently supported...
          # ${pkgs.rust-bin.nightly.latest.default}/bin/cargo-fmt fmt --offline --manifest-path ${./.}/Cargo.toml -- --check
          format = pkgs.runCommand "check-format"
            {
              buildInputs = with pkgs; [ ];
            } ''
            ${pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt --check ${./.}
            touch $out # it worked!
          '';
          pkgs-cargo-pgx = pkgs.cargo-pgx_debug.out;
        });

      defaultTemplate = self.templates.default;
      templates = {
        default = {
          path = ./nix/templates/default;
          description = "A basic PGX extension";
        };
      };
    };
}
