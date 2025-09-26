<?php

/**
 * PHP script to process Rust files:
 * - Finds all struct and enum types.
 * - Comments out existing #[cfg(test)] mod tests and #[cfg(test)] fn.
 * - Adds a new test module with bincode serialization/deserialization tests for each type.
 *
 * Usage: php script_name.php <top_level_directory>
 *
 * WARNING: This script modifies files in place. Please back up your files before running.
 */

/**
 * Main function to process Rust files recursively from a given root directory.
 *
 * @param string $rootDir The top-level directory to start the recursion.
 * @return void
 */
function processRustFiles(string $rootDir): void
{
    if (!is_dir($rootDir)) {
        echo "Error: Root directory '$rootDir' not found.\n";
        return;
    }

    echo "Processing Rust files in: " . realpath($rootDir) . "\n";
    echo "WARNING: This script modifies files in place. Please back up your files before running.\n\n";

    // Create a recursive iterator to traverse the directory
    $iterator = new RecursiveIteratorIterator(
        new RecursiveDirectoryIterator($rootDir, RecursiveDirectoryIterator::SKIP_DOTS),
        RecursiveIteratorIterator::SELF_FIRST
    );

    foreach ($iterator as $file) {
        // Process only .rs files
        if ($file->isFile() && $file->getExtension() === 'rs') {
            $filePath = $file->getPathname();
            echo "Processing file: $filePath\n";
            handleRustFile($filePath);
        }
    }
    echo "\nProcessing complete.\n";
}

/**
 * Handles a single Rust file: reads content, modifies it, and writes back.
 *
 * @param string $filePath The path to the Rust file.
 * @return void
 */
function handleRustFile(string $filePath): void
{
    $content = file_get_contents($filePath);
    if ($content === false) {
        echo "Error: Could not read file $filePath\n";
        return;
    }

    // 1. Find all struct or enum types defined in the file
    $types = findStructEnumTypes($content);

    // 2. Comment out existing test code (mod tests and #[cfg(test)] fn)
    $content = commentOutTestCode($content);

    // 3. Add a new test module and 4. add new unit tests for each type
    $newTestModule = generateNewTestModule($types);
    $content .= "\n" . $newTestModule; // Append the new test module to the end of the file

    // Write back the modified content
    if (file_put_contents($filePath, $content) === false) {
        echo "Error: Could not write to file $filePath\n";
    } else {
        echo "Successfully updated file: $filePath\n";
    }
}

/**
 * Finds all struct and enum type names within the given Rust file content.
 * This function is updated to correctly identify newtype structs.
 *
 * @param string $content The content of the Rust file.
 * @return array An array of unique struct and enum names found.
 */
function findStructEnumTypes(string $content): array
{
    $types = [];
    // Regex to find struct or enum declarations.
    // It now correctly handles both braced blocks ({}) and newtype structs (();).
    // - Optional 'pub' keyword.
    // - 'struct' or 'enum' keyword.
    // - The type name (alphanumeric and underscore, starting with letter/underscore).
    // - Optional generics (e.g., <T, U>).
    // - Optional 'where' clause.
    // - Followed by either '{' or '(...);'
    preg_match_all(
        '/(?:pub\s+)?(?:struct|enum)\s+([a-zA-Z_][a-zA-Z0-9_]*)(?:\s*<[^>]*>)?(?:\s+where\s+.*?)?(?:\s*\{|\s*\(.*?\);)/s',
        $content,
        $matches
    );
    foreach ($matches[1] as $typeName) {
        $types[] = $typeName;
    }
    return array_unique($types); // Ensure no duplicate type names are returned
}

/**
 * Comments out existing Rust test code (mod tests and #[cfg(test)] functions).
 * This function now correctly identifies and replaces the full test blocks
 * by finding #[cfg(test)] or #[cfg(any(test, feature = "arbitrary-impls"))]
 * and then determining the full extent of the block.
 *
 * @param string $content The original content of the Rust file.
 * @return string The content with test code commented out.
 */
