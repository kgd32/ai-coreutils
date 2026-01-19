#!/usr/bin/env node

/**
 * Ralph Loop - Autonomous AI Development Loop
 * JavaScript/CommonJS implementation
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

// Configuration
const CONFIG = {
  CLAUDE_CMD: process.env.CLAUDE_CMD || 'claude',
  CLAUDE_TIMEOUT_MINUTES: 15,
  MAX_CALLS_PER_HOUR: 100,
  RALPH_SKIP_PERMISSIONS: process.env.RALPH_SKIP_PERMISSIONS === 'true' || true,
  MAX_CONSECUTIVE_TEST_LOOPS: 3,
  MAX_CONSECUTIVE_DONE_SIGNALS: 2,
  TEST_PERCENTAGE_THRESHOLD: 30
};

// Colors for console output
const COLORS = {
  RED: '\x1b[0;31m',
  GREEN: '\x1b[0;32m',
  YELLOW: '\x1b[1;33m',
  BLUE: '\x1b[0;34m',
  PURPLE: '\x1b[0;35m',
  CYAN: '\x1b[0;36m',
  BRIGHT: '\x1b[1m',
  DIM: '\x1b[2m',
  NC: '\x1b[0m',
  RESET: '\x1b[0m'
};

class RalphLoop {
  constructor(projectRoot) {
    this.projectRoot = projectRoot || path.resolve(__dirname, '..');
    this.ralphDir = path.join(this.projectRoot, '.ralph');

    // File paths
    this.paths = {
      stateFile: path.join(this.ralphDir, 'state.json'),
      ralphYml: path.join(this.ralphDir, 'ralph.yml'),
      sessionFile: path.join(this.ralphDir, 'session.md'),
      scratchpadFile: path.join(this.ralphDir, 'scratchpad.md'),
      promptFileMd: path.join(this.ralphDir, 'prompt.md'),
      claudeMd: path.join(this.projectRoot, 'CLAUDE.md'),
      specMd: path.join(this.projectRoot, 'spec.md'),
      checkpointsDir: path.join(this.ralphDir, 'checkpoints'),
      historyDir: path.join(this.ralphDir, 'history'),
      sessionsDir: path.join(this.ralphDir, 'sessions')
    };
  }

  /**
   * Log with timestamp and color
   */
  log(level, message) {
    const timestamp = new Date().toISOString().replace('T', ' ').substring(0, 19);
    const color = COLORS[level] || COLORS.BLUE;

    console.log(`${color}[${timestamp}] [${level}] ${message}${COLORS.NC}`);

    // Log to file
    const logFile = path.join(
      this.paths.historyDir,
      `ralph-${new Date().toISOString().split('T')[0].replace(/-/g, '')}.log`
    );

    if (fs.existsSync(this.paths.historyDir)) {
      fs.appendFileSync(logFile, `[${timestamp}] [${level}] ${message}\n`);
    }
  }

  /**
   * Initialize .ralph directory structure
   */
  initRalphStructure() {
    this.log('INFO', 'Initializing .ralph directory structure...');

    // Create directories
    [this.paths.checkpointsDir, this.paths.historyDir, this.paths.sessionsDir].forEach(dir => {
      if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
      }
    });

    // Initialize state.json if it doesn't exist
    if (!fs.existsSync(this.paths.stateFile)) {
      const initialState = {
        version: '1.0.0',
        loop: {
          iteration: 0,
          max_iterations: 1000,
          state: 'idle',
          last_run: null,
          last_status: null
        },
        tasks: {
          current_task_id: null,
          completed_tasks: [],
          blocked_tasks: [],
          total_tasks: 0,
          completion_percentage: 0
        },
        session: {
          session_id: null,
          started_at: null,
          last_activity: null,
          expires_at: null
        },
        circuit_breaker: {
          state: 'CLOSED',
          consecutive_no_progress: 0,
          consecutive_same_error: 0,
          last_progress_loop: 0,
          total_opens: 0
        },
        rate_limit: {
          calls_this_hour: 0,
          max_calls_per_hour: CONFIG.MAX_CALLS_PER_HOUR,
          hour_reset_timestamp: null
        },
        exit_conditions: {
          test_only_loops: 0,
          done_signals: 0,
          completion_indicators: 0
        },
        metadata: {
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
          ralph_version: '1.0.0'
        }
      };

      fs.writeFileSync(this.paths.stateFile, JSON.stringify(initialState, null, 2));
      this.log('SUCCESS', 'Created state.json');
    }

    // Check for required files
    if (!fs.existsSync(this.paths.ralphYml)) {
      this.log('ERROR', `ralph.yml not found in ${this.ralphDir}`);
      this.log('INFO', 'Please create ralph.yml with your project tasks');
      process.exit(1);
    }

    if (!fs.existsSync(this.paths.claudeMd)) {
      this.log('WARN', 'CLAUDE.md not found in project root');
    }

    if (!fs.existsSync(this.paths.specMd)) {
      this.log('WARN', 'spec.md not found in project root');
    }
  }

  /**
   * Get state value using dot notation
   */
  getState(field) {
    try {
      const state = JSON.parse(fs.readFileSync(this.paths.stateFile, 'utf8'));
      const keys = field.split('.');
      let value = state;

      for (const key of keys) {
        value = value?.[key];
      }

      return value;
    } catch (error) {
      return null;
    }
  }

  /**
   * Update state value using dot notation
   */
  updateState(field, value) {
    try {
      const state = JSON.parse(fs.readFileSync(this.paths.stateFile, 'utf8'));
      const keys = field.split('.');
      let current = state;

      for (let i = 0; i < keys.length - 1; i++) {
        current = current[keys[i]];
      }

      current[keys[keys.length - 1]] = value;
      state.metadata.updated_at = new Date().toISOString();

      fs.writeFileSync(this.paths.stateFile, JSON.stringify(state, null, 2));
    } catch (error) {
      this.log('ERROR', `Failed to update state: ${error.message}`);
    }
  }

  /**
   * Start new session
   */
  startSession() {
    const sessionId = `ralph_${Date.now()}_${Math.floor(Math.random() * 10000)}`;
    const now = new Date().toISOString();
    const expiresAt = new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString();

    this.updateState('session.session_id', sessionId);
    this.updateState('session.started_at', now);
    this.updateState('session.last_activity', now);
    this.updateState('session.expires_at', expiresAt);

    this.log('SUCCESS', `Started session: ${sessionId}`);

    // Create session log file
    const sessionLog = path.join(this.paths.sessionsDir, `${sessionId}.md`);
    const sessionContent = `# Ralph Session: ${sessionId}

**Started**: ${now}
**Status**: active

---

## Session Log

`;
    fs.writeFileSync(sessionLog, sessionContent);

    return sessionId;
  }

  /**
   * Update session activity timestamp
   */
  updateSessionActivity() {
    const now = new Date().toISOString();
    this.updateState('session.last_activity', now);
  }

  /**
   * Parse RALPH_STATUS block from output
   */
  parseRalphStatus(outputFile) {
    try {
      const output = fs.readFileSync(outputFile, 'utf8');
      const statusMatch = output.match(/---RALPH_STATUS---([\s\S]*?)---END_RALPH_STATUS---/);

      if (!statusMatch) {
        return { exitSignal: false };
      }

      const statusBlock = statusMatch[1];
      const parseField = (field) => {
        const match = statusBlock.match(new RegExp(`^${field}:\\s*(.*)$`, 'm'));
        return match ? match[1].trim() : '';
      };

      const status = {
        status: parseField('STATUS'),
        currentTask: parseField('CURRENT_TASK'),
        tasksCompleted: parseInt(parseField('TASKS_COMPLETED_THIS_LOOP') || '0'),
        filesModified: parseInt(parseField('FILES_MODIFIED') || '0'),
        testsStatus: parseField('TESTS_STATUS'),
        workType: parseField('WORK_TYPE'),
        exitSignal: parseField('EXIT_SIGNAL') === 'true',
        recommendation: parseField('RECOMMENDATION')
      };

      return status;
    } catch (error) {
      this.log('ERROR', `Failed to parse RALPH_STATUS: ${error.message}`);
      return { exitSignal: false };
    }
  }

  /**
   * Parse Claude output JSON for API metrics
   */
  parseClaudeOutput(outputFile) {
    try {
      const content = fs.readFileSync(outputFile, 'utf8');
      const json = JSON.parse(content);

      return {
        duration_ms: json.duration_ms || 0,
        duration_api_ms: json.duration_api_ms || 0,
        num_turns: json.num_turns || 0,
        total_cost_usd: json.total_cost_usd || 0,
        input_tokens: json.usage?.input_tokens || 0,
        output_tokens: json.usage?.output_tokens || 0,
        cache_read_tokens: json.usage?.cache_read_input_tokens || 0
      };
    } catch (error) {
      return null;
    }
  }

  /**
   * Display compact iteration summary
   */
  displayIterationSummary(iteration, ralphStatus, apiMetrics) {
    const width = process.stdout.columns || 120;
    const border = '═'.repeat(width - 2);

    // Status color
    const statusColors = {
      'COMPLETE': COLORS.GREEN,
      'IN_PROGRESS': COLORS.YELLOW,
      'BLOCKED': COLORS.RED
    };
    const statusColor = statusColors[ralphStatus.status] || COLORS.CYAN;

    // Format metrics
    const duration = apiMetrics ? `${(apiMetrics.duration_ms / 1000).toFixed(1)}s` : 'N/A';
    const apiTime = apiMetrics ? `${(apiMetrics.duration_api_ms / 1000).toFixed(1)}s` : 'N/A';
    const cost = apiMetrics ? `$${apiMetrics.total_cost_usd.toFixed(4)}` : 'N/A';
    const tokens = apiMetrics ? `${(apiMetrics.input_tokens / 1000).toFixed(0)}k in / ${(apiMetrics.output_tokens / 1000).toFixed(1)}k out` : 'N/A';
    const cached = apiMetrics ? `${(apiMetrics.cache_read_tokens / 1000).toFixed(0)}k` : 'N/A';
    const turns = apiMetrics ? apiMetrics.num_turns : 'N/A';

    console.log(`\n${COLORS.CYAN}╔${border}╗${COLORS.RESET}`);
    console.log(`${COLORS.CYAN}║${COLORS.RESET} ${COLORS.BRIGHT}ITERATION ${iteration} COMPLETE${COLORS.RESET}${' '.repeat(width - 25)}${COLORS.CYAN}║${COLORS.RESET}`);
    console.log(`${COLORS.CYAN}╠${border}╣${COLORS.RESET}`);

    // Row 1: Status, Task, Work Type
    const row1 = `${COLORS.CYAN}║${COLORS.RESET} Status: ${statusColor}${COLORS.BRIGHT}${ralphStatus.status}${COLORS.RESET}` +
                 ` │ Task: ${COLORS.YELLOW}${ralphStatus.currentTask || 'N/A'}${COLORS.RESET}` +
                 ` │ Type: ${COLORS.MAGENTA}${ralphStatus.workType || 'N/A'}${COLORS.RESET}`;
    const row1Clean = row1.replace(/\x1b\[[0-9;]*m/g, '');
    const row1Padding = width - row1Clean.length + 1;
    console.log(`${row1}${' '.repeat(Math.max(0, row1Padding))}${COLORS.CYAN}║${COLORS.RESET}`);

    // Row 2: Files, Tests, Tasks Completed
    const testsColor = ralphStatus.testsStatus === 'PASSING' ? COLORS.GREEN :
                       ralphStatus.testsStatus === 'FAILING' ? COLORS.RED : COLORS.YELLOW;
    const row2 = `${COLORS.CYAN}║${COLORS.RESET} Files: ${COLORS.BLUE}${ralphStatus.filesModified}${COLORS.RESET}` +
                 ` │ Tests: ${testsColor}${ralphStatus.testsStatus || 'N/A'}${COLORS.RESET}` +
                 ` │ Tasks Done: ${COLORS.GREEN}${ralphStatus.tasksCompleted}${COLORS.RESET}` +
                 ` │ Exit: ${ralphStatus.exitSignal ? COLORS.GREEN + 'YES' : COLORS.DIM + 'no'}${COLORS.RESET}`;
    const row2Clean = row2.replace(/\x1b\[[0-9;]*m/g, '');
    const row2Padding = width - row2Clean.length + 1;
    console.log(`${row2}${' '.repeat(Math.max(0, row2Padding))}${COLORS.CYAN}║${COLORS.RESET}`);

    console.log(`${COLORS.CYAN}╠${border}╣${COLORS.RESET}`);

    // Row 3: API Metrics
    const row3 = `${COLORS.CYAN}║${COLORS.RESET} ${COLORS.DIM}Duration:${COLORS.RESET} ${duration}` +
                 ` │ ${COLORS.DIM}API:${COLORS.RESET} ${apiTime}` +
                 ` │ ${COLORS.DIM}Turns:${COLORS.RESET} ${turns}` +
                 ` │ ${COLORS.DIM}Tokens:${COLORS.RESET} ${tokens}` +
                 ` │ ${COLORS.DIM}Cached:${COLORS.RESET} ${cached}` +
                 ` │ ${COLORS.DIM}Cost:${COLORS.RESET} ${cost}`;
    const row3Clean = row3.replace(/\x1b\[[0-9;]*m/g, '');
    const row3Padding = width - row3Clean.length + 1;
    console.log(`${row3}${' '.repeat(Math.max(0, row3Padding))}${COLORS.CYAN}║${COLORS.RESET}`);

    // Row 4: Recommendation
    if (ralphStatus.recommendation) {
      console.log(`${COLORS.CYAN}╠${border}╣${COLORS.RESET}`);
      const maxRecommendationWidth = width - 6;
      const recommendation = ralphStatus.recommendation.length > maxRecommendationWidth
        ? ralphStatus.recommendation.substring(0, maxRecommendationWidth - 3) + '...'
        : ralphStatus.recommendation;
      const row4 = `${COLORS.CYAN}║${COLORS.RESET} ${COLORS.DIM}→${COLORS.RESET} ${recommendation}`;
      const row4Clean = row4.replace(/\x1b\[[0-9;]*m/g, '');
      const row4Padding = width - row4Clean.length + 1;
      console.log(`${row4}${' '.repeat(Math.max(0, row4Padding))}${COLORS.CYAN}║${COLORS.RESET}`);
    }

    console.log(`${COLORS.CYAN}╚${border}╝${COLORS.RESET}\n`);
  }

  /**
   * Check if exit conditions are met
   */
  checkExitConditions() {
    const exitSignalCount = this.getState('exit_conditions.completion_indicators');
    const testOnlyLoops = this.getState('exit_conditions.test_only_loops');
    const doneSignals = this.getState('exit_conditions.done_signals');

    // Check for explicit exit signal
    if (exitSignalCount >= 2) {
      this.log('SUCCESS', 'Exit signal threshold reached');
      return true;
    }

    // Check for too many test-only loops
    if (testOnlyLoops >= CONFIG.MAX_CONSECUTIVE_TEST_LOOPS) {
      this.log('SUCCESS', 'All features implemented (test-only threshold reached)');
      return true;
    }

    // Check for consecutive done signals
    if (doneSignals >= CONFIG.MAX_CONSECUTIVE_DONE_SIGNALS) {
      this.log('SUCCESS', 'Consecutive done signals threshold reached');
      return true;
    }

    return false;
  }

  /**
   * Count tasks from ralph.yml with subtask tracking
   */
  countTasks() {
    try {
      const ymlContent = fs.readFileSync(this.paths.ralphYml, 'utf8');

      // Match both formats for tasks
      const idMatches = ymlContent.match(/^[\s]*-[\s]+id:/gm) || [];
      const totalTasks = idMatches.length;

      // Count completed tasks
      const completedTasks = (ymlContent.match(/status:[\s]+"?done"?/gm) || []).length;

      // Count subtasks (lines starting with "- " under subtasks:)
      let totalSubtasks = 0;
      let completedSubtasks = 0;

      // Find all subtask sections
      const subtaskSections = ymlContent.match(/subtasks:\s*\n([\s\S]*?)(?=\n[\s]{0,4}\w+:|$)/gm) || [];

      subtaskSections.forEach(section => {
        // Count all subtask items (lines with "- ")
        const items = section.match(/^[\s]+-[\s]+"[^"]*"/gm) || [];
        totalSubtasks += items.length;

        // Count completed subtasks (items with ✓, ✔, [x], or strikethrough ~~)
        const completed = section.match(/^[\s]+-[\s]+"[^"]*[✓✔]/gm) || [];
        const checked = section.match(/^[\s]+-[\s]+\[x\]/gm) || [];
        const strikethrough = section.match(/^[\s]+-[\s]+"?~~[^~]*~~/gm) || [];

        completedSubtasks += completed.length + checked.length + strikethrough.length;
      });

      return {
        totalTasks,
        completedTasks,
        totalSubtasks,
        completedSubtasks
      };
    } catch (error) {
      this.log('ERROR', `Failed to count tasks: ${error.message}`);
      return {
        totalTasks: 0,
        completedTasks: 0,
        totalSubtasks: 0,
        completedSubtasks: 0
      };
    }
  }

  /**
   * Build prompt for Claude
   */
  buildPrompt(iteration, sessionId) {
    const taskStats = this.countTasks();
    const { totalTasks, completedTasks, totalSubtasks, completedSubtasks } = taskStats;
    const completionPct = totalTasks > 0 ? Math.floor((completedTasks * 100) / totalTasks) : 0;
    const subtaskPct = totalSubtasks > 0 ? Math.floor((completedSubtasks * 100) / totalSubtasks) : 0;

    // Update state
    this.updateState('tasks.total_tasks', totalTasks);
    this.updateState('tasks.completion_percentage', completionPct);

    // Get project name from ralph.yml
    let projectName = 'Project';
    try {
      const ymlContent = fs.readFileSync(this.paths.ralphYml, 'utf8');
      // Try both formats: "name:" at root or "project.name:"
      const nameMatch = ymlContent.match(/^[\s]*name:[\s]*["']?([^"'\n]+)["']?/m);
      if (nameMatch) {
        projectName = nameMatch[1].trim();
      }
    } catch (error) {
      this.log('WARN', `Failed to parse project name: ${error.message}`);
    }

    // Get current task (first non-done task)
    let currentTask = 'No active task';
    let currentTaskSubtasks = '';
    try {
      const ymlContent = fs.readFileSync(this.paths.ralphYml, 'utf8');

      // Find all task blocks
      const taskBlocks = ymlContent.split(/^[\s]*-[\s]+id:/gm).slice(1);

      for (const block of taskBlocks) {
        // Check if this task is not done
        if (!block.match(/status:[\s]+"?done"?/)) {
          const idMatch = block.match(/^[\s]*["']?([^"'\n]+)["']?/);
          const titleMatch = block.match(/title:[\s]*["']?([^"'\n]+)["']?/m);
          const statusMatch = block.match(/status:[\s]*["']?([^"'\n]+)["']?/m);

          // Extract subtasks
          const subtasksMatch = block.match(/subtasks:\s*\n([\s\S]*?)(?=\n[\s]{0,4}\w+:|$)/);
          if (subtasksMatch) {
            const subtaskLines = subtasksMatch[1].match(/^[\s]+-[\s]+.+$/gm) || [];
            if (subtaskLines.length > 0) {
              currentTaskSubtasks = '\n  Subtasks:\n' + subtaskLines.map(line => `  ${line.trim()}`).join('\n');
            }
          }

          if (idMatch) {
            const id = idMatch[1].trim();
            const title = titleMatch ? titleMatch[1].trim() : 'Untitled';
            const status = statusMatch ? statusMatch[1].trim() : 'pending';
            currentTask = `${id} - ${title} (${status})${currentTaskSubtasks}`;
            break;
          }
        }
      }
    } catch (error) {
      this.log('WARN', `Failed to parse current task: ${error.message}`);
    }

    // Read prompt template
    let promptContent = '';
    if (fs.existsSync(this.paths.promptFileMd)) {
      promptContent = fs.readFileSync(this.paths.promptFileMd, 'utf8');

      // Replace template variables
      promptContent = promptContent
        .replace(/\{\{PROJECT_NAME\}\}/g, projectName)
        .replace(/\{\{ITERATION\}\}/g, iteration)
        .replace(/\{\{SESSION_ID\}\}/g, sessionId)
        .replace(/\{\{COMPLETION_PERCENTAGE\}\}/g, completionPct);
    }

    // Get previous scratchpad notes
    let scratchpadNotes = 'No previous notes';
    if (fs.existsSync(this.paths.scratchpadFile)) {
      const lines = fs.readFileSync(this.paths.scratchpadFile, 'utf8').split('\n');
      scratchpadNotes = lines.slice(-50).join('\n');
    }

    const prompt = `${promptContent}

---

## Current Loop Context

- **Iteration**: ${iteration}
- **Session**: ${sessionId}
- **Task Progress**: ${completedTasks}/${totalTasks} tasks (${completionPct}%)
- **Subtask Progress**: ${completedSubtasks}/${totalSubtasks} subtasks (${subtaskPct}%)
- **Current Task**:
${currentTask}

---

## Previous Scratchpad Notes

${scratchpadNotes}

---

Begin your work on the highest priority task from ralph.yml.
When you complete a subtask, mark it with ✓ or ~~strikethrough~~.
`;

    return prompt;
  }

  /**
   * Execute Claude Code
   */
  executeClaude(promptFile, iteration) {
    this.log('INFO', `Executing Claude Code (iteration ${iteration})...`);

    const outputFile = path.join(this.paths.historyDir, `claude_output_${iteration}.json`);

    try {
      // Build command - pipe prompt file through stdin
      const args = [
        '--dangerously-skip-permissions',
        '--output-format', 'json',
        '-p', '"Follow instructions and start working!"'
      ];

      // Use shell to pipe the file content to claude
      // This mimics: cat prompt.md | claude --args -p "work on this"
      const isWindows = process.platform === 'win32';
      const catCmd = isWindows ? 'type' : 'cat';
      const cmd = `${catCmd} "${promptFile}" | ${CONFIG.CLAUDE_CMD} ${args.join(' ')}`;

      this.log('INFO', `Executing: ${cmd}`);

      const output = execSync(cmd, {
        encoding: 'utf8',
        maxBuffer: 10 * 1024 * 1024,
        timeout: CONFIG.CLAUDE_TIMEOUT_MINUTES * 60 * 1000,
        shell: true // Important for pipe to work
      });

      fs.writeFileSync(outputFile, output);

      // Update rate limit
      const callsThisHour = this.getState('rate_limit.calls_this_hour') + 1;
      this.updateState('rate_limit.calls_this_hour', callsThisHour);

      return true;
    } catch (error) {
      this.log('ERROR', `Claude Code failed: ${error.message}`);
      if (error.stdout) {
        this.log('ERROR', `stdout: ${error.stdout}`);
      }
      if (error.stderr) {
        this.log('ERROR', `stderr: ${error.stderr}`);
      }
      return false;
    }
  }

  /**
   * Create checkpoint
   */
  createCheckpoint(iteration, sessionId) {
    const timestamp = new Date().toISOString().replace(/:/g, '-').split('.')[0];
    const checkpointFile = path.join(this.paths.checkpointsDir, `checkpoint_${timestamp}.txt`);

    // Ensure checkpoints directory exists
    if (!fs.existsSync(this.paths.checkpointsDir)) {
      fs.mkdirSync(this.paths.checkpointsDir, { recursive: true });
    }

    const scratchpadPreview = fs.existsSync(this.paths.scratchpadFile)
      ? fs.readFileSync(this.paths.scratchpadFile, 'utf8').split('\n').slice(-20).join('\n')
      : 'See scratchpad.md';

    // Get state values safely
    const completedTasks = this.getState('tasks.completed_tasks');
    const completedCount = Array.isArray(completedTasks) ? completedTasks.length : 0;
    const completionPct = this.getState('tasks.completion_percentage') || 0;
    const loopState = this.getState('loop.state') || 'unknown';

    const checkpointContent = `Ralph Loop Checkpoint
=====================
Time: ${new Date().toISOString()}
Iteration: ${iteration}
Session: ${sessionId}
State: ${loopState}

Project Status:
- Tasks Completed: ${completedCount}
- Completion: ${completionPct}%

Next Steps:
${scratchpadPreview}
`;

    try {
      fs.writeFileSync(checkpointFile, checkpointContent);
      this.log('SUCCESS', `Checkpoint created: ${path.basename(checkpointFile)}`);
    } catch (error) {
      this.log('ERROR', `Failed to create checkpoint: ${error.message}`);
    }
  }

  /**
   * Main loop execution
   */
  async run() {
    this.log('LOOP', '=== Ralph Loop Starting ===');
    this.log('INFO', `Project root: ${this.projectRoot}`);
    this.log('INFO', `.ralph directory: ${this.ralphDir}`);

    // Initialize structure
    this.initRalphStructure();

    // Start session
    const sessionId = this.startSession();

    // Get current iteration
    let iteration = this.getState('loop.iteration') || 0;
    const maxIterations = this.getState('loop.max_iterations') || 1000;

    this.log('INFO', `Starting from iteration: ${iteration}`);
    this.log('INFO', `Max iterations: ${maxIterations}`);

    // Main loop
    while (iteration < maxIterations) {
      iteration++;
      this.updateState('loop.iteration', iteration);
      this.updateState('loop.state', 'running');

      this.log('LOOP', `--- Iteration ${iteration} ---`);

      // Check rate limit
      const callsThisHour = this.getState('rate_limit.calls_this_hour');
      if (callsThisHour >= CONFIG.MAX_CALLS_PER_HOUR) {
        this.log('WARN', `Rate limit reached (${callsThisHour}/${CONFIG.MAX_CALLS_PER_HOUR})`);
        this.log('INFO', 'Waiting for hour reset...');
        await new Promise(resolve => setTimeout(resolve, 3600000)); // 1 hour
      }

      // Build prompt
      const promptFile = path.join(this.paths.historyDir, `prompt_${iteration}.md`);
      const prompt = this.buildPrompt(iteration, sessionId);
      fs.writeFileSync(promptFile, prompt);

      // Execute Claude
      if (!this.executeClaude(promptFile, iteration)) {
        this.log('ERROR', 'Claude execution failed');
        this.updateState('loop.last_status', 'error');
        break;
      }

      // Parse output
      const outputFile = path.join(this.paths.historyDir, `claude_output_${iteration}.json`);
      const ralphStatus = this.parseRalphStatus(outputFile);
      const apiMetrics = this.parseClaudeOutput(outputFile);

      // Display iteration summary
      this.displayIterationSummary(iteration, ralphStatus, apiMetrics);

      if (ralphStatus.exitSignal) {
        const count = this.getState('exit_conditions.completion_indicators') + 1;
        this.updateState('exit_conditions.completion_indicators', count);
        this.log('INFO', `Exit signal received (${count}/2)`);
      }

      // Update session activity
      this.updateSessionActivity();

      // Check exit conditions
      if (this.checkExitConditions()) {
        this.log('SUCCESS', 'Exit conditions met, stopping loop');
        this.updateState('loop.state', 'complete');
        this.updateState('loop.last_status', 'success');
        break;
      }

      // Check circuit breaker
      const cbState = this.getState('circuit_breaker.state');
      if (cbState === 'OPEN') {
        this.log('ERROR', 'Circuit breaker is OPEN, stopping loop');
        this.updateState('loop.last_status', 'circuit_open');
        break;
      }

      // Small delay between iterations
      await new Promise(resolve => setTimeout(resolve, 2000));
    }

    // Finalize
    this.log('LOOP', '=== Ralph Loop Complete ===');
    this.log('INFO', `Total iterations: ${iteration}`);

    // Create checkpoint
    this.createCheckpoint(iteration, sessionId);
  }
}

// CLI execution
if (require.main === module) {
  const loop = new RalphLoop(process.cwd());
  loop.run().catch(error => {
    console.error('Fatal error:', error);
    process.exit(1);
  });
}

module.exports = RalphLoop;
