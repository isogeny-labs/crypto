# Quadratic-twist extension of the odd-five degree-seven cover

16 November 2026 UTC. The exact equations and saved receipts below are a second trace-sign orbit obtained by quadratic twisting the previously verified degree-seven cover. This is a **new explicit map in this project**, but quadratic twisting is a standard operation and does not establish a new attack method or an independent moduli family. The (s=3) benchmark favors matched rho; the (s=7) prime subgroup has a verified map but no complete attack-cost measurement. Finite claims remain [pcc:unverified] until PCC admission.

## Exact construction and rigorous scope

Let (K=F_3[a]/(a^5+2a+1)), (b=a+a^3), and

```text
 f(x)=x^{11}+x^9+2x^8+x^7+2x^5+x^3+x^2+x+1.
```

The old cover has (Y:y^2=f(x)), (E:v^2=u^3+2u^2+b), and (u=A/B,v=yC/B^2), where the full degree-seven (A,B,C∈ K[x]) are in `../ghs_odd5_carrier_20261115/degree7_quadratic_square_30.json`. Its exact identity is (fC^2=B(A^3+2A^2B+bB^3)). Define

```text
 Y^-: y'^2 = 2f(x);  E^-: v'^2 = u'^3 + u'^2 + 2b.
```

Then (u'=2A/B, v'=y'C/B^2) is a separable degree-seven (K)-map (Y^-_K→ E^-), because (2fC^2=B((2A)^3+(2A)^2B+2bB^3)) in characteristic three. `verify_twist_s7.sage` checks both identities exactly, including the pole-fiber correction in divisor pullback. (E^-(K)) has order (270=2·3^3·5), trace (-26); (Y^-) has genus five and Weil polynomial (X^{10}+26X^5+3^5). The latter is the negative-trace class already identified abstractly in the earlier carrier census.

For every (s≥1) coprime to five, base change gives the same degree-seven map over (K_s=F_{3^{5s}}) with carrier over (k_s=F_{3^s}). The twist of the previously established isogeny (Res_{K/F_3} E→ J(Y)) gives an isogeny (Res_{K/F_3} E^-→ J(Y^-)). Base change preserves it, and the coprime-to-five linear-disjointness identification gives the (K_s/k_s) transfer. Thus the **genus-five algebraic-transfer floor** from the irreducible five-dimensional isogeny class is retained for this twist sequence. A prime-order subgroup is preserved whenever its prime avoids the fixed isogeny degree; the explicit (s=7) evaluation below verifies one such subgroup directly. The available degree-at-most-six exclusion was over the untwisted (K)-models; it is not asserted as a minimum-cover-degree theorem for these twisted models.

For odd (s), this is the negative-trace twist sequence; for even (s), the twist becomes trivial over (k_s). These are field-size base changes of one twist orbit, not independent parameters. The five conjugate positive masks yield five conjugate twisted equations by (b↦ b^3).

## Verified (s=7) subgroup map

At (s=7), (k_s=F_{3^7}), (K_s=F_{3^{35}}), and the exact modulus and embedding are saved in `twist_s7_transfer.json`. The source order is

```text
 #E^-(K_s)=50,031,544,838,921,370
              =270·185,302,017,921,931.
```

The last factor (r) is a checked prime. The saved (P) has order (r). The corrected pullback (D=phi^*(P)) and five-term (F_{3^7})-Frobenius trace (T(D)) both have order (r); (T(D)≠0), is fixed by (3^7)-Frobenius, and (T(phi^*(37P))=37T(D)). The receipt includes the source coordinates and Mumford representatives. An independent checker rebuilt the cover from saved coefficient vectors, matched the source point and divisor images, and repeated the order and 37-fold checks. The (s=7) transfer is verified, while relation collection, descent and same-target rho costs at this size remain unmeasured.

## Complete (s=3) run and matched rho

The (s=3) twist has (q=27), source order (14,347,530=810·17,713), and selected prime (r=17,713). `twist_s3_full_attack.json` saves the exact source point (P), validation target (Q=12345P), nonzero mapped points, factor base, 698 split relations from 39,000 attempts, modular system, target descent, and 30 rho solves. The relation matrix has 12 columns and rank 11. Its one-dimensional right kernel is saved; the target row annihilates that kernel. Hence every solution of the consistent relation equations gives the same target logarithm. The target descended after 11 offsets and the scalar 12,345 checks on both the source curve and the Jacobian. `audit_twist_s3_full_attack.py` independently reconstructed the twist cover and map, checked every recorded relation as a Jacobian group equality, recomputed rank 11, checked target-row orthogonality, and verified the scalar and 30 rho outputs.

| In-process CPU phase | Seconds |
|---|---:|
| GHS setup and factor base | 0.244649 |
| Pullback and trace | 0.077610 |
| Relations and modular linear algebra | 13.585796 |
| Target descent and verification | 0.007763 |
| **GHS total including setup** | **13.915818** |
| Standalone rho setup | 0.102620 |
| Median rho trial, 30 verified solves | 0.021492 |
| **Median rho total including setup** | **0.124112** |

The measured ratio is **112.12**, favoring rho. The rho code uses the checked order-three (F_{3^5})-Frobenius eigenvalue 6367 modulo (r) on the same saved (P,Q); median trial length was 85 steps. Both totals exclude Sage interpreter startup and include each algorithm's own in-process setup. The GHS run used the rank-11 target-row criterion; it did not determine all 12 individual factor-base logarithms, and the audit checks the sufficient group equations instead.

## Cost bound and prior-work status

For genus five, [Diem's theorem](https://arxiv.org/abs/math/0606607) gives expected Otilde(q^{8/5}) under its cyclicity or known-group-structure hypotheses, with constants and model costs omitted. A Frobenius-aware source rho scale is sqrt(r/s) when an order-(s) automorphism acts efficiently on the chosen subgroup. The bare exponent comparison is r^5>s^5q^{16}. `TwistArithmetic.lean` checks that integer inequality for (s=7), along with the trace-recurrence step and order factorization. At (s=7), (q^{8/5}≈220,677) small-field units and sqrt(r/7)≈5,145,067 source-group operations. These are different units and provide **no measured speedup or concrete upper bound** for this instance.

The [Iijima–Momose–Chao 2015 classification](https://eprint.iacr.org/2015/805.pdf), Section 5 Case 13, gives a degree-five genus-five direct (2,...,2) construction with four branch roots rational over the extension field. The present (E^-) has order (2 mod 4), hence only one nontrivial rational 2-torsion point, so this exact model lies outside that specified direct case. The paper does not classify all curve-to-elliptic maps; this comparison does not prove global priority. Quadratic twists and isogeny-assisted transfer are prior mechanisms. The (s=3) total is slower than rho, and (s=7) is a map-verified, cost-unresolved lead. One sentence in the older prior-work audit incorrectly says the positive-orbit (s=3) result was below rho; its detailed result file correctly reports rho ahead. This report records the correction while that source directory is held by another coordination attempt.

## Reproduction

Run `sage research/ghs_twist_extension_20261116/verify_twist_s7.sage`, `sage -python research/ghs_twist_extension_20261116/audit_twist_s7.py`, and `lean research/ghs_twist_extension_20261116/TwistArithmetic.lean`. The (s=3) producer refuses to overwrite its saved receipt; to reproduce it, work in a fresh copy of this directory, run `sage twist_s3_full_attack.sage`, then `sage twist_s3_rho_standalone.sage`, and run `sage -python audit_twist_s3_full_attack.py`. All five checked runs completed in this worktree. The source hashes in the JSON receipts bind the executable files.
