# fireworks demo ported from python

This is the [fireworks demo](https://github.com/erikh/fireworks) that I wrote
to learn python from. I tried to see what it would also look like in rust, and
I was pleasantly surprised at the results.

One thing this version does differently is that it moves the cursor position
based on a difference of the previous draw and the current; this results in a
much more optimized draw routine. You will notice the frames per second much
higher and I feel the animation is a little better. It is also much easier on
the CPU.

This is published as [`fireworks-erikh` on
crates.io](https://crates.io/crates/fireworks-erikh) and installs a `fireworks`
binary when you `cargo install` it. This conflicts with the
[`fireworks`](https://crates.io/crates/fireworks) crate which does more or less
the same thing, just done by someone else, hah. Enjoy the weird terminal
screensavers I guess.

## Author

Erik Hollensbe <git@hollensbe.org>