function commentOutTestCode(string $content): string
{
    $modifiedContent = $content;
    $replacements = [];

    // Pattern to find mod, fn, or use declarations.
    // It captures the declaration itself and its starting offset.
    $declarationPattern = '/(?P<declaration>(?:mod\s+[a-zA-Z_][a-zA-Z0-9_]*|fn\s+[a-zA-Z_][a-zA-Z0-9_]*\s*\(.*?\)|use\s+.*?))\s*(?P<terminator>\{|;)/s';
    preg_match_all($declarationPattern, $modifiedContent, $declarationMatches, PREG_OFFSET_CAPTURE | PREG_SET_ORDER);

    // Prepare line offsets for efficient line-by-line backward scanning
    $lines = explode("\n", $modifiedContent);
    $lineOffsets = [];
    $currentOffset = 0;
    foreach ($lines as $line) {
        $lineOffsets[] = $currentOffset;
        $currentOffset += strlen($line) + 1; // +1 for the newline character
    }
    $totalLines = count($lines);

    // Process declarations in reverse order to avoid offset issues when applying replacements
    foreach (array_reverse($declarationMatches) as $match) {
        $declarationOffset = $match['declaration'][1];
        $terminator = $match['terminator'][0];
        $terminatorOffset = $match['terminator'][1];

        // Determine the end of the block based on its terminator
        $blockEnd = -1;
        if ($terminator === '{') {
            $braceStart = $terminatorOffset;
            $blockEnd = findBalancedBraceEnd($modifiedContent, $braceStart);
            if ($blockEnd === -1) {
                echo "Warning: Unbalanced braces for block starting at offset $declarationOffset. Skipping.\n";
                continue;
            }
        } elseif ($terminator === ';') {
            $blockEnd = $terminatorOffset; // For single-line 'use' statements
        }

        if ($blockEnd === -1) {
            continue; // Could not determine block end, skip this match
        }

        // Determine the line number of the declaration
        $declarationLineNum = -1;
        for ($i = 0; $i < $totalLines; $i++) {
            if ($declarationOffset >= $lineOffsets[$i] && ($i + 1 >= $totalLines || $declarationOffset < $lineOffsets[$i+1])) {
                $declarationLineNum = $i;
                break;
            }
        }

        if ($declarationLineNum === -1) {
            continue; // Should not happen for a valid match, but as a safeguard
        }

        // Determine the effective start of the block, including its preamble
        // (all preceding attributes, doc-comments, and regular comments).
        $blockStart = $lineOffsets[$declarationLineNum]; // Initial assumption: start at the declaration's line

        // Scan backward line by line from the declaration line
        $currentLineNum = $declarationLineNum - 1; // Start from the line *before* the declaration
        while ($currentLineNum >= 0) {
            $line = $lines[$currentLineNum];
            $trimmedLine = trim($line);

            // Check if the line is empty, an attribute, or a comment.
            if (empty($trimmedLine) ||
                str_starts_with($trimmedLine, '#[') ||
                str_starts_with($trimmedLine, '//') ||
                str_starts_with($trimmedLine, '/*')) {
                // This line is part of the preamble. Update blockStart to this line's offset.
                $blockStart = $lineOffsets[$currentLineNum];
                $currentLineNum--; // Move to the previous line
            } else {
                // Found a line that is not part of the preamble. Stop scanning.
                break;
            }
        }

        // Now, extract the full preamble text from the determined blockStart up to the declaration offset.
        $preambleText = substr($modifiedContent, $blockStart, $declarationOffset - $blockStart);

        // Check if the collected preamble contains a test-related cfg attribute.
        $cfgTestPattern = '/#\[cfg\((?:test|any\(test,\s*feature\s*=\s*"arbitrary-impls"\))\)]/s';
        if (preg_match($cfgTestPattern, $preambleText)) {
            // This block is a test-related block, so mark it for commenting out.
            $originalBlockText = substr($modifiedContent, $blockStart, $blockEnd - $blockStart + 1);
            $commentedBlock = "/*\n" . $originalBlockText . "\n*/";

            $replacements[] = [
                'start' => $blockStart,
                'length' => $blockEnd - $blockStart + 1,
                'replacement' => $commentedBlock
            ];
        }
    }

    // Apply all stored replacements in reverse order to ensure correct offsets.
    foreach (array_reverse($replacements) as $r) {
        $modifiedContent = substr_replace($modifiedContent, $r['replacement'], $r['start'], $r['length']);
    }

    return $modifiedContent;
}

/**
 * Helper function to find the index of the closing brace '}' that balances
 * an opening brace '{' starting at a given index. This function now
 * correctly skips over comments and string literals to avoid miscounting braces.
 *
 * @param string $content The string to search within.
 * @param int $startIndex The index of the opening curly brace.
 * @return int The index of the matching closing brace, or -1 if not found (e.g., unclosed string/comment/brace).
 */
function findBalancedBraceEnd(string $content, int $startIndex): int
{
    $braceCount = 0;
    $len = strlen($content);

    // Start scanning from the character *after* the initial opening brace
    for ($i = $startIndex; $i < $len; $i++) {
        $char = $content[$i];
        $nextChar = ($i + 1 < $len) ? $content[$i+1] : '';

        // Handle comments
        if ($char === '/' && $nextChar === '/') { // Single-line comment //
            $nextLinePos = strpos($content, "\n", $i);
            if ($nextLinePos === false) {
                // Reached end of file within a single-line comment, no more braces possible
                return -1; // Consider this an error or unclosed block if braceCount > 0
            }
            $i = $nextLinePos; // Move index to the newline character
            continue; // Continue loop from after the newline
        } elseif ($char === '/' && $nextChar === '*') { // Multi-line comment /* */
            $endCommentPos = strpos($content, "*/", $i + 2); // Search for "*/" after "/*"
            if ($endCommentPos === false) {
                // Unclosed multi-line comment
                return -1; // Consider this an error or unclosed block if braceCount > 0
            }
            $i = $endCommentPos + 1; // Move index to the character after '*' of "*/"
            continue;
        }

        // Handle string literals (double and single quotes)
        if ($char === '"' || $char === '\'') {
            $quoteChar = $char;
            $i++; // Move past the opening quote
            while ($i < $len) {
                if ($content[$i] === '\\') { // Handle escaped characters
                    $i++; // Skip the escaped character
                } elseif ($content[$i] === $quoteChar) { // Found closing quote
                    break;
                }
                $i++;
            }
            if ($i >= $len) { // Unclosed string literal
                return -1; // Consider this an error or unclosed block if braceCount > 0
            }
            continue;
        }

        // Count braces
        if ($char === '{') {
            $braceCount++;
        } elseif ($char === '}') {
            $braceCount--;
        }

        // If braceCount is 0, we found the matching closing brace
        if ($braceCount === 0) {
            return $i;
        }
    }
    return -1; // Balanced brace not found (e.g., unclosed block)
}

