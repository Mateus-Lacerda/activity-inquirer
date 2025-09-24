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

print_status "🍎 Compilando Activity Inquirer para macOS..."

# Verificar se estamos no diretório correto
if [[ ! -f "Cargo.toml" ]]; then
    print_error "Execute este script no diretório raiz do projeto"
    exit 1
fi

# Verificar se o target macOS está instalado
if ! rustup target list --installed | grep -q "x86_64-apple-darwin"; then
    print_status "Instalando target para macOS (Intel)..."
    rustup target add x86_64-apple-darwin
fi

if ! rustup target list --installed | grep -q "aarch64-apple-darwin"; then
    print_status "Instalando target para macOS (Apple Silicon)..."
    rustup target add aarch64-apple-darwin
fi

# Compilar para macOS Intel
print_status "Compilando para macOS Intel (x86_64-apple-darwin)..."
if cargo build --release --target x86_64-apple-darwin; then
    print_success "Compilação para macOS Intel concluída!"
else
    print_warning "Falha na compilação para macOS Intel (pode precisar de Xcode tools)"
fi

# Compilar para macOS Apple Silicon
print_status "Compilando para macOS Apple Silicon (aarch64-apple-darwin)..."
if cargo build --release --target aarch64-apple-darwin; then
    print_success "Compilação para macOS Apple Silicon concluída!"
else
    print_warning "Falha na compilação para macOS Apple Silicon"
fi

# Criar diretório de distribuição
DIST_DIR="dist/macos"
mkdir -p "$DIST_DIR"

# Copiar executáveis se existirem
if [[ -f "target/x86_64-apple-darwin/release/acv-inq" ]]; then
    cp target/x86_64-apple-darwin/release/acv-inq "$DIST_DIR/acv-inq-intel"
    print_success "Executável Intel copiado"
fi

if [[ -f "target/aarch64-apple-darwin/release/acv-inq" ]]; then
    cp target/aarch64-apple-darwin/release/acv-inq "$DIST_DIR/acv-inq-arm64"
    print_success "Executável ARM64 copiado"
fi

# Criar executável universal se ambos existirem
if [[ -f "$DIST_DIR/acv-inq-intel" ]] && [[ -f "$DIST_DIR/acv-inq-arm64" ]]; then
    print_status "Criando executável universal..."
    if command -v lipo &> /dev/null; then
        lipo -create -output "$DIST_DIR/acv-inq" "$DIST_DIR/acv-inq-intel" "$DIST_DIR/acv-inq-arm64"
        print_success "Executável universal criado!"
    else
        print_warning "lipo não disponível. Mantendo executáveis separados."
        # Usar o Intel como padrão se lipo não estiver disponível
        cp "$DIST_DIR/acv-inq-intel" "$DIST_DIR/acv-inq"
    fi
fi

# Copiar assets se existirem
if [[ -d "assets" ]]; then
    cp -r assets "$DIST_DIR/"
fi

# Copiar README
cp README.md "$DIST_DIR/"

# Criar arquivo de informações para macOS
cat > "$DIST_DIR/INSTALL.txt" << EOF
Activity Inquirer - macOS

Para instalar:
1. Copie o executável 'acv-inq' para /usr/local/bin ou ~/.local/bin
   Exemplo: cp acv-inq /usr/local/bin/

2. Torne-o executável:
   chmod +x /usr/local/bin/acv-inq

3. Execute:
   acv-inq --help

Modos disponíveis:
- acv-inq              (Visualizador)
- acv-inq --inquiry    (Inquérito manual)
- acv-inq --daemon     (Daemon automático)

Banco de dados será criado em: ~/Library/Application Support/activity-inquirer/

Executáveis incluídos:
- acv-inq: Universal (Intel + Apple Silicon) ou Intel
- acv-inq-intel: Específico para Intel
- acv-inq-arm64: Específico para Apple Silicon

Nota: No primeiro uso, o macOS pode pedir permissão para executar
o aplicativo. Vá em Preferências > Segurança e Privacidade se necessário.
EOF

# Criar script de instalação para macOS
cat > "$DIST_DIR/install-macos.sh" << 'EOF'
#!/bin/bash
set -e

echo "🍎 Instalando Activity Inquirer no macOS..."

# Verificar se /usr/local/bin existe
if [[ ! -d "/usr/local/bin" ]]; then
    echo "Criando /usr/local/bin..."
    sudo mkdir -p /usr/local/bin
fi

# Copiar executável
echo "Copiando executável..."
sudo cp acv-inq /usr/local/bin/
sudo chmod +x /usr/local/bin/acv-inq

echo "✅ Activity Inquirer instalado com sucesso!"
echo "Execute: acv-inq --help"
EOF

chmod +x "$DIST_DIR/install-macos.sh"

print_success "Distribuição macOS criada em: $DIST_DIR"

# Mostrar tamanhos dos executáveis
if [[ -f "$DIST_DIR/acv-inq-intel" ]]; then
    print_status "Tamanho executável Intel: $(du -h "$DIST_DIR/acv-inq-intel" | cut -f1)"
fi
if [[ -f "$DIST_DIR/acv-inq-arm64" ]]; then
    print_status "Tamanho executável ARM64: $(du -h "$DIST_DIR/acv-inq-arm64" | cut -f1)"
fi
if [[ -f "$DIST_DIR/acv-inq" ]]; then
    print_status "Tamanho executável universal: $(du -h "$DIST_DIR/acv-inq" | cut -f1)"
fi
