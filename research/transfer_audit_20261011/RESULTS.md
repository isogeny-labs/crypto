# Transfer audit results (Lead 5)

Protocol: [PROTOCOL.md](PROTOCOL.md) (frozen before the first run; amendments 1 and 2 are dated and
explain every change made while building). Tool: `transfer-audit`
([`src/bin/transfer_audit.rs`](../../src/bin/transfer_audit.rs) and
[`src/bin/transfer_audit/`](../../src/bin/transfer_audit/)). Base revision
`71dd7db5c487f1694ee77c241f011f95f2261e59`.

**Status of every number below.** Cost rows are formula-level estimates from the repository's frozen
`CostModel` unless a row says `measured`; no cost cell of any certificate is measured. What *was*
executed is stated per row (map status, subgroup status). Nothing here is an ECDLP speedup, a key
recovery, or a statement about the security of a deployed curve beyond the verdict vocabulary. Each
statement is labelled **executed**, **derived** (arithmetic from checked antecedents),
**modeled** (formula) or **interpretation**.

## 1. Outcome in one paragraph

On the six audited registered curves (sect113r1, ECC2K-130, K-163, B-163, P-192, secp256k1) and four
toy Koblitz curves, no enumerated candidate has a priced total below the matched Pollard-rho
baseline: every verdict is `no weakness found in scope`. That is an expected outcome and a statement
about the enumerated family only (section 2). The tool also does what the protocol asked of a tool
rather than of a curve: it reproduced the repository's recorded negatives (section 4), executed every
map the repository can execute, certified subgroup preservation or its failure on the actual
generator, and, run on the registered pairing-friendly curve `mnt/mnt1`, returned `class-weak`
(an end-to-end positive control, section 5). Two pre-registered expectations were not met or could
not be met: F9(a) failed (section 6) and no GHS descent with magic number `m >= 2` can be executed
in the repository (section 7).

## 2. Verdict table: curve x transfer type

One row per curve and transfer type. `log2(route/rho)` is `log2` of the priced route over the
matched baseline of the same curve (positive: the route costs more than rho); `>=` marks a total that
omits an unpriced required phase and is therefore a lower bound. The full per-candidate table is
[RESULTS_TABLE.md](RESULTS_TABLE.md) (79 rows) and [cost_ratio.csv](cost_ratio.csv). Subgroup
statuses: `verified` = the map was executed on the generator and every check passed; `arithmetic` =
a proof-level argument with checked antecedents, map not executed; `not_evaluated` = nothing in the
repository can discharge it; `refuted` = the subgroup is not carried; `not_applicable`.

