# {
#   pkgs ? import <nixpkgs> { },
#   ...
# }:
# pkgs.mkShellNoCC {
#   buildInputs = with pkgs; [
#     xorg.libxcb.dev
#   ];
# }
#
{
  pkgs ? import <nixpkgs> { },
  ...
}:
pkgs.mkShell {
  # buildInputs = with pkgs; [
  #   vulkan-loader
  # ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (
    with pkgs;
    [
      # wayland
      # vulkan-validation-layers
      # vulkan-loader
      # libxkbcommon
      xorg.libxcb.dev
    ]
  );

  # VK_LAYER_PATH = "${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d";
}
