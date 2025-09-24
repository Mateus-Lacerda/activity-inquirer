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

print_status "🐧 Compilando Activity Inquirer para Linux..."

# Verificar se estamos no diretório correto
if [[ ! -f "Cargo.toml" ]]; then
    print_error "Execute este script no diretório raiz do projeto"
    exit 1
fi

# Compilar para Linux (nativo)
print_status "Compilando versão nativa para Linux..."
cargo build --release

if [[ $? -eq 0 ]]; then
    print_success "Compilação para Linux concluída!"
    print_status "Executável disponível em: target/release/acv-inq"
else
    print_error "Falha na compilação para Linux"
    exit 1
fi

# Criar diretório de distribuição
DIST_DIR="dist/linux"
mkdir -p "$DIST_DIR"

# Copiar executável
cp target/release/acv-inq "$DIST_DIR/"

# Copiar assets se existirem
if [[ -d "assets" ]]; then
    cp -r assets "$DIST_DIR/"
fi

# Copiar README
cp README.md "$DIST_DIR/"

# Criar arquivo de informações
cat > "$DIST_DIR/INSTALL.txt" << EOF
Activity Inquirer - Linux

Para instalar:
1. Copie o executável 'acv-inq' para um diretório no seu PATH
   Exemplo: cp acv-inq ~/.local/bin/

2. Torne-o executável:
   chmod +x ~/.local/bin/acv-inq

3. Execute:
   acv-inq --help

Modos disponíveis:
- acv-inq              (Visualizador)
- acv-inq --inquiry    (Inquérito manual)
- acv-inq --daemon     (Daemon automático)

Banco de dados será criado em: ~/.config/activity-inquirer/
EOF

print_success "Distribuição Linux criada em: $DIST_DIR"
print_status "Tamanho do executável: $(du -h target/release/acv-inq | cut -f1)"
