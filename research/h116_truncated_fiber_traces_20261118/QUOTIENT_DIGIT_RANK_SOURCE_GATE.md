# H116 quotient-digit rank source gate

Protocol `H116-QUOTIENT-DIGIT-RANK-001`, the launch manifest, the executable
source, its included scorer source, and all retained OOS inputs were frozen
before the first rank run.

## Frozen hashes

```text
981daaf413a8ecdb6893fd1abe4db5b3c1617d76a2d79f91667eaa732c221dfe  QUOTIENT_DIGIT_RANK_PROTOCOL.md
f122a426c3e1fedf2f1bcbe922867208c9af6f8a899cc170323f672cb721c348  QUOTIENT_DIGIT_RANK_LAUNCH.json
4501ab59343eb048129986b4e3934388ab547de202cc4e5cf64950fb617d4793  h116_quotient_digit_rank.rs
52b28350a37483547165cd6b7efc115005aa3453b1fa802ad1cb85f8177207a7  h116_quotient_digit_scorer.rs
c69de26d62be755c6cb27e793c2ed7bab20e7da066b77ed5abb027598ebd3016  QUOTIENT_DIGIT_OOS_MOMENTS.tsv
11386d5840d4404554d709423d7aba930937489ac86ac4412c8cdaa813538e25  QUOTIENT_DIGIT_OOS_LABELS.tsv
7c13c6d0da2f2828422fa48ce6cb0d18e05f08822e1f6a33a01cf6fd22f40cf8  QUOTIENT_DIGIT_OOS_RESULT.json
```

## Pre-run build gate

The source was compiled without warnings using the pinned local compiler:

```text
/workspace/.h116-rustup/toolchains/1.100.0-x86_64-unknown-linux-gnu/bin/rustc \
  --edition=2021 -O -D warnings h116_quotient_digit_rank.rs \
  -o /tmp/h116_quotient_digit_rank
/tmp/h116_quotient_digit_rank --selftest
h116 quotient digit rank selftest: pass
2b0fa5542a25f0d7fba1c210c7459b6209e56ee2356a0af1e2ec8dfb7998dd86  /tmp/h116_quotient_digit_rank
```

No rank histogram, recursive path statistic, or shuffle score affected by this
protocol existed when this gate was written. The first permissible measured
run is from the first pushed commit containing these exact bytes. Any source or
input change requires a new protocol identifier and source gate.
