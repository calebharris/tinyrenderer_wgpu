---
layout: post
title: Introduction
date: 2024-04-15 22:29:00 -0400
tags: site news tinyrenderer wgpu webgpu
excerpt_separator: <!--more-->
---

Behold:

<!-- markdownlint-disable html -->
<style>
    canvas {
        background-color: black;
        display: block;
        margin-left: auto;
        margin-right: auto;
    }
</style>
<div id="tr-wgpu-intro"></div>
<script type="module">
    import init from "{{'/assets/tr_wgpu_intro.js' | relative_url }}";
    init().then(() => {
        console.log("WASM loaded");
    });
</script>

I know this picture doesn't look like much, but let me tell you why there's more
going on here than it appears.

<!--more-->

You may be thinking _What's the big deal? Is this a blog about putting images on
a web page? I already know how to do that._

Well, yes, technically it is a blog about putting images on a web page. But in,
like, a really complicated way. Stay with me.

By now, maybe you've decided to open the inspector and look around, assuming
you're still even here. If so, you've probably discovered the big secret:
there's no `<img>` tag!

_Gasp!_

Instead, you'll see something like the below, depending on which browser you're
using.

```html
<div id="wasm-example">
  <canvas
    tabindex="0"
    data-raw-handle="1"
    alt="winit window"
    width="800"
    height="800"
    style="width: 400px; height: 400px;"
  ></canvas>
</div>
<script type="module">
  import init from "/tinyrenderer_wgpu/assets/tinyrenderer_wgpu.js";
  init().then(() => {
    console.log("WASM loaded");
  });
</script>
```

As you can see, no image in sight (pun intended)! Instead, we have a canvas and
a JavaScript module, and a mysterious log message about loading WASM. So what's
going on here?

If you clone this repository, or click the "View on GitHub" button above, you'll
see that along with the blog content, there is also a Rust program. That program
(as of this writing) uses a library called [wgpu]{:target="\_blank"} to create a
3D rendering pipeline, load a texture, and display it on the screen. The arrows
are there so I know I got the various coordinate systems right and didn't
accidentally flip or rotate the image.

And it's pixelated because I made the original image kind of small and was too
lazy to make a bigger one.

That brings us to this blog's purpose. At some point in the last few months I
decided I wanted to learn how to make a video game. But not the "easy" way,
using an engine like Unreal or Godot. No, that's not how I do things. I wanted
to learn how the underlying graphics pipeline works and understand the math of
3D rendering. And I wanted to do it using Rust, which is a language I've admired
from afar, but haven't done significant work with yet.

Some early Googling led me to [wgpu]{:target="\_blank"}, which is a
cross-platform Rust graphics library that's based on the [WebGPU
API][webgpu]{:target="\_blank"} and also serves as the core of the WebGPU
integrations in [Firefox][firefox]{:target="\_blank"} and
[Deno][deno]{:target="\_blank"}.

In my searches, I came across [this great wgpu
primer][learn-wgpu]{:target="\_blank"}. But, after going through all of it, I
realized I still had a lot to learn about the principles and algorithms of 3D
rendering. I still didn't have all the tools I needed to create anything more
than the simplest of games.

Some more Googling led me to [tinyrenderer]{:target="\_blank"}, which is the
work of a [professor]{:target="\_blank"} at the University of Lorraine in
France. It's a series of lessons, generously put out on the Web for free, that
teaches one how to build a 3D renderer from scratch.

After running through the first lesson, the concept of this blog occurred to me:
for each lesson, get it working more-or-less according to the spirit of the
course (which is not to rely on 3D graphics libraries) and then come up with a
wgpu equivalent. And write about it!

I don't know if anyone is going to read this blog, but it has already served as
a scaffolding for my thoughts about this challenge. Imagining some of you out
there waiting for the next update will help motivate me to keep going. And
finally, one of my goals for the game I eventually plan to write is to have it
work on the Web. Preparing examples for this blog will help teach me how to do
that.

So there you have it, the inspiration behind the very words you're reading now.
In the next post, I'll provide some instructions on how to actually run the
code. And then we'll be off to the races.

Thanks for reading!

[wgpu]: https://wgpu.rs
[webgpu]: https://github.com/gpuweb/gpuweb
[firefox]: https://www.mozilla.org/en-US/firefox/
[deno]: https://deno.com
[learn-wgpu]: https://sotrh.github.io/learn-wgpu/
[tinyrenderer]: https://github.com/ssloy/tinyrenderer
[professor]: https://members.loria.fr/DSokolov/
