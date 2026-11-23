# Character-domain certificate

[Derivation](DERIVATION.md), [results](RESULTS.md), [report](report.pdf).
This internal specialization relies on character-factory provenance followed
only by intersections. It is not a general-purpose replacement for phase filtering.

```sh
set -e
timeout 90 rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info driver.rs -o /tmp/certified-character-check
timeout 60 /tmp/certified-character-check
timeout 60 /tmp/certified-character-check --inherited
```

`generate.py` copies hash-bound predecessor sources and generates the optional
initializer, image, adapter and integrated checks. `certified-lift.rs` is authored.
`record.py` captures the named validation02 binary/log paths; use a new immutable
validation directory and adjust paths for a new replay.
