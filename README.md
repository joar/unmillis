# unmillis

Given 𝑛, solves for 𝑥 in the equation 

> 1970 + 𝑛 milliseconds = 𝑥  

where `1970` refers to `1970-01-01T00:00+00:00`.

In other words, 
```console
$ unmillis -10
1969-12-31T23:59:59.990+00:00
$ unmillis 1000
1970-01-01T00:00:01+00:00
```

## Installation

- Binaries can be downloaded from [Releases](https://github.com/joar/unmillis/releases); or
- if you have `cargo`, you can run 
  ```
  cargo install unmillis
  ```
