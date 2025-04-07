#!/bin/bash

# Script per compilare il progetto Rust e spostare il binario nella directory radice
set -e

# Funzione per verificare se un comando esiste
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Verifica se Rust (cargo) è installato
if ! command_exists cargo; then
    echo "Errore: Rust non è installato. Per favore, installa Rust da https://www.rust-lang.org/tools/install"
    exit 1
fi

rustup target add x86_64-unknown-linux-musl
cargo clean
cargo update
cargo build --release

# Estrae il nome del pacchetto dal Cargo.toml
PACKAGE_NAME=$(cargo metadata --format-version 1 --no-deps | sed -n 's/.*"name":"\([^"]*\)".*/\1/p' | head -n 1)

# Verifica se il nome del pacchetto è stato estratto correttamente
if [ -z "$PACKAGE_NAME" ]; then
    echo "Errore: impossibile determinare il nome del pacchetto dal Cargo.toml"
    exit 1
fi

# Percorso del binario compilato
BINARY_PATH="target/release/$PACKAGE_NAME"

# Verifica se il binario esiste
if [ ! -f "$BINARY_PATH" ]; then
    echo "Errore: il binario compilato non è stato trovato in $BINARY_PATH"
    exit 1
fi