| curve | transfer type | candidates | subgroup statuses | cheapest carried route, log2(route/rho) | verdict |
|:--|:--|--:|:--|--:|:--|
| B-163 | Weil/GHS descent | 2 | not_applicable, not_evaluated | >= 370.7 (`ghs:l=1`) | no weakness found in scope |
| B-163 | isogeny move | 2 | arithmetic, not_evaluated | >= 0.0 (`isogeny:class_invariants`) | no weakness found in scope |
| B-163 | genus-g cover | 2 | not_evaluated, verified | 5.6 (`cover:binary_g3_deg3`) | no weakness found in scope |
| B-163 | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 253.9 (`pairing:tate_mov_frey_ruck`) | no weakness found in scope |
| BLS12-381 [positive control] | Weil/GHS descent | 1 | not_applicable | no carried route | no weakness found in scope |
| BLS12-381 [positive control] | isogeny move | 1 | arithmetic | >= 0.0 (`isogeny:class_invariants`) | no weakness found in scope |
| BLS12-381 [positive control] | genus-g cover | 2 | arithmetic, not_evaluated | 5.3 (`cover:prime_g2_deg2`) | no weakness found in scope |
| BLS12-381 [positive control] | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 3.7 (`pairing:tate_mov_frey_ruck`) | no weakness found in scope |
| mnt/mnt1 [positive control] | Weil/GHS descent | 1 | not_applicable | no carried route | no weakness found in scope |
| mnt/mnt1 [positive control] | isogeny move | 1 | arithmetic | >= 0.0 (`isogeny:class_invariants`) | no weakness found in scope |
| mnt/mnt1 [positive control] | genus-g cover | 2 | arithmetic, not_evaluated | 4.5 (`cover:prime_g2_deg2`) | no weakness found in scope |
| mnt/mnt1 [positive control] | pairing (MOV/Frey-Ruck) | 1 | arithmetic | -14.1 (`pairing:tate_mov_frey_ruck`) | class-weak |
| ECC2K-130 | Weil/GHS descent | 2 | not_applicable, refuted | no carried route | no weakness found in scope |
| ECC2K-130 | isogeny move | 1 | arithmetic | >= 0.0 (`isogeny:class_invariants`) | no weakness found in scope |
| ECC2K-130 | genus-g cover | 2 | not_evaluated, verified | 9.1 (`cover:binary_g3_deg3`) | no weakness found in scope |
| ECC2K-130 | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 186.7 (`pairing:tate_mov_frey_ruck`) | no weakness found in scope |
| K-163 | Weil/GHS descent | 2 | not_applicable, refuted | no carried route | no weakness found in scope |
| K-163 | isogeny move | 1 | arithmetic | >= 0.0 (`isogeny:class_invariants`) | no weakness found in scope |
| K-163 | genus-g cover | 2 | not_evaluated, verified | 9.3 (`cover:binary_g3_deg3`) | no weakness found in scope |
| K-163 | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 244.4 (`pairing:tate_mov_frey_ruck`) | no weakness found in scope |
| P-192 | Weil/GHS descent | 1 | not_applicable | no carried route | no weakness found in scope |
| P-192 | isogeny move | 1 | arithmetic | >= 0.0 (`isogeny:class_invariants`) | no weakness found in scope |
| P-192 | genus-g cover | 2 | arithmetic, not_evaluated | 4.5 (`cover:prime_g2_deg2`) | no weakness found in scope |
| P-192 | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 296.2 (`pairing:tate_mov_frey_ruck`) | no weakness found in scope |
| secp256k1 | Weil/GHS descent | 1 | not_applicable | no carried route | no weakness found in scope |
| secp256k1 | isogeny move | 1 | arithmetic | >= 0.0 (`isogeny:class_invariants`) | no weakness found in scope |
| secp256k1 | genus-g cover | 2 | arithmetic, not_evaluated | 5.3 (`cover:prime_g2_deg2`) | no weakness found in scope |
| secp256k1 | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 399.6 (`pairing:tate_mov_frey_ruck`) | no weakness found in scope |
| sect113r1 | Weil/GHS descent | 2 | not_applicable, not_evaluated | >= 254.1 (`ghs:l=1`) | no weakness found in scope |
| sect113r1 | isogeny move | 3 | arithmetic, not_evaluated, verified | >= 0.0 (`isogeny:class_invariants`) | no weakness found in scope |
| sect113r1 | genus-g cover | 2 | not_evaluated, verified | 5.6 (`cover:binary_g3_deg3`) | no weakness found in scope |
| sect113r1 | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 173.1 (`pairing:tate_mov_frey_ruck`) | no weakness found in scope |
| toy-k0-n15 | Weil/GHS descent | 3 | refuted | no carried route | no weakness found in scope |
| toy-k0-n15 | isogeny move | 1 | arithmetic | >= 0.3 (`isogeny:class_invariants`) | no weakness found in scope |
| toy-k0-n15 | genus-g cover | 2 | not_evaluated, verified | 7.6 (`cover:binary_g3_deg3`) | no weakness found in scope |
| toy-k0-n15 | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 9.9 (`pairing:tate_mov_frey_ruck`) | no weakness found in scope |
| toy-k0-n15 (control) | Weil/GHS descent | 1 | verified | 4.0 (`ghs:l=5`) | no weakness found in scope |
| toy-k1-n15 | Weil/GHS descent | 3 | refuted | no carried route | no weakness found in scope |
| toy-k1-n15 | isogeny move | 1 | arithmetic | >= 0.6 (`isogeny:class_invariants`) | no weakness found in scope |
| toy-k1-n15 | genus-g cover | 2 | not_evaluated, verified | 7.6 (`cover:binary_g3_deg3`) | no weakness found in scope |
| toy-k1-n15 | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 9.1 (`pairing:tate_mov_frey_ruck`) | no weakness found in scope |
| toy-k1-n15 (control) | Weil/GHS descent | 2 | verified | 4.0 (`ghs:l=5`) | no weakness found in scope |
| toy-k1-n21 | Weil/GHS descent | 3 | refuted, verified | 3.4 (`ghs:l=7`) | no weakness found in scope |
| toy-k1-n21 | isogeny move | 1 | arithmetic | >= 1.0 (`isogeny:class_invariants`) | no weakness found in scope |
| toy-k1-n21 | genus-g cover | 2 | not_evaluated, verified | 7.9 (`cover:binary_g3_deg3`) | no weakness found in scope |
| toy-k1-n21 | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 7.5 (`pairing:tate_mov_frey_ruck`) | no weakness found in scope |
| toy-k1-n35 | Weil/GHS descent | 3 | refuted | no carried route | no weakness found in scope |
| toy-k1-n35 | isogeny move | 1 | arithmetic | >= 0.5 (`isogeny:class_invariants`) | no weakness found in scope |
| toy-k1-n35 | genus-g cover | 2 | not_evaluated, verified | 8.2 (`cover:binary_g3_deg3`) | no weakness found in scope |
| toy-k1-n35 | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 8.3 (`pairing:tate_mov_frey_ruck`) | no weakness found in scope |
| toy-k1-n35 (control) | Weil/GHS descent | 1 | verified | 5.1 (`ghs:l=5`) | no weakness found in scope |

