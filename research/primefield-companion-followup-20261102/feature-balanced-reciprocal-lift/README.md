# Balanced homogeneous reciprocal lift

[Derivation](DERIVATION.md), [results](RESULTS.md), [illustrated report](report.pdf).
The standalone homogeneous transform accepts coefficient vectors with declared
length; the recovery lift additionally requires character-domain provenance.

```sh
set -e
timeout 90 rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info driver.rs -o /tmp/balanced-lift-check
timeout 60 /tmp/balanced-lift-check
timeout 60 /tmp/balanced-lift-check --inherited
```

`generate.py` copies hash-bound predecessor Rust files and generates the optional
adapter. `homogeneous.rs` is authored and includes independent coefficient and
point-evaluation checks. `record.py` captures the named validation01 binary/log
paths; use a fresh validation directory and adjust paths for later replay.
