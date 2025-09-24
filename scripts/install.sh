#!/bin/bash
set -e

# Cores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Função para imprimir mensagens coloridas
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Detectar sistema operacional
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "linux"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "macos"
    elif [[ "$OSTYPE" == "cygwin" ]] || [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
        echo "windows"
    else
        echo "unknown"
    fi
}

# Função para perguntar sim/não
ask_yes_no() {
    local question="$1"
    local default="${2:-n}"

    if [[ "$default" == "y" ]]; then
        prompt="[Y/n]"
    else
        prompt="[y/N]"
    fi

    while true; do
        read -p "$question $prompt: " answer
        answer=${answer:-$default}
        case $answer in
            [Yy]* ) return 0;;
            [Nn]* ) return 1;;
            * ) echo "Por favor, responda sim (y) ou não (n).";;
        esac
    done
}

OS=$(detect_os)
print_status "Sistema operacional detectado: $OS"

# Funções para configurar inicialização automática
setup_autostart_linux() {
    local autostart_dir="$HOME/.config/autostart"
    local desktop_file="$autostart_dir/activity-inquirer-daemon.desktop"

    mkdir -p "$autostart_dir"

    cat > "$desktop_file" << EOF
[Desktop Entry]
Type=Application
Name=Activity Inquirer Daemon
Comment=Daemon para inquéritos automáticos de atividades
Exec=$LOCAL_BIN/acv-inq --daemon
Icon=activity-inquirer
Terminal=false
Hidden=false
X-GNOME-Autostart-enabled=true
StartupNotify=false
Categories=Utility;
EOF

    print_success "Inicialização automática configurada para Linux (XDG autostart)"
    print_status "Arquivo criado: $desktop_file"
}

setup_autostart_macos() {
    local plist_dir="$HOME/Library/LaunchAgents"
    local plist_file="$plist_dir/com.activity-inquirer.daemon.plist"

    mkdir -p "$plist_dir"

    cat > "$plist_file" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.activity-inquirer.daemon</string>
    <key>ProgramArguments</key>
    <array>
        <string>$LOCAL_BIN/acv-inq</string>
        <string>--daemon</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>$HOME/Library/Logs/activity-inquirer.log</string>
    <key>StandardErrorPath</key>
    <string>$HOME/Library/Logs/activity-inquirer-error.log</string>
</dict>
</plist>
EOF

    # Carregar o serviço
    launchctl load "$plist_file" 2>/dev/null || true

    print_success "Inicialização automática configurada para macOS (LaunchAgent)"
    print_status "Arquivo criado: $plist_file"
    print_status "Logs em: ~/Library/Logs/activity-inquirer.log"
}

setup_autostart_windows() {
    local startup_dir="$APPDATA/Microsoft/Windows/Start Menu/Programs/Startup"
    local batch_file="$startup_dir/activity-inquirer-daemon.bat"

    # Criar diretório se não existir (no caso de estar rodando via WSL/Cygwin)
    mkdir -p "$startup_dir" 2>/dev/null || true

    cat > "$batch_file" << EOF
@echo off
cd /d "$(dirname "$LOCAL_BIN")"
start "" "$LOCAL_BIN/acv-inq.exe" --daemon
EOF

    print_success "Inicialização automática configurada para Windows"
    print_status "Arquivo criado: $batch_file"
    print_warning "No Windows, você pode precisar ajustar manualmente o caminho"
}

remove_autostart_linux() {
    local autostart_file="$HOME/.config/autostart/activity-inquirer-daemon.desktop"
    if [[ -f "$autostart_file" ]]; then
        rm "$autostart_file"
        print_success "Inicialização automática removida do Linux"
    fi
}

remove_autostart_macos() {
    local plist_file="$HOME/Library/LaunchAgents/com.activity-inquirer.daemon.plist"
    if [[ -f "$plist_file" ]]; then
        launchctl unload "$plist_file" 2>/dev/null || true
        rm "$plist_file"
        print_success "Inicialização automática removida do macOS"
    fi
}

