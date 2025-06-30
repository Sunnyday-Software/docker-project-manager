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

# Funzione per ottenere la versione corrente di Rust
get_current_rust_version() {
    if command_exists rustc; then
        rustc --version | awk '{print $2}'
    else
        echo ""
    fi
}

# Funzione per confrontare le versioni (restituisce 0 se current >= required)
version_compare() {
    local current="$1"
    local required="$2"

    # Converte le versioni in numeri confrontabili
    current_num=$(echo "$current" | awk -F. '{printf "%d%03d%03d", $1, $2, $3}')
    required_num=$(echo "$required" | awk -F. '{printf "%d%03d%03d", $1, $2, $3}')

    [ "$current_num" -ge "$required_num" ]
}

# Aggiorna il PATH per includere Rust se è già installato ma non nel PATH
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
    echo "Ambiente Rust caricato."
fi

# Verifica se Rust è installato e lo installa se necessario
if ! command_exists cargo || ! command_exists rustup; then
    echo "Rust non è installato. Avvio dell'installazione..."
    chmod +x install-rust-linux.sh
    ./install-rust-linux.sh

    # Aggiorna il PATH per la sessione corrente dopo l'installazione
    if [ -f "$HOME/.cargo/env" ]; then
        source "$HOME/.cargo/env"
        echo "Ambiente Rust aggiornato."
    fi

    # Verifica che cargo e rustup siano ora disponibili
    if ! command_exists cargo || ! command_exists rustup; then
        echo "Errore: Impossibile trovare cargo o rustup nel PATH anche dopo l'installazione."
        echo "Prova ad aprire un nuovo terminale o eseguire manualmente: source $HOME/.cargo/env"
        exit 1
    fi
fi

# Verifica la versione di Rust
REQUIRED_RUST_VERSION=$(get_required_rust_version)
CURRENT_RUST_VERSION=$(get_current_rust_version)

if [ -n "$REQUIRED_RUST_VERSION" ]; then
    echo "Versione Rust richiesta: $REQUIRED_RUST_VERSION"
    echo "Versione Rust corrente: $CURRENT_RUST_VERSION"

    if [ -z "$CURRENT_RUST_VERSION" ]; then
        echo "Errore: impossibile determinare la versione corrente di Rust"
        exit 1
    fi

    if ! version_compare "$CURRENT_RUST_VERSION" "$REQUIRED_RUST_VERSION"; then
        echo "Errore: la versione di Rust installata ($CURRENT_RUST_VERSION) è inferiore alla versione richiesta ($REQUIRED_RUST_VERSION)"
        echo "Aggiorna Rust con: rustup update"
        exit 1
    fi

    echo "✓ Versione di Rust verificata con successo"
else
    echo "Avviso: impossibile determinare la versione Rust richiesta dal Cargo.toml"
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

# Rileva il sistema operativo Linux corrente
get_linux_distro() {
    if [ -f /etc/os-release ]; then
        # freedesktop.org and systemd
        . /etc/os-release
        DISTRO="$NAME $VERSION_ID"
    elif type lsb_release >/dev/null 2>&1; then
        # linuxbase.org
        DISTRO=$(lsb_release -si)
        VERSION=$(lsb_release -sr)
        DISTRO="$DISTRO $VERSION"
    elif [ -f /etc/lsb-release ]; then
        # Per alcune distribuzioni basate su Ubuntu
        . /etc/lsb-release
        DISTRO="$DISTRIB_ID $DISTRIB_RELEASE"
    elif [ -f /etc/debian_version ]; then
        # Debian senza lsb_release
        DISTRO="Debian $(cat /etc/debian_version)"
    elif [ -f /etc/redhat-release ]; then
        # Red hat, CentOS, etc.
        DISTRO=$(cat /etc/redhat-release)
    else
        # Fallback
        DISTRO="$(uname -s) $(uname -r)"
    fi
    echo "$DISTRO"
}

# Ottieni il nome della distribuzione Linux
LINUX_DISTRO=$(get_linux_distro)

# Compilazione standard per il sistema corrente
echo "=== Compilazione standard per il sistema corrente: $LINUX_DISTRO ==="
cargo build --release

# Percorso del binario compilato per il sistema corrente
BINARY_PATH="target/release/$PACKAGE_NAME"

# Verifica se il binario esiste
if [ ! -f "$BINARY_PATH" ]; then
    echo "Errore: il binario compilato non è stato trovato in $BINARY_PATH"
    exit 1
fi

# Formatta il nome della distribuzione per il nome del file:
# - converte tutto in minuscolo
# - sostituisce caratteri non alfanumerici con trattini
FORMATTED_DISTRO=$(echo "$LINUX_DISTRO" | tr '[:upper:]' '[:lower:]' | sed -E 's/[^a-z0-9]+/-/g')

# Copia il binario del sistema corrente nella directory build
cp "$BINARY_PATH" "build/${PACKAGE_NAME}-${FORMATTED_DISTRO}"
echo "Binario per il sistema corrente copiato in build/${PACKAGE_NAME}-${FORMATTED_DISTRO}"

echo "=== Riepilogo dei binari compilati ==="
ls -la build/

echo ""
echo "La compilazione è stata completata con successo."
