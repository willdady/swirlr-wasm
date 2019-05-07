import * as wasm from "swirlr-wasm";

const getStream = async () => {
  return navigator.mediaDevices.getUserMedia({
    audio: false,
    video: {
      width: 1280,
      height: 720
    }
  });
};

const snapshotVidToCanvas = () => {
  const vid = document.getElementById("vid");
  const canvas = document.getElementById("canvas");
  const ctx = canvas.getContext("2d");
  ctx.drawImage(vid, (1280 - 720) * 0.5, 0, 720, 720, 0, 0, 512, 512);
  wasm.render();
  window.requestAnimationFrame(snapshotVidToCanvas);
};

const main = async () => {
  const stream = await getStream();
  const vid = document.getElementById("vid");
  vid.srcObject = stream;
  snapshotVidToCanvas();
};

main();
