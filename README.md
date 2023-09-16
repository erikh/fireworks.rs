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

This is a TUI "screen saver" that displays fireworks on the terminal. It will
display them until Ctrl+C is pressed and is designed to work with `tmux`, etc,
to provide some bling on your terminal when you only need half of it.

Some notes:

10 fireworks are launched at start, and every 5 iterations (about half a
second) a new one is launched. Fireworks trail north and have a chance of
exploding on every frame. Each flare and explosion have random components to
the trail and characters used. If the terminal is resized, it clears and
rebuilds the scene anew.

## Author

Erik Hollensbe <git@hollensbe.org>
