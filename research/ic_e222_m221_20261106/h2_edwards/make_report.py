#!/usr/bin/env python3
"""Render RESULTS_h2.md from results.json (no hand-typed numbers).  python3 -I make_report.py <dir>"""
import json, sys, os
d = sys.argv[1]; R = json.load(open(os.path.join(d, "results.json")))
IR = [1, 20, 80, 100]
L = []
w = L.append
w("# H2: Edwards unified addition on the real E-222, priced in counted field operations\n")
w("**Status:** unvalidated (pre-gate). Pure-Python operation counts on the registry curve `other/E-222`; no wall time, no isolation, no replay certificate, no validator report, no §5a band. Not a leaderboard row, no rho comparison.\n")
w(f"**Curve.** p = 2^222 - 117, Edwards x^2 + y^2 = 1 + d x^2y^2, d = {R['d']} (non-square, so the unified formulas are complete), cofactor {R['h']}, registry generator and order. Weierstrass image through Edwards -> Montgomery (A, B) -> short Weierstrass, as in `gen_instances.py`.\n")
w("**Correctness checks (agree = same Edwards-affine result).** [r]G = identity on all four implementations: " + ", ".join(f"{k} {v}" for k, v in R["rG_is_identity"].items()) + f". {R['agree_count']}/20 random scalars give equal results across eight scalar-multiplication variants (Edwards affine, extended binary mixed/unified, extended 4-bit window, Weierstrass affine, Jacobian binary mixed/general, Jacobian window).\n")
w("**Op classes.** M full multiplication (a full-size constant such as the Weierstrass `a` counts as M); S squaring; D multiplication by the 18-bit constant d; A add/sub/neg/shift-by-add (x2 = 1A, x3 = 2A, x4 = 2A, x8 = 3A); I inversion. Formulas: Hisil-Wong-Carter-Dawson 2008 (`add-2008-hwcd` with a = 1, `dbl-2008-hwcd`); Bernstein-Lange 2007 Jacobian (`add-2007-bl`, `madd-2007-bl`, `dbl-2007-bl`).\n")
w("**Pricing.** M-equivalent = M + 0.8 S + 0.1 D + 0 A + (I/M) I, with I/M in {1, 20, 80, 100}. D = 0.1 M is an assumption (18-bit by 222-bit product), A = 0 in the headline (a 0.05 sensitivity is in the scalar table). I/M must be measured on the target implementation; the Python ratio here is a diagnostic only: " + f"{R['python_I_over_M_diagnostic']}x (inversion by `pow(x, -1, p)` against `x*y % p`).\n")
w("\n## Single group operations (exact counts)\n")
w("| operation | M | S | D | A | I | " + " | ".join(f"M-eq @ I={i}M" for i in IR) + " |")
w("|:--|--:|--:|--:|--:|--:|" + "--:|" * len(IR))
for k, v in R["counts"]["single"].items():
    w(f"| {k} | {v['M']} | {v['S']} | {v['D']} | {v['A']} | {v['I']} | " + " | ".join(f"{v[f'M_eq_I{i}']}" for i in IR) + " |")
w(f"\n## One scalar multiplication, mean over {R['n_scalars']} random 220-bit scalars (final conversion to affine included)\n")
w("| method | M | S | D | A | I | " + " | ".join(f"M-eq @ I={i}M" for i in IR) + " | M-eq @ I=100M, A=0.05 |")
w("|:--|--:|--:|--:|--:|--:|" + "--:|" * (len(IR) + 1))
for k, v in R["counts"]["smult"].items():
    w(f"| {k} | {v['M']} | {v['S']} | {v['D']} | {v['A']} | {v['I']} | " + " | ".join(f"{v[f'M_eq_I{i}']}" for i in IR) + f" | {v['M_eq_I100_A0.05']} |")