/**
 * Generates the content for the new Rust test module, including unit tests
 * for each provided type.
 *
 * @param array $types An array of Rust type names (structs/enums).
 * @return string The generated Rust code for the new test module.
 */
function generateNewTestModule(array $types): string
{
    $testModuleContent = "\n#[cfg(test)]\nmod generated_tests {\n";
    $testModuleContent .= "    use super::*;\n"; // Import all types from the parent module
    $testModuleContent .= "    use bincode;\n";
    $testModuleContent .= "    use neptune_cash::api::export;\n";
    $testModuleContent .= "    use serde::{Serialize, Deserialize};\n\n";

    if (empty($types)) {
        $testModuleContent .= "    // No struct or enum types found in this file to generate tests for.\n";
    } else {
        foreach ($types as $type) {
            $testModuleContent .= "    #[test]\n";
            $testModuleContent .= "    fn test_bincode_serialization_for_{$type}() {\n";
            $testModuleContent .= "        // TODO: Instantiate your type '{$type}' here.\n";
            $testModuleContent .= "        // For complex types, you might need to manually construct an instance with specific values.\n";
            $testModuleContent .= "        // If your type implements `Default`, you can use `let original_instance = {$type}::default();`.\n";
            $testModuleContent .= "        // Ensure the instantiated type implements `PartialEq`, `Debug`, `Serialize`, and `Deserialize`.\n";
            $testModuleContent .= "        let original_instance: {$type} = todo!(\"Instantiate {$type} for test\");\n\n";

            $testModuleContent .= "        // a) serialize the type to string with bincode\n";
            $testModuleContent .= "        let encoded: Vec<u8> = bincode::serialize(&original_instance)\n";
            $testModuleContent .= "            .expect(\"Failed to serialize {$type}\");\n\n";

            $testModuleContent .= "        // c) deserialize the type from string to original type\n";
            $testModuleContent .= "        let decoded: {$type} = bincode::deserialize(&encoded)\n";
            $testModuleContent .= "            .expect(\"Failed to deserialize {$type}\");\n\n";

            $testModuleContent .= "        // d) verify the deserialized type matches the original type\n";
            $testModuleContent .= "        assert_eq!(original_instance, decoded, \"Deserialized {$type} should match original\");\n\n";

            $testModuleContent .= "        // e) deserialize the type from string for neptune_cash::api::export::<TypeName>\n";
            $testModuleContent .= "        // This assumes that `neptune_cash::api::export::{$type}` has a compatible structure\n";
            $testModuleContent .= "        // and also implements `Deserialize` and `PartialEq`.\n";
            $testModuleContent .= "        let exported_decoded: export::{$type} = bincode::deserialize(&encoded)\n";
            $testModuleContent .= "            .expect(\"Failed to deserialize {$type} into neptune_cash::api::export::{$type}\");\n\n";

            $testModuleContent .= "        // f) serialize the result of step (e)\n";
            $testModuleContent .= "        let exported_encoded: Vec<u8> = bincode::serialize(&exported_decoded)\n";
            $testModuleContent .= "            .expect(\"Failed to serialize neptune_cash::api::export::{$type}\");\n\n";

            $testModuleContent .= "        // g) verify the result of step (f) matches the serialized result of step (b)\n";
            $testModuleContent .= "        assert_eq!(encoded, exported_encoded, \"Serialized neptune_cash::api::export::{$type} should match original serialized {$type}\");\n";
            $testModuleContent .= "    }\n\n";
        }
    }

    $testModuleContent .= "}\n";
    return $testModuleContent;
}

// --- Script Execution ---
// Check if the script is run from the command line (CLI)
if (php_sapi_name() == 'cli') {
    // Check for the correct number of command-line arguments
    if ($argc < 2) {
        echo "Usage: php " . basename(__FILE__) . " <top_level_directory>\n";
        exit(1); // Exit with an error code
    }
    // Get the root directory from the command-line arguments
    $rootDir = $argv[1];
    processRustFiles($rootDir);
} else {
    echo "This script is intended to be run from the command line.\n";
}

?>

