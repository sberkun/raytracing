const canvas = document.getElementById("myCanvas");
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;
const ctx = canvas.getContext("2d");

function drawRenderData(x, y, w, h, arbuf) {
    let imdata = ctx.createImageData(w,h);
    let data = imdata.data;
    let srcdata = new Uint8Array(arbuf);
    for(let a = 0, b = 0; a < data.length; a += 4, b+= 3) {
        data[a + 0] = srcdata[b + 0];
        data[a + 1] = srcdata[b + 1];
        data[a + 2] = srcdata[b + 2];
        data[a + 3] = 255;
    }
    ctx.putImageData(imdata, x, y);
}


function worker_message(e) {
    switch(e.data[0]) {
        case 0: 
            myWorker.postMessage(["render", canvas.width, canvas.height]); break;
        case 1: 
            drawRenderData(e.data[1], e.data[2], e.data[3], e.data[4], e.data[5]); break;
    }

}

function setWorker() {
    myWorker = new Worker('static/worker.js');
    myWorker.onmessage = worker_message;
}

setWorker()


