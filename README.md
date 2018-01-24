**work in progress**

# bosh [![](https://img.shields.io/crates/v/bosh_compiler.svg)](https://crates.io/crates/bosh_compiler) [![](https://docs.rs/bosh_compiler/badge.svg)](https://docs.rs/bosh_compiler)
<sup>(docs.rs builds are broken on their end at the moment, please use `cargo doc` locally)</sup><br>
bosh is a lisp dialect that compiles to [Scratch 3.0](https://github.com/LLK/scratch-gui) project files. Vaguely inspired by the awesome [tosh](https://github.com/tjvr/tosh) :heart:

```
> cargo install bosh
> bosh --help
```

> **Note:** bosh compiles to Scratch 3.0 JSON. It will _not_ work with Scratch 2.0 or below.

Check out the [playground](http://bosh.imalex.xyz)!

```scheme
; raw sauce, no ketchup
(sprite "Big Shaq"
  (costumes
    (from-url "https://a.pomf.space/kjimqhiplqfa.png"))
  (scripts
    (when-gf-clicked
      (say "2 plus 2 is 4"))))
```

---

### building
bosh is comprised of a single backend, implemented in rust, and two frontends:
* the cli, written in rust
* the playground, written in javascript

```sh
> git clone https://github.com/heyitsmeuralex/bosh
> cd bosh
```

### the cli
To build the standalone compiler, you'll need [rust](https://rustup.rs).

```sh
> cd cli
> cargo install
> bosh --help
```

Then use `bosh project.txt > project.json` to compile your projects.

#### the playground
You'll need [node](https://nodejs.org/) and [rust](https://rustup.rs) **nightly** (`rustup install stable`) to build the [playground](http://bosh.imalex.xyz) locally. This is useful for testing with the actual [Scratch 3.0 runtime](https://github.com/LLK/scratch-vm).

The compiler is compiled to webassembly in order for the playground javascript to use it.

```sh
> rustup target add wasm32-unknown-unknown
> cargo install cargo-web
> npm install

> npm run build
# visit app/index.html in a web browser
```

You can automatically build `compiler/` and `app/` on file change with `npm run watch`.
