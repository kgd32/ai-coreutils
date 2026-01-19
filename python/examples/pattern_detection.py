"""
AI-Coreutils Python Example: Pattern Detection

This example demonstrates AI-powered pattern detection in text.
"""

from ai_coreutils import PatternDetector, PatternType

def main():
    # Sample text with various patterns
    text = """
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
    """

    # Create pattern detector
    detector = PatternDetector()

    # Detect all patterns
    print("Pattern Detection Results:")
    print("=" * 60)

    matches = detector.detect_patterns(text)

    # Group by pattern type
    by_type = {}
    for match in matches:
        ptype = match.pattern_type.name
        if ptype not in by_type:
            by_type[ptype] = []
        by_type[ptype].append(match)

    # Print summary by type
    for ptype, type_matches in sorted(by_type.items()):
        print(f"\n{ptype}: {len(type_matches)} matches")
        for match in type_matches[:3]:  # Show first 3 of each type
            print(f"  - {match.matched_text} (confidence: {match.confidence:.2f})")

    # Full analysis with statistics
    print("\n" + "=" * 60)
    print("Full Content Analysis:")
    print("=" * 60)

    analysis = detector.analyze_content(text, "example.txt")

    print(f"\nPath: {analysis.path}")
    print(f"Total patterns: {analysis.total_patterns}")
    print(f"\nStatistics:")
    stats = analysis.statistics
    print(f"  Characters: {stats.characters}")
    print(f"  Lines: {stats.lines}")
    print(f"  Words: {stats.words}")
    print(f"  Bytes: {stats.bytes}")
    print(f"  Avg line length: {stats.avg_line_length:.1f}")
    print(f"  Max line length: {stats.max_line_length}")
    print(f"  Whitespace ratio: {stats.whitespace_ratio:.2%}")
    print(f"  Entropy: {stats.entropy:.2f}")

    print(f"\nIssues detected:")
    for issue in analysis.issues:
        print(f"  ⚠️  {issue}")

    # Pattern type statistics
    print(f"\nPattern distribution:")
    patterns_by_type = {}
    for match in matches:
        ptype = match.pattern_type.name
        patterns_by_type[ptype] = patterns_by_type.get(ptype, 0) + 1

    for ptype, count in sorted(patterns_by_type.items()):
        print(f"  {ptype}: {count}")

if __name__ == "__main__":
    main()
