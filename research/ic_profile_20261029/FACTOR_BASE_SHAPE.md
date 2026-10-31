# Factor-base shape at 20 bits: contiguous, strided, coverage-selected

Follow-up to the N=32/45/56/64 sweep, which fixed the base *size* at 45. This
asks the size-free question: does the *selection* of 45 abscissae matter?

Counted natively (no pipeline run, no cloud), on the profile curve from
`PrimeSearch { bits: 20, seed: 59297 }`:

```
p = 563153   a = 51142   b = 548946   r = 562427
```

Each policy picks 45 on-curve abscissae; a base has 90 signed points, so 4,095
unordered pairs. Pair sums are keyed by the sign-folded x the oracle uses, so
the ceiling is 2,048 distinct keys.

| policy | distinct keys | coverage | vs contiguous |
|:--|--:|--:|--:|
| contiguous (45 smallest) | 2,011 | 0.3576% | — |
| strided (spread over the first 200) | 2,017 | 0.3586% | +0.30% |
| coverage-selected (best of 200 random 45-subsets) | 2,023 | 0.3597% | +0.60% |

The baseline is at 98.2% of the ceiling. The largest gain any policy can take
is +0.6% coverage, which is at most ~0.4% off S — against 8.9% measured for
N=64 and ~35% predicted for m=3. Shape is therefore closed, and the earlier
analytic bound (`|F+F|/r <= yield`) is confirmed on the real curve: 2,011 keys
fold to 0.3576% per probe, and the measured walk hits either sign, so the
observed 0.727% yield is 2x that, exactly as expected.

Reproduce: `examples/fb_coverage.rs`, run
`rustup run 1.99.1 cargo run --example fb_coverage --ignore-rust-version`.

Not a result: coverage is a counting exercise, not a pipeline measurement, and
it makes no speed claim. Its use is to say which experiment *not* to run.
