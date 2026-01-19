@echo off
REM Ralph Loop Quick Copy Script for Windows
REM Usage: COPY.bat "C:\path\to\target\project"

setlocal

if "%~1"=="" (
    echo Usage: COPY.bat "C:\path\to\target\project"
    echo.
    echo Copies Ralph Loop to the specified project directory.
    echo.
    echo Example:
    echo   COPY.bat "C:\Users\Kimpa\Documents\MyProject"
    goto :eof
)

set TARGET=%~1

echo Copying Ralph Loop to: %TARGET%
echo.

REM Create directories
mkdir "%TARGET%\.ralph\checkpoints" 2>nul
mkdir "%TARGET%\.ralph\history" 2>nul
mkdir "%TARGET%\.ralph\sessions" 2>nul

REM Copy files
copy /Y ".ralph\*.sh" "%TARGET%\.ralph\" >nul 2>&1
copy /Y ".ralph\*.json" "%TARGET%\.ralph\" >nul 2>&1
copy /Y ".ralph\*.md" "%TARGET%\.ralph\" >nul 2>&1
copy /Y "CLAUDE.md" "%TARGET%\" >nul 2>&1
copy /Y "README.md" "%TARGET%\" >nul 2>&1

echo.
echo ============================================
echo Ralph Loop copied to: %TARGET%
echo ============================================
echo.
echo Next steps:
echo   1. cd %TARGET%
echo   2. bash .ralph/init.sh
echo   3. Edit .ralph\ralph.yml
echo   4. bash .ralph\ralph_loop.sh
echo.

endlocal
