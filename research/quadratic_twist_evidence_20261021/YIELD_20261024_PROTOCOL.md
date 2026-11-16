# Relation yield and conditional regularity cost — frozen proof plan

2026-10-24, run id `yield-20261024`, starting head
`171285079f285b65f1b2443d3bc8c22d5d307d6e`.

The user requests Lean theorems relating twist/main relation yield and showing
how increased yield and lower degree of regularity could make index calculus
faster. Do not assume either improvement follows merely from twisting.

1. Define a relation as an actual zero sum of integer multiples of a finite
   indexed factor base, with a finite explicit candidate coefficient set.
   Prove injective additive transport preserves the entire accepted coefficient
   set, count and normalized yield. Instantiate on the already proved explicit
   short-model twist map into E(K). Identify the target correctly: the mapped
   minus subgroup, not an asserted isomorphism with E(k).
2. Distinguish raw relation yield from useful, independent-row yield. Attempt
   a kernel-checked F5 control on already registered E:y²=x³+x+1 and its d=2
   monic twist Y²=X³+4X+3. For scalar candidates 0,...,8, count zero relations
   on the stated actual points. Test whether any increased raw count survives
   reduction modulo the smaller subgroup order. Preserve failed elaborations.
   This is a correctness control, not a solver speed experiment.
3. Define a transparent cost model charging setup, transfer, relation work,
   linear algebra/descent/verification. Normalize finite-batch useful yield
   explicitly. Prove a sufficient improvement inequality and exact overhead
   threshold. Include useful-yield and per-attempt work assumptions; do not
   identify raw zero counts with independent rows or assume stochastic m/p
   completion without a probability argument.
4. Use binomial(n+D,n)^omega as an explicit dense Macaulay work model, with
   assumptions on variables, exponent and coefficients. Prove monotonicity in
   D where feasible. A supplied D is a solver-degree parameter; proving it
   equals an actual system's degree of regularity is a separate obligation.
   Do not claim all twist ideals have lower regularity or that degree alone
   determines F4/F5 runtime.
5. A cheaper twist DLP only helps the main DLP after a verified scalar-preserving
   injective subgroup reduction whose full cost is charged. Retain the proved
   eigenspace/order obstructions. No invalid-curve scope or hardness/speed claim.

Add new Lean modules; leave all old source-bound proofs/certificates and frozen
results unchanged. Keep exact commands, sources, pins, failed builds and final
gate/axiom receipts. Run the unchanged scoped gate, mandatory release library
suite and relevant workflow metadata checks. Produce a source-linked report,
editable diagram and inspected PDF. Any numerical model illustration must be
labelled synthetic, with no benchmark/leaderboard promotion.

The existing PR #2381 is the delivery target. Keep every research, shared
Conductor, exact-head CI/review, catalog and conflict hold. The actual Conductor
check exits127 in this new workspace; no reservation or replacement coordinator
is claimed. No conflicted generated/claim file or registry is in scope.

Read the existing ECDLP lead atlas before proposing new mechanisms. This round
formalizes the user-requested comparison; atlas references/handoff will identify
related existing leads for the next atlas review rather than change their
research status or silently claim a new algorithm.
