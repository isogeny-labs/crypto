# Bounded projector and moment extraction

T-1933 adds a capped algebraic extractor to the implicit three-mode circuit.
See DERIVATION.md, RESULTS.md and report.pdf. The public adapter can recover
singleton images without receiving Y. Multiple images or an unanchored
singleton remain unresolved. There is no runtime scalar/root scan or hidden
factoring fallback. Caller certifies the same prime/order/source configuration
as the public-source predecessor.

From the repository root:

```sh
set -e
timeout 90 rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info research/primefield-companion-followup-20261102/feature-support-projector/driver.rs -o /tmp/projector-check
timeout 60 /tmp/projector-check
```

Use new immutable output paths for replay. record.py captures original
qualification paths; it is producer metadata, not independent verification.
The successful qualification includes an intentional retained large-field
term-cap refusal. No dense fallback follows a refusal.

Per-map support, cumulative retained node slots, entered monomial pairs and
output degree have separate caps. They are not process-RAM or total-work caps.
Source setup/calibration, API calls, point shifts, container operations, copies,
allocations, coefficient arithmetic, Newton inversions and final point checks
remain additional charged work. The sparse-projector lower bound explains why
the next direction is selected trace extraction, not enlarging this attempt.
