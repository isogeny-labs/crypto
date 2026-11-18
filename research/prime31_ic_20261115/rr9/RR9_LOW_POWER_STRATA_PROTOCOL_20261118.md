# RR9 low-power support strata: prospective exact Prime31 control

2026-11-18 UTC · PR #4717 · **protocol frozen before the new Rust source or any result**. No solver improvement is asserted.

## Objective and exact curve

Reuse `prime31_algebraic_ic::{PRIME31,Arithmetic,check_curve}` from `research/prime31_ic_20261115/native/lib.rs`, ideally on immutable Rust 1.100.0 source `1df714645aa683cf975bc76fcacb9842cb4780fd`.
The only curve is the original ordinary cofactor-one
`p=2147483647, a=847514846, b=2087444724, n=2147446831, G=(625037715,2100627251)`.
The B31 `H_B` is the monic product of the first 31 distinct rational liftable x-coordinates, in the existing code's first-x convention. The code must independently check the group certificate and all selected point lifts.

## The exact predicate and non-equivalences

For any monic degree-eight `M` and the squarefree split `H_B=prod_{x in B}(X-x)`, define `P_k(M) := (M divides H_B^k)` for `k=1,2,4,8`. Over a field and with `deg M=8`, `P_k` is true iff every irreducible root factor of `M` occurs among the roots of `H_B` with multiplicity at most `k`. In particular `P_1 => P_2 => P_4 => P_8`, and `P_8` is the full support test for `M`, including multiplicities.

This is a restriction on an **unknown divisor M**, not a method for extracting M from a public residual. Missing `P_1/P_2/P_4` does **not** imply no signed relation. Any nonenumerative extractor may use those as *early restricted branches* but must retain the full `P_8` branch, or complete signed search/verified target randomization, to retain completeness. The earlier radical-support lemma (`RR9_RADICAL_SUPPORT_RESULT_20261117.md`, #4541) proves `P_8 <=> rad(M)|H_B`; this study does **not** rediscover that lemma.

## Exact finite controls (pre-registered)

1. Reuse the original native Rust curve verifier; generate 31 first liftable x with nonzero y, and a rational omitted x outside B. Do not substitute a toy or deliberately weak curve.
2. Build monic degree-eight M from multiplicity partitions `[8]`, `[4,4]`, `[3,2,1,1,1]`, `[2,2,2,2]`, `[2,1,1,1,1,1,1]` and `[1,1,1,1,1,1,1,1]`; assert the first admitting power is respectively `8,4,4,2,2,1`. A polynomial with one omitted rational root must be rejected at every power.
3. Calculate `H_B mod M` once and perform three consecutive modular squarings to evaluate `P_1,P_2,P_4,P_8`; independently compare against maximum support multiplicity in these planted controls. Do not use a generic group-sum table to implement the predicate.
4. Independently calculate exact 8-slot occupancy counts over 31 x labels by the recurrence `D_{b+1}(k+j)+=D_b(k) binom(8-k,j)` for `0<=j<=cap`, `D_0(0)=1`. Expected exact totals for cap 1,2,4,8: `318073392000;809127509400;852843376770;852891037441` of `31^8=852891037441`. Counts concern uniformly sampled **ordered label sequences**, not a theorem about the residual distribution or relation rank.
5. Public-point sums used only to replay the planted eight-leaf witnesses; no target secret, discrete-log labels, BSGS/rho fallback or prior enumerated relation table enters the predicate.

## Measurement and stopping rules

This is a mathematical/implementation *cost-gating* experiment, not a 1,024-target timed ECDLP solver. Record whether the exact predicates/controls agree. Record raw Rust operation counters if available, never substitute them for a timed F4 coefficient extractor. The next independent study must freeze separately before any timing, build the genuine public-residual RR9 solver (currently #4692), enforce complete positive/negative B4/B8 fibre gates including multiple roots, and measure each stratum's cost and **actual** relation yield, rank and target recovery at B31.

Source-specific 2026-11-18 #4714 benchmarks, **not new measurements here**, found weighted M6 at 3359.6/3391.5 ms versus same-runner cached BSGS 144.0/157.9 ms and existing mixed IC 1897.6 ms (first runner, 1024 public Q). At the frozen 3766 M6 training+target probes, `144.049ms/3766=38.25us/probe` is only an optimistic ceiling before RR9 setup/LA/verification. Reject claims that an inexpensive supplied-M test implies inexpensive unknown-M extraction.

## Prior sources and provenance

- Lab #4541 radical support, #4539 chart, #4545 signed fibres, #4549 B8 Macaulay, #4692 ongoing shared-remainder extractor, #4714 fully charged matched frontier; [pcc:unverified] unless explicitly attested.
- Amadori–Pintore–Sala, *On the discrete logarithm problem for prime-field elliptic curves*, Finite Fields Appl. 51 (2018), DOI 10.1016/j.ffa.2018.01.009.
- McGuire–Mueller, ePrint 2017/1262 (the polynomial GCD/rootfinding background).
- The `research-kb` MCP and checkout wrapper are unavailable in this runtime; GitHub primary files and above papers are the bounded fallback. No exhaustive novelty claim.
