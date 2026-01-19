"""
AI-Coreutils Python Example: File Classification

This example demonstrates intelligent file type detection.
"""

from ai_coreutils import FileClassifier

def main():
    classifier = FileClassifier()

    # Example files with different types
    examples = {
        "script.py": b'#!/usr/bin/env python3\nprint("Hello, World!")\n',
        "config.json": b'{"name": "test", "value": 123}\n',
        "styles.css": b'body { color: red; margin: 0; }\n',
        "README.md": b'# Project Title\n\nThis is a sample markdown file.\n',
        "data.csv": b'name,age,city\nAlice,30,NYC\nBob,25,LA\n',
        "app.rs": b'fn main() {\n    println!("Hello!");\n}\n',
        "index.html": b'<!DOCTYPE html>\n<html><body>Hello</body></html>\n',
        "binary.bin": bytes([0, 1, 2, 3, 0, 0, 0, 0xFF, 0xFE]),  # Binary with null bytes
    }

    print("File Classification Results:")
    print("=" * 80)

    for filename, content in examples.items():
        classification = classifier.classify(filename, content)

        print(f"\n📄 {filename}")
        print(f"   Type: {classification.file_type}")
        print(f"   MIME: {classification.mime_type}")
        print(f"   Encoding: {classification.encoding}")
        print(f"   Binary: {classification.is_binary}")
        print(f"   Confidence: {classification.confidence:.1%}")

        if classification.language:
            print(f"   Language: {classification.language}")

    # Test unknown file with extension detection
    print("\n" + "=" * 80)
    print("Unknown file (content-based detection):")

    unknown_content = b'This is plain text without a file extension.'
    classification = classifier.classify("unknown", unknown_content)
    print(f"   Type: {classification.file_type}")
    print(f"   Binary: {classification.is_binary}")
    print(f"   Confidence: {classification.confidence:.1%}")

if __name__ == "__main__":
    main()
