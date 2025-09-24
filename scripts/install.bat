@echo off
setlocal enabledelayedexpansion

:: Cores para output (limitadas no Windows)
set "GREEN=[92m"
set "YELLOW=[93m"
set "RED=[91m"
set "BLUE=[94m"
set "NC=[0m"

:: Função para imprimir mensagens coloridas
echo %BLUE%[INFO]%NC% Iniciando instalação do Activity Inquirer (release pré-compilado)...

:: Verificar se o executável existe
if not exist "acv-inq.exe" (
    echo %RED%[ERROR]%NC% Binário 'acv-inq.exe' não encontrado no diretório atual
    echo %RED%[ERROR]%NC% Este script deve ser executado no diretório do release extraído
    pause
    exit /b 1
)

:: Definir diretórios
set "LOCAL_BIN=%USERPROFILE%\bin"
set "STARTUP_DIR=%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup"

:: Criar diretório bin se não existir
if not exist "%LOCAL_BIN%" (
    echo %BLUE%[INFO]%NC% Criando diretório %LOCAL_BIN%...
    mkdir "%LOCAL_BIN%"
)

:: Copiar executável
echo %BLUE%[INFO]%NC% Instalando executável em %LOCAL_BIN%...
copy "acv-inq.exe" "%LOCAL_BIN%\" >nul
if errorlevel 1 (
    echo %RED%[ERROR]%NC% Falha ao copiar executável
    pause
    exit /b 1
)

:: Copiar ícones se existirem
if exist "logo.png" (
    echo %BLUE%[INFO]%NC% Copiando logo...
    copy "logo.png" "%LOCAL_BIN%\activity-inquirer-logo.png" >nul
)

echo %GREEN%[SUCCESS]%NC% Instalação concluída com sucesso!
echo.

:: Perguntar sobre inicialização automática
echo %BLUE%[INFO]%NC% 🚀 Configuração de Inicialização Automática
echo O modo daemon executa inquéritos automáticos a cada hora.
echo.
set /p "autostart=Deseja configurar o daemon para iniciar automaticamente no boot? (s/N): "

if /i "!autostart!"=="s" (
    echo %BLUE%[INFO]%NC% Configurando inicialização automática...
    
    :: Criar arquivo batch para startup
    echo @echo off > "%STARTUP_DIR%\activity-inquirer-daemon.bat"
    echo cd /d "%LOCAL_BIN%" >> "%STARTUP_DIR%\activity-inquirer-daemon.bat"
    echo start "" "%LOCAL_BIN%\acv-inq.exe" --daemon >> "%STARTUP_DIR%\activity-inquirer-daemon.bat"
    
    echo %GREEN%[SUCCESS]%NC% ✨ Inicialização automática configurada!
    echo %BLUE%[INFO]%NC% O daemon será iniciado automaticamente na próxima reinicialização.
    echo %BLUE%[INFO]%NC% Arquivo criado: %STARTUP_DIR%\activity-inquirer-daemon.bat
) else (
    echo %BLUE%[INFO]%NC% Inicialização automática não configurada.
    echo %BLUE%[INFO]%NC% Você pode executar manualmente: acv-inq --daemon
)

echo.
echo %BLUE%[INFO]%NC% 📋 Resumo da Instalação:
echo   Executável: %LOCAL_BIN%\acv-inq.exe
echo   Banco de dados: %%APPDATA%%\activity-inquirer\
echo.

echo %BLUE%[INFO]%NC% 🎯 Como usar:
echo   1. Modo Visualizador: acv-inq
echo   2. Modo Inquérito: acv-inq --inquiry
echo   3. Modo Daemon: acv-inq --daemon
echo.

echo %BLUE%[INFO]%NC% 🔧 Configuração do PATH:
echo %YELLOW%[WARNING]%NC% Adicione %LOCAL_BIN% ao PATH do Windows para usar globalmente
echo   1. Pressione Win+R, digite 'sysdm.cpl' e pressione Enter
echo   2. Clique em 'Variáveis de Ambiente'
echo   3. Em 'Variáveis do usuário', selecione 'Path' e clique 'Editar'
echo   4. Clique 'Novo' e adicione: %LOCAL_BIN%
echo   5. Clique 'OK' em todas as janelas
echo.
echo %BLUE%[INFO]%NC% Ou execute diretamente: %LOCAL_BIN%\acv-inq.exe
echo.

pause
