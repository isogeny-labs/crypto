# P2: twist-pure coset selection for 2-power factor bases on prime-order curves (protocol, frozen 2026-11-07)

Proposal `IDEA-20261107-TWIST-PURE-COSET` (`../../ideas/idea-generator.md` lines
136–227). Class: **stage diagnostic** (relation coverage per F4 work at toy scale).
No pipeline `S`.

## Question

A 2-power base V_c (Kummer: x^{2^t} = c; Dickson torus: D_{2^t}(x) = c) has about
half of its 2^t elements on E and half on the quadratic twist. (a) Is the F4 cost of
the relation system independent of the coset c? (b) Does exact relation coverage
scale as c_E^m, where c_E counts E-abscissae? (c) So does choosing the coset with
the largest c_E among C candidates raise coverage per F4 work by (c_sel/c_rand)^m,
with the decay in t predicted by the binomial order statistic? (d) Does the purity
lemma hold exhaustively at small p (no relation with an E-target has a twist
summand for m ≤ 3)? The lemma is Lean-checked abstractly [pcc:unverified] in `aburan28/crypto-lean` PR #3
(`twist_part_sum_eq_zero`); (d) checks the application to x-only relations.

## Instances

- Primes (rule: smallest prime above 2^k with 2^8 | p − 1, resp. 2^8 | p + 1):
  Kummer 1,049,089 (k = 20) and 16,777,729 (k = 24); Dickson 1,049,599 (k = 20)
  and 16,780,031 (k = 24).
- Curves: 3 prime-order curves per prime (seeds `P2/curve/<p>/<i>`, i = 0, 1, 2).
- t ∈ {4, 6, 8}; C = min(10,000, number of cosets) candidates, enumerated from seed
  `P2/cosets/<p>/<i>/<t>`. The "random" arm is candidate 0; the "selected" arm is the
  argmax of c_E (lowest index on ties).

## Measurements

- M1: c_E over the C candidates; c_mean, c_max, and the exact expected maximum of C
  i.i.d. Binomial(2^t, q), q the empirical E-fraction.
- M2: exact m = 2 coverage (full enumeration of ±P_i ± P_j over E-columns) for the
  random and the selected coset, every (prime, curve, t); exact m = 3 coverage for
  t ≤ 6 at the 2^20 primes.
- M3: F4 on the relation system (summation polynomial S_{m+1} plus the quadratic
  membership tower for each x_i), random vs selected coset, m = 2 at t = 4 and 6,
  m = 3 at t = 4, 4 planted targets per arm, curve 0 of each prime; msolve 0.6.5
  `-g 2 -t 1 -v 2`, 1.5 GiB, 1,200 s. Metrics: D, widest matrix, F4 work proxy
  (sum over F4 steps of rows × columns), quotient dimension.
- M4: purity, exhaustive: S_3 at the primes 1009, 2003, 4001, 7001, 9973 and S_4 at
  101, 151, 199, three prime-order curves each; count relations with an E-target
  (R ≠ O) and any twist summand.

## Predictions (frozen)

- M1: c_sel/c_mean within 10 % of the exact order-statistic expectation, decaying
  with t (proposal: about 1.9 at t = 4, 1.33 at t = 6, 1.12 at t = 8, C = 10^4).
- M2: coverage(selected)/coverage(random) within 20 % of (c_sel/c_rand)^2.
- M3: F4 work proxy, D and widest matrix differ by less than 10 % between the
  random and the selected coset at equal (m, t) (mean over the 4 targets).
- M4: zero violations.

## Decision rules (frozen)

1. **Lemma application fails:** any M4 violation (the x-only purity statement is
   false as applied; the abstract Lean lemma stands, its hypotheses do not).
2. **Killed (twist columns not inert):** M3 work proxy differs by more than 10 %
   between cosets at any equal (m, t).
3. **Coverage law fails:** M2 ratio off (c_sel/c_rand)^2 by more than 20 % in more
   than one (prime, curve, t) cell.
