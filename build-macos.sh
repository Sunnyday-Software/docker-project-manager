#!/bin/zsh

# Script per compilare il progetto Rust per macOS
set -e

# Funzione per verificare se un comando esiste
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Verifica se Rust è installato e lo installa se necessario
if ! command_exists cargo || ! command_exists rustup; then
    echo "Rust non è installato. Avvio dell'installazione..."
    chmod +x install-rust-macos.sh
    ./install-rust-macos.sh
fi

# Estrae il nome del pacchetto dal Cargo.toml
PACKAGE_NAME=$(cargo metadata --format-version 1 --no-deps | sed -n 's/.*"name":"\([^"]*\)".*/\1/p' | head -n 1)

# Verifica se il nome del pacchetto è stato estratto correttamente
if [ -z "$PACKAGE_NAME" ]; then
    echo "Errore: impossibile determinare il nome del pacchetto dal Cargo.toml"
    exit 1
fi

# Crea directory per i binari compilati
mkdir -p build

echo "=== Pulizia e aggiornamento del progetto ==="
#cargo clean
cargo update

# Determina l'architettura del sistema
ARCH=$(uname -m)

# Compilazione per macOS (Intel)
    echo "=== Compilazione per macOS (x86_64) ==="
    rustup target add x86_64-apple-darwin
    cargo build --release --target x86_64-apple-darwin

    # Verifica se il binario macOS esiste
    MACOS_BINARY_PATH="target/x86_64-apple-darwin/release/$PACKAGE_NAME"
    if [ -f "$MACOS_BINARY_PATH" ]; then
        echo "Binario macOS (Intel) compilato con successo: $MACOS_BINARY_PATH"
        cp "$MACOS_BINARY_PATH" "build/${PACKAGE_NAME}-macos-x86_64"
    else
        echo "Errore: il binario macOS non è stato trovato in $MACOS_BINARY_PATH"
    fi
# Compilazione per macOS (ARM)

    echo "=== Compilazione per macOS (ARM64) ==="
    rustup target add aarch64-apple-darwin
    cargo build --release --target aarch64-apple-darwin

    # Verifica se il binario macOS ARM esiste
    MACOS_ARM_BINARY_PATH="target/aarch64-apple-darwin/release/$PACKAGE_NAME"
    if [ -f "$MACOS_ARM_BINARY_PATH" ]; then
        echo "Binario macOS (ARM) compilato con successo: $MACOS_ARM_BINARY_PATH"
        cp "$MACOS_ARM_BINARY_PATH" "build/${PACKAGE_NAME}-macos-arm64"
    else
        echo "Errore: il binario macOS ARM non è stato trovato in $MACOS_ARM_BINARY_PATH"
    fi


echo "=== Riepilogo dei binari compilati ==="
ls -la build/

echo ""
echo "La compilazione è stata completata con successo."
