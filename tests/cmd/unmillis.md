# [`trycmd`](https://github.com/assert-rs/trycmd) Integration Tests for [`unmillis`](https://github.com/joar/unmillis)

## Happy `unmillis` usage
```
$ unmillis --help
unmillis [..]
Joar Wandborg
Given 𝑛, solves for 𝑥 in the equation `unix-epoch + 𝑛 milliseconds = 𝑥`

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

```
$ unmillis --version
unmillis [..]

```

```
$ unmillis -10
1969-12-31T23:59:59.990+00:00

```

```
$ unmillis 1640995200000th
2022-01-01T00:00:00+00:00

```

## Infuriating `unmillis` usage

```
$ unmillis 1111111111111111111
? 1
Error: could not generate RFC 3339 datetime from millis: 1111111111111111111

Caused by:
    FromTimestamp error: Sorry, we can't handle timestamps outside the range (-8334632851200000, 8210298412799999), because we can't represent datetimes outside the range (-262144-01-01T00:00:00Z, +262143-12-31T23:59:59.999999999Z)

```

```
$ unmillis
? 2
error: The following required arguments were not provided:
    <TIMESTAMP_MILLIS>

USAGE:
    unmillis <TIMESTAMP_MILLIS>

For more information try --help

```

```
$ unmillis nine 
? 1
Error: Failed to parse timestamp millis from "nine"

Caused by:
    0: could not parse integer from trimmed string ""
    1: cannot parse integer from empty string

```

```
$ unmillis nine hundred
? 2
error: Found argument 'hundred' which wasn't expected, or isn't valid in this context

USAGE:
    unmillis <TIMESTAMP_MILLIS>

For more information try --help

```