# bosh
bosh is a Lisp that compiles to [Scratch](https://github.com/LLK/scratch-gui) project files. Vaguely inspired by the awesome [tosh](https://github.com/tjvr/tosh) :heart:

> Note: bosh was designed for Scratch 3.0 but produces Scratch 2.0 project files. This will likely change in the future, as the 3.0 file format becomes better documented.

---

```scheme
(declare-sprite "Sprite1"
  (when-gf-clicked
    (say "Hello, World!")
    (forever
      (next-costume)
      (wait 0.5))))
```
