#!/bin/bash
set -e

# Lire le nom du package principal
PROJECT_NAME=$(grep '^name =' Cargo.toml | head -n1 | cut -d '"' -f2)

TARGETS=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
    "x86_64-pc-windows-gnu"
)

DIST_DIR="dist"
mkdir -p "$DIST_DIR"

# Vérifier si cross est installé
if ! command -v cross &> /dev/null; then
    echo "🔧 Installing cross..."
    cargo install cross
fi

echo "🚀 Building $PROJECT_NAME for multiple targets..."

README_CONTENT=$(cat <<'EOF'
# Installation Instructions

Thank you for downloading the autocompose toolset.

This package includes multiple binaries that help you generate `docker-compose.yml` files from running Docker or Podman containers.

## 🐧 Linux / 🍎 macOS

You can install all the binaries in this archive to your system using either the provided script or Makefile.

### Option 1: Using install.sh

```bash
chmod +x install.sh
sudo ./install.sh
```

### Option 2: Using Makefile

```bash
sudo make install
```

All binaries will be installed to `/usr/local/bin/`.

## 🪟 Windows

For Windows users, manual installation is required:

1. Extract the `.tar.gz` archive using a tool like 7-Zip.
2. Copy the `.exe` files to a directory included in your system `PATH`.
   - Example: `C:\Program Files\autocompose\` or `%USERPROFILE%\bin`
3. Optionally add the binary folder to the PATH variable:
   - Open *System Properties* > *Environment Variables*
   - Edit the `PATH` variable and add the binary folder path
4. Use the tools from Command Prompt or PowerShell.

## ✅ Included Binaries

- `docker-autocompose`
- `podman-autocompose`

(Names may vary depending on the archive.)

---

For help or documentation, visit the [GitHub repo](https://github.com/Drasrax/autocompose-podman-docker).

Happy hacking! 🚀
EOF
)

for TARGET in "${TARGETS[@]}"; do
    echo "🛠️ Building for $TARGET..."
    cross build --release --target "$TARGET"

    TARGET_DIR="package-${TARGET}"
    mkdir -p "$TARGET_DIR"

    RELEASE_PATH="target/$TARGET/release"
    BINARIES=$(find "$RELEASE_PATH" -maxdepth 1 -type f -executable ! -name '*.d')

    for BIN_PATH in $BINARIES; do
        BIN_FILE=$(basename "$BIN_PATH")
        cp "$BIN_PATH" "$TARGET_DIR/"
    done

    # Générer install.sh
    {
        echo "#!/bin/bash"
        echo "set -e"
        echo "echo \"Installing binaries...\""
        for BIN_PATH in $BINARIES; do
            BIN_FILE=$(basename "$BIN_PATH")
            echo "install -Dm755 $BIN_FILE /usr/local/bin/$BIN_FILE"
        done
        echo 'echo "✅ Installation complete."'
    } > "$TARGET_DIR/install.sh"
    chmod +x "$TARGET_DIR/install.sh"

    # Générer Makefile
    {
        echo "install:"
        for BIN_PATH in $BINARIES; do
            BIN_FILE=$(basename "$BIN_PATH")
            echo -e "\tinstall -Dm755 $BIN_FILE /usr/local/bin/$BIN_FILE"
        done
    } > "$TARGET_DIR/Makefile"

    # Ajouter le README.md
    echo "$README_CONTENT" > "$TARGET_DIR/README.md"

    tar -czf "$DIST_DIR/${PROJECT_NAME}-${TARGET}.tar.gz" -C "$TARGET_DIR" .
    rm -rf "$TARGET_DIR"

    echo "✅ Archive created: ${PROJECT_NAME}-${TARGET}.tar.gz"
done

echo "🎉 All builds complete. Archives are in ./$DIST_DIR"
