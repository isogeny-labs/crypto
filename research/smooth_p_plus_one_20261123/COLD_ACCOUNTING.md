# Cold-recovery accounting protocol

2026-11-24. Development qualification, not an admitted performance comparison.

## Question and controls

Can one recovery driver account for setup, every public-mask draw, complete or
failed relation queries, the production relation consumer and a fresh scalar
replay without changing the existing solver's acceptance rules? The new optional
`account=SEED` path uses the actual `RelationAdapter`; legacy `recover=` remains
unchanged. `account-suite=SEED` starts a fresh process for each explicit arm.

The fixed local control is F127, selector depth 4, seed 781815, at most 30 mask
draws, in order Direct, Refined, ProductionCofactorMsolve. Its public fixture is
`y²=x³+x+7`, subgroup order 109, G=(1,3), Q=(11,98). A known scalar is used only
in tests; neither the run interface nor its relation queries take that scalar.
The same first sorted signed row is consumed after every complete nonempty query.
Direct alone constructs and uses the pair index. Algebraic arms have no fallback.

Direct and cofactor use the full distinct-x, target-disjoint four-leaf chart.
Refined requires four distinct coarse Dickson images, a narrower chart. Equal
recovery outcomes therefore do not establish equal query scope. This control
qualifies accounting software, not a faster relation method.

## Intervals and evidence

The child starts its clock on entering main, before argument parsing. Disjoint
intervals cover input validation, base construction, oracle setup, consumer setup,
public-mask generation, the entire oracle build/solve/extract call, production
consumer calls, final scalar replay, and per-draw record emission. Unattributed
work is explicit; phases plus unattributed time must equal the pre-summary child
wall interval exactly. The consumer interval bundles canonicalization, equality
replay, duplicate handling, rank, incremental linear algebra and its scalar replay.
Partial operation counters do not constitute a full arithmetic price.

Every draw is retained, including identity and repeated targets. Failed queries
remain undetermined, with their diagnostic systems and chart counts. Consumer
rank counts base coefficients plus the target scalar, unlike the legacy base-only
rank. The parent records spawn through observed child exit, including summary
serialization, teardown and up to 5 ms process-monitoring latency. Parent parsing,
forwarding and receipt emission are outside that interval. Fresh algorithm state
does not mean flushed OS caches. No local timing ratios will be reported.

A successful process receipt requires exit zero, one final matching summary,
contiguous draw records, matching outcome counts, all declared phases, exact
accounting reconciliation and child time no larger than parent time. Recovered
and draw-budget-exhausted outcomes remain distinct. A valid process receipt is
not independent verification or benchmark admission. Malformed, oversized,
missing, killed or unsuccessful output cannot count as a completed recovery.

## Bounds and acceptance

Each query retains its existing 3-second budget and unchanged solver resource
limits. Each child has a 120-second hard deadline; the three-arm parent invocation
has a 400-second outer timeout plus 5-second kill grace, one worker and 2 GiB
virtual-memory limit. Parent stdout/stderr files are capped at 32 MiB each and
individual JSONL records at 1 MiB. A fatal child stops the suite with unrun arms
listed. No automatic research retries or increased limits are authorized by this
protocol. stderr is preserved up to 64 KiB, with any omitted byte count explicit.

Before the control, unit tests exercise Direct through the production consumer,
replay selected rows through the historical reducer, force an external solver
failure without fallback, mutate/miss accounting receipts, retain identity/duplicate draws, kill a timed-out
child process group and reject unterminated output. Then source,
protocol and exact command are committed/pushed and the built executable is
frozen by hash. Run the local three-arm qualification once and retain every
record, exit code and failure. Compare recovery/row histories to the previous
frozen qualification where scopes match; preserve any disagreement. Successful
accounting requires all three receipts to reconcile; successful recovery is a
separate recorded outcome, never inferred from a zero process exit alone.

The EKS route remains a separate integration obligation: current main's workflow
builds ecbench/fresh-owner and a named census payload, not this driver plus msolve.
No cloud job is represented as submitted. An admitted comparison still requires
the registered workload/build, fixed protocol, reference and A/A controls,
warmup and repeated matched trials with independently replayed artifacts.
