import {MDCSlider} from '@material/slider';

window.enableClock = () => {
  const sendTick = () => {
    tick(new Date().toLocaleTimeString());
  };
  sendTick();

  setInterval(() => {
    sendTick();
  }, 1000);
};

let run = (wasm) => {
  const [js_ready, tick, slider_change] = wasm.start();
  window.tick = tick;
  js_ready(true);

  let slider = new MDCSlider(document.querySelector('.mdc-slider'));
  //slider.listen('MDCSlider:change', () => slider_change(slider.value.toString()));
}

async function load() {
    let wasm = await import('./pkg/package.js')
    run(wasm);
}

load();

