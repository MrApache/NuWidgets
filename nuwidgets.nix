{
    stdenv,
    fetchzip,
}:

stdenv.mkDerivation {
    pname = "nuwidgets";
    version = "1.0.0";

    src = {
        url = "*.tar.gz";
        sha256 = "";
    };
}
