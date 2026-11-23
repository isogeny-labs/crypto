# Frozen common-public-query panel: the two 16,627-class bases

Frozen 2026-10-20 UTC before implementing or running this panel. This is
a new card, preserving the earlier interrupted full-base card and the
smaller-base XOR-cache comparison unchanged.

## Launch prerequisites

1. Finish the sixteen-attempt `xor-cache-v2` card. Its native receipt audit
   must pass all four matched endpoints, exact work conservation and the
   off/on control model/row comparisons. A wall-capped unmatched endpoint,
   changed search result, missing receipt or interruption stops this gate.
2. Implement an explicit `full-m4` mode with the policy below; keep the
   previous small-base command contracts intact. Test policy rejection,
   rebuild worker and general-field replayer from one committed revision,
   record their hashes and test output, then commit a host card before
   execution. Compile only after the preceding remote card has ended.
3. Run full-base constructed controls U off/on and D off/on, each followed
   by a separate general-field replay. Require the same complete SAT-model
   and sparse-row digests within each base; require identical search work
   and cache visit conservation. Stop before public queries on any failure.

## Exact comparison

- Curve `icv1-f2m83-tm6151469093347-debefd74`, a=0, subgroup
  2417851639230796216685689, cofactor 4, field polynomial
  z^83+z^45+z^2+z+1.
- U: retained public-x-hash base, object BLAKE3
  `1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d`.
- D: exported direct-subgroup D18 base, object BLAKE3
  `e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682`.
- Both have 16,627 signed-Frobenius columns / 2,760,082 points. Their
  accepted sets differ. This panel tests natural search on equal-size
  stored bases; the separate same-set preparation experiment supplies the
  prebuilt/on-demand comparison.
- Public corpus SHA-256
  `2895a78d28935a992d8d6155492eae928e165eafcc622e6c866080b8f91e70b5`.
  Use all eight indexed points. Exclude the validation scalar sidecar.
- Target 0 order U,D,D,U. For indices 1 through 7, use D,U at odd indices
  and U,D at even indices. Exactly eighteen natural attempts, all cache
  on; a repeat pair at target 0 checks execution repeatability. Public
  target indices, deterministic solver behavior and budgets are identical
  across U/D for each point.
- Policy `full-m4-common-public/v1`. Four summands, selectors and phases
  free during natural queries, one model, 10,000-conflict limit,
  300,000-variable and 8,000,000-domain-clause limits. Internal deadline
  1,790 s / independent outer cap 1,800 s. Durable ten-second progress and
  final/cap receipts preserve all completed work. Cache off is accepted
  only for the full-base constructed controls.

## Host, budget and failures

Use the already authorized SSH Spot instance and exact Docker image in
the preceding card, CPU 31, one CPU quota, hard 4 GiB, zero swap,
read-only root/inputs and no worker network. Launch no additional instance
and change no other jobs. The maximum natural allocation is eighteen
times 1,800 seconds (nine hours), plus the eight control/replay process
caps. Record host activity and label observations L0 while shared.

Run once in a new output directory. Every returned natural row needs
independent general-field replay; aggregate useful rank only from replayed
rows, never from the count of SAT models. Preserve caps, failures and
Spot interruptions. A cap is censored, not a zero-yield or UNSAT result.
An interrupted card requires a separate frozen continuation before retry.
Stop on invalid arithmetic/replay, missing instrumentation or policy drift.

## Accounting and decisions

Retain every attempt's import/index, model, root/search, relation/rank,
CPU/memory and container boundary receipts. Charge failed and capped work.
The recorded worker interval ends before final serialization; preserve
serialization, custody, construction/export and independent verification
as separate costs. No complete IC or same-point-rho result follows from
this panel alone. Candidate and best-runtime fields stay null.

A replayed natural row and useful-rank increment justify the next rank
collection/linear-algebra stage. Complete caps establish bounded search
effort for each base/point, with natural yield/rank unresolved. Exact
unverified solver UNSAT is preserved without upgrading it to a
mathematical proof. L0 timing cannot select a fastest solver or base.
The original full-cost, matched-rho and external-tool obligations remain.

Coordination: the explicit Crypto project scope check for the worker,
replayer and study was attempted before editing; the local Conductor
service refused the connection. Work remains on the isolated existing
feature branch; no exclusive source lease is claimed by this note.
