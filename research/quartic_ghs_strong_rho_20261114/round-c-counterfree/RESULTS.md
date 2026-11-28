# Round C-CF result: field-counter-free strong rho

Status: deterministic implementation gates pass. No timing or frontier claim.

The merged GHS strong-rho arithmetic now has one const-generic implementation.
`strong-b<W>` selects `COUNT=true` and remains the exact counted reference;
`fast-b<W>` selects `COUNT=false` and emits `count_authoritative=false`. The
fast specialization removes `FastF3`'s `FieldCounts` read/modify/write hooks at
optimization time. It deliberately retains `NativeCounts` in the walk hot
loop, so “counter-free” in this round means field-counter-free only.
The analyzer requires `count_authoritative` to be boolean `true` when present
and rejects false, null, numeric and string values before modeled-cost parsing.
Legacy counted records without the field remain accepted. This prevents
fast-mode zero totals or malformed authority markers from entering M-eq.

## Deterministic evidence

- Field values, batch inverses, curve additions and scalar multiples match the
  existing frozen controls in counted and fast modes.
- Paired rho controls at widths 1, 8 and 32 recover the same planted scalar
  with identical steps, DPs, restarts, look-ahead, cycles, cycle additions and
  native diagnostics.
- All setup, walk and verify field totals in fast mode are zero.
- The q=223 `strong-b8` record is identical to retained Round B after excluding
  only `wall_s`; its operation totals remain authoritative.
- Optimized LLVM IR contains 185 arithmetic and 6 walk counter-SSA marker
  occurrences in the counted specialization and zero in `COUNT=false`. These
  are marker occurrences, not operation totals.

See `MANIFEST.json` for exact source, toolchain, artifact hash and authority;
`q223-paired.jsonl` for raw paired records; and `validation.txt` plus
`ir-gate.txt` for the gates.

## Claim boundary

The captured wall fields came from a single development correctness command
without checkpoint, A/A or ABAB gates and are excluded. This round does not
show a throughput gain, does not change M-eq, does not alter the end-to-end
frontier and does not move a leaderboard. A later performance round must use
the counted path for M-eq and independently qualify the fast path for wall-time
claims.
