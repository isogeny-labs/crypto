# Amendment 5: prepared pair arithmetic and walk-epoch mechanism

Freeze before observing seeds41/43/47 in these treatments. Scale05 recovered
all targets but graph wall/rho remained5.36–14.27 at p1039. Its legacy model
assigns roughly two-thirds of priced work to the oracle, while sampled
residuals62284–175997 exceed sqrt(n). This motivates separately testing
per-residual arithmetic and walk coalescence. [pcc:unverified]

## Candidate and mechanism controls

1. Reference symmetric pair versus prepared symmetric pair. The latter caches
   a²,2a,4b once per cold solve, charges construction and uses degree<=4
   stack polynomial arithmetic. A nonzero constant remainder proves there
   are no common roots and stops Euclid immediately. Exceptional and split
   recovery is shared with the reference. Require identical admitted relation
   digests, residual counts, group operations and recovered scalars at every
   warmup and measurement for each backend. Field-operation counts may differ.
2. Existing walk epochs min(ceil(sqrt(n)),1024) versus long epochs
   min(ceil(sqrt(n)),1000000). Keep uniform restarts and point-only within-epoch
   repeat detection. Charge all work. No collision coefficients/recovery are
   added to the sampler. Selectable long epochs use a different residual
   stream once the short epoch bound would fire; do not require stream
   equality across epoch treatments.
3. Diagnostic telemetry counts decomposable residuals and distinct/repeated
   abscissae, plus the first repeat's residual index. It stores no coefficients
   and supplies no equations. Repeated x means a repeated point up to sign;
   concentration at recovery is evidence consistent with generic trajectory
   coalescence, not a proof of improved independent IC relation density.

## Frozen runs

Two create-new runs on the same published source/binary, short first then long.
Both run p127/523/1039 and fresh seeds41/43/47; both oracles; the existing
pivot_a/graph/pivot_b/rho_a/rho_b controls, retained warmup1, measured5,
max2000000residuals, pinnedCPU0,8GiBvirtual,900seconds each, zero retries.
All failures, caps and control rejections stay in the cohort. No alternative
seed is substituted. Runtime plans pin binary/source and component hashes.
Local raw/checkpoints live in local-prepared-short-20261115-06 and
local-prepared-long-20261115-07. Source/spec and resume precede execution.

## Predictions, acceptance and interpretation

- Prepared arithmetic should lower oracle field work without altering the
  stream. Report end-to-end local wall ratios and round uncertainty for both
  oracles; do not infer a pipeline win from multiplication counts alone.
- Longer epochs may reduce sampled residuals by allowing trajectories from
  separate restarts to coalesce before visiting a decomposable point. If
  recovery accompanies repeated smooth abscissae, explicitly classify that
  behavior as a distinguished-point-like mechanism. It does not establish a
  sub-generic exponent or a better algebraic relation density.
- Preserve the10% median local A/A threshold, all-five-round correctness and
  input equality,10000 bootstrap draws and n>=2^30/upper interval<=1 local
  reference flag. Broader tier/parity claims still require stronger references,
  full pricing, independent signed evidence and admission. Do not promote a
  local flag into those claims.
- Retain p127/seed23's earlier failed control; these fresh seeds test a changed
  implementation/mechanism, not an unchanged retry to improve that control.

Sources checked: existing symmetric S3 implementation, run_rho3 and residual
walk note11.4/11.9 and its earlier distinguished-point discussion. Primary
source: van Oorschot–Wiener, *Parallel Collision Search with Cryptanalytic
Applications*, author preprint dated23September1996,
[sections4.1/5.1](https://people.scs.carleton.ca/~paulv/papers/JoC97.pdf),
SHA256049182522b2a38a1df4564c464ffe3f110d5e64938d37843db093718e5dd7c11.
It explains coalescing deterministic trails, bounded trails without a
distinguished point, and retained linear coefficients at distinguished
points. That is the mechanism this telemetry tests; the current sampler
continues after smooth points and uses its separately specified epoch rule.
KB MCP unavailable and wrapper search has no executable; no novelty claim.
The generic r-adding/distinguished-point interpretation is a hypothesis to
test with telemetry, not a complexity conclusion.
