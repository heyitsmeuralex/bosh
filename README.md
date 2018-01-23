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
