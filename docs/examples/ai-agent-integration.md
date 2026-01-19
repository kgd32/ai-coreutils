# AI Agent Integration Examples

Examples of integrating AI-Coreutils with AI agents and LLMs.

## Table of Contents

1. [LLM Integration](#llm-integration)
2. [Agent Workflows](#agent-workflows)
3. [Data Preparation](#data-preparation)
4. [Result Parsing](#result-parsing)

## LLM Integration

### Preparing Context for LLMs

```python
import json
import subprocess

def prepare_file_context(directory: str, max_files: int = 10) -> str:
    """Prepare file context for LLM processing."""
    context = []

    # List files with metadata
    result = subprocess.run(['ai-ls', '-l', directory],
                          capture_output=True, text=True)

    for line in result.stdout.splitlines():
        record = json.loads(line)
        if record.get('type') == 'file_entry':
            context.append(f"- {record['path']} ({record['size']} bytes)")

    return "\n".join(context[:max_files])


def prepare_code_summary(directory: str) -> str:
    """Prepare code summary for code analysis."""
    summary = []

    # Find all source files
    result = subprocess.run(['ai-find', directory, '-name', '*.rs'],
                          capture_output=True, text=True)

    for line in result.stdout.splitlines():
        record = json.loads(line)
        path = record.get('path', '')

        # Get line count
        wc_result = subprocess.run(['ai-wc', '-l', path],
                                  capture_output=True, text=True)
        summary.append(f"{path}: {wc_result.stdout}")

    return "\n".join(summary)
```

### Structured Output for LLMs

```python
import json

def format_llm_response(data: dict) -> str:
    """Format structured data for LLM consumption."""
    return json.dumps({
        "type": "analysis",
        "timestamp": datetime.utcnow().isoformat(),
        "data": data
    })

def parse_llm_instructions(text: str) -> dict:
    """Parse LLM instructions into structured commands."""
    import re

    commands = []
    for line in text.split('\n'):
        if line.strip().startswith('ai-'):
            parts = line.split()
            commands.append({
                "utility": parts[0],
                "args": parts[1:]
            })

    return {"commands": commands}
```

## Agent Workflows

### File Analysis Agent

```python
class FileAnalysisAgent:
    def __init__(self):
        self.tools = {
            'list_files': self.list_files,
            'read_file': self.read_file,
            'search': self.search,
            'analyze': self.analyze
        }

    def list_files(self, directory: str) -> list:
        """List files in directory with metadata."""
        result = subprocess.run(['ai-ls', '-l', directory],
                              capture_output=True, text=True)
        files = []
        for line in result.stdout.splitlines():
            record = json.loads(line)
            files.append(record)
        return files

    def read_file(self, path: str) -> str:
        """Read file contents."""
        result = subprocess.run(['ai-cat', path],
                              capture_output=True, text=True)
        return result.stdout

    def search(self, pattern: str, path: str) -> list:
        """Search for pattern in files."""
        result = subprocess.run(['ai-grep', pattern, path],
                              capture_output=True, text=True)
        matches = []
        for line in result.stdout.splitlines():
            record = json.loads(line)
            matches.append(record)
        return matches

    def analyze(self, path: str) -> dict:
        """Analyze file with AI features."""
        result = subprocess.run(['ai-analyze', '-p', '-c', '-s', path],
                              capture_output=True, text=True)
        analysis = {"patterns": [], "classification": {}, "statistics": {}}

        for line in result.stdout.splitlines():
            record = json.loads(line)
            if record.get('type') == 'result':
                data = record.get('data', {})
                if data.get('type') == 'analysis':
                    analysis['patterns'].append(data)
                elif data.get('type') == 'classification':
                    analysis['classification'] = data

        return analysis

    def process_request(self, request: dict) -> dict:
        """Process agent request."""
        tool = request.get('tool')
        params = request.get('params', {})

        if tool in self.tools:
            return {"result": self.tools[tool](**params)}
        else:
            return {"error": f"Unknown tool: {tool}"}
```

### Code Review Agent

```python
class CodeReviewAgent:
    def review_code(self, directory: str) -> dict:
        """Review code in directory."""
        review = {
            "files_reviewed": 0,
            "issues_found": [],
            "patterns_detected": {},
            "statistics": {}
        }

        # Find all source files
        result = subprocess.run(['ai-find', directory, '-name', '*.py'],
                              capture_output=True, text=True)

        for line in result.stdout.splitlines():
            record = json.loads(line)
            file_path = record.get('path')

            if file_path:
                # Search for common issues
                todo_result = subprocess.run(['ai-grep', '-n', 'TODO', file_path],
                                           capture_output=True, text=True)
                if todo_result.stdout:
                    review['issues_found'].append({
                        "file": file_path,
                        "type": "TODO",
                        "matches": self._parse_matches(todo_result.stdout)
                    })

                # Analyze file
                analyze_result = subprocess.run(['ai-analyze', '-p', '-s', file_path],
                                              capture_output=True, text=True)
                analysis = self._parse_analysis(analyze_result.stdout)
                review['patterns_detected'][file_path] = analysis.get('patterns', {})
                review['statistics'][file_path] = analysis.get('statistics', {})

                review['files_reviewed'] += 1

        return review

    def _parse_matches(self, output: str) -> list:
        matches = []
        for line in output.splitlines():
            if line.strip():
                try:
                    record = json.loads(line)
                    matches.append(record)
                except json.JSONDecodeError:
                    pass
        return matches

    def _parse_analysis(self, output: str) -> dict:
        analysis = {"patterns": {}, "statistics": {}}
        for line in output.splitlines():
            try:
                record = json.loads(line)
                if record.get('type') == 'result':
                    data = record.get('data', {})
                    if data.get('type') == 'analysis':
                        analysis['patterns'] = data.get('patterns_by_type', {})
                        analysis['statistics'] = data.get('statistics', {})
            except json.JSONDecodeError:
                pass
        return analysis
```

## Data Preparation

### Context Window Management

```python
def prepare_context_for_llm(
    directory: str,
    max_tokens: int = 4000,
    avg_chars_per_token: int = 4
) -> str:
    """Prepare context within LLM token limits."""
    max_chars = max_tokens * avg_chars_per_token
    context_parts = []
    current_chars = 0

    # Get file list
    result = subprocess.run(['ai-ls', '-R', directory],
                          capture_output=True, text=True)

    files = []
    for line in result.stdout.splitlines():
        record = json.loads(line)
        if record.get('type') == 'file_entry':
            files.append(record)

    # Add files until we hit token limit
    for file_record in files:
        file_path = file_record['path']
        file_size = file_record['size']

        # Estimate token count
        estimated_tokens = file_size // avg_chars_per_token

        if current_chars + file_size > max_chars:
            # Read just the beginning of the file
            head_result = subprocess.run(['ai-head', '-c', str(max_chars - current_chars), file_path],
                                        capture_output=True, text=True)
            context_parts.append(f"# {file_path}\n{head_result.stdout}")
            break

        # Read entire file
        cat_result = subprocess.run(['ai-cat', file_path],
                                  capture_output=True, text=True)
        context_parts.append(f"# {file_path}\n{cat_result.stdout}")
        current_chars += file_size

    return "\n\n".join(context_parts)
```

### Structured Data Extraction

```python
def extract_structured_data(file_path: str) -> dict:
    """Extract structured data using ai-analyze."""
    result = subprocess.run(['ai-analyze', '-p', '-c', '-s', '-v', file_path],
                          capture_output=True, text=True)

    structured_data = {
        "file": file_path,
        "patterns": {},
        "classification": {},
        "statistics": {},
        "matches": []
    }

    for line in result.stdout.splitlines():
        if not line.strip():
            continue

        try:
            record = json.loads(line)
            record_type = record.get('type', '')
            data = record.get('data', {})

            if data.get('type') == 'classification':
                structured_data['classification'] = data
            elif data.get('type') == 'analysis':
                structured_data['patterns'] = data.get('patterns_by_type', {})
                structured_data['statistics'] = data.get('statistics', {})
            elif data.get('type') == 'pattern_match':
                structured_data['matches'].append(data)

        except json.JSONDecodeError:
            continue

    return structured_data
```

## Result Parsing

### JSONL Parser for Agents

```python
class JSONLParser:
    def __init__(self):
        self.handlers = {
            'result': self.handle_result,
            'error': self.handle_error,
            'info': self.handle_info,
            'progress': self.handle_progress,
            'match_record': self.handle_match,
            'file_entry': self.handle_file_entry
        }

    def parse_stream(self, stream: str) -> list:
        """Parse JSONL stream and return structured data."""
        records = []

        for line in stream.splitlines():
            if not line.strip():
                continue

            try:
                record = json.loads(line)
                record_type = record.get('type')

                if record_type in self.handlers:
                    processed = self.handlers[record_type](record)
                    records.append(processed)

            except json.JSONDecodeError:
                continue

        return records

    def handle_result(self, record: dict) -> dict:
        return {"type": "result", "data": record.get('data')}

    def handle_error(self, record: dict) -> dict:
        return {"type": "error", "code": record.get('code'), "message": record.get('message')}

    def handle_info(self, record: dict) -> dict:
        return {"type": "info", "data": record.get('data')}

    def handle_progress(self, record: dict) -> dict:
        return {"type": "progress", "current": record.get('current'), "total": record.get('total')}

    def handle_match(self, record: dict) -> dict:
        return {"type": "match", "file": record.get('file'), "line": record.get('line_number')}

    def handle_file_entry(self, record: dict) -> dict:
        return {"type": "file", "path": record.get('path'), "size": record.get('size')}
```

### LLM Response Executor

```python
class LLMCommandExecutor:
    def __init__(self):
        self.parser = JSONLParser()

    def execute_commands(self, llm_response: str) -> dict:
        """Execute commands from LLM response."""
        results = {"executed": [], "failed": []}

        lines = llm_response.split('\n')
        i = 0

        while i < len(lines):
            line = lines[i].strip()

            # Check if line is a command
            if line.startswith('ai-'):
                parts = line.split()
                command = parts[0]
                args = parts[1:]

                # Collect multi-line output
                output_lines = []
                i += 1

                # Handle commands that produce output
                try:
                    result = subprocess.run([command] + args,
                                          capture_output=True, text=True,
                                          timeout=30)

                    # Parse JSONL output
                    records = self.parser.parse_stream(result.stdout)

                    if result.returncode == 0:
                        results["executed"].append({
                            "command": line,
                            "results": records
                        })
                    else:
                        results["failed"].append({
                            "command": line,
                            "error": result.stderr
                        })

                except subprocess.TimeoutExpired:
                    results["failed"].append({
                        "command": line,
                        "error": "Command timed out"
                    })
                except Exception as e:
                    results["failed"].append({
                        "command": line,
                        "error": str(e)
                    })

            i += 1

        return results

    def format_results_for_llm(self, results: dict) -> str:
        """Format execution results for LLM consumption."""
        output = []

        if results["executed"]:
            output.append("## Executed Commands")
            for cmd in results["executed"]:
                output.append(f"### {cmd['command']}")
                output.append(f"Results: {len(cmd['results'])} records")

        if results["failed"]:
            output.append("\n## Failed Commands")
            for cmd in results["failed"]:
                output.append(f"### {cmd['command']}")
                output.append(f"Error: {cmd['error']}")

        return "\n".join(output)
```

## Example Agent Workflows

### Document Analysis Workflow

```python
def analyze_document_directory(directory: str) -> dict:
    """Analyze documents and prepare summary for LLM."""
    workflow = {
        "steps": [],
        "summary": {}
    }

    # Step 1: List all files
    print("Step 1: Listing files...")
    ls_result = subprocess.run(['ai-ls', '-R', directory],
                              capture_output=True, text=True)

    files = []
    for line in ls_result.stdout.splitlines():
        record = json.loads(line)
        if record.get('type') == 'file_entry':
            files.append(record)

    workflow["steps"].append({"action": "list", "count": len(files)})

    # Step 2: Classify files
    print("Step 2: Classifying files...")
    classifications = {}
    for file_record in files[:50]:  # Limit to 50 files
        file_path = file_record['path']
        analyze_result = subprocess.run(['ai-analyze', '-c', file_path],
                                      capture_output=True, text=True)

        for line in analyze_result.stdout.splitlines():
            record = json.loads(line)
            if record.get('type') == 'result':
                data = record.get('data', {})
                if data.get('type') == 'classification':
                    classifications[file_path] = data

    workflow["steps"].append({"action": "classify", "classified": len(classifications)})
    workflow["summary"]["classifications"] = classifications

    # Step 3: Detect patterns
    print("Step 3: Detecting patterns...")
    patterns = {}
    for file_path in list(classifications.keys())[:20]:
        analyze_result = subprocess.run(['ai-analyze', '-p', file_path],
                                      capture_output=True, text=True)

        for line in analyze_result.stdout.splitlines():
            record = json.loads(line)
            if record.get('type') == 'result':
                data = record.get('data', {})
                if data.get('type') == 'analysis':
                    patterns[file_path] = data.get('patterns_by_type', {})

    workflow["steps"].append({"action": "patterns", "analyzed": len(patterns)})
    workflow["summary"]["patterns"] = patterns

    return workflow
```

### Code Search Workflow

```python
def search_codebase(directory: str, query: str) -> dict:
    """Search codebase and prepare results for LLM."""
    results = {
        "query": query,
        "files_searched": 0,
        "matches": [],
        "context": {}
    }

    # Search recursively
    grep_result = subprocess.run(['ai-grep', '-r', '-C', '3', query, directory],
                                capture_output=True, text=True)

    for line in grep_result.stdout.splitlines():
        try:
            record = json.loads(line)
            if record.get('type') == 'match_record':
                results['matches'].append({
                    "file": record.get('file'),
                    "line": record.get('line_number'),
                    "content": record.get('line_content')
                })
        except json.JSONDecodeError:
            continue

    # Get context for matches
    unique_files = set(m['file'] for m in results['matches'])
    results['files_searched'] = len(unique_files)

    for file_path in list(unique_files)[:10]:  # Limit context to 10 files
        # Get file statistics
        wc_result = subprocess.run(['ai-wc', file_path],
                                  capture_output=True, text=True)
        results['context'][file_path] = wc_result.stdout.strip()

    return results
```
