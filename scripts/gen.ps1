# Regenerate parser/lexer into gen/
param(
  [string]$GrammarPath = "grammar\Language.g4",
  [string]$OutDir = "gen"
)

if (!(Test-Path $OutDir)) { New-Item -ItemType Directory -Force -Path $OutDir | Out-Null }

# Assumes you have 'antlr4' alias that runs the jar
antlr4 -Dlanguage=Cpp -visitor -listener -o $OutDir $GrammarPath

Write-Host "Generated ANTLR C++ sources in $OutDir"
