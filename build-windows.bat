@echo off
setlocal EnableDelayedExpansion
REM Script per compilare il progetto Rust per Windows

REM Verifica se Rust è installato e lo installa se necessario
where cargo >nul 2>&1
where rustup >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo Rust non è installato. Avvio dell'installazione...
    call install-rust-windows.bat
)

REM Estrae il nome del pacchetto dal Cargo.toml
for /f "tokens=2 delims==" %%a in ('findstr /C:"name =" Cargo.toml') do (
    set PACKAGE_NAME=%%a
    set PACKAGE_NAME=!PACKAGE_NAME:"=!
    set PACKAGE_NAME=!PACKAGE_NAME: =!
)

REM Verifica se il nome del pacchetto è stato estratto correttamente
if "%PACKAGE_NAME%"=="" (
    echo Errore: impossibile determinare il nome del pacchetto dal Cargo.toml
    exit /b 1
)

REM Crea directory per i binari compilati
if not exist build mkdir build

echo === Pulizia e aggiornamento del progetto ===
cargo clean
cargo update

REM Compilazione per Windows
echo === Compilazione per Windows (x86_64) ===
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

REM Verifica se il binario Windows esiste
set WINDOWS_BINARY_PATH=target\x86_64-pc-windows-gnu\release\%PACKAGE_NAME%.exe
if exist "%WINDOWS_BINARY_PATH%" (
    echo Binario Windows compilato con successo: %WINDOWS_BINARY_PATH%
    copy "%WINDOWS_BINARY_PATH%" "build\%PACKAGE_NAME%-windows-x86_64.exe"
) else (
    echo Errore: il binario Windows non è stato trovato in %WINDOWS_BINARY_PATH%
)

REM Compilazione standard per il sistema corrente
echo === Compilazione standard per il sistema corrente ===
cargo build --release

REM Percorso del binario compilato per il sistema corrente
set BINARY_PATH=target\release\%PACKAGE_NAME%.exe

REM Verifica se il binario esiste
if not exist "%BINARY_PATH%" (
    echo Errore: il binario compilato non è stato trovato in %BINARY_PATH%
    exit /b 1
)

REM Copia il binario del sistema corrente nella directory build
copy "%BINARY_PATH%" "build\%PACKAGE_NAME%.exe"
echo Binario per il sistema corrente copiato in build\%PACKAGE_NAME%.exe

echo === Riepilogo dei binari compilati ===
dir build

echo.
echo La compilazione è stata completata con successo.
