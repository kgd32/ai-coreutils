@echo off
REM Ralph Loop Status Display Script for Windows
REM Displays Ralph Loop status after git commits

setlocal enabledelayedexpansion

set "RALPH_STATE_FILE=.ralph\state.json"
set "RALPH_SESSION_FILE=.ralph\session.md"
set "RALPH_SCRATCHPAD_FILE=.ralph\scratchpad.md"

REM Check if .ralph folder exists
if not exist ".ralph" (
    echo.
    echo ┌────────────────────────────────────────────────────────────────────┐
    echo │                    ╠╟ Ralph Loop Not Initialized                  │
    echo ├────────────────────────────────────────────────────────────────────┤
    echo │                                                                    │
    echo │  No .ralph^/ folder found.                                         │
    echo │                                                                    │
    echo │  To get started:                                                   │
    echo │    /ralph:init "your project idea"                                 │
    echo │                                                                    │
    echo └────────────────────────────────────────────────────────────────────┘
    echo.
    exit /b 0
)

REM Check if state.json exists
if not exist "%RALPH_STATE_FILE%" (
    echo.
    echo ┌────────────────────────────────────────────────────────────────────┐
    echo │                    ╠╟ Ralph Loop Not Initialized                  │
    echo ├────────────────────────────────────────────────────────────────────┤
    echo │                                                                    │
    echo │  .ralph^/state.json not found.                                     │
    echo │                                                                    │
    echo │  Run /ralph:init to initialize.                                    │
    echo │                                                                    │
    echo └────────────────────────────────────────────────────────────────────┘
    echo.
    exit /b 0
)

REM Parse state.json - simple extraction with PowerShell
for /f "tokens=*" %%a in ('powershell -Command "Get-Content '%RALPH_STATE_FILE%' | ConvertFrom-Json | Select-Object -ExpandProperty iteration" 2^>nul') do set "ITERATION=%%a"
for /f "tokens=*" %%a in ('powershell -Command "Get-Content '%RALPH_STATE_FILE%' | ConvertFrom-Json | Select-Object -ExpandProperty phase" 2^>nul') do set "PHASE=%%a"
for /f "tokens=*" %%a in ('powershell -Command "Get-Content '%RALPH_STATE_FILE%' | ConvertFrom-Json | Select-Object -ExpandProperty current_task_id" 2^>nul') do set "CURRENT_TASK=%%a"
for /f "tokens=*" %%a in ('powershell -Command "Get-Content '%RALPH_STATE_FILE%' | ConvertFrom-Json | Select-Object -ExpandProperty current_task_status" 2^>nul') do set "TASK_STATUS=%%a"
for /f "tokens=*" %%a in ('powershell -Command "Get-Content '%RALPH_STATE_FILE%' | ConvertFrom-Json | Select-Object -ExpandProperty tasks | Select-Object -ExpandProperty total" 2^>nul') do set "TOTAL_TASKS=%%a"
for /f "tokens=*" %%a in ('powershell -Command "Get-Content '%RALPH_STATE_FILE%' | ConvertFrom-Json | Select-Object -ExpandProperty tasks | Select-Object -ExpandProperty done" 2^>nul') do set "DONE_TASKS=%%a"
for /f "tokens=*" %%a in ('powershell -Command "Get-Content '%RALPH_STATE_FILE%' | ConvertFrom-Json | Select-Object -ExpandProperty tasks | Select-Object -ExpandProperty in_progress" 2^>nul') do set "IN_PROGRESS_TASKS=%%a"
for /f "tokens=*" %%a in ('powershell -Command "Get-Content '%RALPH_STATE_FILE%' | ConvertFrom-Json | Select-Object -ExpandProperty tasks | Select-Object -ExpandProperty todo" 2^>nul') do set "TODO_TASKS=%%a"
for /f "tokens=*" %%a in ('powershell -Command "Get-Content '%RALPH_STATE_FILE%' | ConvertFrom-Json | Select-Object -ExpandProperty last_updated" 2^>nul') do set "LAST_UPDATED=%%a"
for /f "tokens=*" %%a in ('powershell -Command "if ((Get-Content '%RALPH_STATE_FILE%' | ConvertFrom-Json).blocked) { 'true' } else { 'false' }" 2^>nul') do set "BLOCKED=%%a"
for /f "tokens=*" %%a in ('powershell -Command "Get-Content '%RALPH_STATE_FILE%' | ConvertFrom-Json | Select-Object -ExpandProperty tech_stack | Select-Object -ExpandProperty language" 2^>nul') do set "TECH_STACK=%%a"

