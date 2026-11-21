# G3 attempt notes (2026-11-21)

PROTOCOL G3: slope of log₂(charged adds/target) vs log₂(|Fₓ|) below the
pair-table floor (~1) at polynomial memory, with exact verdicts.

## What was tried / ruled out

| Idea | Why it fails G3 / exactness |
|---|---|
| Random subsample of F of size \|F\|^α | False negatives on true hits → exactness fail; expected e2e adds stay Θ(\|F\|) |
| Charge adds only on Bloom passes | Accounting diverges from sibling_pair equivalence; field work still Θ(\|F\|) — not a PROTOCOL advance |
| 8-wide scalar lanes (lever 2) | Wall/throughput only; charged adds/target unchanged → slope stays ~1 |
| IFMA subtract-gpu | Constant-factor wall on same m=3 search; same S / same floor |
| IFMA kmitm lockstep (`IFMA_ATTEMPT.md`) | Wall/throughput only (≤~1.5× vs scalar8 at m=3–4); charged adds unchanged → slope stays ~1 |
| Higher m (kmitm:m=5) | Lower absolute e2e S at fixed recipes; oracle adds/target still ~2\|F\| on misses |

## Still open for a true pass

An algorithm that, on typical misses, performs **o(\|F\|)** group-addition
equivalents while never missing a true hit, at polynomial memory. Known
prime-field Semaev/MITM rootfinders sit on the pair-table or enumeration
floors (pdp_root_finder_20261012).

## Claim hygiene

G3 remains **fail-closed**. The e2e cost-vs-bits slope move (SCOREBOARD) is
a different measurement and must not be read as a G3 pass.
