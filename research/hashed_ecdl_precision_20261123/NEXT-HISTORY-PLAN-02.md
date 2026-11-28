# Next instance gate: finite source precision history

Status: proposed, not a checked theorem. Preserve the published scalar report,
original review and manifests; this is a new development record.

1. Structure above: existing Nat call-indexed arrays M,u,a,i,p and existing
   Finset.range sums; existing rational ordered field only for summing the
   geometrically shrinking edge sequence. No new algebra or evaluator.
2. Laws as stated: Monotone.seq_le_seq (finite index version),
   geom_sum_Ico_le_of_lt_one, Finset.sum_le_sum, Finset.mul_sum,
   Finset.sum_range_succ, standard Nat/Rat casts and ordered multiplication;
   shared CryptoLeanCore.TaperedPrecision.increment_le.
3. Object as instance: a finite successful strict source-shaped path j<d,
   p(j+1)=childPrecision(p(j),M(j),u(j),a(j),i(j)), positive u(j)<=M(j),
   8*M(j+1)<=M(j), a(j)<=A, M(0)<=s and p(0)<=s. The a bound must be bound
   later to the actual cumulative-exponent parameter. These supplied numeric
   premises do not establish path extraction from the Rust implementation.
4. Assembly probe already passed at phase01f: named geometric laws over Rat,
   ceiling arithmetic and Nat powers. Before another Mathlib compile, restore
   needed compacted dependency cache from verified archives and preflight RAM;
   one original Lean worker, unchanged600-second/15,032,385,536-byte guard.
5. Missing components: finite-horizon geometric instantiation; exact Nat sum
   of source increments; composition with the shared increment theorem; bind
   both initial bounds and produce p(d)<=81*s*2^A. Neither a single-call bound
   nor a supplied inherited-budget hypothesis substitutes for this recurrence.
6. Inheritance/fidelity: reuse existing generic laws. Monotone.seq_le_seq
   requires shrink only for k<j, so use k<j<d rather than silently assuming an
   infinite positive shrinking path. All exponents and source arithmetic stay
   Nat; Rat is only a proof embedding for the finite sum. Exact initial native
   normalization is s=2E,p0=E+1 for nonempty E. General P0<=C*s changes81.

Dependency order: prove finite mass sum<=2s; use shared increment_le and
a(j)<=A to bound the increment sum by80*s*2^A; telescope the exact precision
recurrence with sum_range_succ; absorb p0<=s<=s*2^A. Then independently review
finite-history source fidelity and controls. Full Rust execution refinement,
admission/absorption and total counter bit-work remain downstream.

Do not expand the proof gate, pins, time/physical limit or trust list. Do not
rename a symbolic path into a publicly admitted allocated graph. Do not count
proof controls as solver cohorts or compare their elapsed times with ECDLP.
