{
    stdenv,
    fetchzip,
}:

stdenv.mkDerivation {
    pname = "nuwidgets";
    version = "1.0.0";

    src = {
        url = "https://github.com/MrApache/NuWidgets/archive/refs/tags/Test.tar.gz";
        sha256 = "";
    };
}
