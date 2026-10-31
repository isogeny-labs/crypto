# Prime-field index-calculus lead review

**Where SAT actually belongs · 2026-10-31**

Repository reviewed: `isogeny-labs/crypto` at
`2c03791383f4bb4ce200e84bdef4309065c61e7f`.

Status: literature and repository assessment complete. No scientific run was
started. The new SAT protocol is a validated draft and remains unfrozen until
the native instrument, solver identities, power and runtime preflight exist.

## Result

The best inspected prime-field index-calculus lead is **hashed factor bases
with compact Boolean tensor contraction**. It is the only route reviewed with
an explicit end-to-end claimed subexponential prime-field ECDLP bound,

\[
2^{O\!\left(b/(\log b)^{1/4}\right)},
\]

conditional on the exact contraction theorem in the pinned OpenAI manuscript.
It is SAT-shaped—Boolean predicates, exact counting and witness
self-reduction—but it is not evidence that ordinary CDCL is the right engine.
The reduction preserves an `O(mb)`-edge tensor network specifically because a
conventional circuit-to-CNF path loses the required compactness.

The best empirical test of the narrower “practical SAT solvers are the lead”
hypothesis is a solver tournament on the **shared divisor-coefficient and
quotient/remainder predicate**. Direct prime-field Semaev QF_BV/QF_FF has
already been tried without a competitive signal; repeating it unchanged has
low information value.

The established algebraic control is the **PKM tower at arity `m >= 4`**.
Current evidence is adverse but not complete: measured F4 degrees are `6,7,7`
at `N=8,12,16`, while the recorded complete-cost model requires degree at most
`4` at `m=4`. The decisive `N=20` closure remains resource-censored.

## Ranked leads

1. **Hashed factor bases + Boolean tensor contraction.** Conditional
   end-to-end theorem chain; highest asymptotic upside. Next: independent
   theorem-to-code audit, then the already frozen finite panel.
2. **Shared divisor coefficients + SAT/XOR/SMT/F4.** Best new practical SAT
   discriminator. Exact tiny witnesses exist; unknown-solve scaling does not.
3. **PKM generalized root finding, `m >= 4`.** Most mature algebraic lane and
   the right control system for a SAT backend. Existing degree/memory growth is
   unfavorable.
4. **Target-parametric clause/tensor-message reuse.** Test only as separately
   labelled multi-target amortization; cold one-target work remains primary.
5. **Special `j`, Solinas primes and chart-aware encodings.** Worth a matched
   constant-factor screen; no exponent-changing evidence yet.

![Lead funnel](figures/lead-funnel.png)

The full evidence matrix and links are in [README.md](README.md) and
[lead-matrix.json](lead-matrix.json).

## What prior work rules out—and what it does not

- Generic pair tables, subtraction, root-finding and MITM remain useful
  references, but current prime rows are `5.47x`–`13.5x` matched rho and the
  methods behave identically on random bases.
- cvc5 coordinate, QF_FF and Semaev encodings returned `unknown` at 120 seconds
  on a recorded 20-bit case; later direct Z3 probes showed no clean scaling.
- The proposed LQ-CDCL formulation is a lattice call in disguise under its
  recorded analysis and remains above rho. This is not a theorem about every
  compact Boolean predicate.
- Binary WDSat/CDCL conflict counts track enumeration for the tested binary
  representation. That is a warning, not a prime-field shared-coefficient
  experiment.
- Convex one-block Coppersmith, simple low-degree univariate factor-base loci,
  and isogeny-transported yield have named scoped obstructions. Those scopes do
  not exclude arbitrary auxiliary-variable circuits.
- First fall alone is not predictive: a curve-free null reproduced the relevant
  digit-presentation fall behavior.

The cross-repository atlas reported 153 leads: 74 closed by named obstruction,
53 open with a decided next step, 17 blocked and 9 unexamined. This review adds
no theorem or measurement, so it changes no atlas disposition.

## Primary-source boundary

Directly retrieved and read on 2026-10-31:

- Akshima et al., v0.2, pinned commit
  `1ee44e562b1b9a50ea339f3b8f81112b5a24b96f`.
  PDF SHA-256 is recorded in full in `README.md` and `lead-matrix.json`.
- OpenAI ETH manuscript, pinned commit
  `7ef5c0cc5ca4b0e072b7a050fca09ab24bc6fa3a`.
  PDF SHA-256 is recorded in full in `README.md` and `lead-matrix.json`.
