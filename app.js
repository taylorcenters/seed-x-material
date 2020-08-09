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
  const [js_ready, tick] = wasm.start();
  window.tick = tick;
  js_ready(true);
}

async function load() {
    let wasm = await import('./pkg/package.js')
    run(wasm);
}

load();

new MDCSlider(document.querySelector('.mdc-slider'));
