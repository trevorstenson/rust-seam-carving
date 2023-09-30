import * as wasm from "seam-rs";

function getImageData(image) {
  const canvas = document.createElement('canvas');
  canvas.width = image.width;
  canvas.height = image.height;
  const ctx = canvas.getContext('2d');
  ctx.drawImage(image, 0, 0);
  return ctx.getImageData(0, 0, image.width, image.height).data;
}

function rawImageDataToDataUrl(data, width, height) {
  const imgData = new ImageData(data, width, height);
  const canvas = document.createElement('canvas');
  canvas.width = width;
  canvas.height = height;
  const ctx = canvas.getContext('2d');
  ctx.putImageData(imgData, 0, 0);
  return canvas.toDataURL();
}

function click_handler() {
  const main_image = document.getElementById('main_image');
  const output_image = document.querySelector('#output_image');
  output_image.src = '';
  const iterations = 250;
  const imageData = getImageData(main_image);
  const output = wasm.process_image(imageData, main_image.width, main_image.height, iterations);
  const dataUrl = rawImageDataToDataUrl(
    new Uint8ClampedArray(output.buffer),
    main_image.width - iterations,
    main_image.height
  );
  output_image.src = dataUrl;
}

const button = document.getElementById('run');
button.addEventListener('click', click_handler);