Figures: [cost_ratio.svg](cost_ratio.svg) (registered curves), [cost_ratio_toy.svg](cost_ratio_toy.svg)
(toy curves and subfield-subgroup controls), [cost_ratio_controls.svg](cost_ratio_controls.svg)
(`mnt/mnt1`, BLS12-381), [pipeline.svg](pipeline.svg) (tool structure with the status counts of this
run). They are generated from the certificates by `transfer-audit report`; the SVG is the source and
the rendering. They were not rasterized or inspected by eye here (no SVG rasterizer is installed:
`rsvg-convert` is absent and ImageMagick delegates to it); a text-extent check found no overlapping
or out-of-bounds labels.

Matched baselines (rho, `sqrt(pi r / 4)` operations of one group operation each, divided by `sqrt(c)`
for the equivalence class `c`):

| curve | ICV1 slug | field bits | log2 r | cofactor | class c | log2 baseline |
|:--|:--|--:|--:|--:|--:|--:|
| sect113r1 | `icv1-f2m113-tm122610772499221213-97df4ac6` | 113 | 112.0 | 2 | 2 | 69.458 |
| ECC2K-130 | `icv1-f2m131-tm22283658519494248867-115e0dc5` | 131 | 129.0 | 4 | 262 (negation, Frobenius) | 74.780 |
| K-163 | `icv1-f2m163-tm4845466632539410776804317-e15e581e` | 163 | 162.0 | 2 | 326 | 91.622 |
| B-163 | `icv1-f2m163-tm6224022517221266226059365-31d88d74` | 163 | 162.0 | 2 | 2 | 95.296 |
| P-192 | `icv1-fp192-t31607402316713927207482677199-52e4af59` | 192 | 192.0 | 1 | 2 | 110.670 |
| secp256k1 | `icv1-fp256-t432420386565659656852420866390673177327-76dadd18` | 256 | 256.0 | 1 | 6 (negation, `j = 0`) | 142.536 |
| mnt/mnt1 (control) | `icv1-fp170-tm52213820118540299389901413-869d5d3c` | 169.3 | 155.4 | 15337 | 2 | 92.070 |
| BLS12-381 (control) | `icv1-fp381-tm15132376222941642751-961de9ba` | 380.7 | 254.9 | 7.6e37 | 6 | 142.872 |

Toy curves are unregistered (no ICV1 slug), fully specified in their certificates: `y^2 + xy = x^3 +
a x^2 + 1` over `F_2[z]/(z^15+z+1)` (`a = 1` and `a = 0`), `z^21+z^2+1`, `z^35+z^2+1`, seed
`20261011`. Their prime subgroups are small (`r` = 211, 751, 71, 631; 6 to 10 bits) because the order
of a Koblitz curve over `F_(2^n)` is divisible by the orders over every subfield; they test the
mechanics, not the economics.

