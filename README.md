A sane date/time library for Rust.

This library primarily ports the API of the excellent java.time library.

It is also leap-second aware.

## Overview

### Instant
Like the Java API, this crate implements its own type named `Instant`.
However, unlike the Java version, this instant is explicitly **not** a UTC instant, but instead is a TAI instant.

### Duration
Like the Java API, this crate implements its own type named `Duration`.
Unlike [`std::time::Duration`](https://doc.rust-lang.org/std/time/struct.Duration.html), `ephemeris::Duration` can be negative.
