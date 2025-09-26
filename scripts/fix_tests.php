#!/usr/bin/env php
<?php

/**
 * PHP script to process Rust files:
 * - Finds all struct and enum types.
 * - Comments out existing #[cfg(test)] mod tests and #[cfg(test)] fn.
 * - Preserves any existing `mod generated_tests` module.
 * - Adds a new test module with serialization tests for each type if `mod generated_tests` does not already exist and types are found.
 * - Skips processing for files named lib.rs and mod.rs.
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
            $fileName = $file->getFilename();
            // Skip lib.rs and mod.rs files
            if ($fileName === 'lib.rs' || $fileName === 'mod.rs') {
                echo "Skipping file: " . $file->getPathname() . "\n";
                continue;
            }

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

    // Check if 'mod generated_tests' already exists.
    $moduleExists = preg_match('/#\[cfg\(test\)\][\s\S]*?mod\s+generated_tests\s*\{/', $content);

    // 2. Comment out existing test code
    $content = commentOutTestCode($content);

    // 3. If the test module doesn't exist AND we found types to test, add a new one.
    if (!$moduleExists) {
        if (!empty($types)) {
            echo "Found types to test. Adding new 'generated_tests' module.\n";
            $newTestModule = generateNewTestModule($types);
            $content .= "\n" . $newTestModule;
        } else {
            echo "No new types found; skipping test module generation.\n";
        }
    } else {
        echo "Preserving existing 'mod generated_tests' module.\n";
    }


    // Write back the modified content
    if (file_put_contents($filePath, $content) === false) {
        echo "Error: Could not write to file $filePath\n";
    } else {
        echo "Successfully updated file: $filePath\n";
    }
}

/**
 * Finds all struct and enum type names within the given Rust file content.
 *
 * @param string $content The content of the Rust file.
 * @return array An array of unique struct and enum names found.
 */
function findStructEnumTypes(string $content): array
{
    $types = [];
    preg_match_all(
        '/(?:pub\s+)?(?:struct|enum)\s+([a-zA-Z_][a-zA-Z0-9_]*)(?:\s*<[^>]*>)?(?:\s+where\s+.*?)?(?:\s*\{|\s*\(.*?\);)/s',
        $content,
        $matches
    );
    foreach ($matches[1] as $typeName) {
        $types[] = $typeName;
    }
    return array_unique($types);
}

/**
 * Comments out existing Rust test code, but preserves 'mod generated_tests'.
 *
 * @param string $content The original content of the Rust file.
 * @return string The content with test code commented out.
 */
function commentOutTestCode(string $content): string
{
    // Normalize line endings to LF to prevent offset calculation errors.
    $modifiedContent = str_replace("\r\n", "\n", $content);
    $modifiedContent = str_replace("\r", "\n", $modifiedContent);

    $replacements = [];

    $declarationPattern = '/(?P<declaration>(?:mod\s+[a-zA-Z_][a-zA-Z0-9_]*|fn\s+[a-zA-Z_][a-zA-Z0-9_]*\s*\(.*?\)|use\s+.*?))\s*(?P<terminator>\{|;)/s';
    preg_match_all($declarationPattern, $modifiedContent, $declarationMatches, PREG_OFFSET_CAPTURE | PREG_SET_ORDER);

    $lines = explode("\n", $modifiedContent);
    $lineOffsets = [];
    $currentOffset = 0;
    foreach ($lines as $line) {
        $lineOffsets[] = $currentOffset;
        $currentOffset += strlen($line) + 1;
    }
    $totalLines = count($lines);

    foreach (array_reverse($declarationMatches) as $match) {
        $declarationOffset = $match['declaration'][1];
        $terminator = $match['terminator'][0];
        $terminatorOffset = $match['terminator'][1];

        $blockEnd = -1;
        if ($terminator === '{') {
            $braceStart = $terminatorOffset;
            $blockEnd = findBalancedBraceEnd($modifiedContent, $braceStart);
            if ($blockEnd === -1) {
                echo "Warning: Unbalanced braces for block starting at offset $declarationOffset. Skipping.\n";
                continue;
            }
        } elseif ($terminator === ';') {
            $blockEnd = $terminatorOffset;
        }

        if ($blockEnd === -1) {
            continue;
        }

        $declarationLineNum = -1;
        for ($i = 0; $i < $totalLines; $i++) {
            if ($declarationOffset >= $lineOffsets[$i] && ($i + 1 >= $totalLines || $declarationOffset < $lineOffsets[$i+1])) {
                $declarationLineNum = $i;
                break;
            }
        }

        if ($declarationLineNum === -1) {
            continue;
        }

        $blockStart = $lineOffsets[$declarationLineNum];
        $currentLineNum = $declarationLineNum - 1;
        while ($currentLineNum >= 0) {
            $line = $lines[$currentLineNum];
            $trimmedLine = trim($line);

            if (empty($trimmedLine) ||
                str_starts_with($trimmedLine, '#[') ||
                str_starts_with($trimmedLine, '//') ||
                str_starts_with($trimmedLine, '/*')) {
                $blockStart = $lineOffsets[$currentLineNum];
                $currentLineNum--;
            } else {
                break;
            }
        }

        $preambleText = substr($modifiedContent, $blockStart, $declarationOffset - $blockStart);
        
        // This regex is now anchored to the start of a line (with the 'm' flag) 
        // to prevent matching attributes inside single-line comments.
        $cfgTestPattern = '/^\s*#\[cfg\((?:test|any\(test,\s*feature\s*=\s*"arbitrary-impls"\))\)\]/m';
        if (preg_match($cfgTestPattern, $preambleText)) {
            $declarationText = trim($match['declaration'][0]);

            if ($declarationText === 'mod generated_tests') {
                continue;
            }

            $originalBlockText = substr($modifiedContent, $blockStart, $blockEnd - $blockStart + 1);
            $commentedBlock = "/*\n" . $originalBlockText . "\n*/";

            $replacements[] = [
                'start' => $blockStart,
                'length' => $blockEnd - $blockStart + 1,
                'replacement' => $commentedBlock
            ];
        }
    }

    foreach (array_reverse($replacements) as $r) {
        $modifiedContent = substr_replace($modifiedContent, $r['replacement'], $r['start'], $r['length']);
    }

    return $modifiedContent;
}

