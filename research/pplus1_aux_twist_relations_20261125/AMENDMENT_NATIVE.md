# Native panel amendment

Frozen 2026-11-25 before the first native run. The original `PROTOCOL.md`
and its Python outputs remain historical diagnostics. Repository policy requires
native research code, so the admitted panel uses `native/src/main.rs` (Rust,
standard library only). The curve, arms, two-column base, and 1024-query cap
remain as frozen. The common query stream is now xorshift64 with initial state
20261125, transitions `s ^= s << 13; s ^= s >> 7; s ^= s << 17`, and each
`a,b = next() mod 131`. Rho starts from seeds 20261125 through 20261140 using
the same generator for two initial scalars.

The native panel records all raw rows, the common query stream, base points,
group-operation counts and elapsed time. Times are local diagnostics; the
repository's EKS-controlled benchmark environment is the performance gate.
No scaling claim follows from this small-field exact panel. The independent
native replay must validate each row on the target curve and solve the row
system over F_131 before a candidate is credited. Retain censored and failed
arms. The `twist_raw` zero-lift result is an expected mathematical control.
