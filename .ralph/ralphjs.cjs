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
  NC: '\x1b[0m'
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
   * Count tasks from ralph.yml
   */
  countTasks() {
    try {
      const ymlContent = fs.readFileSync(this.paths.ralphYml, 'utf8');
      const totalTasks = (ymlContent.match(/^  - id:/gm) || []).length;
      const completedTasks = (ymlContent.match(/status: done/gm) || []).length;
      
      return { totalTasks, completedTasks };
    } catch (error) {
      return { totalTasks: 0, completedTasks: 0 };
    }
  }

  /**
   * Build prompt for Claude
   */
  buildPrompt(iteration, sessionId) {
    const { totalTasks, completedTasks } = this.countTasks();
    const completionPct = totalTasks > 0 ? Math.floor((completedTasks * 100) / totalTasks) : 0;

    // Update state
    this.updateState('tasks.total_tasks', totalTasks);
    this.updateState('tasks.completion_percentage', completionPct);

    // Get project name from ralph.yml
    let projectName = 'Project';
    try {
      const ymlContent = fs.readFileSync(this.paths.ralphYml, 'utf8');
      const nameMatch = ymlContent.match(/^name:\s*(.+)$/m);
      if (nameMatch) projectName = nameMatch[1].trim();
    } catch (error) {
      // Use default
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
- **Completion**: ${completedTasks}/${totalTasks} tasks (${completionPct}%)

---

## Previous Scratchpad Notes

${scratchpadNotes}

---

Begin your work on the highest priority task from ralph.yml.
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
      // Build command
      const args = ['--dangerously-skip-permissions', '--output-format', 'json'];
      
      // Read prompt and execute
      const promptText = fs.readFileSync(promptFile, 'utf8');
      const cmd = `${CONFIG.CLAUDE_CMD} ${args.join(' ')} -p "${promptText.replace(/"/g, '\\"')}"`;
      
      const output = execSync(cmd, {
        encoding: 'utf8',
        maxBuffer: 10 * 1024 * 1024,
        timeout: CONFIG.CLAUDE_TIMEOUT_MINUTES * 60 * 1000
      });

      fs.writeFileSync(outputFile, output);

      // Update rate limit
      const callsThisHour = this.getState('rate_limit.calls_this_hour') + 1;
      this.updateState('rate_limit.calls_this_hour', callsThisHour);

      return true;
    } catch (error) {
      this.log('ERROR', `Claude Code failed: ${error.message}`);
      return false;
    }
  }

  /**
   * Create checkpoint
   */
  createCheckpoint(iteration, sessionId) {
    const timestamp = new Date().toISOString().replace(/:/g, '-').split('.')[0];
    const checkpointFile = path.join(this.paths.checkpointsDir, `checkpoint_${timestamp}.txt`);
    
    const scratchpadPreview = fs.existsSync(this.paths.scratchpadFile)
      ? fs.readFileSync(this.paths.scratchpadFile, 'utf8').split('\n').slice(-20).join('\n')
      : 'See scratchpad.md';

    const checkpointContent = `Ralph Loop Checkpoint
=====================
Time: ${new Date().toISOString()}
Iteration: ${iteration}
Session: ${sessionId}
State: ${this.getState('loop.state')}

Project Status:
- Tasks Completed: ${this.getState('tasks.completed_tasks')?.length || 0}
- Completion: ${this.getState('tasks.completion_percentage')}%

Next Steps:
${scratchpadPreview}
`;

    fs.writeFileSync(checkpointFile, checkpointContent);
    this.log('SUCCESS', `Checkpoint created: ${checkpointFile}`);
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
      const status = this.parseRalphStatus(outputFile);

      if (status.exitSignal) {
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