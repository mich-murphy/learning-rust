{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay"; # A helper for Rust + Nix
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      overlays = [
        # Makes a `rust-bin` attribute available in Nixpkgs
        (import rust-overlay)
        (self: super: {
          rustToolchain = super.rust-bin.stable.latest.default;
        })
      ];

      allSystems = [
        "x86_64-linux" # 64-bit Intel/AMD Linux
        "aarch64-linux" # 64-bit ARM Linux
        "x86_64-darwin" # 64-bit Intel macOS
        "aarch64-darwin" # 64-bit ARM macOS
      ];

      # Helper to provide system-specific attributes
      forAllSystems = f: nixpkgs.lib.genAttrs allSystems (system: f {
        pkgs = import nixpkgs { inherit overlays system; };
      });
    in
    {
      devShells = forAllSystems ({ pkgs }: {
        default = pkgs.mkShell {
          packages = (with pkgs; [
            # The package provided by our custom overlay. Includes cargo, Clippy, cargo-fmt,
            # rustdoc, rustfmt, and other tools.
            rustToolchain
          ]) ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs; [ libiconv ]);
        };
      });
    };
}
