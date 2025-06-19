#!/bin/bash

# Script per installare Rust su Linux
set -e

echo "=== Installazione di Rust per Linux ==="

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

# Aggiorna Rust alla versione più recente
echo "Aggiornamento di Rust..."
rustup update

echo "=== Installazione di Rust completata con successo ==="