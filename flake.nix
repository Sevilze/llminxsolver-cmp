{
  description = "llminxsolver - Megaminx Last Layer Solver with Compose Multiplatform UI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable."1.92.0".default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };

        appVersion = builtins.getEnv "APP_VERSION";
        version = if appVersion != "" then appVersion else "0.0.0-dev";

        nativeLib = rustPlatform.buildRustPackage {
          pname = "llminxsolver-uniffi";
          inherit version;

          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ openssl ];
          cargoBuildFlags = [ "-p" "llminxsolver-uniffi" ];

          meta = with pkgs.lib; {
            description = "Megaminx puzzle solver with UniFFI bindings";
            homepage = "https://github.com/Sevilze/llminxsolver-cmp";
            license = licenses.mit;
          };
        };

        libExtension = if pkgs.stdenv.isDarwin then "dylib" else "so";

        gradleDeps = import ./nix/gradle-deps.nix {
          inherit pkgs;
          src = ./.;
          jdk = pkgs.jdk21;
        };

      in
      {
        packages = {
          lib = nativeLib;
          inherit gradleDeps;

          default = pkgs.stdenv.mkDerivation {
            pname = "llminxsolver";
            inherit version;

            src = ./.;

            nativeBuildInputs = with pkgs; [
              jdk21
              makeWrapper
              cacert
            ];

            buildInputs = with pkgs; [
              jdk21
            ];

            buildPhase = ''
              runHook preBuild

              export HOME=$(mktemp -d)
              export SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt

              # Copy prefetched deps to writable location
              export GRADLE_USER_HOME=$(mktemp -d)
              cp -r ${gradleDeps}/* $GRADLE_USER_HOME/ || true
              chmod -R u+w $GRADLE_USER_HOME

              mkdir -p shared/src/desktopMain/resources
              cp ${nativeLib}/lib/libllminxsolver_uniffi.${libExtension} shared/src/desktopMain/resources/

              chmod +x gradlew
              ./gradlew :desktopApp:createReleaseDistributable --no-daemon --offline

              runHook postBuild
            '';

            installPhase = ''
              runHook preInstall

              mkdir -p $out
              cp -r desktopApp/build/compose/binaries/main-release/app/* $out/

              mkdir -p $out/bin
              makeWrapper "$out/LLMinx Solver/bin/LLMinx Solver" $out/bin/llminxsolver \
                --set JAVA_HOME ${pkgs.jdk21}

              runHook postInstall
            '';

            meta = with pkgs.lib; {
              description = "Megaminx Last Layer Solver with Compose Multiplatform GUI";
              homepage = "https://github.com/Sevilze/llminxsolver-cmp";
              license = licenses.mit;
              mainProgram = "llminxsolver";
            };
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkg-config
            openssl
            jdk21
          ];

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
      }
    );
}
