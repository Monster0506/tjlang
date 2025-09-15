# Test script for TJ Language Compiler
# This script runs various test cases and shows the results

Write-Host "=== TJ Language Compiler Test Suite ===" -ForegroundColor Green
Write-Host ""

# Initialize counters
$totalTests = 0
$passedTests = 0
$failedTests = 0

# Function to run tests from a folder
function Run-TestCategory {
    param(
        [string]$CategoryName,
        [string]$FolderPath,
        [bool]$ShouldFail = $false
    )
    
    Write-Host "=== $CategoryName ===" -ForegroundColor Yellow
    
    $testFiles = Get-ChildItem -Path "tests\$FolderPath" -Filter "*.tj" | Sort-Object Name
    $categoryPassed = 0
    $categoryFailed = 0
    
    foreach ($testFile in $testFiles) {
        $testPath = "$FolderPath\$($testFile.Name)"
        Write-Host "Testing: $testPath" -ForegroundColor Cyan
        
        $result = & ".\build\Release\tjlang.exe" "tests\$testPath" 2>&1
        $script:totalTests++
        
        if ($ShouldFail) {
            if ($LASTEXITCODE -ne 0) {
                Write-Host "  CORRECTLY FAILED" -ForegroundColor Green
                Write-Host "  Error: $($result -join ' ')" -ForegroundColor Yellow
                $categoryPassed++
                $script:passedTests++
            } else {
                Write-Host "  SHOULD HAVE FAILED" -ForegroundColor Red
                $categoryFailed++
                $script:failedTests++
            }
        } else {
            if ($LASTEXITCODE -eq 0) {
                Write-Host "  PASSED" -ForegroundColor Green
                $categoryPassed++
                $script:passedTests++
            } else {
                Write-Host "  FAILED" -ForegroundColor Red
                Write-Host "  Output: $result" -ForegroundColor Red
                $categoryFailed++
                $script:failedTests++
            }
        }
        Write-Host ""
    }
    
    Write-Host "Category Results: $categoryPassed passed, $categoryFailed failed" -ForegroundColor Gray
    Write-Host ""
}

# Run all test categories
Run-TestCategory "VALID TEST CASES - FUNCTIONS" "valid\functions"
Run-TestCategory "VALID TEST CASES - EXPRESSIONS" "valid\expressions"
Run-TestCategory "VALID TEST CASES - LITERALS" "valid\literals"
Run-TestCategory "TYPE SYSTEM TEST CASES" "type_system"
Run-TestCategory "LEXICAL ERROR TEST CASES" "lexer\errors" -ShouldFail $true
Run-TestCategory "SYNTAX ERROR TEST CASES" "syntax\errors" -ShouldFail $true

# Summary
Write-Host "=== TEST SUITE SUMMARY ===" -ForegroundColor Green
Write-Host "Total Tests: $totalTests" -ForegroundColor White
Write-Host "Passed: $passedTests" -ForegroundColor Green
Write-Host "Failed: $failedTests" -ForegroundColor Red

if ($failedTests -eq 0) {
    Write-Host "All tests passed!" -ForegroundColor Green
} else {
    Write-Host " Some tests failed. Please review the output above." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "=== Test Suite Complete ===" -ForegroundColor Green
