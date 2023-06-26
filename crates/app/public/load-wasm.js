import init from "/./assets/dioxus/app.js";
init("/./assets/dioxus/app_bg.wasm").then((wasm) => {
  if (wasm.__wbindgen_start == undefined) {
    wasm.main();
  }
});
