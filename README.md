# Method
- Everything is benchmarked using [`wrk`](https://github.com/wg/wrk) (4.0.2).
- Code is formatted using formatting tools default configurations ([`prettier`](https://github.com/prettier/prettier), [`rustfmt`](https://github.com/rust-lang/rustfmt) ...).
- LoCs are reported by [`tokei`](https://github.com/Aaronepower/tokei), blank lines and comments are ignored. 

## Hello world
Best out of 5, bench command: \
`wrk -t12 -c400 -d10s http://localhost:8080/hello/noob`

Results for `Intel(R) Core(TM) i5-4690K CPU @ 3.50GHz` (4 cores):

| Language      | Platform/Toolchain | Framework     | LoC | Requests/sec | Transfer/sec | Perf. | 
|:-------------:|:------------------:|:-------------:|:---:|:------------:|:------------:|---| 
| Rust | rustc 1.33.0 | Warp | 5 | 298,769 | 36.47MB | 100% |
| Rust | rustc 1.33.0 | Actix | 10 | 287,908 | 35.15MB | 96.3% |
| JavaScript | Node 11.10.1 (PM2) | Koa | 9 | 82,880 | 12.01MB | 27.7% |
| JavaScript | Node 11.10.1 (PM2) | Express | 7 | 56,349 | 11.55MB | 18.9% |
| JavaScript | Node 11.10.1 | Koa | 9 | 30,016 | 4.35M | 10.0% |
| JavaScript | Node 11.10.1 | Express | 7 | 17,756 | 3.64MB | 5.9% |
