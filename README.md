# Method

-   Everything is benchmarked using [`wrk`](https://github.com/wg/wrk) (4.0.2).
-   Code is formatted using formatting tools default configurations ([`prettier`](https://github.com/prettier/prettier), [`rustfmt`](https://github.com/rust-lang/rustfmt), [`gofmt`](https://golang.org/cmd/gofmt/) ...).
-   LoCs are reported by [`tokei`](https://github.com/Aaronepower/tokei), blank lines and comments are ignored.

## Hello world

Best out of 5, bench command: \
`wrk -t12 -c400 -d10s http://localhost:8080/hello/noob`

[Laptop] Results for `Intel(R) Core(TM) i5-8350U CPU @ 1.70GHz` (8 cores):

|  Language  | Platform/Toolchain | Framework | LoC | Requests/sec | Transfer/sec | Perf. |
| :--------: | :----------------: | :-------: | :-: | :----------: | :----------: | ----- |
|    Rust    |    rustc 1.33.0    |   Warp    |  5  |   328,201    |   40.06MB    | 100%  |
|    Rust    |    rustc 1.33.0    |   Actix   | 10  |   299,213    |   36.53MB    | 91.2% |
|     Go     |      Go 1.12       |     -     | 14  |   193,198    |   23.77MB    | 58.9% |
|  Haskell   |     GHC 8.6.3      |  Scotty   |  6  |   139,532    |   22.75MB    | 42.5% |
| JavaScript | Node 11.10.1 (PM2) |  Express  |  7  |    44,327    |    9.09MB    | 13.5% |
| JavaScript | Node 11.10.1 (PM2) |    Koa    |  9  |    40,770    |    5.91MB    | 12.4% |
| JavaScript |    Node 11.10.1    |    Koa    |  9  |    29,476    |    4.27M     | 9.0%  |
| JavaScript |    Node 11.10.1    |  Express  |  7  |    20,445    |    4.19MB    | 6.2%  |

[Desktop] Results for `Intel(R) Core(TM) i5-4690K CPU @ 3.50GHz` (4 cores):

|  Language  | Platform/Toolchain | Framework | LoC | Requests/sec | Transfer/sec | Perf. |
| :--------: | :----------------: | :-------: | :-: | :----------: | :----------: | ----- |
|    Rust    |    rustc 1.33.0    |   Warp    |  5  |   298,769    |   36.47MB    | 100%  |
|    Rust    |    rustc 1.33.0    |   Actix   | 10  |   287,908    |   35.15MB    | 96.3% |
|  Haskell   |     GHC 8.6.3      |  Scotty   |  6  |   112,167    |   18.29MB    | 37.3% |
| JavaScript | Node 11.10.1 (PM2) |    Koa    |  9  |    82,880    |   12.01MB    | 27.7% |
| JavaScript | Node 11.10.1 (PM2) |  Express  |  7  |    56,349    |   11.55MB    | 18.9% |
| JavaScript |    Node 11.10.1    |    Koa    |  9  |    30,016    |    4.35M     | 10.0% |
| JavaScript |    Node 11.10.1    |  Express  |  7  |    17,756    |    3.64MB    | 5.9%  |

## JSON serialization

Best out of 5, bench command: \
`wrk -t12 -c400 -d10s http://localhost:8080/users/2`

[Laptop] Results for `Intel(R) Core(TM) i5-8350U CPU @ 1.70GHz` (8 cores):

|  Language  | Platform/Toolchain | Framework | LoC | Requests/sec | Transfer/sec | Perf. |
| :--------: | :----------------: | :-------: | :-: | :----------: | :----------: | ----- |
|    Rust    |    rustc 1.33.0    |   Warp    | 30  |   338,968    |   44.89MB    | 100%  |
|    Rust    |    rustc 1.33.0    |   Actix   | 39  |   321,699    |   45.41MB    | 94.9% |
|     Go     |      Go 1.12       |     -     | 51  |   194,672    |   27.48MB    | 57.4% |
|  Haskell   |     GHC 8.6.3      |  Scotty   | 22  |   102,412    |   19.14MB    | 30.2% |
| JavaScript | Node 11.10.1 (PM2) |  Express  | 21  |    42,815    |    9.92MB    | 12.6% |
| JavaScript | Node 11.10.1 (PM2) |    Koa    | 23  |    40,662    |    6.67MB    | 12.0% |
| JavaScript |    Node 11.10.1    |    Koa    | 23  |    28,068    |    4.60M     | 8.3%  |
| JavaScript |    Node 11.10.1    |  Express  | 21  |    18,386    |    4.26MB    | 5.4%  |
