{ pkgs, rustPlatform }:
rustPlatform.buildRustPackage rec {
  pname = "dioxus-cli";
  version = "0.3.2";

  # need to fetch from source + update the cargo lock to fix https://github.com/DioxusLabs/dioxus/issues/1101
  # `cargo install --git` ignores the lock but nix needs it
  cargoPatches = [ ./update-dixous-lock.patch ];
  src = pkgs.fetchFromGitHub {
    owner = "DioxusLabs";
    repo = "cli";
    rev = "5f35fa6e16886c8eb71814d98f193ab1e3d0c1fc";
    sha256 = "sha256-dCYUECAIhC7oKxNnnU6sxslIEqIvpcBb7RDtIPusQiw=";
  };

  # src = pkgs.fetchCrate {
  #   inherit pname version;
  #   sha256 = "sha256-8S8zUOb2oiXbJQRgY/g9H2+EW+wWOQugr8+ou34CYPg=";
  # };

  nativeBuildInputs = with pkgs; [ pkg-config ];
  buildInputs = with pkgs;
    [ openssl ] ++ pkgs.lib.optionals stdenv.isDarwin [ CoreServices ];

  cargoSha256 = "sha256-dw5zYLxtLrBFbOcESji4/9HmbZVXUPeE++8hQdxjSco=";

  doCheck = false;
}

# overlay route is sad....
# final: prev: {
#   # overrideAttrs works on the call to mkDerviation not on buildRustpackages https://discourse.nixos.org/t/is-it-possible-to-override-cargosha256-in-buildrustpackage/4393/6
#   dioxus-cli = prev.dioxus-cli.overrideAttrs (old: rec {
#     pname = "dioxus-cli";
#     version = "0.3.2";
#     src = prev.fetchCrate {
#       pname = pname;
#       version = version;
#       sha256 = "sha256-8S8zUOb2oiXbJQRgY/g9H2+EW+wWOQugr8+ou34CYPg=";
#     };
#     cargoDeps = old.cargoDeps.overrideAttrs (prev.lib.const {
#       name = "${pname}-vendor.tar.gz";
#       inherit src;
#       outputHash = "sha256-KHRgieUWFLY/+2lShmVKhaO6Gt763q0GPkFaC4+mKFQ=";
#     });
#   });
# }
