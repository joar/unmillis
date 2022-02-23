# unmillis

Given 𝑛, solves for 𝑥 in the equation 

> 1970 + 𝑛 milliseconds = 𝑥  

where `1970` refers to `1970-01-01T00:00+00:00`.

In other words, 
```console
$ # can't remember what you did on the 1640995200000th?
$ unmillis 1640995200000th
2022-01-01T00:00:00+00:00
$ unmillis -1000
1969-12-31T23:59:59+00:00
$ unmillis 1000
1970-01-01T00:00:01+00:00
$ unmillis 1000,  # garbage characters will be ignored
1970-01-01T00:00:01+00:00
```

## Installation

- Binaries can be downloaded from [Releases](https://github.com/joar/unmillis/releases); or
- if you have `cargo`, you can run 
  ```
  cargo install unmillis
  ```

## Usage

```
USAGE:
    unmillis <TIMESTAMP_MILLIS>

ARGS:
    <TIMESTAMP_MILLIS>    A timestamp formulated as the number of milliseconds since "1970-01-
                          01T00:00:00+00:00".
                           • Trailing and leading garbage is thrown away, i.e.
                           • `1 hello there`, `1,` and `"1",` would all be interpreted as `1`.
                           • Negative numbers are fine, positive numbers are ok too, both have
                          some limitations:
                           • We can't construct datetimes outside the range of (-262144-01-
                          01T00:00:00Z, +262143-12-31T23:59:59.999999999Z), so
                           • we only accept input values in the range of (-8334632851200000,
                          8210298412799999)

OPTIONS:
        --help       Print help information
        --version    Print version information

```