## 3. What was executed, what is arithmetic only, what was not evaluated

| item | status | where |
|:--|:--|:--|
| instance validation (field, curve, generator, `[r]G = O`, `r` prime, order consistency, two random points annihilated by `#E`) on all 12 curves | executed | `curve.instance_checks` of every certificate |
| trace descent `E(F_(2^n)) -> E(F_(2^l))`, `m = 1`: ECC2K-130 and K-163 at full size (131 and 163 Frobenius steps), toys at every tower | executed; `<G>` killed on both Koblitz curves, as the arithmetic `gcd(r, #E(F_2)) = 1` predicts | `ecc2k-130/ghs_l_1.json`, `k-163/ghs_l_1.json`, `toy-*/ghs_l_*.json` |
| subgroup checker on those maps: image order, kernel, additivity on 4 sample pairs, scalar equivariance on 3 scalars | executed | `subgroup_preservation.executed` |
| same-field genus-3 degree-3 cover, pullback of `G` into `Jac(H)(F_q)`, `[r]phi^*(G) = O`, `phi^*(G) != O`, push-pull `[3]` witness: sect113r1, ECC2K-130, K-163, B-163 at full size, 4 toys | executed, `verified` on all 8 | `*/cover_binary_g3_deg3.json` |
| explicit degree-5 Velu isogenies of sect113r1 (replay of `sect113r1_audit::run_audit` in-process) | executed, `verified` | `sect113r1/isogeny_explicit_degree5.json` |
| GHS magic number, type, genus, condition (8), Weil capacity, achievable magic numbers per tower | derived (exact) | `ghs_l_*.json`, `ghs_proper_subfield.json` |
| GHS descent with `m >= 2`: sect113r1 `l = 1` (genus `2^112`), B-163 `l = 1` | arithmetic only; map construction not evaluated (section 7) | `sect113r1/ghs_l_1.json`, `b-163/ghs_l_1.json` |
| prime-field genus-2 cover `v^2 = F(u^2 + c)`: P-192, secp256k1 | arithmetic only: `phi_* phi^* = [2]`, `gcd(2, r) = 1`; Jacobian arithmetic for the two-infinity model does not exist | `p-192/cover_prime_g2_deg2.json` |
| pairing reduction: all curves | arithmetic only (Frey-Ruck non-degeneracy); no generic pairing in the repository | `*/pairing_tate_mov_frey_ruck.json` |
| isogeny class invariants, volcano data, GHS isogeny extension | derived / heuristic; no destination representative exhibited | `*/isogeny_*.json` |
| `f_E` | `End(E) = O_K` is established for all six curves by the repository's trait records, so `f_E = 1` and an isogeny cannot lower it | `isogeny_class_invariants.json`, check `end_ring_conductor_f_E` |
| (2,2)-gluing / Richelot candidates | not evaluated; the `E[2]` Galois structure is recorded (P-192 and secp256k1: irreducible cubic, `E(F_p)[2]` trivial) | `cover_prime_g2_deg2.json` |
| index calculus on a genus-`g` binary Jacobian | does not exist in the repository; the destination price is the frozen formula | open obligation on every `m >= 2` and cover row |

## 4. The repository's recorded negatives, reproduced (interpretation labelled)

**F1, sect113r1 (PASS, derived).** One tower `l = 1` (113 is prime), magic number 113, type II, genus
`2^112 = 5192296858534827628530496329220096`, MMT condition (8) holds (`n` odd). The least genus whose
Jacobian over `F_2` can hold the 112-bit subgroup is 45, i.e. genus `<= 44` is excluded, the same
threshold as `tests/sect113r1_ghs_evidence.rs` (`weil_capacity_gate_excludes_genus_at_most_44`); a unit
test reproduces it from the Hasse-Weil bound. Class-wide, any curve over `F_(2^113)` has magic number
1, 29, 57, 85 or 113 for this tower (the factorization of `x^113 - 1` over `F_2` has one linear and
four degree-28 factors), so every isogenous representative has genus `>= 2^28 - 1`; the best-case
priced GHS route through `F_2` at that genus is 57.0 bits above the baseline.

