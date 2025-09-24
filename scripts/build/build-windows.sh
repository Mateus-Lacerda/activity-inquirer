#!/bin/bash
set -e

# Cores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_status "🪟 Compilando Activity Inquirer para Windows..."

# Verificar se estamos no diretório correto
if [[ ! -f "Cargo.toml" ]]; then
    print_error "Execute este script no diretório raiz do projeto"
    exit 1
fi

# Verificar se o target Windows está instalado
if ! rustup target list --installed | grep -q "x86_64-pc-windows-gnu"; then
    print_status "Instalando target para Windows..."
    rustup target add x86_64-pc-windows-gnu
fi

# Verificar se mingw-w64 está disponível
if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    print_warning "mingw-w64 não encontrado. Instale com:"
    print_warning "  Ubuntu/Debian: sudo apt install mingw-w64"
    print_warning "  Arch: sudo pacman -S mingw-w64-gcc"
    print_warning "  Fedora: sudo dnf install mingw64-gcc"
fi

# Compilar para Windows
print_status "Compilando para Windows (x86_64-pc-windows-gnu)..."
cargo build --release --target x86_64-pc-windows-gnu

if [[ $? -eq 0 ]]; then
    print_success "Compilação para Windows concluída!"
    print_status "Executável disponível em: target/x86_64-pc-windows-gnu/release/acv-inq.exe"
else
    print_error "Falha na compilação para Windows"
    exit 1
fi

# Criar diretório de distribuição
DIST_DIR="dist/windows"
mkdir -p "$DIST_DIR"

# Copiar executável
cp target/x86_64-pc-windows-gnu/release/acv-inq.exe "$DIST_DIR/"

# Copiar assets se existirem
if [[ -d "assets" ]]; then
    cp -r assets "$DIST_DIR/"
fi

# Copiar README
cp README.md "$DIST_DIR/"

# Criar arquivo de informações para Windows
cat > "$DIST_DIR/INSTALL.txt" << EOF
Activity Inquirer - Windows

Para instalar:
1. Copie acv-inq.exe para um diretório de sua escolha
2. Adicione esse diretório ao PATH do Windows (opcional)

Para usar:
- Abra o Prompt de Comando ou PowerShell
- Execute: acv-inq.exe --help

Modos disponíveis:
- acv-inq.exe              (Visualizador)
- acv-inq.exe --inquiry    (Inquérito manual)
- acv-inq.exe --daemon     (Daemon automático)

Banco de dados será criado em: %APPDATA%\activity-inquirer\

Nota: No Windows, o modo daemon pode precisar de permissões especiais
para executar em background.
EOF

# Criar script batch para facilitar execução
cat > "$DIST_DIR/run-daemon.bat" << EOF
@echo off
echo Iniciando Activity Inquirer em modo daemon...
acv-inq.exe --daemon
pause
EOF

cat > "$DIST_DIR/run-inquiry.bat" << EOF
@echo off
echo Executando inquérito...
acv-inq.exe --inquiry
pause
EOF

print_success "Distribuição Windows criada em: $DIST_DIR"
print_status "Tamanho do executável: $(du -h target/x86_64-pc-windows-gnu/release/acv-inq.exe | cut -f1)"
