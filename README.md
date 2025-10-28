# Activity Inquirer

[![CI](https://github.com/Mateus-Lacerda/activity-inquirer/workflows/CI/badge.svg)](https://github.com/Mateus-Lacerda/activity-inquirer/actions)
[![Release](https://github.com/Mateus-Lacerda/activity-inquirer/workflows/Release/badge.svg)](https://github.com/Mateus-Lacerda/activity-inquirer/actions)
[![Latest Release](https://img.shields.io/github/v/release/Mateus-Lacerda/activity-inquirer)](https://github.com/Mateus-Lacerda/activity-inquirer/releases/latest)

Uma aplicação em Rust para rastrear atividades pessoais com interface gráfica moderna, modo daemon e funcionalidades avançadas.

## Funcionalidades

- **Modo Inquérito** (`acv-inq --inquiry`): Pergunta sFobre sua atividade atual
  - Primeira pergunta do dia: "O que você está fazendo agora?"
  - Perguntas subsequentes: "Você ainda está fazendo [atividade anterior]?"
- **Modo Visualizador** (`acv-inq`): Interface para visualizar atividades registradas
  - Seletor de data com navegação rápida
  - Exibição detalhada com horários destacados
  - Cálculo de intervalos entre atividades
  - Estatísticas básicas do dia
  - Interface visual aprimorada com grupos
- **Modo Daemon** (`acv-inq --daemon`): Execução automática de inquéritos
  - Primeiro inquérito imediato ao iniciar
  - Inquéritos automáticos no intervalo configurado (1min a 8h)
  - Execução em background
  - Logs informativos com timestamps
- **Modo Configurações** (`acv-inq --settings`): Interface de configuração
  - Configurar intervalo do daemon (1 minuto a 8 horas)
  - Alterar tema (Gruvbox Dark/Light)
  - Configurações salvas automaticamente
  - Restaurar configurações padrão
- **Temas**: Suporte ao tema Gruvbox (Dark/Light)
- **Banco de dados**: SQLite armazenado em `~/.config/activity-inquirer/`

## 📦 Instalação

### 🚀 Instalação Rápida (Recomendada)

1. **Baixe a versão mais recente** dos [releases](https://github.com/user/activity-inquirer/releases/latest)
2. **Escolha o arquivo para seu sistema**:
   - 🐧 Linux: `acv-inq-linux-amd64.tar.gz` (glibc 2.31+)
   - 🍎 macOS: `acv-inq-darwin-amd64.tar.gz` (Intel) ou `acv-inq-darwin-arm64.tar.gz` (M1/M2)
   - 🪟 Windows: `acv-inq-windows-amd64.zip`
3. **Extraia e execute o instalador**:
   ```bash
   # Linux/macOS
   tar -xzf acv-inq-*.tar.gz
   cd acv-inq-*
   ./install.sh

   # Windows
   # Extraia o ZIP e execute install.bat
   ```

   **📦 Cada release inclui:**
   - Binário executável
   - Scripts de instalação
   - Logo em PNG e SVG
   - Documentação (README e LICENSE)

### 🔧 Instalação via Script (Compilação Local)
```bash
# Clonar o repositório
git clone <url-do-repositorio>
cd activity-inquirer

# Executar script de instalação
./scripts/install.sh
```

O script de instalação irá:
- Detectar automaticamente seu sistema operacional (Linux/macOS/Windows)
- Gerar a logo do aplicativo
- Compilar o projeto
- Instalar o executável no local apropriado
- Configurar entrada no menu de aplicações (Linux)
- Perguntar sobre inicialização automática do daemon

### Compilação para Múltiplas Plataformas
```bash
# Compilar para todos os sistemas
./scripts/build/build-all.sh

# Ou compilar para sistema específico
./scripts/build/build-linux.sh
./scripts/build/build-windows.sh
./scripts/build/build-macos.sh
```

### Instalação Manual
```bash
# Clonar o repositório
git clone <url-do-repositorio>
cd activity-inquirer

# Compilar
cargo build --release

# O executável estará em target/release/acv-inq
```

## Uso

### Modo Inquérito
```bash
./target/release/acv-inq --inquiry
```

### Modo Visualizador
```bash
./target/release/acv-inq
```

### Modo Daemon
```bash
# Iniciar daemon (inquéritos automáticos no intervalo configurado)
./target/release/acv-inq --daemon

# Para parar o daemon, use Ctrl+C
```

### Modo Configurações
```bash
# Abrir tela de configurações
./target/release/acv-inq --settings
```

## Estrutura do Projeto

- `src/main.rs` - Ponto de entrada e parsing de argumentos
- `src/database.rs` - Gerenciamento do banco SQLite
- `src/models.rs` - Estruturas de dados
- `src/ui/` - Módulos da interface gráfica
  - `inquiry.rs` - Tela de inquérito
  - `viewer.rs` - Tela de visualização
  - `theme.rs` - Definições de tema Gruvbox
  - `mod.rs` - Módulo principal da UI

## Compatibilidade Cross-Platform

### Sistemas Suportados
- **Linux**: Totalmente suportado com integração desktop
- **macOS**: Suporte completo (Intel + Apple Silicon)
- **Windows**: Suporte completo via MinGW

### Localização do Banco de Dados
- **Linux**: `~/.config/activity-inquirer/activities.db`
- **macOS**: `~/Library/Application Support/activity-inquirer/activities.db`
- **Windows**: `%APPDATA%\activity-inquirer\activities.db`

### Inicialização Automática
- **Linux**: XDG autostart (`~/.config/autostart/`)
- **macOS**: LaunchAgent (`~/Library/LaunchAgents/`)
- **Windows**: Startup folder

## Dependências

- `eframe` / `egui` - Interface gráfica
- `rusqlite` - Banco de dados SQLite
- `clap` - Parsing de argumentos CLI
- `chrono` - Manipulação de datas/horários
- `dirs` - Diretórios do sistema
- `anyhow` - Tratamento de erros
- `tokio` - Runtime assíncrono para o daemon

## Banco de Dados

O banco SQLite é criado automaticamente em `~/.config/activity-inquirer/activities.db` com a seguinte estrutura:

```sql
CREATE TABLE activities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    date TEXT NOT NULL
);
```

## Temas

Atualmente suporta apenas o tema Gruvbox em duas variantes:
- Gruvbox Dark (padrão)
- Gruvbox Light

O tema pode ser alterado no modo visualizador através do seletor no canto superior direito.

## 🚀 Releases Automáticos

O projeto usa GitHub Actions para automação de releases:

### **Release Manual**
```bash
# Criar release patch (1.0.0 → 1.0.1)
./scripts/release.sh patch

# Criar release minor (1.0.0 → 1.1.0)
./scripts/release.sh minor

# Criar release major (1.0.0 → 2.0.0)
./scripts/release.sh major
```

### **Builds Automáticos**
Cada release gera binários para:
- Linux x86_64 (glibc 2.31+)
- macOS x86_64 (Intel)
- macOS aarch64 (Apple Silicon)
- Windows x86_64

### **Compatibilidade Linux**
- **Compatível com**: Ubuntu 20.04+, Debian 11+, CentOS 8+, RHEL 8+
- **Requer**: glibc 2.31 ou superior

### **Funcionalidades do Sistema**
- **Builds paralelos** em runners nativos
- **Changelog automático** baseado em commits
- **Arquivos organizados** por plataforma
- **Scripts de instalação** incluídos
- **Release automático** no GitHub

## 📄 Licença

MIT License - veja o arquivo LICENSE para detalhes.
