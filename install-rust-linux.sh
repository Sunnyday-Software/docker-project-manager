#!/bin/bash

# Script per installare Rust su Linux
set -e

# Ottieni la versione desiderata dal primo parametro (opzionale)
DESIRED_VERSION="$1"

echo "=== Installazione di Rust per Linux ==="
if [ -n "$DESIRED_VERSION" ]; then
    echo "Versione richiesta: $DESIRED_VERSION"
fi

# Funzione per verificare se un comando esiste
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Verifica se Rust (cargo) è già installato
if command_exists cargo; then
    echo "Rust (cargo) è già installato."
else
    echo "Installazione di Rust..."
    # Installa Rust usando rustup (metodo ufficiale)
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

    # Aggiorna il PATH per la sessione corrente
    source "$HOME/.cargo/env"

    # Verifica se l'installazione è riuscita
    if command_exists cargo; then
        echo "Rust è stato installato con successo."
    else
        echo "Errore: l'installazione di Rust non è riuscita."
        exit 1
    fi
fi

# Verifica se rustup è installato
if command_exists rustup; then
    echo "rustup è già installato."
else
    echo "Errore: rustup non è stato installato correttamente."
    exit 1
fi

# Installa o aggiorna alla versione specifica se richiesta
if [ -n "$DESIRED_VERSION" ]; then
    echo "Installazione della versione specifica: $DESIRED_VERSION"
    rustup toolchain install "$DESIRED_VERSION"
    rustup default "$DESIRED_VERSION"
    echo "Versione $DESIRED_VERSION impostata come default"
else
    echo "Aggiornamento di Rust alla versione più recente..."
    rustup update
fi

echo "=== Installazione di Rust completata con successo ==="
