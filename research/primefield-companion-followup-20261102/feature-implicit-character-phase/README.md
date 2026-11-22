# Implicit character phase and observation fibers

[Derivation](DERIVATION.md), [results](RESULTS.md), [report](report.pdf).
`implicit.rs` intersects a supplied monic constraint with the phase.
`observation.rs` checks the summary-only degree bound and sign-word invariance.
Enumeration is confined to correctness controls, not a recovery algorithm.

```sh
set -e
python3 generate.py
timeout 90 rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info driver.rs -o /tmp/implicit-phase-check
timeout 60 /tmp/implicit-phase-check
```

The caller certifies prime r. Input q must be monic, degree<r, and within the
supplied cap; L is in1..r-1 and epsilon is+/-1. Source generation binds the
native polynomial engine and GCD extension. `record.py` captures the preserved
versioned binaries and logs; later replays require new immutable paths.
