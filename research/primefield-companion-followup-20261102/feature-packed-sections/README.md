# Univariate encoding of mode sections

T-1930 extends the oriented-radius adapter using the exact ring encoding
in [DERIVATION.md](DERIVATION.md). See [RESULTS.md](RESULTS.md) for scoped
qualification and [report.pdf](report.pdf) for the visual report.

From this directory, compile driver.rs with rustc --edition=2021 -O
-C overflow-checks=yes --emit=link,dep-info -o /tmp/packed-check, bounded by90s.
Run /tmp/packed-check and /tmp/packed-check --inherited, each bounded by60s.
The standalone driver includes immutable copied predecessor modules; runtime
uses public points, and exhaustive scalars occur only in correctness fixtures.
The original4563-case cohort is unchanged. No cloud work was submitted.

Validation01 is kernel-only; validation02 adds the public adapter. Receipts
bind sources, logs and binaries. record.py records the original qualification
paths; it is not an independent verifier and should not overwrite frozen
receipts when replaying. Use a new output directory for future checks.
