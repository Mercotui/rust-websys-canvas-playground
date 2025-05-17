# rust-websys-canvas-playground

This is a vibe-coded HTML5 canvas test using `web-sys` and `wasm-bindgen`.
Let's see if LLMs can teach me some Rust techniques.

Conclusion after some prompts: 
Eh, yeah it can teach me, but man I dislike the vibe-coding workflow.
It's like babysitting a really smart toddler.

## Build & Run

1. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/):

   ```
   cargo install wasm-pack
   ```

2. Build the project:

   ```
   wasm-pack build --target web --out-dir static/pkg
   ```

3. Serve the `static/` directory:

   ```
   cd static
   python -m http.server 8080
   ```

4. Open [http://localhost:8080](http://localhost:8080) in your browser.

Click on the canvas to draw red circles!

---

**Dependencies:**
- `wasm-bindgen`
- `web-sys`
