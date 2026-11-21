# All integer third-coefficient shifts exclude the coefficient-six rational relation

For the verified local bridge `P=(2,2)`, `Q=(-16,17)`, `k1=-3`, `k2=-11`, and **every integer** `k3=1+77t`, the exact rational-function numerator of `x(6Q(t))−x(P)` has no integer root. It is a primitive degree-24 polynomial with no root modulo 23. Hence no nonsingular rational curve in this fixed one-parameter family satisfies `P+6Q=O`. The source curve at `t=0` has the two local coefficient-six relations recorded in the [bridge packet](../xedni_bridge6_aux8_20261120/README.md); varying `k3` by 77 preserves its reductions at 7 and 11 because the source x-coordinate difference is 18, coprime to both primes.

| Exact result | Value |
| --- | ---: |
| Generic numerator degree | 24 |
| Generic denominator degree | 22 |
| First rootless prime in the frozen search | 23 |
| Integer shifts covered | All `t∈ℤ` with fixed pair, `k1=-3`, `k2=-11` |
| Unresolved certificate status | 0 |

The [protocol](protocol.json) was committed before execution as `193673973c0`. The [checked Sage runner](run_sage.py) constructs the curve over `ℚ(t)`, verifies generic nonsingularity and point membership, computes `x(6Q(t))−x(P)`, cancels the rational function, and stores primitive integer numerator and denominator coefficient lists in [polynomial.jsonl](polynomial.jsonl). It searches primes 13 through 257 in order for a rootless modulus. The [result](result.json) pins the polynomial SHA-256 and source bridge digest; the [runtime receipt](sage-runtime-info.json) records the accepted build. The [independent SymPy audit](audit.py) reconstructs the general Weierstrass group law, matches every numerator and denominator coefficient, verifies earlier searched primes had roots, checks all 23 residues, and binds the exact coefficient list to the Lean module. Its [result](audit-result.json) is `PASS`, with empty stderr.

## Proof and formalization map

If a nonsingular specialization satisfied `P+6Q=O`, then `6Q=-P`, so `x(6Q)=x(P)`. The latter is a necessary equation represented by the generic rational function. The primitive numerator `N(t)` therefore must vanish at such an integer specialization. The stored polynomial has no zero in `F₂₃`, so it has no integer zero. This argument excludes rational relations throughout the declared family, including shifts outside any finite height box.

| Result | Lean declaration or exact receipt | Exact scope | Status |
| --- | --- | --- | --- |
| Concrete polynomial has no root modulo 23 | `XedniBridge6K3.concrete_rootless` in [Polynomial.lean](formal/Polynomial.lean) | Exact 25 stored integer coefficients, all `z:ZMod 23` | Lean checked |
| No integer root of the concrete polynomial | `XedniBridge6K3.no_integer_root` | Same coefficient list, every `t:ℤ` | Lean checked |
| Concrete curve gives the stored numerator | [Sage result](result.json), [SymPy audit](audit-result.json) | Generic curve over `ℚ(t)`, exact fixed pair and coefficient shifts | Independent symbolic replay |
| Specialization from point-group relation to polynomial root | Written proof above; formal rational-function specialization and elliptic-section binding remain open | Nonsingular integer specializations | Lean formalization partial |

The [Lean build](formal/build.sh) pins `v4.35.0-rc4` and Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9`. Its [output](formal/build-output.txt) reports `propext`, `Classical.choice`, and `Quot.sound` under `#print axioms`; no reported theorem uses `sorry`, `admit`, a custom claim axiom, or native evaluation. `lean4checker --fresh` is unavailable on this host.

This certificate fixes the source pair and `k1,k2`. Shifts in `k1` or `k2`, changed point lifts, and a different auxiliary policy change the generic polynomial and require separate checks. The next construction experiment should vary those remaining representatives under a frozen input-independent selection and cost rule, while retaining novel relation rank and target coverage if a rational bridge is found.

## Replay

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_bridge6_k3_polynomial_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python /Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119/research/exotic_ecdlp_20261021/xedni_bridge6_k3_polynomial_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_bridge6_k3_polynomial_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_bridge6_k3_polynomial_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_bridge6_k3_polynomial_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_bridge6_k3_polynomial_20261120/audit-result.json 2> research/exotic_ecdlp_20261021/xedni_bridge6_k3_polynomial_20261120/audit.stderr
sh research/exotic_ecdlp_20261021/xedni_bridge6_k3_polynomial_20261120/formal/build.sh > research/exotic_ecdlp_20261021/xedni_bridge6_k3_polynomial_20261120/formal/build-output.txt 2> research/exotic_ecdlp_20261021/xedni_bridge6_k3_polynomial_20261120/formal/build-stderr.txt
```
