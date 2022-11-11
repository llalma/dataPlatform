Setup:
To build web assembly ensure the vite.config.ts allowed paths is extended so the rust dir can be accessed. Otherwise svelte wont be able to use it.

RUN:

Runproject by excuting the following in powershell from src dir
"wasm-pack build rust --target web ; npm run dev"

1st Command "wasm-pack build rust --target web" Builds the latest rust code and compiles to wasm

2nd Command "npm run dev" Runs the server