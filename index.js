
import init, { test } from "https://arwinneil.github.io/wasm-router-poc/pkg/wasm_router.js";
const runWasm = async () => {
    const rustWasm = await init(
        "https://arwinneil.github.io/wasm-router-poc/pkg/wasm_router_bg.wasm"
    );

    rustWasm.test()
};
runWasm();