REM Set defaults if parsing failed
if "!ITERATION!"=="" set "ITERATION=N/A"
if "!PHASE!"=="" set "PHASE=N/A"
if "!CURRENT_TASK!"=="" set "CURRENT_TASK=N/A"
if "!TASK_STATUS!"=="" set "TASK_STATUS=N/A"
if "!TOTAL_TASKS!"=="" set "TOTAL_TASKS=N/A"
if "!DONE_TASKS!"=="" set "DONE_TASKS=N/A"
if "!IN_PROGRESS_TASKS!"=="" set "IN_PROGRESS_TASKS=N/A"
if "!TODO_TASKS!"=="" set "TODO_TASKS=N/A"
if "!LAST_UPDATED!"=="" set "LAST_UPDATED=N/A"
if "!BLOCKED!"=="" set "BLOCKED=false"
if "!TECH_STACK!"=="" set "TECH_STACK=Unknown"

REM Format timestamp (simple version)
set "LAST_UPDATED=%LAST_UPDATED:T= UTC%"

REM Calculate progress percentage
if not "%TOTAL_TASKS%"=="N/A" if %TOTAL_TASKS% GTR 0 (
    set /a PROGRESS=DONE_TASKS*100/TOTAL_TASKS
    set /a FILLED=DONE_TASKS*20/TOTAL_TASKS
    set /a EMPTY=20-FILLED
    set "PROGRESS_BAR="
    for /l %%i in (1,1,!FILLED!) do set "PROGRESS_BAR=!PROGRESS_BAR!█"
    for /l %%i in (1,1,!EMPTY!) do set "PROGRESS_BAR=!PROGRESS_BAR!░"
) else (
    set "PROGRESS=N/A"
    set "PROGRESS_BAR=░░░░░░░░░░░░░░░░░░░░"
)

REM Display status
echo.
echo ┌────────────────────────────────────────────────────────────────────┐
echo │                         🤖 Ralph Loop Status                       │
echo ├────────────────────────────────────────────────────────────────────┤
echo │                                                                    │
echo │  Session                                                           │
echo │  ┌────────────────────────────────────────────────────────────┐    │
echo │  │  Iteration: %ITERATION%                                                  │    │
echo │  │  Phase: %PHASE%                                                    │    │
echo │  │  Last Update: %LAST_UPDATED%                    │    │
echo │  └────────────────────────────────────────────────────────────┘    │
echo │                                                                    │
echo │  Current Task                                                      │
echo │  ┌────────────────────────────────────────────────────────────┐    │

if "%BLOCKED%"=="true" (
    echo │  │  🚧 %CURRENT_TASK% (%TASK_STATUS% - BLOCKED^)                       │    │
) else if "%TASK_STATUS%"=="in-progress" (
    echo │  │  ⚑ %CURRENT_TASK% (%TASK_STATUS%)                                 │    │
) else if "%TASK_STATUS%"=="done" (
    echo │  │  ✓ %CURRENT_TASK% (%TASK_STATUS%)                                  │    │
) else (
    echo │  │  ○ %CURRENT_TASK% (%TASK_STATUS%)                                  │    │
)

echo │  └────────────────────────────────────────────────────────────┘    │
echo │                                                                    │
echo │  Tasks Summary                                                     │
echo │  ┌────────────────────────────────────────────────────────────┐    │
echo │  │  Total: %TOTAL_TASKS%  │  Done: %DONE_TASKS%  │  In Progress: %IN_PROGRESS_TASKS%  │  Todo: %TODO_TASKS%       │    │
echo │  └────────────────────────────────────────────────────────────┘    │
echo │                                                                    │
echo │  Overall Progress:  %PROGRESS_BAR%  %PROGRESS%%% (%DONE_TASKS%/%TOTAL_TASKS% tasks^)          │
echo │                                                                    │
echo │  Tech Stack                                                        │
echo │  ┌────────────────────────────────────────────────────────────┐    │
echo │  │  Language: %TECH_STACK%                                             │    │
echo │  └────────────────────────────────────────────────────────────┘    │
echo │                                                                    │

if "%BLOCKED%"=="true" (
    echo │  Blockers                                                          │
    echo │  ┌────────────────────────────────────────────────────────────┐    │
    echo │  │  🚧 Ralph Loop is BLOCKED - check .ralph^/scratchpad.md     │    │
    echo │  └────────────────────────────────────────────────────────────┘    │
    echo │                                                                    │
)

echo └────────────────────────────────────────────────────────────────────┘
echo.

endlocal
