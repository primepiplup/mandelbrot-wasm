import {add_one} from "mandelbrot-wasm";

export function setupCounter(element: HTMLButtonElement) {
  let counter = 0
  const setCounter = (count: number) => {
    counter = count
    element.innerHTML = `count is ${counter}`
  }
  element.addEventListener('click', () => setCounter(add_one(counter)))
  setCounter(0)
}
