{ pkgs, src, jdk }:

pkgs.stdenv.mkDerivation {
  pname = "llminxsolver-gradle-deps";
  version = "0";

  inherit src;

  nativeBuildInputs = [ jdk pkgs.cacert ];

  outputHashMode = "recursive";
  outputHashAlgo = "sha256";
  outputHash = "sha256-yYEIZhZmBVkp6Q5uipbmU0EdySYVg+ckwDLZpgSbx7c=";

  buildPhase = ''
    runHook preBuild

    export HOME=$(mktemp -d)
    export GRADLE_USER_HOME=$out
    export SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt

    chmod +x gradlew
    ./gradlew :desktopApp:createReleaseDistributable --no-daemon || true

    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall

    # Remove transient files
    rm -rf $out/daemon 2>/dev/null || true
    rm -rf $out/native 2>/dev/null || true
    rm -rf $out/workers 2>/dev/null || true
    rm -rf $out/notifications 2>/dev/null || true
    rm -rf $out/jdks 2>/dev/null || true
    
    # Remove file-based locks
    find $out -name "*.lock" -type f -delete 2>/dev/null || true
    find $out -name "gc.properties" -type f -delete 2>/dev/null || true

    runHook postInstall
  '';

  dontFixup = true;
}
