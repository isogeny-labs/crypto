# Transfer audit results (Lead 5)

Protocol: [PROTOCOL.md](PROTOCOL.md) (frozen before the first run; amendments 1 and 2 are dated and
explain every change made while building; amendment 3 records the changes made after an independent
review of the merged tool). Tool: `transfer-audit`
([`src/bin/transfer_audit.rs`](../../src/bin/transfer_audit.rs) and
[`src/bin/transfer_audit/`](../../src/bin/transfer_audit/)). Base revision
`71dd7db5c487f1694ee77c241f011f95f2261e59`.

**Revision (2026-10-26, amendment 3).** An independent review of the merged tool (PR #2594) found
a baseline 0.5 bit too low, an uncalibrated pairing verdict, an over-claimed
`computational_advantage = "refuted"`, and several smaller errors in this file. The certificates are
now schema `transfer-audit/certificate/v2` (tool `transfer-audit/2`), regenerated with the tool's own
`suite` command; every number below is the v2 number. Where it differs from the v1 run, section 10
lists the change and the old value. The F9 records (`controls/hec_ic_control*.json`) and
`certificates/run-log.json` are the original measured run and were not regenerated, so the
timings in `run-log.json` are those of the v1 suite.

The v2 regeneration wrote over the v1 certificates, index and report files in place. They are
restored byte for byte under [`v1/`](v1/README.md) (from commit `3f31162cd`, the study as it stood
before PR #2658) so that both sets are in the tree. Every file outside `v1/` is v2.

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
about the enumerated family only (section 2). It is not one kind of statement, and each certificate
now says which (the `evaluation` field, a field and not a verdict label): of the 65 candidates on
those ten curves none is merely evaluated, **42 are excluded** (the subgroup is not carried, or the
priced total, a lower bound where phases are unpriced, is not below the baseline) and **23 are not
evaluable**: the route may exist, but the repository cannot construct or price it far enough to
decide, so `no weakness found in scope` is vacuous for it (10 identity-path isogeny rows, 10 formula
sweeps, the sect113r1 degree-5 replay, 2 isogeny-extension heuristics). The tool also does what the
protocol asked of a tool rather than of a curve: it reproduced the repository's recorded negatives
(section 4), executed every map the repository can execute, certified subgroup preservation or its
failure on the actual generator, and, run on the registered pairing-friendly curve `mnt/mnt1`,
returned `class-weak` (an end-to-end positive control, section 5). That verdict, and the 12 other
`class-weak` rows of the registry sweep, say the conservative pairing price is below rho on E under
this model by at least the 8-bit model margin; they are not statements about absolute security
(section 5). Two pre-registered expectations were not met or could not be met: F9(a) failed
(section 6) and no GHS descent with magic number `m >= 2` can be executed in the repository
(section 7).

## 2. Verdict table: curve x transfer type

One row per curve and transfer type. `log2(route/rho)` is `log2` of the priced route over the
matched baseline of the same curve (positive: the route costs more than rho); `>=` marks a total that
omits an unpriced required phase and is therefore a lower bound. The full per-candidate table is
[RESULTS_TABLE.md](RESULTS_TABLE.md) (104 rows) and [cost_ratio.csv](cost_ratio.csv). Subgroup
statuses: `verified` = the map was executed on the generator and every check passed; `arithmetic` =
a proof-level argument with checked antecedents, map not executed; `not_evaluated` = nothing in the
repository can discharge it; `refuted` = the subgroup is not carried; `not_applicable`. The
**evaluated / excluded / not evaluable** column counts the candidates of the row by the certificates'
`evaluation.status`: *evaluated* = priced completely and compared (including an inconclusive
comparison: a price inside the margin, a band that straddles rho); *excluded* = cannot yield a
weakness (the subgroup is not carried or the transfer does not exist, or the priced total, a lower
bound where phases are unpriced, is not below the baseline); *not evaluable* = the route may exist
but its map or price could not be constructed far enough to decide (an identity-path price, a formula
sweep or heuristic, an unconstructed map whose priced part is below the baseline), so the verdict
`no weakness found in scope` is vacuous for it. Reading the verdict column without this one
over-reads it: a `0 / 0 / 1` row is a row the tool did not decide.

| curve | transfer type | candidates | subgroup statuses | cheapest carried route, log2(route/rho) | evaluated / excluded / not evaluable | verdict |
|:--|:--|--:|:--|--:|:--|:--|
| B-163 | Weil/GHS descent | 2 | not_applicable, not_evaluated | >= 370.2 (`ghs:l=1`) | 0 / 2 / 0 | no weakness found in scope |
| B-163 | isogeny move | 2 | arithmetic, not_evaluated | >= 0.0 (`isogeny:class_invariants`) | 0 / 0 / 2 | no weakness found in scope |
| B-163 | genus-g cover | 2 | not_evaluated, verified | 5.1 (`cover:binary_g3_deg3`) | 0 / 1 / 1 | no weakness found in scope |
| B-163 | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 253.4 (`pairing:tate_mov_frey_ruck`) | 0 / 1 / 0 | no weakness found in scope |
| BLS12-638 [boundary control] | Weil/GHS descent | 1 | not_applicable | no carried route | 0 / 1 / 0 | no weakness found in scope |
| BLS12-638 [boundary control] | isogeny move | 1 | arithmetic | >= 0.0 (`isogeny:class_invariants`) | 0 / 0 / 1 | no weakness found in scope |
| BLS12-638 [boundary control] | genus-g cover | 2 | arithmetic, not_evaluated | 4.8 (`cover:prime_g2_deg2`) | 0 / 1 / 1 | no weakness found in scope |
| BLS12-638 [boundary control] | pairing (MOV/Frey-Ruck) | 1 | arithmetic | -7.1 (`pairing:tate_mov_frey_ruck`) | 1 / 0 / 0 | no weakness found in scope |
| BN254 [boundary control] | Weil/GHS descent | 1 | not_applicable | no carried route | 0 / 1 / 0 | no weakness found in scope |
| BN254 [boundary control] | isogeny move | 1 | arithmetic | >= 0.0 (`isogeny:class_invariants`) | 0 / 0 / 1 | no weakness found in scope |
| BN254 [boundary control] | genus-g cover | 2 | arithmetic, not_evaluated | 4.8 (`cover:prime_g2_deg2`) | 0 / 1 / 1 | no weakness found in scope |
| BN254 [boundary control] | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 3.2 (`pairing:tate_mov_frey_ruck`) | 1 / 0 / 0 | no weakness found in scope |
| BN318 [boundary control] | Weil/GHS descent | 1 | not_applicable | no carried route | 0 / 1 / 0 | no weakness found in scope |
| BN318 [boundary control] | isogeny move | 1 | arithmetic | >= 0.0 (`isogeny:class_invariants`) | 0 / 0 / 1 | no weakness found in scope |
| BN318 [boundary control] | genus-g cover | 2 | arithmetic, not_evaluated | 4.8 (`cover:prime_g2_deg2`) | 0 / 1 / 1 | no weakness found in scope |
| BN318 [boundary control] | pairing (MOV/Frey-Ruck) | 1 | arithmetic | -3.5 (`pairing:tate_mov_frey_ruck`) | 1 / 0 / 0 | no weakness found in scope |
| BN350 [boundary control] | Weil/GHS descent | 1 | not_applicable | no carried route | 0 / 1 / 0 | no weakness found in scope |
| BN350 [boundary control] | isogeny move | 1 | arithmetic | >= 0.0 (`isogeny:class_invariants`) | 0 / 0 / 1 | no weakness found in scope |
| BN350 [boundary control] | genus-g cover | 2 | arithmetic, not_evaluated | 4.8 (`cover:prime_g2_deg2`) | 0 / 1 / 1 | no weakness found in scope |
| BN350 [boundary control] | pairing (MOV/Frey-Ruck) | 1 | arithmetic | -13.3 (`pairing:tate_mov_frey_ruck`) | 1 / 0 / 0 | class-weak |
| BN446 [boundary control] | Weil/GHS descent | 1 | not_applicable | no carried route | 0 / 1 / 0 | no weakness found in scope |
| BN446 [boundary control] | isogeny move | 1 | arithmetic | >= 0.0 (`isogeny:class_invariants`) | 0 / 0 / 1 | no weakness found in scope |
| BN446 [boundary control] | genus-g cover | 2 | arithmetic, not_evaluated | 4.8 (`cover:prime_g2_deg2`) | 0 / 1 / 1 | no weakness found in scope |
| BN446 [boundary control] | pairing (MOV/Frey-Ruck) | 1 | arithmetic | -44.5 (`pairing:tate_mov_frey_ruck`) | 1 / 0 / 0 | class-weak |
| BLS12-381 [positive control] | Weil/GHS descent | 1 | not_applicable | no carried route | 0 / 1 / 0 | no weakness found in scope |
| BLS12-381 [positive control] | isogeny move | 1 | arithmetic | >= 0.0 (`isogeny:class_invariants`) | 0 / 0 / 1 | no weakness found in scope |
| BLS12-381 [positive control] | genus-g cover | 2 | arithmetic, not_evaluated | 4.8 (`cover:prime_g2_deg2`) | 0 / 1 / 1 | no weakness found in scope |
| BLS12-381 [positive control] | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 3.2 (`pairing:tate_mov_frey_ruck`) | 1 / 0 / 0 | no weakness found in scope |
| mnt/mnt1 [positive control] | Weil/GHS descent | 1 | not_applicable | no carried route | 0 / 1 / 0 | no weakness found in scope |
| mnt/mnt1 [positive control] | isogeny move | 1 | arithmetic | >= 0.0 (`isogeny:class_invariants`) | 0 / 0 / 1 | no weakness found in scope |
| mnt/mnt1 [positive control] | genus-g cover | 2 | arithmetic, not_evaluated | 4.0 (`cover:prime_g2_deg2`) | 0 / 1 / 1 | no weakness found in scope |
| mnt/mnt1 [positive control] | pairing (MOV/Frey-Ruck) | 1 | arithmetic | -14.6 (`pairing:tate_mov_frey_ruck`) | 1 / 0 / 0 | class-weak |
| ECC2K-130 | Weil/GHS descent | 2 | not_applicable, refuted | no carried route | 0 / 2 / 0 | no weakness found in scope |
| ECC2K-130 | isogeny move | 1 | arithmetic | >= 0.0 (`isogeny:class_invariants`) | 0 / 0 / 1 | no weakness found in scope |
| ECC2K-130 | genus-g cover | 2 | not_evaluated, verified | 5.1 (`cover:binary_g3_deg3`) | 0 / 1 / 1 | no weakness found in scope |
| ECC2K-130 | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 186.2 (`pairing:tate_mov_frey_ruck`) | 0 / 1 / 0 | no weakness found in scope |
| K-163 | Weil/GHS descent | 2 | not_applicable, refuted | no carried route | 0 / 2 / 0 | no weakness found in scope |
| K-163 | isogeny move | 1 | arithmetic | >= 0.0 (`isogeny:class_invariants`) | 0 / 0 / 1 | no weakness found in scope |
| K-163 | genus-g cover | 2 | not_evaluated, verified | 5.1 (`cover:binary_g3_deg3`) | 0 / 1 / 1 | no weakness found in scope |
| K-163 | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 243.9 (`pairing:tate_mov_frey_ruck`) | 0 / 1 / 0 | no weakness found in scope |
| P-192 | Weil/GHS descent | 1 | not_applicable | no carried route | 0 / 1 / 0 | no weakness found in scope |
| P-192 | isogeny move | 1 | arithmetic | >= 0.0 (`isogeny:class_invariants`) | 0 / 0 / 1 | no weakness found in scope |
| P-192 | genus-g cover | 2 | arithmetic, not_evaluated | 4.0 (`cover:prime_g2_deg2`) | 0 / 1 / 1 | no weakness found in scope |
| P-192 | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 295.7 (`pairing:tate_mov_frey_ruck`) | 0 / 1 / 0 | no weakness found in scope |
| secp256k1 | Weil/GHS descent | 1 | not_applicable | no carried route | 0 / 1 / 0 | no weakness found in scope |
| secp256k1 | isogeny move | 1 | arithmetic | >= 0.0 (`isogeny:class_invariants`) | 0 / 0 / 1 | no weakness found in scope |
| secp256k1 | genus-g cover | 2 | arithmetic, not_evaluated | 4.8 (`cover:prime_g2_deg2`) | 0 / 1 / 1 | no weakness found in scope |
| secp256k1 | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 399.1 (`pairing:tate_mov_frey_ruck`) | 0 / 1 / 0 | no weakness found in scope |
| sect113r1 | Weil/GHS descent | 2 | not_applicable, not_evaluated | >= 253.6 (`ghs:l=1`) | 0 / 2 / 0 | no weakness found in scope |
| sect113r1 | isogeny move | 3 | arithmetic, not_evaluated, verified | >= 0.0 (`isogeny:class_invariants`) | 0 / 0 / 3 | no weakness found in scope |
| sect113r1 | genus-g cover | 2 | not_evaluated, verified | 5.1 (`cover:binary_g3_deg3`) | 0 / 1 / 1 | no weakness found in scope |
| sect113r1 | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 172.6 (`pairing:tate_mov_frey_ruck`) | 0 / 1 / 0 | no weakness found in scope |
| toy-k0-n15 | Weil/GHS descent | 3 | refuted | no carried route | 0 / 3 / 0 | no weakness found in scope |
| toy-k0-n15 | isogeny move | 1 | arithmetic | >= 0.3 (`isogeny:class_invariants`) | 0 / 0 / 1 | no weakness found in scope |
| toy-k0-n15 | genus-g cover | 2 | not_evaluated, verified | 5.2 (`cover:binary_g3_deg3`) | 0 / 1 / 1 | no weakness found in scope |
| toy-k0-n15 | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 9.4 (`pairing:tate_mov_frey_ruck`) | 0 / 1 / 0 | no weakness found in scope |
| toy-k0-n15 (control) | Weil/GHS descent | 1 | verified | 3.5 (`ghs:l=5`) | 0 / 1 / 0 | no weakness found in scope |
| toy-k1-n15 | Weil/GHS descent | 3 | refuted | no carried route | 0 / 3 / 0 | no weakness found in scope |
| toy-k1-n15 | isogeny move | 1 | arithmetic | >= 0.4 (`isogeny:class_invariants`) | 0 / 0 / 1 | no weakness found in scope |
| toy-k1-n15 | genus-g cover | 2 | not_evaluated, verified | 5.3 (`cover:binary_g3_deg3`) | 0 / 1 / 1 | no weakness found in scope |
| toy-k1-n15 | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 8.6 (`pairing:tate_mov_frey_ruck`) | 0 / 1 / 0 | no weakness found in scope |
| toy-k1-n15 (control) | Weil/GHS descent | 2 | verified | 3.5 (`ghs:l=5`) | 0 / 2 / 0 | no weakness found in scope |
| toy-k1-n21 | Weil/GHS descent | 3 | refuted, verified | 3.0 (`ghs:l=7`) | 0 / 3 / 0 | no weakness found in scope |
| toy-k1-n21 | isogeny move | 1 | arithmetic | >= 0.8 (`isogeny:class_invariants`) | 0 / 0 / 1 | no weakness found in scope |
| toy-k1-n21 | genus-g cover | 2 | not_evaluated, verified | 5.4 (`cover:binary_g3_deg3`) | 0 / 1 / 1 | no weakness found in scope |
| toy-k1-n21 | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 7.0 (`pairing:tate_mov_frey_ruck`) | 0 / 1 / 0 | no weakness found in scope |
| toy-k1-n35 | Weil/GHS descent | 3 | refuted | no carried route | 0 / 3 / 0 | no weakness found in scope |
| toy-k1-n35 | isogeny move | 1 | arithmetic | >= 0.4 (`isogeny:class_invariants`) | 0 / 0 / 1 | no weakness found in scope |
| toy-k1-n35 | genus-g cover | 2 | not_evaluated, verified | 5.3 (`cover:binary_g3_deg3`) | 0 / 1 / 1 | no weakness found in scope |
| toy-k1-n35 | pairing (MOV/Frey-Ruck) | 1 | arithmetic | 7.8 (`pairing:tate_mov_frey_ruck`) | 0 / 1 / 0 | no weakness found in scope |
| toy-k1-n35 (control) | Weil/GHS descent | 1 | verified | 4.6 (`ghs:l=5`) | 0 / 1 / 0 | no weakness found in scope |

Figures: [cost_ratio.svg](cost_ratio.svg) (registered curves), [cost_ratio_toy.svg](cost_ratio_toy.svg)
(toy curves and subfield-subgroup controls), [cost_ratio_controls.svg](cost_ratio_controls.svg)
(the pairing-friendly positive and boundary controls), [pipeline.svg](pipeline.svg) (tool structure with the status counts of this
run). They are generated from the certificates by `transfer-audit report`; the SVG is the source and
the rendering. They were not rasterized or inspected by eye (no SVG rasterizer is installed:
`rsvg-convert` is absent and ImageMagick delegates to it). The v1 figures had a text-extent check
that found no overlapping or out-of-bounds labels; the regenerated v2 figures (the controls chart now
has 35 rows) were not re-checked.

Matched baselines (Pollard rho with an equivalence class of `c` elements, negation included:
`sqrt(pi r / (2c))` group operations of one group operation each). The cost model's reference
`sqrt(pi r / 4)` is the `c = 2` case, so a class enters as `sqrt(c / 2)`; the v1 run divided that
reference by `sqrt(c)` and counted negation twice, a baseline 0.5 bit too low for every curve
(section 10):

| curve | ICV1 slug | field bits | log2 r | cofactor | class c | log2 baseline (v1) |
|:--|:--|--:|--:|--:|--:|--:|
| sect113r1 | `icv1-f2m113-tm122610772499221213-97df4ac6` | 113 | 112.0 | 2 | 2 | 69.958 (69.458) |
| ECC2K-130 | `icv1-f2m131-tm22283658519494248867-115e0dc5` | 131 | 129.0 | 4 | 262 (negation, Frobenius) | 75.280 (74.780) |
| K-163 | `icv1-f2m163-tm4845466632539410776804317-e15e581e` | 163 | 162.0 | 2 | 326 | 92.122 (91.622) |
| B-163 | `icv1-f2m163-tm6224022517221266226059365-31d88d74` | 163 | 162.0 | 2 | 2 | 95.796 (95.296) |
| P-192 | `icv1-fp192-t31607402316713927207482677199-52e4af59` | 192 | 192.0 | 1 | 2 | 111.170 (110.670) |
| secp256k1 | `icv1-fp256-t432420386565659656852420866390673177327-76dadd18` | 256 | 256.0 | 1 | 6 (negation, `j = 0`) | 143.036 (142.536) |
| mnt/mnt1 (control) | `icv1-fp170-tm52213820118540299389901413-869d5d3c` | 169.3 | 155.4 | 15337 | 2 | 92.570 (92.070) |
| BLS12-381 (control) | `icv1-fp381-tm15132376222941642751-961de9ba` | 380.7 | 254.9 | 7.6e37 | 6 | 143.372 (142.872) |
| BN254 (boundary control) | `icv1-fp254-t129607518034317099905336561907183648775-8181e611` | 253.2 | 253.2 | 1 | 6 | 141.618 |
| BN318 (boundary control) | `icv1-fp318-t548079839685593379889100768354473324754975588407-f4d63804` | 317.2 | 317.2 | 1 | 6 | 174.111 |
| BN350 (boundary control) | `icv1-fp350-t35917316178020966140770026648761406645010658995732487-6e1654f7` | 349.2 | 349.2 | 1 | 6 | 190.331 |
| BN446 (boundary control) | `icv1-fp446-t10109980000181489923001201093401906549415298505746886291297633566727-278b8869` | 445.2 | 445.2 | 1 | 6 | 238.886 |
| BLS12-638 (boundary control) | `icv1-fp638-tm121684554101595739501484314722271-a81b2097` | 637.9 | 426.3 | 4.9e11 | 6 | 230.293 |
Toy curves are unregistered (no ICV1 slug), fully specified in their certificates: `y^2 + xy = x^3 +
a x^2 + 1` over `F_2[z]/(z^15+z+1)` (`a = 1` and `a = 0`), `z^21+z^2+1`, `z^35+z^2+1`, seed
`20261011`. Their prime subgroups are small (`r` = 211, 751, 71, 631; 6 to 10 bits) because the order
of a Koblitz curve over `F_(2^n)` is divisible by the orders over every subfield; they test the
mechanics, not the economics.

## 3. What was executed, what is arithmetic only, what was not evaluated

| item | status | where |
|:--|:--|:--|
| instance validation (field, curve, generator, `[r]G = O`, `r` prime, order consistency, two random points annihilated by `#E`) on all 17 curves (the 12 of the v1 run and the 5 boundary controls) | executed | `curve.instance_checks` of every certificate |
| trace descent `E(F_(2^n)) -> E(F_(2^l))`, `m = 1`: ECC2K-130 and K-163 at full size (131 and 163 Frobenius steps), toys at every tower | executed; `<G>` killed on both Koblitz curves, as the arithmetic `gcd(r, #E(F_2)) = 1` predicts | `ecc2k-130/ghs_l_1.json`, `k-163/ghs_l_1.json`, `toy-*/ghs_l_*.json` |
| subgroup checker on those maps: image order, kernel, additivity on 4 sample pairs, scalar equivariance on 3 scalars | executed | `subgroup_preservation.executed` |
| same-field genus-3 degree-3 cover, pullback of `G` into `Jac(H)(F_q)`, `[r]phi^*(G) = O`, `phi^*(G) != O`, push-pull `[3]` witness: sect113r1, ECC2K-130, K-163, B-163 at full size, 4 toys | executed, `verified` on all 8 | `*/cover_binary_g3_deg3.json` |
| explicit degree-5 Velu isogenies of sect113r1 (replay of `sect113r1_audit::run_audit` in-process) | executed, `verified` | `sect113r1/isogeny_explicit_degree5.json` |
| GHS magic number, type, genus, condition (8), Weil capacity, achievable magic numbers per tower | derived (exact) | `ghs_l_*.json`, `ghs_proper_subfield.json` |
| GHS descent with `m >= 2`: sect113r1 `l = 1` (genus `2^112`), B-163 `l = 1` | arithmetic only; map construction not evaluated (section 7) | `sect113r1/ghs_l_1.json`, `b-163/ghs_l_1.json` |
| prime-field genus-2 cover `v^2 = F(u^2 + c)`: P-192, secp256k1 | arithmetic only: `phi_* phi^* = [2]`, `gcd(2, r) = 1`; Jacobian arithmetic for the two-infinity model does not exist | `p-192/cover_prime_g2_deg2.json` |
| pairing reduction: all curves | arithmetic only (Frey-Ruck non-degeneracy); no generic pairing in the repository | `*/pairing_tate_mov_frey_ruck.json` |
| isogeny class invariants, volcano data, GHS isogeny extension | derived / heuristic; no destination representative exhibited | `*/isogeny_*.json` |
| `f_E` | `End(E) = O_K` is **proved** (trait record status `proved`) for sect113r1, ECC2K-130, K-163 and B-163, so `f_E = 1` and an isogeny cannot lower it; for P-192 and secp256k1 the record is only `probable` (exact if every Miller-Rabin probable prime in the discriminant's factorization is prime), so the check reads `INDETERMINATE` and the statement is conditional (v1 read `PASS` for all six, and this row said "established for all six") | `isogeny_class_invariants.json`, check `end_ring_conductor_f_E`, invariant `end_ring_status` |
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
priced GHS route through `F_2` at that genus is 56.5 bits above the baseline.

**F2, route totals vs `docs/curves/jacobian-transfers.json` (PASS, 15 of 15 within 0.002 bits; a
wiring check).** The GHS rows with `m >= 2` (sect113r1 `l = 1`: 323.598 on both sides; B-163
`l = 1`: 465.948) and the same-field cover rows (thirteen curves: the eight of the v1 run and the five
boundary controls) agree to the printed precision; the check is attached to each certificate as
`invariants.crosscheck_jacobian_transfers`. It is a wiring check, not independent validation: the
tool calls the repository's own cost functions, so agreement shows the inputs reached them, not that
the formulas are right. It compares the frozen-model total (`costs.log2_total_frozen_model`, present
where the tool discounts rho on the cover image by the Frobenius class); for K-163 and ECC2K-130 the
reported total is lower than that by 3.674 and 3.516 bits (`0.5 log2 163` and `0.5 log2 131`).

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
cheaper: the destination solve is 2^6.3 to 2^7.4 bit operations, but the transfer evaluation
(2^11.1 to 2^14.3) and the one verifying scalar multiplication (2^11.6 to 2^13.8) alone exceed the
whole matched rho (2^8.8 to 2^10.4), so the ratios are 3.5 to 4.6 bits above the baseline. Structure
exists; there is no speedup, not even a constant-factor one, at toy size. The exponent is unchanged by construction
(same `r'`, destination an elliptic curve).

**F7, binary genus-3 cover (PASS, executed).** On all eight binary curves `phi^*(G)` has exact order
`r` in `Jac(H)(F_q)` (image order equals `r`, kernel trivial), 4 of 4 additivity samples and 3 of 3
scalar-equivariance samples pass, and the push-pull `[3]` witness holds. For sect113r1 (113-bit
field, 112-bit `r`) this is a verified subgroup-preserving map into a genus-3 Jacobian; the best
destination solver on it is rho on the image (priced 5.1 bits above the baseline: one Jacobian
operation costs about 34 elliptic ones), while double-large-prime index calculus on the Jacobian
costs `2^133.8` (genus 2) to `2^225.9` (genus 8) against `2^74.0`..`2^78.0` for rho on the image.

**F8, genus-`g` covers (PASS, modeled).** For every curve and every `g` in 2..8 the leading term
`(2 - 2/g) log2 q` exceeds `0.5 log2 r`, the index-calculus total exceeds the baseline, and the best
priced route exceeds it. This reproduces `IDEA-20261009-275a14` (cover attacks cost `p^(2-2/g) >= p`,
worse than rho) under the frozen model and is not a new result. For P-192: index calculus at genus 2
costs `2^214.0` against `2^115.2` for rho on the image and `2^111.2` for the baseline.

**Pairing rows (modeled).** The embedding degree is exact on all audited curves (the repository's
committed factor hints complete the factorization of `r - 1`): for sect113r1
`k = (r - 1)/2 = 2596148429267413844917941289415351`; for secp256k1 and P-192 it is also of the size of
`r`. A single element of `F_(q^k)` has `10^35` bits or more, so no pairing can be evaluated (check
`pairing_target_element_storable` = `FAIL`), and the priced route is 173 to 399 bits above the
baseline at both ends of the band (the generic rho term in the subgroup of `F_(q^k)^*` is the cheaper
of the two, so the ends coincide).

## 5. End-to-end controls (F10, F13), boundary controls and the registry calibration sweep

* **F10, synthetic (PASS, through the production decision path).** A genus-4 descent over
  `F_(2^40)` for a 158-bit subgroup of a 160-bit-field curve is classed `formula_advantage` and
  returns `no weakness found in scope` (no map, no subgroup certificate: a lead, not a weakness),
  and the same priced route with the map and the subgroup granted as a counterfactual returns
  `source-transfer-weak`. v1 classified on `repo_ratio_log2` in a unit test and on
  `log2_cost_ratio` in the control, and neither called `Ctx::finish` or `decide`. The control now
  builds a `Spec` and calls `ctx::judge`, the function `Ctx::finish` uses for every certificate; a
  negative control (the same route over `F_(2^80)`, where index calculus loses to rho) is a unit
  test (`controls/synthetic_formula_advantage.json`, `controls.rs`).
* **F13, `mnt/mnt1` (embedding degree 3, 155-bit subgroup, 170-bit prime) returns `class-weak`
  (as amended: expectation set in Amendment 2, after a scratch run of the suite).** Pairing route:
  conservative price `2^77.9` (generic NFS constant, weighted by field-multiplication cost) against a
  baseline of `2^92.6`, i.e. 14.6 bits below it (past the 8-bit margin); the attacker-favourable end
  is 42.0 bits below. Subgroup status `arithmetic` (Frey-Ruck), map basis `Theorem`, embedding degree
  exact, one element of `F_(p^3)` has 508 bits. The curve is an MNT example designed so that the
  pairing is the attack (registry adoption status `not_asserted`); the verdict says the pairing route
  is priced below rho under this model, not that the curve is broken at any absolute level. Its trait
  record was read from the committed `docs/curves/traits.json` because a live
  `curve_traits::compute` of this curve takes about six minutes (not in this tool's code).
* **F13, BLS12-381 (`k = 12`) returns `no weakness found in scope` with a marginal lead
  (as amended: Amendment 2, after a scratch run).** The price band is `2^130.0` (favourable) to
  `2^146.5` (conservative) against `2^143.4`: it straddles rho, so the formula band cannot decide
  (`evaluation`: evaluated; `computational_advantage`: `unknown`, where v1 said `refuted`). The
  conservative end is the one allowed to support a weakness; refining that estimate (tower-NFS and
  exact constants) is an open obligation, not a finding. **The rule as originally written (section
  5D, the attacker-favourable end against the baseline) would have made BLS12-381 `class-weak`:
  `2^130.0` against `2^142.9` in the v1 run, `2^143.4` now.** Amendment 2 moved the verdict to the
  conservative end after the scratch run had shown that; it was not the rule pre-registered.

### What the pairing verdict means, and what it does not

`class-weak` on a pairing row says: *the conservative price of a discrete logarithm in `F_(q^k)`
under the frozen formula model (generic NFS constant `(64/9)^(1/3)`, `o(1)` dropped, weighted by the
field-multiplication cost) is below the matched rho on `<G>` by at least the model's 8-bit margin.*
It is a statement **relative to rho on E**, not a statement of absolute security. The conservative
price for BN638 is `2^223.1` against `2^335.7` for rho on its 637-bit subgroup (112.6 bits below), and
for BN446 `2^194.4` against `2^238.9`. Both are far above 2^128: these curves are "weak" only in the
relative sense, because their subgroup is much larger than the field-extension logarithm needs. BN446,
the registry curve nearest to BN462, stays `class-weak` for that reason. Conversely
`no weakness found in scope` on a pairing row does not say the curve is strong: the price is an
`L[1/3]` formula with `o(1)` dropped, "can move it by many bits" (`cost.rs`), and no pairing is
evaluated. The certificate says so in its `notes`.

### Boundary controls and the calibration sweep (Amendment 3)

In v1 the pairing verdict was decided by the sign of the price difference: BN318 read `class-weak` on
a `-2.987`-bit difference of an `L[1/3]` formula with `o(1)` dropped. `decide` now requires the
difference to reach the model's own margin (`CostModel::advantage_margin_bits`, 8 bits, the
`formula_advantage` threshold the lead class already used); a price inside the margin stays
`no weakness found in scope` with the lead recorded as `formula_marginal`. The suite audits boundary
controls on both sides of both lines: BN254 (price above rho), BN318 (the v1 flip, -3.5 bits),
BLS12-638 (-7.1 bits, just inside the margin), BN350 (-13.3, the smallest curve past it) and BN446.

`transfer-audit sweep` audits every prime-field registry curve that has a generator and an embedding
degree of at most 64 (trait records from the committed `docs/curves/traits.json`). It found **29**
curves (the review reported 34; the difference of five is unexplained and no further curve with
`k <= 64` and a generator was found). The v1 tool called 15 of them `class-weak`
(`mnt/mnt1` and 14 BN/BLS curves, margins `-3.0` to `-112.1`); the margin rule leaves **13**, pinned
by the unit test `suite::tests::registry_sweep_pins_the_class_weak_set`:

| curve (registry alias) | k | field bits | log2 r | conservative log2(route/rho) | favourable end | lead class | verdict |
|:--|--:|--:|--:|--:|--:|:--|:--|
| bn/bn158 | 12 | 157 | 157.2 | 3.2 | -2.9 | formula_marginal | no weakness found in scope |
| mnt/mnt1 | 3 | 169 | 155.4 | -14.6 | -42.0 | formula_advantage | class-weak |
| bn/bn190 | 12 | 189 | 189.2 | 3.2 | -12.0 | formula_marginal | no weakness found in scope |
| bn/bn222 | 12 | 221 | 221.2 | 3.2 | -21.7 | formula_marginal | no weakness found in scope |
| other/Fp224BN | 12 | 224 | 224.0 | 3.2 | -22.6 | formula_marginal | no weakness found in scope |
| other/Fp254BNa | 12 | 253 | 253.1 | 3.2 | -32.0 | formula_marginal | no weakness found in scope |
| bn/bn254 | 12 | 253 | 253.2 | 3.2 | -32.0 | formula_marginal | no weakness found in scope |
| other/Fp256BN | 12 | 256 | 256.0 | 3.2 | -32.9 | formula_marginal | no weakness found in scope |
| bn/bn286 | 12 | 285 | 285.2 | 3.2 | -42.6 | formula_marginal | no weakness found in scope |
| bn/bn318 | 12 | 317 | 317.2 | -3.5 | -53.6 | formula_marginal | no weakness found in scope |
| bn/bn350 | 12 | 349 | 349.2 | -13.3 | -64.9 | formula_advantage | class-weak |
| bls/BLS12-377 | 12 | 377 | 252.2 | 3.2 | -12.6 | formula_marginal | no weakness found in scope |
| bls/BLS12-381 | 12 | 381 | 254.9 | 3.2 | -13.4 | formula_marginal | no weakness found in scope |
| bn/bn382 | 12 | 381 | 381.2 | -23.4 | -76.5 | formula_advantage | class-weak |
| other/Fp384BN | 12 | 384 | 384.0 | -24.3 | -77.5 | formula_advantage | class-weak |
| bn/bn414 | 12 | 413 | 413.2 | -33.8 | -88.2 | formula_advantage | class-weak |
| bn/bn446 | 12 | 445 | 445.2 | -44.5 | -100.2 | formula_advantage | class-weak |
| bls/BLS12-446 | 12 | 446 | 298.3 | 3.2 | -26.7 | formula_marginal | no weakness found in scope |
| bls/BLS12-455 | 12 | 454 | 304.0 | 3.2 | -28.4 | formula_marginal | no weakness found in scope |
| bls/BLS24-477 | 24 | 476 | 382.5 | 4.7 | -18.0 | formula_marginal | no weakness found in scope |
| bn/bn478 | 12 | 477 | 477.2 | -55.4 | -112.3 | formula_advantage | class-weak |
| bn/bn510 | 12 | 509 | 509.2 | -66.5 | -124.5 | formula_advantage | class-weak |
| other/Fp512BN | 12 | 512 | 512.0 | -67.5 | -125.6 | formula_advantage | class-weak |
| bn/bn542 | 12 | 541 | 541.2 | -77.8 | -136.9 | formula_advantage | class-weak |
| bn/bn574 | 12 | 573 | 573.2 | -89.2 | -149.5 | formula_advantage | class-weak |
| bls/BLS48-581-G1 | 48 | 580 | 517.2 | 6.3 | -3.7 | formula_marginal | no weakness found in scope |
| bn/bn606 | 12 | 605 | 605.2 | -100.9 | -162.1 | formula_advantage | class-weak |
| bn/bn638 | 12 | 637 | 637.2 | -112.6 | -174.9 | formula_advantage | class-weak |
| bls/BLS12-638 | 12 | 638 | 426.3 | -7.1 | -69.4 | formula_marginal | no weakness found in scope |

The ratio is `log2(pairing route / matched rho)`; the favourable end is the same with the smallest
published `L[1/3]` constant, unweighted. The v1 rule called `class-weak` every row with a negative
conservative ratio: the 13 above plus BN318 (`-3.5` now, `-3.0` in v1) and BLS12-638 (`-7.1`
now, `-6.6` in v1). The registry sweep is a calibration of this tool's verdict against the registry,
not a security assessment of any listed curve.

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

**Design note on the threshold (added in the post-merge review; the recorded result is unchanged).**
The frozen model's genus-2 total is *exactly linear* in the field size: relation collection `g! q = 2q`
plus linear algebra `g^2 q^(2 - 2/g) = 4q` is `6q` Jacobian operations (`cost.rs`, `ic_phases`). The
log-log slope of `6q + c` for any additive overhead `c > 0` (walk precomputation, lower-order terms)
is `6q / (6q + c) < 1` at every finite `q`. A threshold of "fitted exponent `>= 1`" at genus 2 was
therefore nearly guaranteed to fail on the real measurements: a flaw in how the criterion was
designed, not evidence about the asymptotic exponent. At genus 3 the model total `6q + 9 q^(4/3)`
has asymptotic slope `4/3`, and overhead lowers the finite-range slope the same way. The sub-1
fits above are what that flaw predicts; they neither support nor undermine the statement the
threshold was meant to test, and F9(a) stays recorded as failed.

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
* **The tool cannot flag a GHS-weak curve (a design limit, stated here and in the output).** Every
  `m >= 2` candidate has map basis `Existence` and an unpriced construction (`ghs.rs`), so `decide`
  can never return a weakness for it, whatever the curve. The certificate's `evaluation` field now
  separates the two cases the verdict hides: `excluded` (the subgroup is not carried, or the priced
  lower bound is already above the baseline: sect113r1 `l = 1` at 253.6 bits and B-163 `l = 1` at
  370.2 bits above) and `not_evaluable` (the priced part is below the baseline, so a composite-degree
  route may be cheap and the repository cannot say). No audited curve reaches the second case: the
  registered binary curves have prime degree and the toys are Koblitz (`m = 1`); the case is covered
  by a unit test on a synthetic candidate (`ctx::tests`). The F10 positive control, which in v1
  bypassed `Ctx::finish` and `decide`, now runs through `ctx::judge` (section 5).
* **No PDF.** `report.pdf` was not built: Typst is not installed, and `pandoc` (3.1.3) has no PDF
  engine here (no LaTeX, `wkhtmltopdf` or `weasyprint`). The report is this Markdown with
  generated SVG figures and CSV; the PDF requirement of `research-visuals` is unmet, with the blocker
  stated, and no old PDF is presented.
* **No benchmark entry.** Nothing here measures how fast an algorithm runs or reports a speedup, so
  the `benchmarks/` copy rule does not apply.
* **No replay by a second implementation.** `transfer-audit check` regenerates the 123 deterministic
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
  `sqrt(pi r / 4)`. The two differ by exactly the negation map: `CostModel`'s figure is rho with
  negation (`c = 2`), `rho_reference`'s is without it. The tool follows `CostModel` and, since
  Amendment 3, counts an equivalence class `c` as `sqrt(c / 2)` against it (v1 used `sqrt(c)`, which
  counted negation twice).
* `curve_traits::compute` for `icv1-fp170-tm52213820118540299389901413-869d5d3c` (`mnt/mnt1`) took 360 s
  in one run (hints loaded); every other audited curve took under one second.

## 9. Reproduction

```sh
export CARGO_TARGET_DIR=/home/user/cargo-target      # any target directory
cargo test  --release --bin transfer-audit            # 43 tests
cargo build --release --bin transfer-audit
# v2 certificates (what is committed). The measured F9 records and run-log.json are the v1 run and
# are not regenerated: omit --measure-ic, and restore run-log.json (`git checkout -- .../run-log.json`).
./target/release/transfer-audit suite  --out research/transfer_audit_20261011/certificates
./target/release/transfer-audit check  --certs research/transfer_audit_20261011/certificates   # 123 files, byte-identical
./target/release/transfer-audit report --certs research/transfer_audit_20261011/certificates --out research/transfer_audit_20261011
./target/release/transfer-audit sweep                         # the 29-curve calibration sweep (section 5)
./target/release/transfer-audit audit  --curve sect113r1     # one curve, summary on stdout
./target/release/transfer-audit presets
```

The v1 run (`--measure-ic --measure-ic-extended`, committed first) used the toolchain `cargo 1.97.0`
on a 4-core host; the lib release build took 12 minutes wall (23.5 CPU-minutes), the suite 3 s
without the measured controls and 20 s plus 68 s for the protocol and extended F9 plans
(`certificates/run-log.json`). Total compute for the study, builds included, was roughly 50
CPU-minutes. The v2 regeneration (this revision) took 8 s for `suite`, 5 s for `sweep`, and a 13-minute
library rebuild.

## 10. Corrections after review (Amendment 3)

Everything in this section is a change to this study after the independent review of the merged tool
(PR #2594). Nothing sealed was altered: the F9 records and `run-log.json` are untouched, the v1
certificates are in git history, and the F9(a) result is unchanged.

| # | review finding | change | effect on the recorded numbers |
|:--|:--|:--|:--|
| 1 | `transfer-audit.yml` could not pass the 25-minute routine PR budget (`timeout-minutes` 30); the path filter listed `Cargo.toml`/`Cargo.lock` and omitted direct dependencies | timeout expression `25` on pull requests (`45` otherwise), as in `curve-traits.yml`; the one real PR run took 14 min 5 s; filter drops `Cargo.*`, adds `src/ecc`, `src/hash/sha256.rs`, `bls12_381`, `binary_semaev`, `isogeny_degree_search.rs`, `binary_velu`, the `curve-traits.yml` list and others | `check-pr-budgets.py` no longer reports `transfer-audit.yml`; none |
| 2 | `class-weak` decided by the sign of a formula difference: 15 of 29 swept curves, BN318 on `-2.987` bits | weakness verdicts need the price below the baseline by the model margin (8 bits); boundary controls; `sweep` and a pinned regression test; the meaning of the verdict stated (section 5) | 13 of 29 `class-weak`; BN318 and BLS12-638 become marginal leads |
| 3 | `computational_advantage = "refuted"` for identity-path rows, formula sweeps and every `carried = false` row | `judge`: `refuted` only for a carried, priced route whose total (a lower bound where phases are unpriced) is not below the baseline; `not_applicable` when the subgroup is not carried; `unknown` otherwise; a straddling pairing band is `unknown` | v1 `refuted` on 12 `class_invariants`, 12 `genus_sweep`, 2 `ghs_extension`, 1 `explicit_degree5` and BLS12-381's pairing; now `unknown` |
| 4 | the tool cannot flag a GHS-weak curve; the F10 control bypassed `Ctx::finish`/`decide`; the output did not distinguish excluded from not evaluable | `evaluation` field (`evaluated`/`excluded`/`not_evaluable`) on every certificate, counts in `summary.json` and the tables; F10 runs through `ctx::judge`, with a counterfactual and a negative control | none; documented in section 7 |
| 5a | negation counted twice in the baseline (`sqrt(pi r / 4)` divided by `sqrt(c)`) | class enters as `sqrt(c / 2)` (`cost::class_discount_bits`); the `GHS` trace destination and F10 use the same function | every baseline `+0.5` bit; every ratio that does not involve a cover image `-0.5` bit |
| 5b | rho on a Koblitz curve's cover image priced without the Frobenius class the baseline gets | `Route::image_class`: rho on the image gets the baseline's class when the cover is defined over the subfield that gives `E` its class (binary cover, and the genus sweep as a best case); `costs.log2_total_frozen_model` keeps the frozen-model total for the F2 cross-check | binary cover ratios K-163 `9.256` to `5.082`, ECC2K-130 `9.098` to `5.082`; B-163, sect113r1 `5.582` to `5.082`: the four now agree, as they should |
| 6a | the subgroup checker returned `Verified` on zero samples and `Refuted` on a map error | zero homomorphism tests or a map evaluation error give `not_evaluated`; a mismatch still refutes | none (every executed certificate has 4 pairs and 3 scalars) |
| 6b | `embedding_degree` ran before the `r \nmid q` antecedent (bogus `k = r - 1` for `r = q`) | antecedent first; an `r \| q` curve gets an unpriced `not_applicable` pairing certificate | none (no audited curve is anomalous); unit tests on `q = r` and on an anomalous toy curve |
| 6c | "`End(E) = O_K` established for all six" while P-192 and secp256k1 are only `probable` | `PASS` only for `proved`; `probable` reads `INDETERMINATE` with the caveat; invariant `end_ring_status` | P-192 and secp256k1: check `PASS` to `INDETERMINATE` |
| 6d | "as pre-registered" on F13 and the pairing band | "as amended (Amendment 2, after a scratch run)"; the original 5D rule would have made BLS12-381 `class-weak` | text only |
| 6e | `problems/ghs-higher-magic-descent-not-executable.md` said "three" and listed four | "four", with a dated history note | text only |
| 6f | F9(a) failure: threshold nearly guaranteed to fail | design note (section 6) | text only |

Not changed: the cost model itself (`CostModel`, including `LOG2_SQRT_PI_OVER_4`), the formulas of the
pairing price band, and the verdict vocabulary, which stays closed (`class-weak`,
`weak representative exists`, `source-transfer-weak`, `implementation-weak`,
`no weakness found in scope`).
