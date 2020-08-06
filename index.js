
import init from "/pkg/wasm_router.js";
const runWasm = async () => {
    const rustWasm = await init(
        "/pkg/wasm_router_bg.wasm"
    );

    rustWasm.main()
};
runWasm();