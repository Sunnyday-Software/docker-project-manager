#!/bin/bash

# Script per compilare il progetto Rust per Linux
set -e

# Funzione per verificare se un comando esiste
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Funzione per estrarre la versione Rust richiesta dal Cargo.toml
get_required_rust_version() {
    if [ -f "Cargo.toml" ]; then
        grep "^rust-version" Cargo.toml | sed 's/rust-version = "\(.*\)"/\1/' | tr -d '"'
    else
        echo ""
    fi
}

# Aggiorna il PATH per includere Rust se è già installato ma non nel PATH
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
    echo "Ambiente Rust caricato."
fi

# Ottieni la versione Rust richiesta dal Cargo.toml
REQUIRED_RUST_VERSION=$(get_required_rust_version)
echo "$REQUIRED_RUST_VERSION" > rust-toolchain

# Installa o verifica Rust con la versione richiesta
echo "Verifica e installazione di Rust..."
chmod +x install-rust-linux.sh
if [ -n "$REQUIRED_RUST_VERSION" ]; then
    echo "Versione Rust richiesta: $REQUIRED_RUST_VERSION"
    ./install-rust-linux.sh "$REQUIRED_RUST_VERSION"
else
    echo "Nessuna versione specifica richiesta, uso la versione più recente"
    ./install-rust-linux.sh
fi

# Aggiorna il PATH per la sessione corrente
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
    echo "Ambiente Rust caricato."
fi

# Verifica che cargo e rustup siano disponibili
if ! command_exists cargo || ! command_exists rustup; then
    echo "Errore: Impossibile trovare cargo o rustup nel PATH."
    echo "Prova ad aprire un nuovo terminale o eseguire manualmente: source $HOME/.cargo/env"
    exit 1
fi

echo "✓ Rust configurato correttamente"

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

# Compilazione per Linux
echo "=== Compilazione per Linux (x86_64) ==="
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl

# Verifica se il binario Linux esiste
LINUX_BINARY_PATH="target/x86_64-unknown-linux-musl/release/$PACKAGE_NAME"
if [ -f "$LINUX_BINARY_PATH" ]; then
    echo "Binario Linux compilato con successo: $LINUX_BINARY_PATH"
    cp "$LINUX_BINARY_PATH" "build/${PACKAGE_NAME}-linux-x86_64-musl"
else
    echo "Errore: il binario Linux non è stato trovato in $LINUX_BINARY_PATH"
fi

echo "=== Riepilogo dei binari compilati ==="
ls -la build/

echo ""
echo "La compilazione è stata completata con successo."