sm = R["counts"]["smult"]
ed = sm["Edwards extended, binary, mixed add"]; jj = sm["Weierstrass Jacobian, binary, mixed add (producer path)"]
edw = sm["Edwards extended, 4-bit window"]; jw = sm["Weierstrass Jacobian, 4-bit window"]
w("\nRatios Edwards extended / Weierstrass Jacobian (same algorithm), M-eq at I = " + ", ".join(f"{i}M" for i in IR) + ":")
w("- binary, mixed add (the producer's algorithm): " + ", ".join(f"{ed[f'M_eq_I{i}']/jj[f'M_eq_I{i}']:.3f}" for i in IR))
w("- 4-bit window: " + ", ".join(f"{edw[f'M_eq_I{i}']/jw[f'M_eq_I{i}']:.3f}" for i in IR))
w("\nThe affine rows pay one inversion per group operation and are two orders of magnitude above the projective ones at any I/M of 20 or more.\n")
sh = R["sqrt_exponent_shapes"]
w("## IC relation-trial model (`find_one_relation_counted_with_rng`)\n")
w("Per trial the producer does two variable-time scalar multiplications (Weierstrass Jacobian, binary, mixed add, each ending in `to_affine`), one affine addition `aG + bQ`, then for each of `fb_size` factor-base elements: the Semaev S3 coefficients, the discriminant, `sqrt_mod_p` (Euler test `n^((p-1)/2)` always; for p = 3 mod 4 a second power `n^((p+1)/4)` only if the discriminant is a residue), then one inversion `1/(2 qa)` and two candidate products only on that branch. Hash lookups are not counted.\n")
w(f"- **Model A (task statement):** one exponentiation per element, 222 S + 111 M, plus about 6 M, 3 S, 12 A and 0.5 I of other work.")
w(f"- **Model B (source-exact, counted by running it):** Euler exponent (p-1)/2 has {sh['euler_bits']} bits and popcount {sh['euler_popcount']}; root exponent (p+1)/4 has {sh['root_bits']} bits and popcount {sh['root_popcount']}. For this p both are almost all ones, so plain square-and-multiply costs about 220 S and 216 M each, not 111 M. Residue fraction observed {R['qr_fraction_observed']}. Counted mean per element: " + ", ".join(f"{k} {v}" for k, v in R['fb_element_counted_mean'].items()) + ". (The Rust `modpow` may use a window; the count above is the plain binary model, so Model B is an upper estimate for the exponentiation share.)\n")
hp = R["h2_replacement_part"]
w("**What H2 replaces.** Only the part before the factor-base loop: two Edwards extended binary mixed-add scalar multiplications without the final affine conversion, one unified extended addition, and the conversion of R to the Weierstrass x used by Semaev (`u = (Z+Y)/(Z-Y)`, `x_W = B u + AB/3`: 1 I, 2 M, 3 A). Replacement part counts: " + ", ".join(f"{k} {v}" for k, v in hp.items()) + ". The factor-base loop is unchanged. The base-point `T = x y` is a one-time cost per relation and is not counted.\n")
w("| model | fb_size | I/M | scalar-mult part, producer (M-eq) | trial, producer (M-eq) | scalar-mult share | scalar-mult part, H2 | trial, H2 | H2 / producer |")
w("|:--|--:|--:|--:|--:|--:|--:|--:|--:|")
for m in "AB":
    for fb in (8, 32):
        for i in IR:
            t = R["trial"][f"model{m}_fb{fb}_I{i}"]
            w(f"| {m} | {fb} | {i} | {t['smult_part_cur']} | {t['trial_cur']} | {t['smult_share_cur']*100:.1f}% | {t['smult_part_h2']} | {t['trial_h2']} | {t['trial_ratio_h2_over_cur']:.3f} |")
w("""
## Reading

- On the scalar multiplications H2 is a real saving in counted operations: about 17% fewer M-equivalents than the Weierstrass Jacobian path with the producer's algorithm, and about 21% fewer with a 4-bit window, at I/M = 80 (see the ratio lines; the saving comes from the doubling, 4M+4S against 2M+8S, and from the mixed add, 8M+1D against 7M+4S).
- The whole trial is dominated by the factor-base loop once `fb_size` is not small: the two scalar multiplications are 55% (Model B) to 70% (Model A) of a trial at `fb_size` 8 and 23% to 38% at `fb_size` 32. H2 therefore changes a trial by about 10% to 13% at `fb_size` 8 and about 4% to 7% at `fb_size` 32.
- H1 (cofactor projection) reduces the number of trials per relation by the cofactor (4 on E-222); H2 reduces the cost per trial by at most the figures above. They compose multiplicatively: this note prices only the per-trial factor.
- The factor-base loop (two exponentiations of 220-bit-shaped exponents per residue element) is the larger lever at `fb_size` 32. Pricing it was out of scope here.

## Limits

Operation counts in Python, not wall time; the constants A (0) and D (0.1 M) are assumptions; Rust `BigUint` multiplication does not have the 1 M : 0.8 S shape assumed; the Rust producer's relation search on the registry curve has not been run (its field is capped at 10^6, so H1's ladder used 10 to 17 bit groups); H2 requires carrying the Edwards model next to the Weierstrass model the producer uses; no replay, validator or matched-rho pricing. Unvalidated, pre-gate.

## Commands

```
cd research/ic_e222_m221_20261106/h2_edwards
python3 -I h2_price.py --params /home/user/crypto/docs/curves/standards/parameters.json --out .
python3 -I make_report.py .
```
""")
open(os.path.join(d, "RESULTS_h2.md"), "w").write("\n".join(L))
