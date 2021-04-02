# cistemrs

Rust translation of the [CISTEM](https://github.com/LeonieWeissweiler/CISTEM) stemmer (Weißweiler & Fraser, 2017). Tested against the data provided [here](https://github.com/LeonieWeissweiler/CISTEM/pull/6#issuecomment-524457146).

Aggressive and fast stemmer for German data. Easy to adjust and extend, works very well for information retrieval tasks.

## Usage

Add the dependency to your `Cargo.toml`:

```rust
[dependencies]
cistemrs = { git = "https://github.com/kldtz/cistemrs.git" }
```

Split or segment tokens.

```rust
use cistemrs::stem;

let s = stem("schönes", false);
assert_eq!(s, "schon");

let (l, r) = segment("schönes", false);
assert_eq!(l, "schön");
assert_eq!(r, "es");
```


## Reference

Weißweiler, L., & Fraser, A. (2017, September). Developing a stemmer for German based on a comparative analysis of publicly available stemmers. In International Conference of the German Society for Computational Linguistics and Language Technology (pp. 81-94). Springer, Cham.

## License

Both the original and this translation are under [MIT license](LICENSE).