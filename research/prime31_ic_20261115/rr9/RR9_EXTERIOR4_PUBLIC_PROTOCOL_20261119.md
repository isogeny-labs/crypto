# Frozen protocol: Prime31 target-only exterior-4 algebraic relation oracle
Date: 2026-11-19 UTC. PR #4776. Status: preregistered **before** candidate execution. [pcc:unverified].

## Scope and reuse
Use the unchanged original `prime31-algebraic-ic` Rust arithmetic pinned at `1df714645aa683cf975bc76fcacb9842cb4780fd`, p=2147483647, E:y^2=x^3+847514846x+2087444724, n=2147446831, cofactor 1, G=(625037715,2100627251). Reuse the canonical first-x factor base (first four x: 0,1,2,3) and existing slot weights (1,3,5,7,11,13,17,19). Not a weak-curve experiment.

## Candidate
Given **only public R** and eight public weighted slot supports, calculate the target-reduced 8x8 RR9 evaluation rows `[(x-r)*x^i,(y+s)*x^i], i=0..3`. Build exterior-four minors for each half. For each ordered pair of half assignments, contract 70 complementary minors over Fp; zero is a *candidate* relation. Reject repeated points, target-point exceptional divisors, and mismatching signed EC sums in final replay. The candidate must not call `Arithmetic::add` or construct a generic EC group-sum dictionary. The external scorer can use existing Rust group operations as a **separate** validation oracle.

## Registered test cases
- P0, planted positive: eight signed-slot labels all index 0 in B4. The extractor receives only computed public target R and supports, not the planted tuple.
- N0, natural/nonplanted public residual: R=G. Run the **entire B4 assignment space**, not a seeded negative oracle. If a witness exists, classify as positive and replay; if absent after exhaustive scan, classify as negative.
- Unit tests: 4x4 exterior contraction exactly equals 8x8 finite-field Gaussian determinant on deterministic synthetic matrices; Mersenne field inversion identities.
- Independent truth grader: separate Rust 4+4 group-sum matching, **not linked to candidate**.

## Costs, fail criteria and promotion
B4 has 8^4=4096 half assignments, 4096^2=16,777,216 pair contractions, each with exactly 70 coordinate-product multiplications, plus 69 or 70 field additions, stopping early only if a valid witness appears. Report table construction + solve wall time, pair checks, coordinate-product count, memory bound and first positive witness. Count every failed/negative probe. B8 has 16^4=65536 half assignments and 16^8=4,294,967,296 pair contractions, or 300,647,710,720 coordinate products per fully exhaustive target; this is a **work count for the specified exhaustive algorithm, not a lower bound for algebraic extraction in general**. No full B8 run required unless B4 stage shows credible cost. The independent grader is excluded from candidate wall time; distinguish this from full ECDLP cost.
Compare against the **same-runner, original-affine** 1024-target signed shared BSGS reference in #4774: 3,342.793 ms and 1,084,071,298 Fp multiplications over 1024 logs. This historical comparison is a provisional scale budget, not a new same-runner pairing.
A B4 correctness pass does **not** qualify as a complete ECDLP solver. Stop this exhaustive exterior-4 method on a B8 work-budget failure; feed the exact contraction structure into #4692's nonenumerative F4/XL/SAT path. Only a B8 **public-only unknown-index** extractor can progress to #4728's 31-rank/1024-log Rust pipeline.

## Prior-lead and literature scope
Read #4466 repeated-point RR9 determinant; #4728 8x8 determinant/Fourier/zeta, B8 grader and four-window control; #4692 algebraic unknown-index extraction; #4772 public 8 resultant and Jacobian; #4773 prefix2; #4774 matched shared BSGS. Negative autoresearcher SA-04 norm and SA-18 determinant and cryptanalysis SAT benchmark noted, not claimed new. Public literature: Petit–Kosters–Messeng (PKC 2016) DOI 10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala (FFA 2018) DOI 10.1016/j.ffa.2018.01.009. The shared research-KB MCP and CLI are unavailable in this environment: novelty search bounded.
