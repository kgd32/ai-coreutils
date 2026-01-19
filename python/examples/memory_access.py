"""
AI-Coreutils Python Example: Memory Access

This example demonstrates memory-mapped file access with SIMD-accelerated operations.
"""

from ai_coreutils import SafeMemoryAccess

def main():
    # Create a test file
    test_file = "example.txt"
    with open(test_file, "w") as f:
        f.write("Hello, World!\n" * 1000)

    try:
        # Memory map the file
        print(f"Memory mapping {test_file}...")
        mem = SafeMemoryAccess(test_file)

        # Get file size
        size = mem.size()
        print(f"File size: {size} bytes")

        # Read first 100 bytes
        data = mem.get(0, 100)
        if data:
            print(f"First 100 bytes: {data[:50]}...")

        # Count newlines
        newlines = mem.count_byte(ord('\n'))
        print(f"Newlines: {newlines}")

        # Count text metrics
        lines, words, bytes_count = mem.count_text_metrics()
        print(f"Lines: {lines}, Words: {words}, Bytes: {bytes_count}")

        # Search for pattern
        pattern = b"World"
        matches = mem.find_pattern(pattern)
        print(f"Found '{pattern.decode()}' {len(matches)} times")

        # Get raw pointer (for advanced use)
        ptr = mem.as_ptr()
        print(f"Memory pointer: 0x{ptr:x}")

    finally:
        # Clean up
        import os
        if os.path.exists(test_file):
            os.remove(test_file)

if __name__ == "__main__":
    main()
