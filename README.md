# Odd Sketch Rust

Lightweight rust implementation of the [Odd Sketch](https://www.itu.dk/people/pagh/papers/oddsketch.pdf) data structure by M. Mitzenmacher et. al. 

## Benchmarking

```bash
cargo bench
```
### Results

**i5-8600K CPU @ 3.60GHz**

* [Insert single](https://hlb8122.github.io/oddsketch-rs/oddsketch%20insert%20single/report/index.html)
* [Insert million](https://hlb8122.github.io/oddsketch-rs/oddsketch%20insert%201%20million/report/index.html)
* [Decode](https://hlb8122.github.io/oddsketch-rs/oddsketch%20decode/report/index.html)
