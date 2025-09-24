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

print_status "🚀 Compilando Activity Inquirer para todos os sistemas operacionais..."

# Verificar se estamos no diretório correto
if [[ ! -f "Cargo.toml" ]]; then
    print_error "Execute este script no diretório raiz do projeto"
    exit 1
fi

# Limpar distribuições anteriores
if [[ -d "dist" ]]; then
    print_status "Limpando distribuições anteriores..."
    rm -rf dist
fi

# Executar builds individuais
print_status "=== Compilando para Linux ==="
./scripts/build/build-linux.sh

echo
print_status "=== Compilando para Windows ==="
./scripts/build/build-windows.sh

echo
print_status "=== Compilando para macOS ==="
./scripts/build/build-macos.sh

echo
print_success "🎉 Compilação para todos os sistemas concluída!"

# Criar arquivo de resumo
cat > "dist/BUILD_SUMMARY.txt" << EOF
Activity Inquirer - Build Summary
================================

Data da compilação: $(date)
Versão: $(grep version Cargo.toml | head -1 | cut -d'"' -f2)

Distribuições criadas:
- dist/linux/     - Executável para Linux
- dist/windows/   - Executável para Windows (.exe)
- dist/macos/     - Executáveis para macOS (Intel + ARM64)

Cada diretório contém:
- Executável principal
- README.md
- INSTALL.txt com instruções específicas
- Assets (ícones, se disponíveis)

Para testar localmente:
- Linux: ./dist/linux/acv-inq --help
- Windows: ./dist/windows/acv-inq.exe --help (via Wine)
- macOS: ./dist/macos/acv-inq --help

Tamanhos aproximados:
EOF

# Adicionar tamanhos dos executáveis ao resumo
echo >> "dist/BUILD_SUMMARY.txt"
if [[ -f "dist/linux/acv-inq" ]]; then
    echo "Linux: $(du -h dist/linux/acv-inq | cut -f1)" >> "dist/BUILD_SUMMARY.txt"
fi
if [[ -f "dist/windows/acv-inq.exe" ]]; then
    echo "Windows: $(du -h dist/windows/acv-inq.exe | cut -f1)" >> "dist/BUILD_SUMMARY.txt"
fi
if [[ -f "dist/macos/acv-inq" ]]; then
    echo "macOS: $(du -h dist/macos/acv-inq | cut -f1)" >> "dist/BUILD_SUMMARY.txt"
fi

print_status "Resumo da compilação salvo em: dist/BUILD_SUMMARY.txt"

# Mostrar estrutura final
print_status "Estrutura final:"
tree dist/ 2>/dev/null || find dist/ -type f | sort

echo
print_success "✨ Todas as distribuições estão prontas para uso!"