**F2, route totals vs `docs/curves/jacobian-transfers.json` (PASS, 10 of 10 within 0.002 bits).** The
GHS rows with `m >= 2` (sect113r1 `l = 1`: 323.598 on both sides; B-163 `l = 1`: 465.948) and the
same-field cover rows (eight curves) agree to the printed precision; the check is attached to each
certificate as `invariants.crosscheck_jacobian_transfers`.

**F3, prime degree (PASS, derived).** For sect113r1 (n = 113), ECC2K-130 (131), K-163 and B-163 (163)
the tool emits `ghs:proper_subfield`, an `out of scope` certificate: no divisor `d` of `n` with
`1 < d < n`, and the magic numbers any curve over the field can reach for the tower
`F_2 <= F_(2^n)` are `{1, 29, 57, 85, 113}`, `{1, 131}`, `{1, 163}`, `{1, 163}`; the window 2..=6 is
empty in all four. This is a scope statement for GHS descent to a proper subfield and nothing more.

**F4, Koblitz trace branch (PASS, executed).** On ECC2K-130 `Tr_{F_(2^131)/F_2}` has image in `E(F_2)`
of order 4, `gcd(r, 4) = 1`, and the executed trace of the generator is the identity (K-163: order 2,
same outcome). This agrees with `research/notes/ecc2k130/RESEARCH_ECC2K130_HYPERELLIPTIC.md` (the
conorm-norm composite is the zero map on the target subgroup); here it is executed on the actual
generator and checked against the arithmetic.

**F5 (PASS, executed).** On the toy towers, the arithmetic prediction (`#E(F_(2^l))` by the Lucas
recurrence, `gcd(r, #E)`) agreed with the executed trace in all 18 executed trace cases (the two deployed Koblitz curves, 12 toy
towers, 4 subfield-subgroup controls). On `toy-k1-n21`, `l = 7`, `r = 71` divides `#E(F_(2^7)) = 142`, so the
natural subgroup survives the trace (`verified`); all other natural subgroups are killed.

**F6, subfield-subgroup control (executed; interpretation).** For the toys with a subfield subgroup
the trace is injective on it (`verified`, image of order `r'`). Priced honestly, the route is not
cheaper: the destination solve is 2^5.8 to 2^6.9 bit operations, but the transfer evaluation
(2^11.1 to 2^14.3) and the one verifying scalar multiplication (2^11.6 to 2^13.8) alone exceed the
whole matched rho (2^8.3 to 2^9.9), so the ratios are 4.0 to 5.1 bits above the baseline. Structure
exists; there is no speedup, not even a constant-factor one, at toy size. The exponent is unchanged by construction
(same `r'`, destination an elliptic curve).

**F7, binary genus-3 cover (PASS, executed).** On all eight binary curves `phi^*(G)` has exact order
`r` in `Jac(H)(F_q)` (image order equals `r`, kernel trivial), 4 of 4 additivity samples and 3 of 3
scalar-equivariance samples pass, and the push-pull `[3]` witness holds. For sect113r1 (113-bit
field, 112-bit `r`) this is a verified subgroup-preserving map into a genus-3 Jacobian; the best
destination solver on it is rho on the image (priced 5.6 bits above the baseline: one Jacobian
operation costs about 34 elliptic ones), while double-large-prime index calculus on the Jacobian
costs `2^133.8` (genus 2) to `2^225.9` (genus 8) against `2^74.0`..`2^78.0` for rho on the image.

**F8, genus-`g` covers (PASS, modeled).** For every curve and every `g` in 2..8 the leading term
`(2 - 2/g) log2 q` exceeds `0.5 log2 r`, the index-calculus total exceeds the baseline, and the best
priced route exceeds it. This reproduces `IDEA-20261009-275a14` (cover attacks cost `p^(2-2/g) >= p`,
worse than rho) under the frozen model and is not a new result. For P-192: index calculus at genus 2
costs `2^214.0` against `2^115.2` for rho on the image and `2^110.7` for the baseline.

