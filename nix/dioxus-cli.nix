{ pkgs, rustPlatform }:
rustPlatform.buildRustPackage rec {
  pname = "dioxus-cli";
  version = "0.3.2";

  src = pkgs.fetchCrate {
    inherit pname version;
    sha256 = "sha256-8S8zUOb2oiXbJQRgY/g9H2+EW+wWOQugr8+ou34CYPg=";
  };

  nativeBuildInputs = with pkgs; [ pkg-config ];
  buildInputs = with pkgs;
    [ openssl ] ++ pkgs.lib.optionals stdenv.isDarwin [ CoreServices ];

  cargoSha256 = "sha256-sCP8njwYA29XmYu2vfuog0NCL1tZlsZiupkDVImrYCE=";

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
