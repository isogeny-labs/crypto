# Every selected coefficient-10 class rejects integer third-coefficient shifts

For the frozen locally selected coefficient-10 CRT stream, **all 32 classes compatible with `P+10Q=O` at the fixed `F₄₇` reduction have a rootless polynomial certificate**. In each class, hold `k1,k2` fixed, set `k3'=k3+77t`, reconstruct `a4,a6` from the two integer points, and allow **any integer `t`**. If a nonsingular rational specialization satisfied `P+10Q=O`, it would give an integer root of the exact numerator of `x(10Q)−x(P)` over `ℚ(t)`. Each of the 32 primitive integer numerators has no root modulo a recorded prime between 13 and 61, so that necessary equation has no integer solution in these classes.

| Stage | Exact count |
| --- | ---: |
| Prior locally selected centered CRT curves | 1,488 |
| Compatible with `P+10Q=O` at `F₄₇` | 32 |
| Generic third-coefficient polynomials | 32, each degree 66 |
| Rootless-modulus certificates | 32 |
| Unresolved / producer-failure cells in final result | 0 / 0 |

Certificate moduli are recorded per class in [polynomial rows](polynomial-rows.jsonl.gz): 13 for 10 classes, 17 for 12, and 23, 37, 43, 59, or 61 for the remaining 10. The [selection rows](selection.jsonl.gz) retain all 1,488 reduced-compatibility statuses, including the 1,456 incompatible rows. The selection is a **post-run diagnostic** based on reduced group work, not an uncharged unknown-target selection rule. The earlier [centered coefficient-10 census](../xedni_residue10_rational_20261120/README.md) already checked all 1,488 rational curves; this packet extends the 32 reduced-compatible fixed-`k1,k2` classes to every integer `k3` representative preserving their auxiliary residues.

## Proof and verification map

Let `F(t) = x(10Q(t))−x(P)` in the rational-function field of the generic reconstructed curve. The [checked Sage runner](run_sage.py) constructs that curve over `ℚ(t)`, checks generic nonsingularity, cancels the fraction, clears denominators from its numerator, and divides its coefficient content. For each primitive integer numerator `N(t)`, the runner searches primes 13 through 257 in order and records the first prime with `N(z)≠0` for every residue `z`. Every selected class received such a certificate by 61.

The [independent SymPy audit](audit.py) recomputes the full Weierstrass group law symbolically for 16 classes and matches the complete primitive numerator coefficient list. It verifies the other 16 through exact sign-mirror point and coefficient identities, including the polynomial transformation `N(t)↦N(−t)`. It separately replays all 1,488 reduced-compatibility decisions with a Python finite-field group law and verifies all 32 recorded modular certificates, including that each earlier searched prime had a root. The [audit result](audit-result.json) is `PASS`; both final stderr files are empty. The initial run hit a JSON serialization [producer failure](attempt1-producer-failure.json) after calculation; the same frozen protocol was rerun with an integer conversion, and its final result retains no failed scientific cell.

| Result | Lean declaration or receipt | Exact scope | Status |
| --- | --- | --- | --- |
| Rootless modulus implies no integer root | `XedniResidue10K3.no_integer_root_of_rootless_mod` | Any integer coefficient list, any modulus, every integer argument | Lean checked |
| Residue evaluation commutes with integer cast | `XedniResidue10K3.evalResidue_cast` | Same exact Horner representation | Lean checked |
| 32 concrete numerator identities and rootless moduli | [Sage result](result.json), [polynomial rows](polynomial-rows.jsonl.gz), [SymPy audit](audit-result.json) | Frozen 32 selected classes, shifts only in `k3` | Complete exact arithmetic replay |
| Full Lean binding from each generic elliptic section to its stored numerator | Requires formal rational-function specialization and the 32 symbolic `10Q` identities | Same classes | Formalization partial; the generic rootless implication is checked, while these concrete symbolic identities remain computational certificates |

The [protocol](protocol.json) fixed the input digests, post-run selection, coefficient shift, necessary polynomial, prime search cap, status rules, and claim boundary before execution. The [result](result.json) includes raw and packed hashes for both streams. The [runtime receipt](sage-runtime-info.json) records the accepted local Sage build. The [Lean build](formal/build.sh) pins `v4.35.0-rc4` and Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9`; [output](formal/build-output.txt) lists only `propext` and `Quot.sound` under `#print axioms`. No reported theorem uses `sorry`, `admit`, a custom claim axiom, or native evaluation. `lean4checker --fresh` is unavailable on this host.

The next algebraic screen should vary `k1,k2` within the same local residue class or change the auxiliary-prime policy. Those variations change the generic numerator and need a new frozen certificate and cost record. A complete unknown-target pipeline also needs relation selection, novel rank, descent, and same-point rho timing under the repository measurement contract.

## Replay

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_residue10_k3_polynomial_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python /Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119/research/exotic_ecdlp_20261021/xedni_residue10_k3_polynomial_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_residue10_k3_polynomial_20261120/result.json
python3 research/exotic_ecdlp_20261021/xedni_residue10_k3_polynomial_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_residue10_k3_polynomial_20261120/audit-result.json
sh research/exotic_ecdlp_20261021/xedni_residue10_k3_polynomial_20261120/formal/build.sh
```
