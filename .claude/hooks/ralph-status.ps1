# Ralph Loop Status Display Script for Windows (PowerShell)
# Displays Ralph Loop status after git commits

$ErrorActionPreference = "SilentlyContinue"
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

$ralphStateFile = ".ralph\state.json"

# Check if .ralph folder exists
if (!(Test-Path ".ralph")) {
    Write-Host ""
    Write-Host "+====================================================================+"
    Write-Host "|                    🚫 Ralph Loop Not Initialized                  |"
    Write-Host "+--------------------------------------------------------------------+"
    Write-Host "|                                                                    |"
    Write-Host "|  No .ralph/ folder found.                                          |"
    Write-Host "|                                                                    |"
    Write-Host "|  To get started:                                                   |"
    Write-Host "|    /ralph:init 'your project idea'                                |"
    Write-Host "|                                                                    |"
    Write-Host "+====================================================================+"
    Write-Host ""
    exit 0
}

# Check if state.json exists
if (!(Test-Path $ralphStateFile)) {
    Write-Host ""
    Write-Host "+====================================================================+"
    Write-Host "|                    🚫 Ralph Loop Not Initialized                  |"
    Write-Host "+--------------------------------------------------------------------+"
    Write-Host "|                                                                    |"
    Write-Host "|  .ralph/state.json not found.                                      |"
    Write-Host "|                                                                    |"
    Write-Host "|  Run /ralph:init to initialize.                                    |"
    Write-Host "|                                                                    |"
    Write-Host "+====================================================================+"
    Write-Host ""
    exit 0
}

# Parse state.json
try {
    $state = Get-Content $ralphStateFile -Raw -Encoding UTF8 | ConvertFrom-Json
    $iteration = $state.iteration
    $phase = $state.phase
    $currentTask = $state.current_task_id
    $taskStatus = $state.current_task_status
    $totalTasks = $state.tasks.total
    $doneTasks = $state.tasks.done
    $inProgressTasks = $state.tasks.in_progress
    $todoTasks = $state.tasks.todo
    $lastUpdated = $state.last_updated
    $blocked = $state.blocked
    $techStack = $state.tech_stack.language
} catch {
    Write-Host "Error parsing .ralph/state.json: $_" -ForegroundColor Red
    exit 1
}

# Set defaults
if (-not $iteration) { $iteration = "N/A" }
if (-not $phase) { $phase = "N/A" }
if (-not $currentTask) { $currentTask = "N/A" }
if (-not $taskStatus) { $taskStatus = "N/A" }
if (-not $totalTasks) { $totalTasks = "N/A" }
if (-not $doneTasks) { $doneTasks = "N/A" }
if (-not $inProgressTasks) { $inProgressTasks = "N/A" }
if (-not $todoTasks) { $todoTasks = "N/A" }
if (-not $lastUpdated) { $lastUpdated = "N/A" }
if ($blocked -eq $null) { $blocked = $false }
if (-not $techStack) { $techStack = "Unknown" }

# Format timestamp
if ($lastUpdated -ne "N/A") {
    $lastUpdated = $lastUpdated -replace 'T', ' ' -replace '\.[0-9]*Z$', ' UTC'
}

# Calculate progress percentage
$progress = "N/A"
$progressBar = "...................."
if ($totalTasks -ne "N/A" -and $totalTasks -gt 0) {
    $progress = [math]::Round(($doneTasks / $totalTasks) * 100)
    $filled = [math]::Floor(($doneTasks / $totalTasks) * 20)
    $empty = 20 - $filled
    $progressBar = "#" * $filled + "." * $empty
}

# Current task symbol
if ($blocked -eq $true) {
    $taskSymbol = "[BLOCKED]"
} elseif ($taskStatus -eq "in-progress") {
    $taskSymbol = "[IN-PROGRESS]"
} elseif ($taskStatus -eq "done") {
    $taskSymbol = "[DONE]"
} else {
    $taskSymbol = "[TODO]"
}

# Display status
Write-Host ""
Write-Host "+====================================================================+"
Write-Host "|                         🤖 Ralph Loop Status                       |"
Write-Host "+--------------------------------------------------------------------+"
Write-Host "|                                                                    |"
Write-Host "|  Session                                                           |"
Write-Host "|  +----------------------------------------------------------------+ |"
Write-Host ("|  |  Iteration: {0,-58} |" -f $iteration)
Write-Host ("|  |  Phase: {1,-63} |" -f $phase)
Write-Host ("|  |  Last Update: {2,-54} |" -f $lastUpdated)
Write-Host "|  +----------------------------------------------------------------+ |"
Write-Host "|                                                                    |"
Write-Host "|  Current Task                                                      |"
Write-Host "|  +----------------------------------------------------------------+ |"
Write-Host ("|  |  {0} {1} ({2}){3,46} |" -f $taskSymbol, $currentTask, $taskStatus, "")
Write-Host "|  +----------------------------------------------------------------+ |"
Write-Host "|                                                                    |"
Write-Host "|  Tasks Summary                                                     |"
Write-Host "|  +----------------------------------------------------------------+ |"
Write-Host ("|  |  Total: {0}  |  Done: {1}  |  In Progress: {2}  |  Todo: {3,11} |" -f $totalTasks, $doneTasks, $inProgressTasks, $todoTasks)
Write-Host "|  +----------------------------------------------------------------+ |"
Write-Host "|                                                                    |"
Write-Host ("|  Overall Progress:  {0}  {1}% ({2}/{3} tasks)                    |" -f $progressBar, $progress, $doneTasks, $totalTasks)
Write-Host "|                                                                    |"
Write-Host "|  Tech Stack                                                        |"
Write-Host "|  +----------------------------------------------------------------+ |"
Write-Host ("|  |  Language: {0,-55} |" -f $techStack)
Write-Host "|  +----------------------------------------------------------------+ |"
Write-Host "|                                                                    |"

if ($blocked -eq $true) {
    Write-Host "|  Blockers                                                          |"
    Write-Host "|  +----------------------------------------------------------------+ |"
    Write-Host "|  |  🚧 Ralph Loop is BLOCKED - check .ralph/scratchpad.md     |"
    Write-Host "|  +----------------------------------------------------------------+ |"
    Write-Host "|                                                                    |"
}

Write-Host "+====================================================================+"
Write-Host ""
