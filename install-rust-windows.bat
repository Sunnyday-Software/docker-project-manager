@echo off
setlocal EnableDelayedExpansion

REM Script per installare Rust su Windows

echo === Installazione di Rust per Windows ===

REM Verifica se Rust (cargo) è già installato
where cargo >nul 2>&1
if %ERRORLEVEL% equ 0 (
    echo Rust (cargo) è già installato.
) else (
    echo Installazione di Rust...
    
    REM Scarica il programma di installazione di Rust
    echo Scaricamento del programma di installazione di Rust...
    powershell -Command "Invoke-WebRequest -Uri 'https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe' -OutFile 'rustup-init.exe'"
    
    REM Esegui il programma di installazione con opzioni predefinite
    echo Esecuzione del programma di installazione di Rust...
    rustup-init.exe -y --default-toolchain stable --profile minimal
    
    REM Elimina il programma di installazione
    del rustup-init.exe
    
    REM Verifica se l'installazione è riuscita
    where cargo >nul 2>&1
    if %ERRORLEVEL% equ 0 (
        echo Rust è stato installato con successo.
    ) else (
        echo Errore: l'installazione di Rust non è riuscita.
        exit /b 1
    )
)

REM Verifica se rustup è installato
where rustup >nul 2>&1
if %ERRORLEVEL% equ 0 (
    echo rustup è già installato.
) else (
    echo Errore: rustup non è stato installato correttamente.
    exit /b 1
)

REM Aggiorna Rust alla versione più recente
echo Aggiornamento di Rust...
rustup update

echo === Installazione di Rust completata con successo ===