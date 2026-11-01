# AMENDMENT to EXP-BINSTD-372679 for Stage 2 (draft, prepared 2026-10-26)

The spec froze Stage 0-1 only ("Stage 2+ SAT census is a successor amendment,
not authorized here"). This file is that successor, written as a draft. It is
**not approved**: the ledger decision that authorises Stage 2 (a `DEC-*` in
crypto-autoresearcher) is outside this cell's remit and has not been filed. It
precedes all data (there is none). The spec text is not edited; where this file
differs, this file governs this cell's runs, and both versions are reported.

Is the spec executable as written? Not literally. Five things in it cannot be
done or are ill-posed as stated; the closest executable cell is what
PROTOCOL.md describes.

| # | spec says | problem | amendment |
|:--|:--|:--|:--|
| A1 | "WDSat and/or CaDiCaL if installable" | Both are C/C++ binaries. The repository requires native Rust for computation. WDSat source exists only inside a sibling run directory (`EXP-BINSTD-b94ec8/.../wdsat_stage2_probe`). Neither is installed. | Use the repository's native CDCL with native XOR rows and Gauss-Jordan (`src/cryptanalysis/sat.rs`), compiled into the census binary by path. Its XOR engine plays the role of WDSat's XOR-Gauss. Every conclusion is for this solver and this priority policy only. The spec's "missing solver is O-IMPEDIMENT" does not apply, because a solver exists; the cost is the narrower scope. |
| A2 | Stage-1 fixture B0 passed ("O-RELABEL-READY") | B0 compared a *variable-count model* (`encoding_export_sizes`: `m*l`, `m*(l+ceil(log2 n))`, `plain+n`), not encodings. "Gauge j=0 equals arm (c)" was true by arithmetic. No clause or XOR row existed. The `rep_shift` count (a binary shift code) is not an encoding of arm (a) either. | The arms are now real encodings. B0 is redefined and tested: (i) the target index pinned to j = 0 with no gauge has the plain verdict on every target (`j0_embedding_equals_plain`); (ii) every sound arm agrees with an exact solver-free oracle on every target of every run, and every SAT model is a certified decomposition; (iii) the domain of each arm has the model count of an independent Rust predicate (`counting_identity_of_the_three_domains_n7`); (iv) the abscissa self-check is kept (it is the theorem Tr(u^2) = Tr(u)). The Stage-1 JSON is not re-used. |
| A3 | Arm (b) "gauge-fixed one-hot orbit system", with "(b) at j = 0 is arm (c) exactly" | If the gauge is j = 0, B is C and the ratio is 1 identically; "excess (b) - (c) grows with n" has no content. | Arm B gauges the *tuple*: x_1 is the lexicographically minimal rotation of every slot (m n - 1 comparators), the remaining slots are ordered, and the target index j is free over n values (one-hot, target = sum_j h_j sigma^j xR). This keeps the spec's count identity: canonical tuples x n targets = |V^m / S_m| up to a stated tie sliver (tuples containing 0 or a sigma-fixed element, and pairs from one orbit, keep some redundancy; the exact counts are checked against a Rust predicate). The j = 0 reading is retained only as the B0 embedding test. |
| A4 | "Ordinary-curve null: arm (b) ratio must be at least the Koblitz ratio" | On b not in F_2 the orbit system is not equivalent to the plain problem (S3 is equivariant only if b is Frobenius-fixed), so arm B is unsound there and a cost ratio between non-equivalent problems is meaningless. | The ordinary-curve cell (n = 17, l = 8, b = t) is a **detector check for unsoundness**: B must disagree with the oracle (or fail certification) on some target while C and A, which are bijective reparameterisations and need no equivariance, must never disagree. Reported as "detector works / failed". No cost ratio is read from it. |
| A5 | "Size-matched generic symmetry-breaking clause set must not reproduce any sub-1 ratio" | "Generic" and "size-matched" are not defined, and a sound clause set that breaks a *non-existent* symmetry cannot exist. | Arm G: C plus exactly B's gauge comparator circuits on the same coordinate bits, each made vacuous by one free selector variable. It is sound by construction and matches B's gauge part variable-for-variable and clause-for-clause (selectors aside; asserted in `structure_counts`). It tests whether the extra auxiliary variables and clauses alone move the count. It does not match B's one-hot target multiplexer (that is not symmetry breaking). Arm U is G with the selectors removed against the fixed target: the literal per-instance clauses, unsound by design, used for the spec's "literal clauses change SAT on planted targets" tripwire. |
| A6 | "two curves per n", "50 UNSAT targets per cell" | S3 does not depend on a, so E_0 and E_1 give the same instance family with disjoint abscissa pools; and Frobenius conjugates of a target are the same problem, so sampling them is pseudo-replication. | 25 UNSAT targets per curve (a = 0, 1), pooled to 50, drawn from distinct sigma-orbits; curve is a blocking factor, not evidence of structure. 10 planted per curve (spec: 20 per cell). Labels come from an exact oracle, not from a solver. |
| A7 | m = 3, n = 31, l in {10, 11}, 50 + 20 targets, 8 h, 4 runs | By the conflict model of PROTOCOL.md section 8 a single complete search at (31,3,10) is about 5e6 conflicts at 0.15 to 0.8 ms per conflict (hours); at l = 11 four times that. 50 targets times 3 or more arms is days of CPU. | The cell is kept as specified in kind but staged: a ladder at (31,3,5) and (31,3,6) with all arms (cheap, fits kappa_3 and the per-conflict cost), then (31,3,10) with arms C, C@2, B only and N = 6 to 10 targets chosen by a pre-stated formula, and (31,3,11) only if the re-estimate allows. At N <= 10 the resolution is far from 0.95 (MDE 0.895 at sd 0.1); the m = 3 cell can show "B is not much cheaper than C", not "B/C >= 0.95". If the budget formula gives fewer than 6 targets the cell is O-IMPEDIMENT(budget), recorded and not evidence. With 6 or more targets the cell carries the label O-BOUNDED (PROTOCOL.md section 7 rule 2b): the B/C interval and "excludes B/C below <lower bound>", no structure claim. The spec's "A at m = 3" is dropped at l >= 10 for cost. |
| A8 | one replicated sub-0.95 cell refutes | "Replicated" was undefined. | Replication = an independent second set of seed slots (3, 4: new variable renamings and phases, same targets and encodings), and the generic control and the polynomial-basis reference must not reproduce it. A new set of targets is not run (conditional follow-up only). |
| A9 | outcome labels O-RELABEL / O-STRUCTURE / O-TRIPWIRE / O-ORD / O-ARTIFACT / O-IMPEDIMENT | A cell whose interval straddles 0.95 has no label. | Add **O-INCONCLUSIVE** (interval straddles 0.95, or a candidate that failed a confirmation condition). Without it the rule would force a relabel claim from an unresolved interval. |
| A10 | metric: conflict count only | Conflicts alone cannot show an encoding is cheaper: A and B have 10x more variables and rows. | Solver `word_ops` at encoder-work weights 0, 0.1, 1 (AGENTS.md section 5a) alongside conflicts; an arm is ranked only if the conflict ratio and all three op ratios agree on the side of 1. |
| A11 | "regress excess against n and against conflicts(c)" | Within a cell conflicts(c) varies by a few percent, so a within-cell regression has no leverage on additive versus multiplicative overhead. | Compare adjacent l at fixed n (C grows about 2x at m = 2, about 4x at m = 3): excess flat in l is additive, excess tracking C is multiplicative. Three values of n give the n-dependence only qualitatively. |
| A12 | "budget: 28 800 s, maximum_runs 4" | Four processes in parallel over 8 hours is 32 CPU-hours; the central estimate for the full plan is about 35 plus replicates, with a factor-ten uncertainty. | The gates of PROTOCOL.md section 8 cut cells before launch from pilot rows; a cut cell is reported as cut. "Runs" is read as four parallel processes. |

## What the amendment does not change

* Claim tier: control / toy. No attack, speedup or exponent claim.
* H1-PRIME band 0.95 as the threshold, linear stable bases only, n in {17, 23,
  31}, no run at n >= 41 or 131, no Magma, Sage, AUXIN or Bedrock.
* KN-FIND-47da4e is used as a prior on the form of the overhead only.

## Items that need a human or a ledger action

* File the decision that authorises Stage 2 under this amendment (not done).
* Approve or reject A1 (single native solver) and A7 (m = 3 staged, small N).
* Take the resume checkpoint (`scripts/checkpoint.sh init`, S3 upload) before
  any round-0 or round-1 command. The container used to prepare this cell could
  not upload one, so nothing was run.

## Revision 2 (after independent review)

A13. The positive control P1 (C vs Cnobreak, 1/m!) had no derivation on this solver and contradicted the section 8 cost model; it is demoted to informative. A new arm Chalf (C plus one unit clause on the top u-bit of slot 1, UNSAT targets only) is the voiding positive control P3 with a derivable effect (about 0.5). This replaces the spec's "positive control" notion by a control constructed on the same code path; the spec does not name one.
A14. Arm G at m = 3 is now fully sound (all comparators made vacuous, including x_2 <= x_3); the earlier build was unsound there.
A15. Censoring tolerance 1% (was 10%); the pilot uses a hard 500 000-conflict cap and a disjoint target stream.

## Revision 3 (after the round-1 review of revision 2)

A16. Chalf now pins the top u-bit of every slot (m unit clauses), not slot 1 alone: C's lex break makes slot 1 the smallest slot, so a slot-1 pin retains 3/4 (m = 2) or 7/8 (m = 3) of the sorted space and could fail to clear 0.95 on a sound instrument. Expectation is a range (0.125 to 0.5), with a numeric pilot threshold of 0.8. Supersedes the "about 0.5" of A13.
A17. Decision rule 3 (O-STRUCTURE) requires G, Cpoly and A/A with at least 10 paired targets each; absent controls leave the candidate not confirmed. This implements PROTOCOL section 7 as written (the code was fail-open).
A18. Freeze mechanics: tests write outside the tree, a cell `.gitignore`, out-of-tree cargo target dir, exact freeze commands. No scientific content changes.

## Revision 4 (after the round-2 review, 2026-10-27)

A19. O-RELABEL requires the positive control P3 (Chalf/C, >= 6 paired targets, detected); without it the label is O-INCONCLUSIVE. Implements PROTOCOL section 7 rule 4 as amended; the earlier code issued the null label with no positive control. No other scientific change.
A20. Freeze command no longer `git add`s the ignored Cargo.lock (it exited 1); the file is hashed in FREEZE.txt only.
