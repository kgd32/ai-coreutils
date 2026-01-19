#!/usr/bin/env node

/**
 * Ralph Monitor - Compact TUI for monitoring Ralph Loop
 * Horizontal layout that fits within terminal screen height
 */

const fs = require('fs');
const path = require('path');

// Configuration
const REFRESH_INTERVAL = 2000; // 2 seconds
const PROJECT_ROOT = process.cwd();
const RALPH_DIR = path.join(PROJECT_ROOT, '.ralph');
const STATE_FILE = path.join(RALPH_DIR, 'state.json');

// Colors
const COLORS = {
  RESET: '\x1b[0m',
  BRIGHT: '\x1b[1m',
  DIM: '\x1b[2m',
  RED: '\x1b[31m',
  GREEN: '\x1b[32m',
  YELLOW: '\x1b[33m',
  BLUE: '\x1b[34m',
  MAGENTA: '\x1b[35m',
  CYAN: '\x1b[36m',
  WHITE: '\x1b[37m'
};

class RalphMonitor {
  constructor() {
    this.lastState = null;
    this.updateCount = 0;
    this.startTime = Date.now();
  }

  /**
   * Count tasks from ralph.yml
   */
  countTasks() {
    try {
      const ralphYml = path.join(process.cwd(), '.ralph', 'ralph.yml');
      if (!fs.existsSync(ralphYml)) {
        return { totalTasks: 0, completedTasks: 0, totalSubtasks: 0, completedSubtasks: 0 };
      }

      const ymlContent = fs.readFileSync(ralphYml, 'utf8');

      const idMatches = ymlContent.match(/^[\s]*-[\s]+id:/gm) || [];
      const totalTasks = idMatches.length;
      const completedTasks = (ymlContent.match(/status:[\s]+"?done"?/gm) || []).length;

      let totalSubtasks = 0;
      let completedSubtasks = 0;

      // Find all subtask sections
      // Lookahead matches: start of next task (  - id:), workflows section, or end of string
      // NOTE: No /m flag because we want $ to match only at end of string, not end of line
      const subtaskSections = ymlContent.match(/subtasks:\s*\n([\s\S]*?)(?=\n  - id:|\nworkflows:|$)/g) || [];

      subtaskSections.forEach(section => {
        const items = section.match(/^[\s]+-[\s]+"[^"]*"/gm) || [];
        totalSubtasks += items.length;

        const completed = section.match(/^[\s]+-[\s]+"[^"]*[✓✔]/gm) || [];
        const checked = section.match(/^[\s]+-[\s]+\[x\]/gm) || [];
        const strikethrough = section.match(/^[\s]+-[\s]+"?~~[^~]*~~/gm) || [];

        completedSubtasks += completed.length + checked.length + strikethrough.length;
      });

      return { totalTasks, completedTasks, totalSubtasks, completedSubtasks };
    } catch (error) {
      return { totalTasks: 0, completedTasks: 0, totalSubtasks: 0, completedSubtasks: 0 };
    }
  }

  /**
   * Get terminal dimensions
   */
  getTerminalSize() {
    return {
      width: process.stdout.columns || 80,
      height: process.stdout.rows || 24
    };
  }

  /**
   * Clear screen
   */
  clearScreen() {
    console.clear();
  }

  /**
   * Load state from state.json
   */
  loadState() {
    try {
      if (!fs.existsSync(STATE_FILE)) {
        return null;
      }
      const content = fs.readFileSync(STATE_FILE, 'utf8');
      return JSON.parse(content);
    } catch (error) {
      return { error: error.message };
    }
  }

  /**
   * Format duration
   */
  formatDuration(startTime) {
    const ms = Date.now() - new Date(startTime).getTime();
    const seconds = Math.floor(ms / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);

    if (hours > 0) {
      return `${hours}h ${minutes % 60}m`;
    } else if (minutes > 0) {
      return `${minutes}m ${seconds % 60}s`;
    } else {
      return `${seconds}s`;
    }
  }

  /**
   * Draw compact progress bar
   */
  drawProgressBar(percentage, width = 15) {
    const filled = Math.floor((percentage / 100) * width);
    const empty = width - filled;

    const color = percentage >= 100 ? COLORS.GREEN :
                  percentage >= 50 ? COLORS.CYAN : COLORS.YELLOW;

    return `${color}█${'█'.repeat(filled)}${'░'.repeat(empty)}${COLORS.RESET} ${percentage}%`;
  }

