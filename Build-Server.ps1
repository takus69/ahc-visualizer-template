Push-Location .\wasm
wasm-pack build --target web --out-dir ../public/wasm
Pop-Location
Remove-Item ./public/wasm/.gitignore