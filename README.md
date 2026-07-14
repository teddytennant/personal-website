# personal-website

My personal site. Rust, compiled to WebAssembly.

Live at [teddytennant.com](https://teddytennant.com).

## Why

Rust on the frontend has a reputation for being a tradeoff: you get speed and type safety, and in exchange you give up the design. The nice motion library is React-only, the shader package ships a React component, the icon set is a React package. So the Rust version of your site ends up looking like the Rust version of your site.

I don't think that trade is real. It just requires you to stop treating those libraries as load-bearing.

This site was a React app. It leaned on Framer Motion for scroll reveals, Lenis for smooth scrolling, and a WebGL shader package for the animated grain background. None of those have Rust equivalents, which is usually where the port stops. But once you look at what they actually do for you, most of it is thinner than the dependency suggests:

- Framer Motion, in this codebase, was doing scroll-triggered reveals and a couple of enter animations. That's an IntersectionObserver and some CSS transitions, which is roughly what Framer compiles to anyway.
- Lenis is exponential damping on a scroll position, driven off requestAnimationFrame. About 150 lines.
- The shader package ships its GLSL unminified. The GLSL was never the hard part. The React wrapper around it was.
- The icons are SVG.

So the shader here is the same shader, running through WebGL2 via web-sys. The smooth scrolling is the same curve. The reveals fire at the same thresholds with the same easing. It looks the way it looked, because none of the design was living in the dependencies. It was living in the CSS and the GLSL, and both of those are portable.

The design system came over untouched. `style/input.css` is the old `index.css`, rules unchanged, and Tailwind scans `.rs` files for class strings exactly like it scanned `.jsx`. That part is not a port at all. It's a copy.

Fast and good-looking are not in tension. They were just in different packages.

## Stack

Leptos 0.8, client-rendered, built with Trunk. Tailwind v4. No JavaScript dependencies, and no JavaScript in the output beyond the wasm-bindgen glue.

## Develop

```bash
trunk serve            # localhost:8080
trunk build --release  # static bundle in dist/
cargo test
```

`dist/` is static. It deploys anywhere.

## Layout

```
src/
  app.rs          routing and the page transition
  motion.rs       scroll reveals and enter animations
  lenis.rs        smooth scrolling
  theme.rs        light/dark, persisted only once you choose
  bg/webgl.rs     the grain gradient shader
  components/
  pages/
style/input.css   the design system, unchanged from the React site
essays-gen/       renders the essay pages to static HTML at build time
```

## Essays

The essays are their own visual world: different fonts, their own stylesheet, no
Tailwind, and none of the site chrome. Rendering them as WASM routes would mean
inheriting the global styles and shipping prose as a WASM download, so instead
`essays-gen` authors them in Rust ([maud](https://maud.lambda.xyz)) and emits
static HTML at build time, via a Trunk pre-build hook. Source of truth is `.rs`;
the `.html` is build output, the same way the site itself is `.rs` compiled to
`.wasm`. The pages stay static and fast, and keep rendering exactly as they did.