  /**
   * Render a horizontal row of boxes
   */
  renderRow(boxes, totalWidth) {
    // Calculate box widths (evenly distributed)
    const boxWidth = Math.floor((totalWidth - 2) / boxes.length) - 2;
    const actualBoxWidth = boxWidth - 2; // Account for borders

    // Split each box content into lines
    const splitContent = boxes.map(box => {
      const lines = box.content.split('\n');
      return {
        ...box,
        lines
      };
    });

    // Find max lines per row
    const maxLines = Math.max(...splitContent.map(b => b.lines.length));

    // Build output
    let output = '';

    // Top border row
    output += COLORS.CYAN + '╔';
    for (let i = 0; i < boxes.length; i++) {
      output += '═'.repeat(actualBoxWidth);
      if (i < boxes.length - 1) output += '╤';
    }
    output += '╗' + COLORS.RESET + '\n';

    // Title row
    output += COLORS.CYAN + '║';
    for (const box of splitContent) {
      const title = box.title.padEnd(actualBoxWidth, ' ');
      output += COLORS.BRIGHT + title + COLORS.RESET;
      output += COLORS.CYAN + '║';
    }
    output += COLORS.RESET + '\n';

    // Separator
    output += COLORS.CYAN + '╟';
    for (let i = 0; i < boxes.length; i++) {
      output += '─'.repeat(actualBoxWidth);
      if (i < boxes.length - 1) output += '┼';
    }
    output += '╢' + COLORS.RESET + '\n';

    // Content rows
    for (let lineIdx = 0; lineIdx < maxLines; lineIdx++) {
      output += COLORS.CYAN + '║';
      for (const box of splitContent) {
        const line = box.lines[lineIdx] || '';
        // Strip ANSI codes for padding calculation
        const cleanLine = line.replace(/\x1b\[[0-9;]*m/g, '');
        const padding = Math.max(0, actualBoxWidth - cleanLine.length);
        output += COLORS[box.color] + line + ' '.repeat(padding) + COLORS.RESET;
        output += COLORS.CYAN + '║';
      }
      output += '\n';
    }

    // Bottom border
    output += COLORS.CYAN + '╚';
    for (let i = 0; i < boxes.length; i++) {
      output += '═'.repeat(actualBoxWidth);
      if (i < boxes.length - 1) output += '╧';
    }
    output += '╝' + COLORS.RESET + '\n';

    return output;
  }

  /**
   * Render the main dashboard
   */
  render(state) {
    this.clearScreen();

    if (!state) {
      console.log(`${COLORS.RED}${COLORS.BRIGHT}ERROR: state.json not found${COLORS.RESET}`);
      console.log(`${COLORS.DIM}Looking for: ${STATE_FILE}${COLORS.RESET}`);
      return;
    }

    if (state.error) {
      console.log(`${COLORS.RED}${COLORS.BRIGHT}ERROR: ${state.error}${COLORS.RESET}`);
      return;
    }

    const { width, height } = this.getTerminalSize();
    const uptime = this.formatDuration(this.startTime);
    const statusColor = state.loop?.state === 'running' ? COLORS.GREEN :
                       state.loop?.state === 'complete' ? COLORS.CYAN :
                       state.loop?.state === 'error' ? COLORS.RED : COLORS.DIM;

    // Header
    const headerText = `RALPH LOOP MONITOR`;
    const headerPad = ' '.repeat(Math.max(0, width - headerText.length - uptime.length - 10));
    console.log(`${COLORS.CYAN}┌${'─'.repeat(width - 2)}┐${COLORS.RESET}`);
    console.log(`${COLORS.CYAN}│${COLORS.RESET}  ${COLORS.BRIGHT}${headerText}${COLORS.RESET}${headerPad}Uptime: ${COLORS.DIM}${uptime}${COLORS.RESET}  ${COLORS.CYAN}│${COLORS.RESET}`);
    console.log(`${COLORS.CYAN}└${'─'.repeat(width - 2)}┘${COLORS.RESET}\n`);

    // Row 1: Loop Status | Session | Rate Limit (3 columns)
    const loopState = (state.loop?.state || 'unknown').toUpperCase();
    const cbColor = state.circuit_breaker?.state === 'OPEN' ? 'RED' : 'GREEN';
    const ratePct = Math.floor((state.rate_limit?.calls_this_hour || 0) / (state.rate_limit?.max_calls_per_hour || 100) * 100);
    const rateColor = ratePct >= 90 ? 'RED' : ratePct >= 70 ? 'YELLOW' : 'GREEN';

    const row1 = [
      {
        title: 'LOOP',
        color: 'CYAN',
        content: [
          `${statusColor}${loopState}${COLORS.RESET}`,
          `Iter: ${COLORS.CYAN}${state.loop?.iteration || 0}${COLORS.RESET}/${state.loop?.max_iterations || 1000}`,
          `Status: ${state.loop?.last_status || 'none'}`
        ].join('\n')
      },
      {
        title: 'SESSION',
        color: 'MAGENTA',
        content: [
          `${COLORS.DIM}${(state.session?.session_id || 'none').slice(-12)}${COLORS.RESET}`,
          `${state.session?.started_at ? this.formatDuration(state.session.started_at) : 'N/A'} runtime`,
          `Active: ${state.exit_conditions?.completion_indicators || 0}/2 exits`
        ].join('\n')
      },
      {
        title: 'RATE LIMIT',
        color: rateColor,
        content: [
          `${COLORS[rateColor]}${state.rate_limit?.calls_this_hour || 0}${COLORS.RESET}/${state.rate_limit?.max_calls_per_hour || 100}`,
          this.drawProgressBar(ratePct, 12),
          `${COLORS.DIM}resets in ${Math.max(0, 60 - new Date().getMinutes())}m${COLORS.RESET}`
        ].join('\n')
      }
    ];

    console.log(this.renderRow(row1, width));
    console.log();

    // Row 2: Tasks | Circuit Breaker | Exit Conditions (3 columns)
    const taskCounts = this.countTasks();
    const completion = state.tasks?.completion_percentage || 0;
    const subtaskPct = taskCounts.totalSubtasks > 0
      ? Math.floor((taskCounts.completedSubtasks * 100) / taskCounts.totalSubtasks)
      : 0;

    const row2 = [
      {
        title: 'TASKS',
        color: 'GREEN',
        content: [
          `${COLORS.BRIGHT}${taskCounts.completedTasks}/${taskCounts.totalTasks}${COLORS.RESET} tasks`,
          this.drawProgressBar(completion, 12),
          taskCounts.totalSubtasks > 0
            ? `${COLORS.DIM}${taskCounts.completedSubtasks}/${taskCounts.totalSubtasks}${COLORS.RESET} subs\n${this.drawProgressBar(subtaskPct, 12)}`
            : `${COLORS.DIM}No subtasks${COLORS.RESET}`
        ].join('\n')
      },
      {
        title: 'CIRCUIT',
        color: cbColor,
        content: [
          `${COLORS[cbColor]}${state.circuit_breaker?.state || 'CLOSED'}${COLORS.RESET}`,
          `No prog: ${state.circuit_breaker?.consecutive_no_progress || 0}`,
          `Same err: ${state.circuit_breaker?.consecutive_same_error || 0}`
        ].join('\n')
      },
      {
        title: 'EXIT CONDITIONS',
        color: 'YELLOW',
        content: [
          `Test loops: ${state.exit_conditions?.test_only_loops || 0}/3`,
          `Done sigs: ${state.exit_conditions?.done_signals || 0}/2`,
          `Complete:  ${state.exit_conditions?.completion_indicators || 0}/2`
        ].join('\n')
      }
    ];

    console.log(this.renderRow(row2, width));

    // Footer
    const timestamp = new Date().toLocaleTimeString();
    const footerText = `${COLORS.DIM}${timestamp}  |  Updates: ${this.updateCount}  |  Size: ${width}x${height}  |  Ctrl+C to exit${COLORS.RESET}`;
    console.log(`\n${' '.repeat(Math.max(0, width - footerText.replace(/\x1b\[[0-9;]*m/g, '').length))}${footerText}`);
  }

  /**
   * Start monitoring
   */
  start() {
    console.log(`${COLORS.CYAN}${COLORS.BRIGHT}Starting Ralph Monitor...${COLORS.RESET}`);
    console.log(`${COLORS.DIM}Watching: ${STATE_FILE}${COLORS.RESET}\n`);

    // Listen for terminal resize
    process.stdout.on('resize', () => {
      const state = this.loadState();
      this.render(state);
    });

    // Initial render
    const state = this.loadState();
    this.render(state);
    this.updateCount++;

    // Auto-refresh
    this.interval = setInterval(() => {
      const state = this.loadState();
      this.render(state);
      this.updateCount++;
    }, REFRESH_INTERVAL);

    // Handle Ctrl+C
    process.on('SIGINT', () => {
      clearInterval(this.interval);
      this.clearScreen();
      console.log(`\n${COLORS.GREEN}Ralph Monitor stopped.${COLORS.RESET}\n`);
      process.exit(0);
    });
  }
}

// CLI execution
if (require.main === module) {
  const monitor = new RalphMonitor();
  monitor.start();
}

module.exports = RalphMonitor;