remove_autostart_windows() {
    local batch_file="$APPDATA/Microsoft/Windows/Start Menu/Programs/Startup/activity-inquirer-daemon.bat"
    if [[ -f "$batch_file" ]]; then
        rm "$batch_file"
        print_success "Inicialização automática removida do Windows"
    fi
}

# Verificar se estamos no diretório correto
if [[ ! -f "Cargo.toml" ]] || [[ ! -d "src" ]]; then
    print_error "Este script deve ser executado no diretório raiz do projeto activity-inquirer"
    exit 1
fi

print_status "Iniciando instalação do Activity Inquirer..."

# Definir diretórios baseados no sistema operacional
if [[ "$OS" == "linux" ]]; then
    LOCAL_BIN="$HOME/.local/bin"
    LOCAL_SHARE="$HOME/.local/share"
    APPLICATIONS_DIR="$LOCAL_SHARE/applications"
    ICONS_DIR="$LOCAL_SHARE/icons/hicolor"
elif [[ "$OS" == "macos" ]]; then
    LOCAL_BIN="/usr/local/bin"
    APPLICATIONS_DIR="/Applications"
    ICONS_DIR="$HOME/Library/Application Support/activity-inquirer"
elif [[ "$OS" == "windows" ]]; then
    LOCAL_BIN="$HOME/bin"
    APPLICATIONS_DIR="$HOME/Desktop"
    ICONS_DIR="$HOME/activity-inquirer"
else
    print_error "Sistema operacional não suportado: $OS"
    exit 1
fi

print_status "Criando diretórios necessários..."
mkdir -p "$LOCAL_BIN"
mkdir -p "$APPLICATIONS_DIR"
mkdir -p "$ICONS_DIR/128x128/apps"
mkdir -p "$ICONS_DIR/64x64/apps"
mkdir -p "$ICONS_DIR/48x48/apps"
mkdir -p "$ICONS_DIR/32x32/apps"
mkdir -p "$ICONS_DIR/16x16/apps"
mkdir -p "$ICONS_DIR/scalable/apps"

# Gerar logo se não existir
if [[ ! -f "assets/activity-inquirer.svg" ]]; then
    print_status "Gerando logo..."
    python3 scripts/generate_logo.py
    if [[ $? -ne 0 ]]; then
        print_warning "Falha ao gerar logo automaticamente. Continuando sem ícones..."
    fi
fi

# Compilar o projeto
print_status "Compilando o projeto..."
cargo build --release

if [[ $? -ne 0 ]]; then
    print_error "Falha na compilação do projeto"
    exit 1
fi

# Copiar executável
print_status "Copiando executável para $LOCAL_BIN..."
cp target/release/acv-inq "$LOCAL_BIN/"
chmod +x "$LOCAL_BIN/acv-inq"

# Copiar ícones se existirem
if [[ -d "assets" ]]; then
    print_status "Copiando ícones..."
    
    # SVG (scalable)
    if [[ -f "assets/activity-inquirer.svg" ]]; then
        cp "assets/activity-inquirer.svg" "$ICONS_DIR/scalable/apps/"
        print_success "Ícone SVG copiado"
    fi
    
    # PNGs em diferentes tamanhos
    for size in 16 32 48 64 128; do
        if [[ -f "assets/activity-inquirer-${size}.png" ]]; then
            cp "assets/activity-inquirer-${size}.png" "$ICONS_DIR/${size}x${size}/apps/activity-inquirer.png"
            print_success "Ícone ${size}x${size} copiado"
        elif [[ -f "assets/activity-inquirer.png" ]]; then
            # Usar o PNG padrão se não houver tamanhos específicos
            cp "assets/activity-inquirer.png" "$ICONS_DIR/${size}x${size}/apps/activity-inquirer.png"
        fi
    done
fi

