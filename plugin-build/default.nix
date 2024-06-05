{ pkgs ? import <nixpkgs> {} }:
{
  buildAssemblyscriptPlugin =
    (import ./assemblyscript.nix { inherit pkgs; }).buildAssemblyscriptPlugin;
}