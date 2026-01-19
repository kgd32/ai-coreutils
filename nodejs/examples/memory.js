/**
 * AI-Coreutils Node.js Example: Memory Access
 *
 * This example demonstrates memory-mapped file access with SIMD-accelerated operations.
 */

const { MemoryAccess } = require('ai-coreutils');
const fs = require('fs');

function main() {
  // Create a test file
  const testFile = 'example.txt';
  const content = 'Hello, World!\n'.repeat(1000);
  fs.writeFileSync(testFile, content);

  try {
    // Memory map the file
    console.log(`Memory mapping ${testFile}...`);
    const mem = new MemoryAccess(testFile);

    // Get file size
    const size = mem.size;
    console.log(`File size: ${size} bytes`);

    // Get memory pointer
    const ptr = mem.ptr;
    console.log(`Memory pointer: 0x${ptr.toString(16)}`);

    // Read first 100 bytes
    const data = mem.get(0, 100);
    if (data) {
      console.log(`First 100 bytes: ${data.toString('utf8', 0, 50)}...`);
    }

    // Count newlines
    const newlineByte = '\n'.charCodeAt(0);
    const newlines = mem.countByte(newlineByte);
    console.log(`Newlines: ${newlines}`);

    // Count text metrics
    const metrics = mem.countTextMetrics();
    console.log(`Lines: ${metrics.lines}, Words: ${metrics.words}, Bytes: ${metrics.bytes}`);

    // Search for pattern
    const pattern = Buffer.from('World');
    const matches = mem.findPattern(pattern);
    console.log(`Found '${pattern.toString()}' ${matches.length} times`);

    // Get a single byte
    const firstByte = mem.getByte(0);
    console.log(`First byte: ${firstByte} ('${String.fromCharCode(firstByte)}')`);

  } finally {
    // Clean up
    if (fs.existsSync(testFile)) {
      fs.unlinkSync(testFile);
    }
  }
}

main();
