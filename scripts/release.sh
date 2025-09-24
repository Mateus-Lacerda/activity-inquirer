#!/bin/bash

# Script para criar releases do Activity Inquirer
# Uso: ./scripts/release.sh [patch|minor|major]

set -e

# Cores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Função para logging
log() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Verificar se estamos no diretório correto
if [ ! -f "Cargo.toml" ]; then
    error "Este script deve ser executado na raiz do projeto (onde está o Cargo.toml)"
fi

# Verificar se git está limpo
if [ -n "$(git status --porcelain)" ]; then
    error "Há mudanças não commitadas. Commit ou stash suas mudanças primeiro."
fi

# Verificar se estamos na branch main/master
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ] && [ "$CURRENT_BRANCH" != "master" ]; then
    warn "Você não está na branch main/master. Branch atual: $CURRENT_BRANCH"
    read -p "Continuar mesmo assim? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Determinar tipo de release
RELEASE_TYPE=${1:-patch}
if [ "$RELEASE_TYPE" != "patch" ] && [ "$RELEASE_TYPE" != "minor" ] && [ "$RELEASE_TYPE" != "major" ]; then
    error "Tipo de release inválido. Use: patch, minor ou major"
fi

log "Tipo de release: $RELEASE_TYPE"

# Obter versão atual do Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
log "Versão atual no Cargo.toml: $CURRENT_VERSION"

# Verificar se há tags no repositório
if git describe --tags --abbrev=0 >/dev/null 2>&1; then
    LAST_TAG=$(git describe --tags --abbrev=0)
    log "Última tag no repositório: $LAST_TAG"
else
    log "Nenhuma tag encontrada - este será o primeiro release"
fi

# Calcular nova versão (versão simples sem dependências externas)
IFS='.' read -ra VERSION_PARTS <<< "$CURRENT_VERSION"
MAJOR=${VERSION_PARTS[0]}
MINOR=${VERSION_PARTS[1]}
PATCH=${VERSION_PARTS[2]}

case $RELEASE_TYPE in
    "major")
        MAJOR=$((MAJOR + 1))
        MINOR=0
        PATCH=0
        ;;
    "minor")
        MINOR=$((MINOR + 1))
        PATCH=0
        ;;
    "patch")
        PATCH=$((PATCH + 1))
        ;;
esac

NEW_VERSION="$MAJOR.$MINOR.$PATCH"
NEW_TAG="v$NEW_VERSION"

log "Nova versão: $NEW_VERSION"
log "Nova tag: $NEW_TAG"

# Confirmar com o usuário
echo
echo -e "${BLUE}Resumo do release:${NC}"
echo "  Versão atual: $CURRENT_VERSION"
echo "  Nova versão:  $NEW_VERSION"
echo "  Tipo:         $RELEASE_TYPE"
echo "  Tag:          $NEW_TAG"
echo

read -p "Continuar com o release? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    log "Release cancelado."
    exit 0
fi

# Atualizar versão no Cargo.toml
log "Atualizando Cargo.toml..."
sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
rm Cargo.toml.bak

# Atualizar Cargo.lock
log "Atualizando Cargo.lock..."
cargo check --quiet

# Executar testes
log "Executando testes..."
cargo test --quiet

# Verificar se compila
log "Verificando compilação..."
cargo build --release --quiet

# Commit das mudanças
log "Criando commit de versão..."
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to $NEW_TAG"

# Criar tag
log "Criando tag $NEW_TAG..."
git tag -a "$NEW_TAG" -m "Release $NEW_TAG"

# Push das mudanças e tag
log "Fazendo push das mudanças..."
git push origin "$CURRENT_BRANCH"
git push origin "$NEW_TAG"

log "✅ Release $NEW_TAG criado com sucesso!"
log "🚀 O GitHub Actions irá automaticamente:"
log "   - Compilar binários para todas as plataformas"
log "   - Criar release no GitHub"
log "   - Fazer upload dos arquivos"
log ""
log "🔗 Acompanhe o progresso em: https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\(.*\)\.git/\1/')/actions"

# Abrir página de releases (opcional)
read -p "Abrir página de releases no navegador? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    REPO_URL=$(git config --get remote.origin.url | sed 's/.*github.com[:/]\(.*\)\.git/https:\/\/github.com\/\1/')
    if command -v xdg-open > /dev/null; then
        xdg-open "$REPO_URL/releases"
    elif command -v open > /dev/null; then
        open "$REPO_URL/releases"
    else
        log "Abra manualmente: $REPO_URL/releases"
    fi
fi
