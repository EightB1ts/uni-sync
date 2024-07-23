{
  description = "Uni-Sync: Your project description";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      llvmPackages = pkgs.llvmPackages_14;
    in
    {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "uni-sync";
        version = "0.3.2";
        src = ./.;
        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        nativeBuildInputs = with pkgs; [
          pkg-config
          rustc
          cargo
          llvmPackages.libclang
          llvmPackages.clang
        ];

        buildInputs = with pkgs; [
          udev
          libudev-zero
          systemd.dev # This provides libudev.pc
          lm_sensors
          glibc.dev
          gcc
        ];

        LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";

        RUSTFLAGS = "-C link-arg=-lsensors -C link-arg=-ludev";

        preBuild = ''
          export LD_LIBRARY_PATH="${with pkgs; lib.makeLibraryPath [
            udev
            systemd.dev
            lm_sensors
            llvmPackages.libclang
            glibc.dev
            gcc.cc.lib
          ]}"
          export PKG_CONFIG_PATH="${pkgs.pkg-config}/lib/pkgconfig:${pkgs.lm_sensors}/lib/pkgconfig:${pkgs.systemd.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
          export SENSORS_LIB_DIR="${pkgs.lm_sensors}/lib"
          export SENSORS_INCLUDE_DIR="${pkgs.lm_sensors}/include"
          export BINDGEN_EXTRA_CLANG_ARGS="-I${llvmPackages.libclang.lib}/lib/clang/${llvmPackages.libclang.version}/include -I${pkgs.glibc.dev}/include -I${pkgs.gcc}/lib/gcc/${pkgs.stdenv.targetPlatform.config}/${pkgs.gcc.version}/include -I${pkgs.gcc}/lib/gcc/${pkgs.stdenv.targetPlatform.config}/${pkgs.gcc.version}/include-fixed -I${pkgs.systemd.dev}/include"
          export CPATH="${with pkgs; lib.makeSearchPathOutput "dev" "include" [
            glibc.dev
            gcc.cc
            systemd.dev
          ]}"
          export LIBRARY_PATH="${pkgs.lib.makeLibraryPath [ pkgs.lm_sensors pkgs.systemd.dev ]}"
        '';

        postInstall = ''
          patchelf --set-rpath "${pkgs.lm_sensors}/lib:${pkgs.systemd.dev}/lib:${pkgs.udev}/lib:$out/lib" $out/bin/uni-sync
        '';

        meta = with pkgs.lib; {
          description = "Your project description";
          homepage = "https://github.com/yourusername/uni-sync"; # Update this
          license = licenses.mit; # Update this with your actual license
          maintainers = [ maintainers.yourgithubusername ]; # Update this
        };
      };

      devShells.${system}.default = pkgs.mkShell {
        inputsFrom = [ self.packages.${system}.default ];
        packages = with pkgs; [
          rustc
          cargo
          rust-analyzer
        ];
        shellHook = ''
          export LIBCLANG_PATH="${llvmPackages.libclang.lib}/lib"
          export LD_LIBRARY_PATH="${with pkgs; lib.makeLibraryPath [
            udev
            systemd.dev
            lm_sensors
            llvmPackages.libclang
            glibc.dev
            gcc.cc.lib
          ]}"
          export PKG_CONFIG_PATH="${pkgs.pkg-config}/lib/pkgconfig:${pkgs.lm_sensors}/lib/pkgconfig:${pkgs.systemd.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
          export SENSORS_LIB_DIR="${pkgs.lm_sensors}/lib"
          export SENSORS_INCLUDE_DIR="${pkgs.lm_sensors}/include"
          export BINDGEN_EXTRA_CLANG_ARGS="-I${llvmPackages.libclang.lib}/lib/clang/${llvmPackages.libclang.version}/include -I${pkgs.glibc.dev}/include -I${pkgs.gcc}/lib/gcc/${pkgs.stdenv.targetPlatform.config}/${pkgs.gcc.version}/include -I${pkgs.gcc}/lib/gcc/${pkgs.stdenv.targetPlatform.config}/${pkgs.gcc.version}/include-fixed -I${pkgs.systemd.dev}/include"
          export CPATH="${with pkgs; lib.makeSearchPathOutput "dev" "include" [
            glibc.dev
            gcc.cc
            systemd.dev
          ]}"
          export LIBRARY_PATH="${pkgs.lib.makeLibraryPath [ pkgs.lm_sensors pkgs.systemd.dev ]}"
          export RUSTFLAGS="-C link-arg=-lsensors -C link-arg=-ludev"
        '';
      };

      nixosModules.default = { config, lib, pkgs, ... }:
        let
          cfg = config.services.uni-sync;
        in
        {
          options.services.uni-sync = {
            enable = lib.mkEnableOption "Uni-Sync service";
            user = lib.mkOption {
              type = lib.types.str;
              default = "uni-sync";
              description = "User account under which uni-sync runs";
            };
            configFile = lib.mkOption {
              type = lib.types.path;
              default = "/etc/uni-sync/uni-sync.json";
              description = "Path to the uni-sync configuration file";
            };
            initialConfig = lib.mkOption {
              type = lib.types.attrs;
              default = { };
              description = "Initial configuration for uni-sync";
            };
          };

          config = lib.mkIf cfg.enable {
            users.users.${cfg.user} = {
              isSystemUser = true;
              group = cfg.user;
              description = "Uni-Sync service user";
            };
            users.groups.${cfg.user} = { };

            systemd.services.uni-sync = {
              description = "Uni-Sync Service";
              after = [ "network.target" ];
              wantedBy = [ "multi-user.target" ];
              serviceConfig = {
                ExecStartPre = pkgs.writeScript "uni-sync-init" ''
                  #!${pkgs.stdenv.shell}
                  mkdir -p $(dirname ${cfg.configFile})
                  if [ ! -f ${cfg.configFile} ]; then
                    echo '${builtins.toJSON cfg.initialConfig}' > ${cfg.configFile}
                  fi
                '';
                ExecStart = "${self.packages.${pkgs.system}.default}/bin/uni-sync --config ${cfg.configFile}";
                Restart = "always";
                User = cfg.user;
              };
            };
          };
        };
    };
}
