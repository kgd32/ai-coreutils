#!/bin/bash

# Ralph Loop Installer for New Projects
# Copies all Ralph Loop files to a new project

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

echo -e "${CYAN}${BOLD}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}${BOLD}║${NC} ${BOLD}Ralph Loop Installer for New Projects${NC}                     ${CYAN}${BOLD}║${NC}"
echo -e "${CYAN}${BOLD}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if target directory specified
if [[ -z "$1" ]]; then
    TARGET_DIR="$(pwd)"
    echo -e "${YELLOW}No target specified, installing to current directory${NC}"
    echo ""
    echo -n "Continue? [Y/n] "
    read -r response
    if [[ "$response" =~ ^[Nn]$ ]]; then
        echo "Installation cancelled."
        echo ""
        echo "Usage: $0 /path/to/target/project"
        exit 0
    fi
else
    TARGET_DIR="$1"
fi

echo -e "${BLUE}Target directory: ${BOLD}$TARGET_DIR${NC}"
echo ""

# Source directory (where this script is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Verify .ralph directory exists in source
if [[ ! -d "$SCRIPT_DIR/.ralph" ]]; then
    echo -e "${RED}Error: .ralph directory not found in package${NC}"
    exit 1
fi

echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}Scanning target directory for existing files...${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
echo ""

# Track what needs to be created/copied
declare -a FILES_TO_CREATE=()
declare -a FILES_TO_SKIP=()
declare -a FILES_TO_OVERWRITE=()

# Check .ralph directory
if [[ -d "$TARGET_DIR/.ralph" ]]; then
    echo -e "${YELLOW}⊙ .ralph/ directory exists${NC}"
    echo -n "  Overwrite .ralph/? This will replace all scripts [y/N] "
    read -r overwrite
    if [[ "$overwrite" =~ ^[Yy]$ ]]; then
        FILES_TO_OVERWRITE+=(".ralph/")
    else
        FILES_TO_SKIP+=(".ralph/ (keeping existing)")
    fi
else
    FILES_TO_CREATE+=(".ralph/")
fi

# Check individual files
declare -a IMPORTANT_FILES=(
    ".ralph/state.json"
    ".ralph/session.md"
    ".ralph/scratchpad.md"
    ".ralph/ralph.yml"
    "CLAUDE.md"
)

for file in "${IMPORTANT_FILES[@]}"; do
    target_file="$TARGET_DIR/$file"
    if [[ -f "$target_file" ]]; then
        echo -e "${YELLOW}⊙ $file exists${NC}"
        echo -n "  Overwrite? [y/N] "
        read -r overwrite
        if [[ "$overwrite" =~ ^[Yy]$ ]]; then
            FILES_TO_OVERWRITE+=("$file")
        else
            FILES_TO_SKIP+=("$file")
        fi
    else
        FILES_TO_CREATE+=("$file")
    fi
done

echo ""
echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
echo ""

# Summary
if [[ ${#FILES_TO_OVERWRITE[@]} -eq 0 ]] && [[ ${#FILES_TO_CREATE[@]} -eq 0 ]]; then
    echo -e "${GREEN}${BOLD}All files already exist and will be preserved.${NC}"
    echo ""
    echo "Installation skipped. No files will be copied."
    exit 0
fi

# Show summary
if [[ ${#FILES_TO_CREATE[@]} -gt 0 ]]; then
    echo -e "${GREEN}${BOLD}Files to be created:${NC}"
    for file in "${FILES_TO_CREATE[@]}"; do
        echo -e "${GREEN}  + ${file}${NC}"
    done
    echo ""
fi

if [[ ${#FILES_TO_OVERWRITE[@]} -gt 0 ]]; then
    echo -e "${YELLOW}${BOLD}Files to be overwritten:${NC}"
    for file in "${FILES_TO_OVERWRITE[@]}"; do
        echo -e "${YELLOW}  ~ ${file}${NC}"
    done
    echo ""
fi

if [[ ${#FILES_TO_SKIP[@]} -gt 0 ]]; then
    echo -e "${BLUE}${BOLD}Files that will be preserved:${NC}"
    for file in "${FILES_TO_SKIP[@]}"; do
        echo -e "${BLUE}  ✓ ${file}${NC}"
    done
    echo ""
fi

# Confirmation
echo -n "Proceed with installation? [Y/n] "
read -r confirm
if [[ "$confirm" =~ ^[Nn]$ ]]; then
    echo "Installation cancelled."
    exit 0
fi

echo ""
echo -e "${BLUE}Installing Ralph Loop...${NC}"
echo ""

# Create directories
if [[ " ${FILES_TO_CREATE[@]} ${FILES_TO_OVERWRITE[@]} " =~ " \.ralph/ " ]]; then
    mkdir -p "$TARGET_DIR/.ralph/checkpoints"
    mkdir -p "$TARGET_DIR/.ralph/history"
    mkdir -p "$TARGET_DIR/.ralph/sessions"
    echo -e "${GREEN}✓ Created .ralph/ directory structure${NC}"
fi

# Copy scripts (always copy these, but only if .ralph/ is being created/overwritten)
if [[ " ${FILES_TO_CREATE[@]} ${FILES_TO_OVERWRITE[@]} " =~ " \.ralph/ " ]]; then
    cp "$SCRIPT_DIR/.ralph/ralph_loop.sh" "$TARGET_DIR/.ralph/" 2>/dev/null && echo -e "${GREEN}✓ Copied ralph_loop.sh${NC}"
    cp "$SCRIPT_DIR/.ralph/monitor.sh" "$TARGET_DIR/.ralph/" 2>/dev/null && echo -e "${GREEN}✓ Copied monitor.sh${NC}"
    cp "$SCRIPT_DIR/.ralph/response_analyzer.sh" "$TARGET_DIR/.ralph/" 2>/dev/null && echo -e "${GREEN}✓ Copied response_analyzer.sh${NC}"
    cp "$SCRIPT_DIR/.ralph/circuit_breaker.sh" "$TARGET_DIR/.ralph/" 2>/dev/null && echo -e "${GREEN}✓ Copied circuit_breaker.sh${NC}"
    cp "$SCRIPT_DIR/.ralph/init.sh" "$TARGET_DIR/.ralph/" 2>/dev/null && echo -e "${GREEN}✓ Copied init.sh${NC}"

    # Make executable
    chmod +x "$TARGET_DIR/.ralph"/*.sh 2>/dev/null
fi

# Copy state files
if [[ " ${FILES_TO_CREATE[@]} ${FILES_TO_OVERWRITE[@]} " =~ " state\.json " ]]; then
    if [[ ! -f "$TARGET_DIR/.ralph/state.json" ]] || [[ " ${FILES_TO_OVERWRITE[@]} " =~ " state\.json " ]]; then
        cp "$SCRIPT_DIR/.ralph/state.json" "$TARGET_DIR/.ralph/"
        echo -e "${GREEN}✓ Created state.json${NC}"
    fi
fi

if [[ " ${FILES_TO_CREATE[@]} ${FILES_TO_OVERWRITE[@]} " =~ " session\.md " ]]; then
    if [[ ! -f "$TARGET_DIR/.ralph/session.md" ]] || [[ " ${FILES_TO_OVERWRITE[@]} " =~ " session\.md " ]]; then
        cp "$SCRIPT_DIR/.ralph/session.md" "$TARGET_DIR/.ralph/"
        echo -e "${GREEN}✓ Created session.md${NC}"
    fi
fi

if [[ " ${FILES_TO_CREATE[@]} ${FILES_TO_OVERWRITE[@]} " =~ " scratchpad\.md " ]]; then
    if [[ ! -f "$TARGET_DIR/.ralph/scratchpad.md" ]] || [[ " ${FILES_TO_OVERWRITE[@]} " =~ " scratchpad\.md " ]]; then
        cp "$SCRIPT_DIR/.ralph/scratchpad.md" "$TARGET_DIR/.ralph/"
        echo -e "${GREEN}✓ Created scratchpad.md${NC}"
    fi
fi

if [[ " ${FILES_TO_CREATE[@]} ${FILES_TO_OVERWRITE[@]} " =~ " ralph\.yml " ]]; then
    if [[ ! -f "$TARGET_DIR/.ralph/ralph.yml" ]] || [[ " ${FILES_TO_OVERWRITE[@]} " =~ " ralph\.yml " ]]; then
        if [[ -f "$SCRIPT_DIR/.ralph/ralph.yml.template" ]]; then
            cp "$SCRIPT_DIR/.ralph/ralph.yml.template" "$TARGET_DIR/.ralph/ralph.yml"
            echo -e "${YELLOW}⊙ Created ralph.yml from template${NC}"
        else
            echo -e "${YELLOW}⊙ Skipped ralph.yml (no template found)${NC}"
        fi
    fi
fi

if [[ " ${FILES_TO_CREATE[@]} ${FILES_TO_OVERWRITE[@]} " =~ " CLAUDE\.md " ]]; then
    if [[ ! -f "$TARGET_DIR/CLAUDE.md" ]] || [[ " ${FILES_TO_OVERWRITE[@]} " =~ " CLAUDE\.md " ]]; then
        cp "$SCRIPT_DIR/CLAUDE.md" "$TARGET_DIR/"
        echo -e "${GREEN}✓ Created CLAUDE.md${NC}"
    fi
fi

echo ""
echo -e "${GREEN}${BOLD}Installation Complete!${NC}"
echo ""
echo -e "${BLUE}Target directory: ${BOLD}$TARGET_DIR${NC}"
echo ""
echo -e "${BLUE}Next steps:${NC}"
echo "  1. cd $TARGET_DIR"
echo "  2. .ralph/init.sh                    # Initialize (optional)"
echo "  3. Edit .ralph/ralph.yml             # Add your tasks"
echo "  4. .ralph/ralph_loop.sh              # Start the loop"
echo "  5. .ralph/monitor.sh                 # Monitor progress"
echo ""
echo -e "${BLUE}Quick commands:${NC}"
echo "  .ralph/ralph_loop.sh                 # Start autonomous loop"
echo "  .ralph/monitor.sh                    # Visual dashboard"
echo "  .ralph/monitor.sh --json             # JSON streaming"
echo "  .ralph/circuit_breaker.sh status     # Circuit breaker status"
echo ""