# Criar arquivo .desktop
print_status "Criando entrada de aplicação desktop..."
cat > "$APPLICATIONS_DIR/activity-inquirer.desktop" << EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=Activity Inquirer
Comment=Rastreador de atividades pessoais
Comment[en]=Personal activity tracker
Exec=$LOCAL_BIN/acv-inq
Icon=activity-inquirer
Terminal=false
Categories=Utility;Office;
Keywords=activity;tracker;productivity;time;
StartupNotify=true

Actions=inquiry;

[Desktop Action inquiry]
Name=Fazer Inquérito
Name[en]=Make Inquiry
Exec=$LOCAL_BIN/acv-inq --inquiry
EOF

# Tornar o arquivo .desktop executável
chmod +x "$APPLICATIONS_DIR/activity-inquirer.desktop"

# Atualizar cache de ícones se possível
if command -v gtk-update-icon-cache &> /dev/null; then
    print_status "Atualizando cache de ícones..."
    gtk-update-icon-cache -t "$ICONS_DIR" 2>/dev/null || true
fi

# Atualizar banco de dados de aplicações se possível
if command -v update-desktop-database &> /dev/null; then
    print_status "Atualizando banco de dados de aplicações..."
    update-desktop-database "$APPLICATIONS_DIR" 2>/dev/null || true
fi

print_success "Instalação concluída com sucesso!"
echo

# Perguntar sobre inicialização automática
echo
print_status "🚀 Configuração de Inicialização Automática"
echo "O modo daemon executa inquéritos automáticos a cada hora."
echo

if ask_yes_no "Deseja configurar o daemon para iniciar automaticamente no boot?" "n"; then
    case "$OS" in
        "linux")
            setup_autostart_linux
            ;;
        "macos")
            setup_autostart_macos
            ;;
        "windows")
            setup_autostart_windows
            ;;
    esac

    echo
    print_success "✨ Inicialização automática configurada!"
    print_status "O daemon será iniciado automaticamente na próxima reinicialização."
    print_status "Para desabilitar, execute este script novamente ou remova manualmente."
else
    print_status "Inicialização automática não configurada."
    print_status "Você pode executar manualmente: acv-inq --daemon"
fi

echo
print_status "📋 Resumo da Instalação:"
echo "  Executável: $LOCAL_BIN/acv-inq"
if [[ "$OS" == "linux" ]]; then
    echo "  Aplicação Desktop: $APPLICATIONS_DIR/activity-inquirer.desktop"
fi
echo "  Banco de dados: $(get_db_location)"
echo

print_status "🎯 Como usar:"
echo "  1. Modo Visualizador: acv-inq"
echo "  2. Modo Inquérito: acv-inq --inquiry"
echo "  3. Modo Daemon: acv-inq --daemon"
if [[ "$OS" == "linux" ]]; then
    echo "  4. Menu de aplicações: 'Activity Inquirer'"
fi

echo
if [[ "$OS" == "linux" ]]; then
    print_status "Certifique-se de que $LOCAL_BIN está no seu PATH:"
    echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo
    print_warning "Reinicie sua sessão ou execute 'source ~/.bashrc' para garantir que o PATH seja atualizado."
elif [[ "$OS" == "macos" ]]; then
    print_status "O executável foi instalado em $LOCAL_BIN"
    print_warning "Pode ser necessário reiniciar o terminal para usar o comando 'acv-inq'"
elif [[ "$OS" == "windows" ]]; then
    print_status "O executável foi instalado em $LOCAL_BIN"
    print_warning "Adicione $LOCAL_BIN ao PATH do Windows para usar globalmente"
fi

# Função auxiliar para mostrar localização do banco
get_db_location() {
    case "$OS" in
        "linux") echo "~/.config/activity-inquirer/" ;;
        "macos") echo "~/Library/Application Support/activity-inquirer/" ;;
        "windows") echo "%APPDATA%\\activity-inquirer\\" ;;
    esac
}
