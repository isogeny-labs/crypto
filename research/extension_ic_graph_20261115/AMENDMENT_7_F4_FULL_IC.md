# Amendment 7: complete native S4/F4 in graph index calculus

Freeze before full-pipeline observations of this oracle. Basis-only probe08
has33of36controlled systems at0.135–0.173of the full reference coordinate
solver's wall time; two reference-closure failures and one A/A rejection stay
retained. That result justifies implementing extraction, not claiming a
pipeline gain. [pcc:unverified]

## Integrated algorithm and correctness boundary

The native F4 run uses degree24 and no early staircase/Hilbert stop. Full
critical-pair completion, finite staircase dimension<=64, and all required
border normal forms are required before action-matrix/eigenvalue/cubic
recovery. Reuse the existing candidate verification and group-sign settlement.
Unsupported/truncated quotients use the existing charged coordinate/triple
fallback. Failed attempt work stays in the total; no rho collision recovery
or generic ECDLP fallback is introduced. Trial root RNG is committed only on
successful native extraction, so a failed attempt cannot contaminate fallback
randomness. Historical solver defaults remain unchanged. [pcc:unverified]

Correctness checks include every target on the p7/seed3 group against an
independent full signed-triple table, both observed dimension60reference
failures without fallback, and matched graph/pivot scalar recovery. The local
run reports39passing filtered cubic tests. [pcc:unverified]

## Cost scope

Use cold per-arm native wall time, including base setup, symbolic S4
preparation, sampling/restarts, native basis, staircase/border extraction,
root/eigenvalue work, fallback, graph/linear algebra and final scalar replay.
The existing incomplete legacy cost is a separate diagnostic. Native F4's
row counter plus counted restriction/border/root products does not include
all polynomial scalar housekeeping; the new report says so explicitly.
Never rank complete costs from that partial counter. Serial costing is
required because native F4 per-report counter differences can interleave
under concurrency. [pcc:unverified]

## Frozen pair of experiments

Run both distinct stages on the same published source/binary, small first
then scale, with no source edits between.

1. Small: p67/127, fresh seeds73/79; original Groebner, new F4Groebner,
   PreparedSymmetricPairOnly.360executions,60warmup+300measured.
2. Scale: p523/1039, same declared seeds73/79 on different curves; new
   F4Groebner and best measured pair baseline.240executions,
   40warmup+200measured. The expensive original solver is evaluated in the
   small stage and earlier phase probe; it is not silently timed as a
   partial large-stage result.

Both use r-adding-long, protected pivot_a/graph/pivot_b/rho_a/rho_b,
one retained warmup and five measured rounds, odd-round arm reversal,
cap500000residuals,900seconds whole-process each,8GiBvirtual,CPU0,
zero automatic retries. Pin public F4 defaults and clear debug/SIMD/Gaudry
overrides as in the runtime templates. All failures, censors, rejected A/A
cases and incomplete cohorts remain raw and invalidate their stratum.

Graph/pivot relation digests must match within each oracle. Different oracle
root paths/fallbacks can produce different streams; cross-oracle prefix
identity is not asserted. Compare scalar recovery, cold work and stopping
counts on the same public targets. All new inputs are held out relative to
this study's previous measurements, not a claim about all lab history.

Use unchanged local10%median A/A tolerance,10000paired-round bootstrap and
n>=2^30/upper<=1local-reference flag. Strongest-external references, complete
pricing, independent signed evidence and lab admission remain separate
requirements for a broader parity/tier claim. Inspect repeated decomposable
abscissae as a mechanism diagnostic; do not infer an exponent from four sizes.

Create-new local-s4-f4-ic-small-20261115-09 and
local-s4-f4-ic-scale-20261115-10 retain source/spec/local checkpoints, copied
binaries, environment/argv, raw JSONL and exit/timing/hashes. The user authorized
local or Runpod; no cloud instance/spend is needed for these bounded trials.

Sources: existing F4 and quotient-recovery code, residual-walk note11.4–11.6,
probe08raw/summary and its failed reference cases. KB remains unavailable and
the primary F4 author PDF retrieval503 remains disclosed. No novelty claim.
