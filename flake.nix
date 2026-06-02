{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable-small";
    systems.url = "github:nix-systems/default";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs: let
    perSystem = inputs.nixpkgs.lib.genAttrs (import inputs.systems);
    perSystemPkgs = perSystem (system: import inputs.nixpkgs {
      inherit system;
      overlays = with inputs; [fenix.overlays.default];
    });
  in {
    devShells = perSystem (system: let
      pkgs = perSystemPkgs.${system};
    in {
      default = pkgs.mkShellNoCC {
        # Use an alternative linker that's faster
        RUSTFLAGS = "-C link-arg=-fuse-ld=${pkgs.mold}/bin/mold";

        # Required for dynamic linkers to find required dependencies/libraries
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
          # Vulkan
          vulkan-loader

          # Other dependencies
          libxkbcommon

          # X11
          libx11
          libxi
          libxcursor
        ]);

        nativeBuildInputs = with pkgs; [
          rust-analyzer-nightly
          (fenix.default.withComponents [
            "cargo"
            "rustc"
            "rustfmt"
          ])

          # For finding dependencies/libraries
          pkg-config

          # Use an alternative linker that's x5 or so faster than LLD and way faster than Rust's default linker
          clang
          mold
        ];
        buildInputs = with pkgs; [
          # Audio
          alsa-lib

          # Cross Platform 3D Graphics API
          vulkan-loader

          # For debugging in Vulkan
          vulkan-tools

          # Wayland
          wayland

          # X11
          libudev-zero
          libx11
          libxcursor
          libxi
          libxrandr

          # Other dependencies
          libxkbcommon
          libudev-zero
        ];
      };
    });
  };
}