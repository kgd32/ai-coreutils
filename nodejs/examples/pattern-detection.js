/**
 * AI-Coreutils Node.js Example: Pattern Detection
 *
 * This example demonstrates AI-powered pattern detection in text.
 */

const { PatternDetectorWrapper } = require('ai-coreutils');

function main() {
  // Sample text with various patterns
  const text = `
    Contact Information:
    - Email: support@example.com, admin@test.org
    - Website: https://example.com, www.test.org
    - Phone: (555) 123-4567, 1-800-555-0199
    - IP Address: 192.168.1.1, 10.0.0.1

    Sensitive Data (DO NOT SHARE):
    - SSN: 123-45-6789
    - Credit Card: 4532-1234-5678-9010
    - UUID: 550e8400-e29b-41d4-a716-446655440000

    Technical:
    - API Key: Base64EncodedStringWith+=Padding/12345678901234567890
    - Hex Color: 0xFF5733
    - File Path: C:\\Users\\example\\file.txt
    - Date: 2024-01-19
  `;

  // Create pattern detector
  const detector = new PatternDetectorWrapper();

  // Detect all patterns
  console.log('Pattern Detection Results:');
  console.log('='.repeat(60));

  const matches = detector.detectPatterns(text);

  // Group by pattern type
  const byType = {};
  for (const match of matches) {
    const ptype = match.patternType;
    if (!byType[ptype]) {
      byType[ptype] = [];
    }
    byType[ptype].push(match);
  }

  // Print summary by type
  const sortedTypes = Object.keys(byType).sort();
  for (const ptype of sortedTypes) {
    const typeMatches = byType[ptype];
    console.log(`\n${ptype}: ${typeMatches.length} matches`);
    for (const match of typeMatches.slice(0, 3)) {
      console.log(`  - ${match.matchedText} (confidence: ${match.confidence.toFixed(2)})`);
    }
  }

  // Full analysis with statistics
  console.log('\n' + '='.repeat(60));
  console.log('Full Content Analysis:');
  console.log('='.repeat(60));

  const analysis = detector.analyzeContent(text, 'example.txt');

  console.log(`\nPath: ${analysis.path}`);
  console.log(`Total patterns: ${analysis.totalPatterns}`);
  console.log('\nStatistics:');
  const stats = analysis.statistics;
  console.log(`  Characters: ${stats.characters}`);
  console.log(`  Lines: ${stats.lines}`);
  console.log(`  Words: ${stats.words}`);
  console.log(`  Bytes: ${stats.bytes}`);
  console.log(`  Avg line length: ${stats.avgLineLength.toFixed(1)}`);
  console.log(`  Max line length: ${stats.maxLineLength}`);
  console.log(`  Whitespace ratio: ${(stats.whitespaceRatio * 100).toFixed(2)}%`);
  console.log(`  Entropy: ${stats.entropy.toFixed(2)}`);

  console.log('\nIssues detected:');
  for (const issue of analysis.issues) {
    console.log(`  ⚠️  ${issue}`);
  }

  // Pattern type statistics
  console.log('\nPattern distribution:');
  const patternCounts = {};
  for (const match of matches) {
    const ptype = match.patternType;
    patternCounts[ptype] = (patternCounts[ptype] || 0) + 1;
  }

  const sortedPatterns = Object.entries(patternCounts).sort((a, b) => a[0].localeCompare(b[0]));
  for (const [ptype, count] of sortedPatterns) {
    console.log(`  ${ptype}: ${count}`);
  }
}

main();
