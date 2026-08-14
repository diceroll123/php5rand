# php5rand

A Rust crate that reproduces PHP5's `rand()`/`srand()` and `mt_rand()`/`mt_srand()` output bit-for-bit. Useful for replicating PHP5 seeded-random sequences (legacy compatibility, save-file/state reproduction, CTF and reverse-engineering work).

## Generators

- `Php5Random` - PHP5's plain `rand()`/`srand()`, which on Linux is backed by glibc's `random()`.
- `Php5MtRandom` - PHP5's `mt_rand()`/`mt_srand()`, a genuine Mersenne Twister (MT19937) with PHP's specific quirks (matrix-A bit selection and output shifting) reproduced exactly.

Both generators are validated against golden output vectors, some of which were cross-checked against a real `php5.6-cli` binary.

## Usage

```rust
use php5rand::{Php5Random, Php5MtRandom};

// rand() / srand()
let mut r = Php5Random::new(1);
let n = r.rand();               // next random u32
let n = r.rand_range(0, 100);   // random u32 in [0, 100]
r.srand(42);                    // reseed

// mt_rand() / mt_srand()
let mut mt = Php5MtRandom::new(1);
let n = mt.rand();
let n = mt.rand_range(0, 100);
mt.srand(42);
```

## License

MIT