4. **Toy-scale gain confirmed:** none of 1–3, and (c_sel/c_rand)^m ≥ 1.2 at t = 4.
   Reported with the decay in t and with the selection cost charged: T* = the
   number of targets on one curve at which C · 2^t Legendre symbols (selection) are
   repaid by the saved relation-search work. Cold end-to-end `S` stays unset; the
   proposer's prior for a cold end-to-end gain is 0.05 and for cryptographic size 0.

## Controls

A/A: a second random coset (candidate 1) in M2 and M3. Restricted-base arm in M2:
V ∩ x(E) only, same c, to separate "twist columns are inert" from "coverage depends
on the coset". Three curves per prime.

## Sources checked

KB unavailable. `../../ideas/idea-generator.md` P2,
`research/curve25519_frame_search_20261103/README.md` (frame-choice precedent),
`RESEARCH_PKM_TOWER_ORACLE.md` (line 202, "about half of V"; line 226, seed-drawn
coset), `research/pkm_dickson_torus_20261015/`.

## Addendum A: instrument and pre-data amendments (pinned before any claim cell)

**Instrument.** `p2coset/` (standalone Rust, `src/main.rs` sha256
`963c2f08970dd3d0602e1ad3091210d3a36b7d4cdbcc6e56f1b277350c4d401c`), built with
`cargo build --release`; msolve 0.6.5 (Debian 0.6.5-1build2). Development
validation (`p2coset/README.md`, `smoke/`): `selftest` 14/14; smoke cells at
p = 12,289 (Kummer) and 65,519 (Dickson) only. No claim cell has run.

Commands, per cell (`$B` = the release binary):

```
$B curve    --p P --cseed I                       # I = 0, 1, 2 (curve index)
$B cosets   --kind K --p P --t T --C C --seed S --out cosets.json
$B coverage --kind K --p P --t T --C C --seed S --select idx:0|idx:1|max
$B emit     --kind K --p P --t T --m M --C C --seed S --select idx:0|idx:1|max --tseed N --out cell.ms
( ulimit -v 1572864; nice -n 10 msolve -g 2 -t 1 -v 2 -f cell.ms -o cell.out > cell.log 2>&1 )   # timeout 1200 s
$B parse    --log cell.log
$B purity   --p P --m 2|3 --cseed I
```

`idx:0` is the random arm, `idx:1` the A/A arm, `max` the selected arm (ties to
the lowest index). Seeds S follow the shared rule (README of `followups/`), the
four targets are `--tseed 1..4`.

**Pre-data amendments (from the builder's development run, before any claim
cell):**

1. **M4 violation is redefined.** The proposal's corollary "every m ≤ 3 solution is
   E-pure" is false for trivial relations: at m = 3 a twist pair with
   P_a = −P_b and P_c = R satisfies S_4. What the lemma states, and what
   `twist_part_sum_eq_zero` (crypto-lean#3) formalizes, is that the twist part
   sums to O. A violation is therefore a relation with an E-target R ≠ O whose
   twist summands do not sum to O (`E_target_with_twist_summand_nontrivial` in the
   `purity` output). Trivial cancelling pairs are counted and reported, not
   violations.
2. **The restricted-base control is dropped.** V ∩ x(E) is not a 2-power tower
   (its defining polynomial has degree c_E, not 2^t), so it needs a different
   system and cannot isolate what it was meant to. The A/A arm (`idx:1`) stays.
3. **No constructed positive control.** A coset with all 2^t columns on E cannot be
   planted on a fixed curve; `max` over C candidates is the nearest arm. M1's
   exact order statistic plays the role of the calibrated expectation.
4. **M1 uses the exact binomial expectation printed by `cosets`**, not the
   proposal's Gaussian approximation (which overshoots: 1.84 vs exact 1.69 at
   t = 4, C = 300 in development).
5. **M3 cells need c_E ≥ m** in every arm; a cell where an arm has c_E < m is
   recorded as "not emittable", never imputed.
6. **Coverage at m = 2 is exactly 2·c_E² when all pair sums are distinct**
   (development: 392 at c_E = 14), so M2's law is close to an identity at these
   sizes; M2 is kept as stated and read as a consistency check, and M3 carries the
   decision.
