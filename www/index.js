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
  const iter_input = document.querySelector('#iter_count');
  output_image.src = '';
  const iterations = parseInt(iter_input.value);
  if (isNaN(iterations)) {
    alert('Invalid iteration count');
    return;
  }
  const imageData = getImageData(main_image);
  const output = wasm.process_image(imageData, main_image.width, main_image.height, iterations);
  const dataUrl = rawImageDataToDataUrl(
    new Uint8ClampedArray(output.buffer),
    main_image.width - iterations,
    main_image.height
  );
  output_image.src = dataUrl;
}

function mark_seam_handler() {
  const main_image = document.getElementById('main_image');
  const output_image = document.querySelector('#output_image');
  output_image.src = '';
  const imageData = getImageData(main_image);
  const output = wasm.mark_seam(imageData, main_image.width, main_image.height);
  const dataUrl = rawImageDataToDataUrl(
    new Uint8ClampedArray(output.buffer),
    main_image.width,
    main_image.height
  );
  output_image.src = dataUrl;
}

const file_upload = document.getElementById('file_input');
file_upload.addEventListener('change', (e) => {
  const file = e.target.files[0];
  const reader = new FileReader();
  reader.onload = (e) => {
    const image = document.getElementById('main_image');
    image.src = e.target.result;
  };
  reader.readAsDataURL(file);
  document.getElementById('output_image').src = '';
});

const button = document.getElementById('run');
button.addEventListener('click', click_handler);
const debug_button = document.getElementById('mark_seam');
debug_button.addEventListener('click', mark_seam_handler);