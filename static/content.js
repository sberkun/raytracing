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
            startRender(); break;
        case 1: 
            drawRenderData(e.data[1], e.data[2], e.data[3], e.data[4], e.data[5]); break;
        case 2:
            showPopup(); break;
    }

}

let skytype;
let scenetype;

function startRender() {
    hidePopup();
    document.getElementById("canvasWrapper").scrollIntoView({behavior: "smooth"});
    let h1 = document.getElementById("hue1inp");
    let h2 = document.getElementById("hue2inp");
    myWorker.postMessage(["render", canvas.width, canvas.height, 
        skytype, scenetype,
        parseInt(h1.value.substring(1,3), 16),
        parseInt(h1.value.substring(3,5), 16),
        parseInt(h1.value.substring(5,7), 16),
        parseInt(h2.value.substring(1,3), 16),
        parseInt(h2.value.substring(3,5), 16),
        parseInt(h2.value.substring(5,7), 16)]);
}

function selectSky(n) {
    skytype = n;
    let ssc = document.getElementById("skySelector").children;
    for(let a of ssc) { a.classList.remove("selected"); }
    ssc[n].classList.add("selected");
}

function selectScene(n) {
    scenetype = n;
    let ssc = document.getElementById("sceneSelector").children;
    for(let a of ssc) { a.classList.remove("selected"); }
    ssc[n].classList.add("selected");
}

selectSky(2);
selectScene(0);



function showPopup() {
    document.getElementById("popupWrapper").style.display = "block";
}

function hidePopup() {
    document.getElementById("popupWrapper").style.display = "none";
}

function scrollToContent() {
    document.getElementById("contentWrapper").scrollIntoView({behavior: "smooth"});
}

function exportPNG() {
    let aDownloadLink = document.createElement('a');
    aDownloadLink.download = 'render.png';
    aDownloadLink.href = canvas.toDataURL();
    aDownloadLink.click();
}


function setWorker() {
    myWorker = new Worker('static/worker.js');
    myWorker.onmessage = worker_message;
}

setWorker()


