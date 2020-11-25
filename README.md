<div align="center">

  <h1><code>wasm-merge-images</code></h1>

  <strong>Image merger.</strong>

<!-- TODO: Update from template -->
  <p>
    <a href="https://travis-ci.org/rustwasm/wasm-pack-template"><img src="https://img.shields.io/travis/rustwasm/wasm-pack-template.svg?style=flat-square" alt="Build Status" /></a>
  </p>

</div>

## About
WebAssembly alternative to [lukechild's `merge-images`](https://github.com/lukechilds/merge-images).

The scenario is simple: I wanted to merge 2 images from [AR.js](https://github.com/AR-js-org/AR.js): 1. the camera and 2. the objects that are drawn in the canvas/video.

Though lukechild's `merge-images` works like a charm, it blocks the main thread and right now [OffscreenCanvas](https://caniuse.com/offscreencanvas) is not an option.

## 🚴 Usage

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
