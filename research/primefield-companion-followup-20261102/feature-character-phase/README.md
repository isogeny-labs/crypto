# Character-phase recovery

See [DERIVATION.md](DERIVATION.md) for the invariant, domain formulas and
costs, [RESULTS.md](RESULTS.md) for qualification scope, and [report.pdf](report.pdf)
for the illustrated report. This additive adapter leaves prior cohorts intact.

Replay from this directory with a fresh binary:

```sh
set -e
timeout 90 rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info driver.rs -o /tmp/character-check
timeout 60 /tmp/character-check
timeout 60 /tmp/character-check --inherited
```

`generate.py` copies hash-bound predecessor sources and generates adapter/image
wrappers. `character-domain.rs` and `character-checks.rs` are authored files.
`record.py` captures the named validation02 binary and logs; replay paths must
be adjusted before recording a new immutable validation directory.
