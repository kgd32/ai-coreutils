/**
 * AI-Coreutils Node.js Example: File Classification
 *
 * This example demonstrates intelligent file type detection.
 */

const { FileClassifierWrapper } = require('ai-coreutils');

function main() {
  const classifier = new FileClassifierWrapper();

  // Example files with different types
  const examples = {
    'script.py': Buffer.from('#!/usr/bin/env python3\nprint("Hello, World!")\n'),
    'config.json': Buffer.from('{"name": "test", "value": 123}\n'),
    'styles.css': Buffer.from('body { color: red; margin: 0; }\n'),
    'README.md': Buffer.from('# Project Title\n\nThis is a sample markdown file.\n'),
    'data.csv': Buffer.from('name,age,city\nAlice,30,NYC\nBob,25,LA\n'),
    'app.rs': Buffer.from('fn main() {\n    println!("Hello!");\n}\n'),
    'index.html': Buffer.from('<!DOCTYPE html>\n<html><body>Hello</body></html>\n'),
    'binary.bin': Buffer.from([0, 1, 2, 3, 0, 0, 0, 0xFF, 0xFE]),  // Binary with null bytes
  };

  console.log('File Classification Results:');
  console.log('='.repeat(80));

  for (const [filename, content] of Object.entries(examples)) {
    const classification = classifier.classify(filename, content);

    console.log(`\n📄 ${filename}`);
    console.log(`   Type: ${classification.fileType}`);
    console.log(`   MIME: ${classification.mimeType}`);
    console.log(`   Encoding: ${classification.encoding}`);
    console.log(`   Binary: ${classification.isBinary}`);
    console.log(`   Confidence: ${(classification.confidence * 100).toFixed(1)}%`);

    if (classification.language) {
      console.log(`   Language: ${classification.language}`);
    }
  }

  // Test unknown file with content-based detection
  console.log('\n' + '='.repeat(80));
  console.log('Unknown file (content-based detection):');

  const unknownContent = Buffer.from('This is plain text without a file extension.');
  const classification = classifier.classify('unknown', unknownContent);
  console.log(`   Type: ${classification.fileType}`);
  console.log(`   Binary: ${classification.isBinary}`);
  console.log(`   Confidence: ${(classification.confidence * 100).toFixed(1)}%`);
}

main();
