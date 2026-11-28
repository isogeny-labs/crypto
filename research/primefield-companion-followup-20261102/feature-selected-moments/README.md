# Selected terminal trace moments

T-1934 replaces the terminal quotient product with count/first-moment pairing.
See DERIVATION.md, RESULTS.md and report.pdf. Earlier representatives/prefixes
may remain dense. Runtime does not enumerate scalars or image roots.

From the repository root, using fresh immutable replay output paths:

```sh
set -e
timeout 90 rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info research/primefield-companion-followup-20261102/feature-selected-moments/driver.rs -o /tmp/selected-moments-check
timeout 60 /tmp/selected-moments-check
```

The predecessor certified-prime/matching-source contract applies. Terms,
retained node slots, monomial pairs and lookup probes are separately capped.
Caps.output is unused: this path returns two field values, not a degree-d
polynomial. Multiple images remain unresolved. Source setup/calls, shifts,
validation, lookups, wrapping/trace scalings, copies, allocation and verification
remain charged. record.py is original producer metadata, not independent
verification. Large-transcript refusal is retained with no fallback.
