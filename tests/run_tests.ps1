# Test script for TJ Language Compiler
# This script runs various test cases and shows the results

Write-Host "=== TJ Language Compiler Test Suite ===" -ForegroundColor Green
Write-Host ""

# Valid test cases
Write-Host "=== VALID TEST CASES ===" -ForegroundColor Yellow
$validTests = @(
    "valid_simple.tj",
    "valid_complex.tj", 
    "valid_expressions.tj",
    "valid_literals.tj"
)

foreach ($test in $validTests) {
    Write-Host "Testing: $test" -ForegroundColor Cyan
    $result = & ".\build\Release\tjlang.exe" "tests\$test" 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  ✅ PASSED" -ForegroundColor Green
    } else {
        Write-Host "  ❌ FAILED" -ForegroundColor Red
        Write-Host "  Output: $result" -ForegroundColor Red
    }
    Write-Host ""
}

# Lexical error test cases
Write-Host "=== LEXICAL ERROR TEST CASES ===" -ForegroundColor Yellow
$lexErrorTests = @(
    "lex_error_unclosed_string.tj",
    "lex_error_invalid_char.tj",
    "lex_error_unclosed_comment.tj",
    "lex_error_invalid_number.tj",
    "lex_error_invalid_escape.tj"
)

foreach ($test in $lexErrorTests) {
    Write-Host "Testing: $test" -ForegroundColor Cyan
    $result = & ".\build\Release\tjlang.exe" "tests\$test" 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  ✅ CORRECTLY FAILED (Lexical Error)" -ForegroundColor Green
        Write-Host "  Error: $($result -join ' ')" -ForegroundColor Yellow
    } else {
        Write-Host "  ❌ SHOULD HAVE FAILED" -ForegroundColor Red
    }
    Write-Host ""
}

# Syntax error test cases
Write-Host "=== SYNTAX ERROR TEST CASES ===" -ForegroundColor Yellow
$syntaxErrorTests = @(
    "syntax_error_missing_paren.tj",
    "syntax_error_missing_brace.tj",
    "syntax_error_missing_colon.tj",
    "syntax_error_missing_arrow.tj",
    "syntax_error_invalid_type.tj",
    "syntax_error_missing_def.tj",
    "syntax_error_missing_return.tj",
    "syntax_error_extra_semicolon.tj",
    "syntax_error_malformed_generic.tj",
    "syntax_error_missing_comma.tj"
)

foreach ($test in $syntaxErrorTests) {
    Write-Host "Testing: $test" -ForegroundColor Cyan
    $result = & ".\build\Release\tjlang.exe" "tests\$test" 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  ✅ CORRECTLY FAILED (Syntax Error)" -ForegroundColor Green
        Write-Host "  Error: $($result -join ' ')" -ForegroundColor Yellow
    } else {
        Write-Host "  ❌ SHOULD HAVE FAILED" -ForegroundColor Red
    }
    Write-Host ""
}

Write-Host "=== Test Suite Complete ===" -ForegroundColor Green
