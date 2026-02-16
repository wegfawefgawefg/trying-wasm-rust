import("./pkg").catch((error) => {
    console.error("Failed to initialize WASM module", error);
});