- Petit–Kosters–Messeng PKC 2016 public PDF. Its SHA-256 is recorded in full
  in `README.md` and `lead-matrix.json`.

The tensor theorem is conditional and asymptotic. The current Rust dispatcher
also cannot enter the source recursion through its public path: it stores
`lambda = ceil(log2(size))` in a `usize` and computes
`root_u = lambda / 131072`. On a 64-bit build, `root_u` is always zero. Forced
small tests exercise the kernel, but timing the public fallback cannot test the
claimed asymptotic regime.

## Quantitative constraint on the algebraic control

![PKM m=4 degree evidence](figures/pkm-m4-degree.png)

The points are recorded F4 solver-stage measurements, not an asymptotic fit.
At `N=16`, the recorded run had 76,837 columns, 55.5 minutes, 8.06 GB peak and
about `2.7e13` multiply-adds. The existing `N=20` projection is 240–380 GB.
Data and provenance: [pkm-m4-degree.csv](figures/pkm-m4-degree.csv).

## Discriminating experiments

### A. Tensor theorem-to-code audit

Map packet packing, mode orientation, precision lifting, taper levels, child
shrink, reconstruction, implicit entry evaluation and bit cost from the pinned
proof to code. Add an arbitrary-precision symbolic guard evaluator, exhaustive
tiny direct-versus-recursive transcripts, and mutation controls.

One verified lemma counterexample or source/implementation mismatch refutes the
chain at that point. Bounded tests alone do not establish the theorem; an
independent proof and checked code-correspondence argument are required.

### B. Shared-coefficient SAT discriminator

Use one canonical semantic IR on four held-out rungs:
`(m,t)=(3,3),(5,4),(7,5),(9,6)`. Cross natural, planted and exact-UNSAT targets.
Run exact enumeration, msolve/F4, CaDiCaL CNF, CryptoMiniSat CNF-XOR and cvc5
QF_FF, plus byte-identical A/A, planted positive and support/model-count matched
curve-free null controls.

Primary metrics are verified model status, complete retired instructions, peak
RSS and a preregistered log-work slope. SAT work is also shown under propagation
weights `0,0.1,1`. Wall time is secondary and requires L2 isolation.

Advance only if one SAT bundle verifies all rows, completes every natural rung,
has corrected upper 95% slope interval at most `0.85`, and beats F4 in complete
instructions on the two largest jointly completed rungs outside A/A spread.
Do not advance if its lower slope interval is at least `0.95` on three
consecutive rungs, or all SAT bundles censor before F4 on the two largest jointly
attempted rungs. Other censoring is inconclusive.

The detailed narrative and schema-checked draft are [PROTOCOL.md](PROTOCOL.md)
and [protocol.draft.json](protocol.draft.json).

### C. SAT on identical PKM systems

Compile the exact registered `m=4`, `N=12,16,20` Kummer systems—not regenerated
lookalikes—to F4, QF_FF, CNF and CNF-XOR. Include natural targets, planted
decompositions, exact UNSAT and the existing shape-matched null. SAT advances as
a backend only if it completes `N=20` correctly with a lower instruction-growth
slope than F4. A SAT completion does not erase a measured last-fall bound.

### D. Target-parametric reuse

Compare cold process, reset-in-process and target-assumption reuse for
`1,8,64,512` targets. Audit every retained clause/message as
target-independent. Keep the cold one-target result primary; reuse is a
separate amortized claim.

### E. Special-prime/curve factorial screen

Cross Solinas versus generic primes, `j=0`, `j=1728` and generic curves,
matched subgroup/cofactor, chart and one frozen solver. Promote beyond an
engineering result only if representation size or work slope changes across
four sizes.

## Scope and dependencies

Finite experiments can find counterexamples and decide continuation gates; they
cannot prove a subexponential complexity theorem. The new protocol remains
unfrozen because its generator, solver/image hashes, analysis script, power and
runtime preflight are absent. No affected data was taken.

The managed environment has no configured AWS identity, so it cannot currently
submit the required EKS isolation job. This is an access dependency, not a
scientific result. The existing 84-case tensor panel is prepared but not run.

No leaderboard, boundary ledger or benchmark graph changes: this review adds no
admitted performance row and no new algebraic identity.