**Pairing rows (modeled).** The embedding degree is exact on all audited curves (the repository's
committed factor hints complete the factorization of `r - 1`): for sect113r1
`k = (r - 1)/2 = 2596148429267413844917941289415351`; for secp256k1 and P-192 it is also of the size of
`r`. A single element of `F_(q^k)` has `10^35` bits or more, so no pairing can be evaluated (check
`pairing_target_element_storable` = `FAIL`), and the priced route is 173 to 400 bits above the
baseline at both ends of the band (the generic rho term in the subgroup of `F_(q^k)^*` is the cheaper
of the two, so the ends coincide).

## 5. End-to-end positive controls (F10, F13)

* **F10, synthetic (PASS).** A genus-4 descent over `F_(2^40)` for a 158-bit subgroup of a
  160-bit-field curve is classed `formula_advantage` by the same classifier (total `2^81.6`,
  `HyperellipticIndexCalculus`). It has no map and no subgroup certificate, so it is a lead, not a
  weakness (`controls/synthetic_formula_advantage.json`).
* **F13, `mnt/mnt1` (embedding degree 3, 155-bit subgroup, 170-bit prime) returns `class-weak`
  (as pre-registered).** Pairing route: conservative price `2^77.9` (generic NFS constant, weighted by
  field-multiplication cost) against a baseline of `2^92.1`, i.e. 14.1 bits below it; the
  attacker-favourable end is 41.5 bits below. Subgroup status `arithmetic` (Frey-Ruck), map basis
  `Theorem`, embedding degree exact, one element of `F_(p^3)` has 508 bits. The curve is an MNT example
  designed so that the pairing is the attack (registry adoption status `not_asserted`); the verdict
  says the pairing route is priced below rho under this model, not that the curve is broken at any
  absolute level. Its trait record was read from the committed `docs/curves/traits.json` because a
  live `curve_traits::compute` of this curve takes about six minutes (not in this tool's code).
* **F13, BLS12-381 (`k = 12`) returns `no weakness found in scope` with a marginal lead
  (as pre-registered).** The price band is `2^130.0` (favourable) to `2^146.5` (conservative) against
  `2^142.9`: it straddles rho, so the formula band cannot decide. The conservative end is the one
  allowed to support a weakness; refining that estimate (tower-NFS and exact constants) is an open
  obligation, not a finding.

## 6. Pre-registered check that failed: F9(a)

F9 measured the repository's own `hyperelliptic_index_calculus` against its own rho on thread curves
of genus 2 and 3 (3 index-calculus and 9 rho trials per instance, `fb-walk`, sparse, gcd), and fitted
the exponent of total group operations in `p`. Pre-registered: exponent `>= 1` for both genera and
index calculus above the modelled source rho (a subgroup of order about `p`) at every size.

| plan | genus | primes | fitted exponent (std. error) | threshold | pass |
|:--|--:|:--|--:|--:|:--|
| protocol (committed) | 2 | 41, 61, 101, 151 | 0.857 (0.065) | 1 | **no** |
| protocol (committed) | 3 | 23, 31, 41, 61 | 0.962 (0.287) | 1 | **no** |
| extended, post hoc | 2 | 41 .. 401 (7 values) | 0.908 (0.032) | 1 | no |
| extended, post hoc | 3 | 23 .. 151 (6 values) | 0.928 (0.089) | 1 | no |

F9(a) **failed as pre-registered**. F9(b) **passed**: every index-calculus run was correct and above
the modelled rho cost on a source subgroup of order about `p` (ratios 11.1 to 17.7 at genus 2 and 34.9
to 58.1 at genus 3 in the protocol plan; 11.2 to 28.2 and 35.3 to 75.1 in the extended plan, growing
with `p`). **Interpretation (labelled):** the fits are below 1 at genus 2 by 2 to 3 standard errors
and consistent with 1 at genus 3; at these sizes an additive constant (walk precomputation) flattens
the slope, and four to seven points do not discriminate the asymptotic exponents `2 - 2/g`. The
result neither confirms nor contradicts the asymptotic statement, and the threshold was not changed
after seeing it (protocol amendment 2). Caveats on the control: the relations are those of a generic
genus-`g` Jacobian, not of a certified image of `E` (the repository cannot produce the odd-
characteristic cover image as input to its index calculus); group-operation equivalents include a
wall-clock calibration, so these two files are not byte-reproducible and `check` skips them; the
two sides of the ratio are in Jacobian and elliptic group operations respectively, which understates
the gap.

## 7. What was not done, and why

* **No GHS descent with `m >= 2` was executed.** The smooth GHS curve and the norm-conorm point map do
  not exist in the repository (`docs/GHS_TRANSPORT.md`, "Remaining construction gates";
  `ghs_descent.rs`). Recorded as [problems/ghs-higher-magic-descent-not-executable.md](../../problems/ghs-higher-magic-descent-not-executable.md)
  (kind: implementation gap). The brief asked for relation counting on an executed composite-degree
  GHS descent with the repository's hyperelliptic index calculus; that code is odd-characteristic only,
  and no input from a binary descent can reach it. What replaces it: the trace descent and the
  genus-3 cover pullback are executed and certified on composite-degree toy curves (n = 15, 21, 35),
  and relation counts are measured on odd-characteristic Jacobians (section 6) with the caveat above.
  Both substitutions are partial; the original requirement stays open.
* **No PDF.** `report.pdf` was not built: Typst is not installed, and `pandoc` (3.1.3) has no PDF
  engine here (no LaTeX, `wkhtmltopdf` or `weasyprint`). The report is this Markdown with
  generated SVG figures and CSV; the PDF requirement of `research-visuals` is unmet, with the blocker
  stated, and no old PDF is presented.
* **No benchmark entry.** Nothing here measures how fast an algorithm runs or reports a speedup, so
  the `benchmarks/` copy rule does not apply.
* **No replay by a second implementation.** `transfer-audit check` regenerates the 93 deterministic
  files byte-identically with the same code; that is determinism, not independent replay. The
  validation gate for leaderboards is not engaged (no row is submitted).
* **Pairing and isogeny prices are bands or lower bounds.** Quasi-polynomial algorithms for
  small-characteristic fields, tower NFS and exact `o(1)` terms are not modeled; the isogeny walk to
  an unidentified representative and the GHS map construction are `not_evaluated`.
* **Absolute-size verdicts are out of scope.** The repository's separate `sect113r1_audit` labels
  sect113r1 `class-weak` on size alone (generic rho on a 112-bit subgroup is below an 80-bit
  threshold). This tool's `no weakness found in scope` for sect113r1 concerns transfers only and does
  not contradict it.

## 8. Observations on existing code (accounting, not changed here)

* `jacobian_transfer::cost::LOG2_SQRT_PI_OVER_4 = -0.1736147`; `0.5 log2(pi/4) = -0.1742519`. The
  difference, 6.4e-4 bits, shifts the last printed digit of some frozen outputs; it was not changed
  (it would regenerate `docs/curves/jacobian-transfers.json`). The cross-checks above are exact
  because this tool calls the same function.
* `isogeny_degree_search::cost::rho_reference` uses `sqrt(pi r / 2)` iterations; `CostModel` uses
  `sqrt(pi r / 4)`. The tool follows `CostModel` and records the choice in each certificate.
* `curve_traits::compute` for `icv1-fp170-tm52213820118540299389901413-869d5d3c` (`mnt/mnt1`) took 360 s
  in one run (hints loaded); every other audited curve took under one second.

## 9. Reproduction

```sh
export CARGO_TARGET_DIR=/home/user/cargo-target      # any target directory
cargo test  --release --bin transfer-audit            # 25 tests
cargo build --release --bin transfer-audit
./target/release/transfer-audit suite  --out research/transfer_audit_20261011/certificates --measure-ic --measure-ic-extended
./target/release/transfer-audit check  --certs research/transfer_audit_20261011/certificates   # 93 files, byte-identical
./target/release/transfer-audit report --certs research/transfer_audit_20261011/certificates --out research/transfer_audit_20261011
./target/release/transfer-audit audit  --curve sect113r1     # one curve, summary on stdout
./target/release/transfer-audit presets
```

The committed run used the toolchain `cargo 1.97.0` on a 4-core host; the lib release build took
12 minutes wall (23.5 CPU-minutes), the suite 3 s without the measured controls and 20 s plus 68 s
for the protocol and extended F9 plans (`certificates/run-log.json`). Total compute for the study, builds included, was roughly
50 CPU-minutes.
