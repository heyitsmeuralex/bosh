# bosh
bosh is a lisp dialect that compiles to [Scratch 3.0](https://github.com/LLK/scratch-gui) project files. Vaguely inspired by the awesome [tosh](https://github.com/tjvr/tosh) :heart:

> **Note:** bosh compiles to Scratch 3.0 JSON. It will _not_ work with Scratch 2.0 or below.

---

```scheme
(declare-sprite "Sprite1"
  (when-gf-clicked
    (say "Hello, World!")
    (forever
      (next-costume)
      (wait 0.5))))
```

### install
You'll need [rust](https://rustup.rs/) and [node](https://nodejs.org/). You may need rust nightly/beta.

```sh
> git clone https://github.com/heyitsmeuralex/bosh
> cd bosh

> rustup target add wasm32-unknown-unknown
> cargo install cargo-web
> npm install

> npm run build
# visit app/index.html in a web browser
```

You can automatically build `compiler/` and `app/` on file change with `npm run watch`.
