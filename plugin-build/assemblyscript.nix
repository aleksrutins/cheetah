{ pkgs ? import <nixpkgs> {} }:
{
  buildAssemblyscriptPlugin = { name, src }:
    pkgs.stdenv.mkDerivation {
      inherit name src;
      buildInputs = with pkgs; [ assemblyscript ];
      buildPhase = ''
        asc main.ts -o plugin.wasm
      '';
      installPhase = ''
        mkdir -p $out
        mv plugin.wasm $out/
      '';
    };
}