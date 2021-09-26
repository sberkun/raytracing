importScripts("../pkg/raytracing.js");
wasm_bindgen("../pkg/raytracing_bg.wasm").then((wasm) => {

    const { Universe } = wasm_bindgen;
    let unv = Universe.new();
    console.log("started universe");
    
    onmessage = function(e) {
        if(e.data[0] == "render") {
            unv.render(e.data[1], e.data[2], e.data[3], e.data[4],
                e.data[5], e.data[6], e.data[7], e.data[8], e.data[9], e.data[10]);
        }
    }
    postMessage([0])
});


function export_tile(x,y,w,h, ar) {
    let dst = new Uint8Array(ar.byteLength);
    dst.set(ar);
    postMessage([1,x,y,w,h,dst.buffer], dst.buffer);
}

function notify(x) {
    console.log("notified! " + x);
}

function finish_render() {
    postMessage([2]);
}