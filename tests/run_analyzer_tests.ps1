# Test script for TJ Language Static Analyzer
# This script runs analyzer tests and shows the results

Write-Host "=== TJ Language Static Analyzer Test Suite ===" -ForegroundColor Green
Write-Host ""

# Initialize counters
$totalTests = 0
$passedTests = 0
$failedTests = 0

# Function to run analyzer tests from a folder
function Run-AnalyzerTestCategory {
    param(
        [string]$CategoryName,
        [string]$FolderPath,
        [bool]$ShouldHaveIssues = $false
    )
    
    Write-Host "=== $CategoryName ===" -ForegroundColor Yellow
    
    $testFiles = Get-ChildItem -Path "tests\analyzer\$FolderPath" -Filter "*.tj" | Sort-Object Name
    $categoryPassed = 0
    $categoryFailed = 0
    
    foreach ($testFile in $testFiles) {
        $testPath = "$FolderPath\$($testFile.Name)"
        Write-Host "Testing: $testPath" -ForegroundColor Cyan
        
        $result = & ".\build\Release\tjlang.exe" "tests\analyzer\$testPath" --analyze 2>&1
        $exitCode = $LASTEXITCODE
        $script:totalTests++
        
        if ($ShouldHaveIssues) {
            # This test should have analyzer issues
            if ($exitCode -eq 0) {
                Write-Host "  SHOULD HAVE ISSUES (but found none)" -ForegroundColor Red
                $categoryFailed++
                $script:failedTests++
            } else {
                Write-Host "  CORRECTLY FOUND ISSUES" -ForegroundColor Green
                Write-Host "  Issues found:" -ForegroundColor Yellow
                $result | ForEach-Object { Write-Host "    $_" -ForegroundColor Yellow }
                $categoryPassed++
                $script:passedTests++
            }
        } else {
            # This test should be clean (no issues)
            if ($exitCode -eq 0) {
                Write-Host "  CLEAN (no issues found)" -ForegroundColor Green
                $categoryPassed++
                $script:passedTests++
            } else {
                Write-Host "  UNEXPECTED ISSUES FOUND" -ForegroundColor Red
                Write-Host "  Unexpected issues:" -ForegroundColor Red
                $result | ForEach-Object { Write-Host "    $_" -ForegroundColor Red }
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
Run-AnalyzerTestCategory "VALID ANALYZER TESTS (should be clean)" "valid" -ShouldHaveIssues $false
Run-AnalyzerTestCategory "INVALID ANALYZER TESTS (should have issues)" "invalid" -ShouldHaveIssues $true

# Summary
Write-Host "=== ANALYZER TEST SUITE SUMMARY ===" -ForegroundColor Green
Write-Host "Total Tests: $totalTests" -ForegroundColor White
Write-Host "Passed: $passedTests" -ForegroundColor Green
Write-Host "Failed: $failedTests" -ForegroundColor Red

if ($failedTests -eq 0) {
    Write-Host "🎉 All analyzer tests passed!" -ForegroundColor Green
} else {
    Write-Host "Some analyzer tests failed. Please review the output above." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "=== Analyzer Test Suite Complete ===" -ForegroundColor Green
