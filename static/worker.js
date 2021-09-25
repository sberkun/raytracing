importScripts("../pkg/raytracing.js");
wasm_bindgen("../pkg/raytracing_bg.wasm").then((wasm) => {

    const { Universe } = wasm_bindgen;
    let unv = Universe.new();
    console.log("started universe");
    
    onmessage = function(e) {
        if(e.data[0] == "render") {
            unv.render(e.data[1], e.data[2]);
        }
    }
    postMessage([0])
});


function export_tile(x,y,w,h, ar) {
    let dst = new Uint8Array(ar.byteLength);
    dst.set(ar);
    postMessage([1,x,y,w,h,dst.buffer], dst.buffer);
}