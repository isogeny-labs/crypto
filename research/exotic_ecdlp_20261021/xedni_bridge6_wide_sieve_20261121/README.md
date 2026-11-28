# All 441 nearby coefficient pairs have all-integer third-shift exclusions

For the fixed source pair \(P=(2,2)\), \(Q=(-16,17)\), set

\[
k_1=-3+77u,\qquad k_2=-11+77v,\qquad k_3=1+77t,
\quad -10\leq u,v\leq10,\quad t\in\mathbb Z.
\]

Each of the 441 fixed \((u,v)\) classes has a concrete prime at which the primitive univariate numerator of \(x(6Q)-x(P)\) is rootless. The first 440 certificates lie at primes at most 257. The remaining cell \((u,v)=(-5,-8)\) has its first certificate at 293, found in the frozen follow-up after the original cap. Consequently, for every integer \(t\) and every nonsingular rational curve in this stated window, the fixed points do not satisfy \(P+6Q=\mathcal O\). This expands the earlier 25-cell exact exclusion while preserving its fixed-source-pair scope.

| Stage | Result |
| --- | ---: |
| Frozen \((u,v)\) cells | 441 |
| Rootless prime by 257 | 440 |
| Additional rootless prime | 293 for \((-5,-8)\) |
| Unresolved cells after follow-up | 0 |
| Exact Lean rootlessness and integer-root exclusions | 441 pairs of declarations |

The initial [protocol](protocol.json) fixed the window, first-prime range 13–257, exact source numerator digest, statuses, and 300-second cap before execution (`9336e30f567`). Its [result](result.json) retains the single unresolved cell. The [follow-up protocol](survivor-protocol.json) fixed the sole cell, an integer scan through \(\lvert t\rvert\leq10{,}000\), exact rational replay for any root, and a prime search through 2003 (`3dfefe5df08`). The scan found no integer root in its bounded interval; the extended search certified the cell at 293. The checked Sage runtime receipt is [saved](sage-runtime-info.json), and both run stderr files are empty. The bounded scan is diagnostic; the modulus-293 certificate supplies the all-integer conclusion for that cell.

The independent [audit](audit.py) multiplies the two saved factors into the full three-variable numerator, specializes it in a separate implementation for all 441 cells, checks every stored coefficient, replays the first-rootless-prime test and the modulus-293 extension, and binds all Lean coefficient lists and moduli to the saved rows. Its [result](audit-result.json) is `PASS`.

The generated [Lean module](formal/WideSieve.lean) uses exact integer coefficients for each cell. For `i=000` through `440`, `rootless<i>` proves that the saved polynomial has no root in its recorded `ZMod p`, and `no_integer_root<i>` proves it has no integer root. The generator was frozen as `23d2510f5a7` before the build. The pinned Lean toolchain and Mathlib manifest, actual [build command](formal/build.sh), [output](formal/build-output.txt), and empty stderr are retained. All 882 `#print axioms` lines show only `propext`, `Classical.choice`, and `Quot.sound`. `lean4checker --fresh` is unavailable on this host.

The structural implication from \(P+6Q=\mathcal O\) to \(x(6Q)=x(P)\) is elementary, and the generic rational function was computed with the checked Sage launcher in the [full-lattice surface packet](../xedni_bridge6_lattice_surface_20261121/). Lean has not yet formalized the elliptic-section specialization that binds those rational-function coefficients to every nonsingular integer curve; that bridge remains a proof obligation. The next experiment should vary the source point pair or the auxiliary local policy, since widening this one pair's coefficient window did not produce a rational coefficient-six relation.

## Replay

```sh
python3 research/exotic_ecdlp_20261021/xedni_bridge6_wide_sieve_20261121/run.py
/Volumes/SSD990/cryptanalysis/sage --runtime-info
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_bridge6_wide_sieve_20261121/survivor_sage.py
python3 research/exotic_ecdlp_20261021/xedni_bridge6_wide_sieve_20261121/generate_lean.py
sh research/exotic_ecdlp_20261021/xedni_bridge6_wide_sieve_20261121/formal/build.sh
python3 research/exotic_ecdlp_20261021/xedni_bridge6_wide_sieve_20261121/audit.py
```