/**
 * Helper function to find the index of the closing brace '}' that balances an opening brace '{'.
 *
 * @param string $content The string to search within.
 * @param int $startIndex The index of the opening curly brace.
 * @return int The index of the matching closing brace, or -1 if not found.
 */
function findBalancedBraceEnd(string $content, int $startIndex): int
{
    $braceCount = 0;
    $len = strlen($content);

    for ($i = $startIndex; $i < $len; $i++) {
        $char = $content[$i];
        $nextChar = ($i + 1 < $len) ? $content[$i+1] : '';

        if ($char === '/' && $nextChar === '/') {
            $nextLinePos = strpos($content, "\n", $i);
            if ($nextLinePos === false) return -1;
            $i = $nextLinePos;
            continue;
        } elseif ($char === '/' && $nextChar === '*') {
            $endCommentPos = strpos($content, "*/", $i + 2);
            if ($endCommentPos === false) return -1;
            $i = $endCommentPos + 1;
            continue;
        }

        if ($char === '"' || $char === '\'') {
            $quoteChar = $char;
            $i++;
            while ($i < $len) {
                if ($content[$i] === '\\') {
                    $i++;
                } elseif ($content[$i] === $quoteChar) {
                    break;
                }
                $i++;
            }
            if ($i >= $len) return -1;
            continue;
        }

        if ($char === '{') {
            $braceCount++;
        } elseif ($char === '}') {
            $braceCount--;
        }

        if ($braceCount === 0) {
            return $i;
        }
    }
    return -1;
}

/**
 * Generates the content for the new Rust test module. Assumes types array is not empty.
 *
 * @param array $types An array of Rust type names (structs/enums).
 * @return string The generated Rust code for the new test module.
 */
function generateNewTestModule(array $types): string
{
    $testModuleContent = "\n#[cfg(test)]\n";
    $testModuleContent .= "#[allow(unused_imports)]\n";
    $testModuleContent .= "#[allow(unused_variables)]\n";
    $testModuleContent .= "#[allow(unreachable_code)]\n";
    $testModuleContent .= "#[allow(non_snake_case)]\n";
    $testModuleContent .= "mod generated_tests {\n";
    $testModuleContent .= "    use super::*;\n";
    $testModuleContent .= "    use crate::test_shared::*;\n";
    $testModuleContent .= "    use bincode;\n";
    $testModuleContent .= "    use serde::{Deserialize, Serialize};\n\n";

    $testModuleContent .= "    pub mod nc {\n";
    foreach ($types as $type) {
        $testModuleContent .= "        pub use neptune_cash::api::export::{$type};\n";
    }
    $testModuleContent .= "    }\n\n";

    foreach ($types as $type) {
        $fn_type_name = strtolower($type);

        $testModuleContent .= "    #[test]\n";
        $testModuleContent .= "    fn test_bincode_serialization_for_{$fn_type_name}() {\n";
        $testModuleContent .= "        let original_instance: {$type} = {$type}::default();\n";
        $testModuleContent .= "        let nc_instance: nc::{$type} = nc::{$type}::default();\n";
        $testModuleContent .= "        test_bincode_serialization_for_type(original_instance, Some(nc_instance));\n";
        $testModuleContent .= "    }\n\n";

        $testModuleContent .= "    #[test]\n";
        $testModuleContent .= "    fn test_serde_json_serialization_for_{$fn_type_name}() {\n";
        $testModuleContent .= "        let original_instance: {$type} = {$type}::default();\n";
        $testModuleContent .= "        let nc_instance: nc::{$type} = nc::{$type}::default();\n";
        $testModuleContent .= "        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));\n";
        $testModuleContent .= "    }\n\n";

        $testModuleContent .= "    #[test]\n";
        $testModuleContent .= "    fn test_serde_json_wasm_serialization_for_{$fn_type_name}() {\n";
        $testModuleContent .= "        let original_instance: {$type} = {$type}::default();\n";
        $testModuleContent .= "        let nc_instance: nc::{$type} = nc::{$type}::default();\n";
        $testModuleContent .= "        test_serde_json_wasm_serialization_for_type(original_instance, Some(nc_instance));\n";
        $testModuleContent .= "    }\n\n";
    }

    $testModuleContent .= "}\n";
    return $testModuleContent;
}


// --- Script Execution ---
if (php_sapi_name() == 'cli') {
    if ($argc < 2) {
        echo "Usage: php " . basename(__FILE__) . " <top_level_directory>\n";
        exit(1);
    }
    $rootDir = $argv[1];
    processRustFiles($rootDir);
} else {
    echo "This script is intended to be run from the command line.\n";
}

?>
