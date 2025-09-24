# Activity Inquirer {{version}}

## 🚀 Funcionalidades

- **Rastreamento de atividades**: Interface intuitiva para registrar o que você está fazendo
- **Modo daemon**: Execução automática em background com intervalos configuráveis  
- **Visualizador avançado**: Timeline das atividades com horários e intervalos
- **Configurações flexíveis**: Interface gráfica para personalizar comportamento
- **Temas Gruvbox**: Dark e Light themes profissionais
- **Fontes Nerd Font**: FiraCode com ícones e ligaduras embarcadas
- **Cross-platform**: Funciona em Linux, macOS e Windows

## 📦 Downloads

Escolha o arquivo apropriado para seu sistema:

| Sistema | Arquivo | Descrição |
|---------|---------|-----------|
| 🐧 Linux | `acv-inq-linux-x86_64.tar.gz` | Binário para Linux x86_64 |
| 🐧 Linux (musl) | `acv-inq-linux-x86_64-musl.tar.gz` | Binário estático para Linux |
| 🍎 macOS (Intel) | `acv-inq-macos-x86_64.tar.gz` | Binário para macOS Intel |
| 🍎 macOS (Apple Silicon) | `acv-inq-macos-aarch64.tar.gz` | Binário para macOS M1/M2 |
| 🪟 Windows | `acv-inq-windows-x86_64.zip` | Binário para Windows x64 |

## 🛠️ Instalação

### Instalação Automática

1. Baixe o arquivo para seu sistema
2. Extraia o conteúdo
3. Execute o script de instalação:
   - **Linux/macOS**: `./install.sh`
   - **Windows**: `install.bat`

### Instalação Manual

1. Extraia o binário `acv-inq` (ou `acv-inq.exe` no Windows)
2. Copie para um diretório no seu PATH:
   - **Linux**: `~/.local/bin/` ou `/usr/local/bin/`
   - **macOS**: `/usr/local/bin/`
   - **Windows**: `C:\Program Files\ActivityInquirer\` (adicione ao PATH)

## 📖 Uso

```bash
# Visualizar atividades do dia
acv-inq

# Registrar nova atividade
acv-inq --inquiry

# Abrir configurações
acv-inq --settings

# Executar em modo daemon (background)
acv-inq --daemon

# Ver ajuda
acv-inq --help
```

## 🔧 Configuração

O Activity Inquirer salva suas configurações em:
- **Linux**: `~/.config/activity-inquirer/`
- **macOS**: `~/Library/Application Support/activity-inquirer/`
- **Windows**: `%APPDATA%\activity-inquirer\`

## 🐛 Problemas Conhecidos

- No primeiro uso, pode demorar alguns segundos para carregar as fontes
- No Windows, o Windows Defender pode alertar sobre o executável (é um falso positivo)

## 📝 Changelog

{{changelog}}

---

**Tamanho dos binários**: ~17-20MB (incluindo fontes Nerd Font embarcadas)
**Requisitos**: Nenhum! Todos os binários são autônomos.

💡 **Dica**: Use `acv-inq --daemon` para monitoramento automático das suas atividades!
