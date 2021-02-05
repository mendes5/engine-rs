### EngineRS (unnamed engine)

Just a experimental WIP game engine im building for fun and for learning Rust.

### Tech:

## 3D Rigid Body Physics
It [Bullet Physics](https://github.com/bulletphysics/bullet3) for real-time 3D collision detection.

Support for rust is provided by the wrapper for rust [bulletrs](https://github.com/not-fl3/bulletrs).

Examples:
```
sphere
```

## Javascript and WebAssembly
It uses [Deno](https://deno.land/) under the hood for Javascript and WebAssembly support.
But instead of using whole Deno itself, it only uses a modified version of it's [core package](https://github.com/denoland/deno/tree/master/core).

`deno_core` itself is a wrapper for [rusty_v8](https://github.com/denoland/rusty_v8), so in the end, we end up using [v8](https://v8.dev/).

As `deno_core` is very opinionated to be used in servers and cli contexts in a secure manner, its Rust/Javascript bridge (called ops) is a [bit slow](https://gist.github.com/mendes5/43d1252142dae0e4339848608d8b7128#file-output-txt), as all data passed must be serialized. The modified version intents to expose the raw V8 api to the `JSRuntime` object, while keeping still useful parts like: module loading, async code and error handling.

Examples:
```
javascript-hello-world
javascript-builtin
javascript-modules
javascript-exceptions
javascript-op-sum
javascript-webassembly
```

## 2D Graphics

It uses [Skia](https://skia.org/) (the graphics engine for Google Chrome) for basic 2D graphics, the created images can be turned into OpenGL textures, and be used for stuff like: GUIs, performance graphs, and writting to disk. Although there isn't a complete [Canvas2D](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D) like API yet, basic support exists.

## ECS Implementation

There is a basic Entity Component System implementation. Although it is not the fastest implementation out there ([build](./images/build.png), [update](./images/update.png)) it's has a very nice syntax, and is easy to use IMO.

## Windowing and OpenGL bindings

The engine ships with windowing functions, OpenGL 4.5 core profile functions, and a basic graphics API.

## Texture Atlas Loading (WIP)

Texture Atlases created with [TexturePacker](https://www.codeandweb.com/texturepacker) can be loaded using the JSON output format.