BIN=paro
REPO='rafaeldelboni/paro'
INSTALL_DIR="/usr/local/bin"
DOWNLOAD_DIR=''
VERSION=''

case "$(uname -s)" in
    Linux*)  PLATFORM=unknown-linux-gnu;;
    Darwin*) PLATFORM=apple-darwin;;
esac

case "$(uname -m)" in
    x86_64*) ARCH=x86_64;;
    aarch64*) ARCH=aarch64;;
esac

if [[ -z "$DOWNLOAD_DIR" ]]; then
    DOWNLOAD_DIR="$(mktemp -d)"
    trap 'rm -rf "$DOWNLOAD_DIR"' EXIT
fi

if [[ "$version" == "" ]]; then
    VERSION=$(curl -s https://api.github.com/repos/${REPO}/releases/latest | grep -o "\"tag_name\": \".*\"" | cut -d'"' -f4)
fi

FILEEXT=zip
FILENAME=paro-${VERSION}-${ARCH}-${PLATFORM}
FILE=${FILENAME}.${FILEEXT}

DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${FILE}"

if [[ "$FILEEXT" == "zip" ]]; then
    UTIL="$(which unzip) -qqo"
else
    UTIL="$(which tar) -zxf"
fi

# Running this part in a subshell so when it finishes we go back to the previous directory
mkdir -p "$DOWNLOAD_DIR" && (
    cd "$DOWNLOAD_DIR"
    echo -e "Downloading $DOWNLOAD_URL to $DOWNLOAD_DIR"

    curl -o "$FILE" -sL "$DOWNLOAD_URL"
    $UTIL "$FILE"
    rm -f "$FILE"
)

if [[ "$DOWNLOAD_DIR" != "$INSTALL_DIR" ]]
then
    mkdir -p "$INSTALL_DIR"
    if [ -f "$INSTALL_DIR/$BIN" ]; then
        echo "Moving $INSTALL_DIR/$BIN to $INSTALL_DIR/$BIN.old"
        mv -f "$INSTALL_DIR/$BIN" "$INSTALL_DIR/$BIN.old"
    fi
    mv -f "$DOWNLOAD_DIR/$FILENAME/$BIN" "$INSTALL_DIR/$BIN"
    chmod +x "$INSTALL_DIR/$BIN"
fi

echo "Successfully installed $BIN in $INSTALL_DIR"
