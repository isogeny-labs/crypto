# Twenty-five coefficient classes exclude the rational bridge for every third-coefficient shift

For the fixed point pair `P=(2,2)`, `Q=(-16,17)`, take every `u,v∈{-2,-1,0,1,2}` and set `k1=-3+77u`, `k2=-11+77v`, `k3=1+77t` for **any integer `t`**. All **25** generic rational curves have a primitive degree-24 necessary numerator for `P+6Q=O` with no root modulo a recorded prime between 13 and 199. Thus no nonsingular rational specialization in these 25 fixed-`k1,k2` classes satisfies the coefficient-six relation. All 25 centered curves retain `#E(F₇)=4`, `#E(F₁₁)=8`, and `P+6Q=O` at both auxiliary reductions; the integer shifts preserve those reductions because the point x-coordinate difference is 18, coprime to 7 and 11.

| Stage | Exact count |
| --- | ---: |
| Frozen `(u,v)` classes | 25 |
| Generic numerator / denominator degrees | 24 / 22 in every class |
| Rootless-modulus certificates | 25 |
| Unresolved / producer-failure cells | 0 / 0 |
| Centered rational coefficient-six relations | 0 |

Certificate primes by class are 13 (eight), 17 (four), 19 (two), 23 (four), 29 (one), 37 (two), 41 (one), 59 (one), 79 (one), and 199 (one). The [25 ordered rows](rows/) retain every `u,v`, centered local check, full numerator and denominator coefficient list, certificate prime, and status. Concatenating the numbered files reproduces the raw SHA-256 in [result.json](result.json). Each row is below the repository artifact threshold. The [protocol](protocol.json) fixed the 25-class grid, arbitrary integer `k3` shift, necessary polynomial, prime cap, and failure statuses before execution as `00c8efdf8b2`. The [runtime receipt](sage-runtime-info.json) records the accepted Sage build; the result is exact arithmetic rather than a timing comparison.

## Proof and verification map

For any nonsingular specialization with `P+6Q=O`, the finite point `6Q=-P` has `x(6Q)=x(P)`. The generic rational function `x(6Q(t))−x(P)` therefore yields a necessary polynomial equation `N_{u,v}(t)=0`. Every stored `N_{u,v}` has no root modulo its recorded prime, so it has no integer root. The two other coefficient shifts are bounded by the declared 25-class grid; the third shift is unbounded.

The [independent SymPy audit](audit.py) reconstructs all 25 full Weierstrass group laws and matches every numerator and denominator coefficient against the checked-Sage rows. It checks that every earlier prime in the frozen search had a root and that the recorded prime has none. It also verifies that the 25 exact integer coefficient lists in Lean match the stored rows. Its [result](audit-result.json) is `PASS`; run and audit stderr are empty.

| Result | Lean declaration or exact receipt | Hypotheses and scope | Status |
| --- | --- | --- | --- |
| Each concrete polynomial is rootless at its recorded prime | `XedniBridge6Grid.rootless000` through `rootless024` in [Grid.lean](formal/Grid.lean) | Exact 25 stored integer coefficient lists; respective `ZMod p` | Lean checked |
| No integer root in each class | `no_integer_root000` through `no_integer_root024` | Same lists; every `t:ℤ` | Lean checked |
| Generic curve-to-polynomial identities | [Sage result](result.json) and [SymPy audit](audit-result.json) | Fixed point pair, 25 coefficient classes over `ℚ(t)` | Independent symbolic replay |
| Specialization from `P+6Q=O` to polynomial root | Written proof above | Nonsingular integer specializations | Formal rational-function/elliptic-section binding remains open in Lean |

The [Lean build](formal/build.sh) pins `v4.35.0-rc4` and Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9`. Its [output](formal/build-output.txt) lists only `propext`, `Classical.choice`, and `Quot.sound` for all 50 declarations under `#print axioms`. None uses `sorry`, `admit`, a custom claim axiom, or native evaluation. `lean4checker --fresh` is unavailable on this host.

This grid uses one source pair selected by its reduced coefficient-six relation as a correctness control. A full input-independent selector must include the source-pair and local coefficient search costs before target timing. The next mathematical step is to expand or characterize the `k1,k2` representative lattice beyond this bounded grid, then measure novel row rank and target coverage if a rational bridge is constructed.

## Replay

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_bridge6_k1k2_grid_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python /Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119/research/exotic_ecdlp_20261021/xedni_bridge6_k1k2_grid_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_bridge6_k1k2_grid_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_bridge6_k1k2_grid_20261120/run.stderr
sh research/exotic_ecdlp_20261021/xedni_bridge6_k1k2_grid_20261120/formal/build.sh > research/exotic_ecdlp_20261021/xedni_bridge6_k1k2_grid_20261120/formal/build-output.txt 2> research/exotic_ecdlp_20261021/xedni_bridge6_k1k2_grid_20261120/formal/build-stderr.txt
python3 research/exotic_ecdlp_20261021/xedni_bridge6_k1k2_grid_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_bridge6_k1k2_grid_20261120/audit-result.json 2> research/exotic_ecdlp_20261021/xedni_bridge6_k1k2_grid_20261120/audit.stderr
```
