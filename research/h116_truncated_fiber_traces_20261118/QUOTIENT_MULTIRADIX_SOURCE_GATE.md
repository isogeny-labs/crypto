# H116 growing-radix quotient-rank source gate

Protocol `H116-QUOTIENT-MULTIRADIX-001`, its launch manifest, executable
source, included scorer and retained inputs were frozen before computing any
rank at a radix other than six.

## Frozen hashes

```text
83b012f1e2ad7a5af3cc4f148a22f62e7de13333d6b73caea2a0cc5bb37b4790  QUOTIENT_MULTIRADIX_PROTOCOL.md
97904cb495321fc86e8f3903f59296bc6345f7ccf0b251f0dc1f035511ef05b4  QUOTIENT_MULTIRADIX_LAUNCH.json
7b4a704fb041c529de455d0e345e416d550e883306c420fe54cf1c3db4ef6147  h116_quotient_multiradix.rs
52b28350a37483547165cd6b7efc115005aa3453b1fa802ad1cb85f8177207a7  h116_quotient_digit_scorer.rs
c69de26d62be755c6cb27e793c2ed7bab20e7da066b77ed5abb027598ebd3016  QUOTIENT_DIGIT_OOS_MOMENTS.tsv
11386d5840d4404554d709423d7aba930937489ac86ac4412c8cdaa813538e25  QUOTIENT_DIGIT_OOS_LABELS.tsv
3efc862194feeef84ae76bb8c1e95fcc7fa3227fe8a924cd47d2d9b32f4b4c31  QUOTIENT_DIGIT_RANK_RESULT.json
```

## Pre-run build gate

```text
/workspace/.h116-rustup/toolchains/1.100.0-x86_64-unknown-linux-gnu/bin/rustc \
  --edition=2021 -O -D warnings h116_quotient_multiradix.rs \
  -o /tmp/h116_quotient_multiradix
/tmp/h116_quotient_multiradix --selftest
h116 quotient multiradix selftest: pass
3121756731fb43b6161e0745ca9ccf59f25561e8e22520e8522fc0f726797735  /tmp/h116_quotient_multiradix
```

No new-radix histogram, path statistic, shuffle score or cross-radix slope
existed when this gate was written. The first permissible measured run is from
the first pushed commit containing these exact bytes. Any affected change
requires a new protocol identifier and source gate.
