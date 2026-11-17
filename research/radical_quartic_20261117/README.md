# Quartic radical root finder over F_p (S4 / m=3 Semaev case) vs Cantor-Zassenhaus

Status: **partial win, honestly scoped.** Correctness fully validated; the speed
win is real but modest against a fair CZ baseline, and one Frobenius class
(about 1/3 of random quartics) cannot be done by F_p radicals at all.
All claims are producer-side [pcc:unverified]; counts are exact field
multiplications (shared counter), not wall time.

Note on base: `research/radical_rootfind_20261117/` was not on the `main` this
worktree was cut from; its prototype was read from commit `02be2ee87e` and the
CZ harness reused verbatim (copied into `prototype/radical_quartic.rs`).

## What was built (`prototype/radical_quartic.rs`, standalone, `rustc -O`)

1. **Clean modular cube root, no brute fallback**: generic
   Adleman-Manders-Miller over any finite field of group order N
   (`Cube<R: Ring>`): bijective power `a^(3^-1 mod N)` when 3 does not divide N
   (F_p, p=2 mod 3); else cube test `a^(N/3)==1`, write N=3^s t, `x0=a^e`
   (3e=1 mod t), `h=a^(3e-1)` in the 3-Sylow, discrete log of h base
   `c=g^t` (g a cubic non-residue) digit by digit (Pohlig-Hellman), root
   `x0*c^(-l/3)`. Instantiated for F_p and F_{p^2}=F_p[i]/(i^2-n).
2. **Cardano** over F_p; **casus irreducibilis** (D=q^2/4+P^3/27 a non-residue)
   via `sqrt(D)=i*sqrt(D/n)` and F_{p^2} cube root; roots are the F_p-real
   `u*w^k+v*w^-k`.
3. **Ferrari**: depress, resolvent `z^3+2Pz^2+(P^2-4R)z-Q^2` (z=(r1+r2)^2), take
   an F_p root z that is a square (alpha), factor `(y^2+alpha*y+beta)(y^2-alpha*y+gamma)`,
   solve both with the quadratic solver; Q=0 handled as a biquadratic.
4. **Fundamental limit found while doing this.** If the resolvent has no F_p root,
   the quartic has Frobenius cycle type (1,3) (one rational root, irreducible
   cubic cofactor): no rational pairing exists, and `r1 = Tr(sqrt z)/2` lives in
   F_{p^3}. That class is **not solvable by F_p radicals**; the solver delegates it
   to the CZ distinct-degree step (`gcd(X^p-X,f)`), and reports it
   (`frobenius_fallback`). It is 8/24 of S4 and 8/15 of quartics that *have* a root
   (observed: 41.7M of 125.3M exhaustive tuples).

## Validation (before any speed measurement)

| Check | Result | Evidence |
|---|---|---|
| Cube root, F_p: all a, 39 primes 5..1459, both p mod 3 classes, 3-adic depth s up to 6 (p=1459) | exact existence + root, vs brute force | `cube_validation.txt` |
| Cube root, F_{p^2}: all p^2 elements for every prime <=700 (incl. p=647, v3(p^2-1)=5) | exact existence + root, vs table of all cubes | `cube_validation.txt` |
| Cube root large p (to 2^62): 300 random cubes recovered in F_p and F_p^2; non-cubes rejected | pass | `cube_validation.txt` |
| Quartic, **every** (a,b,c,d,e), a!=0, p in {5,7,11,13,17,19,23,29,31,37} (5 are p=2 mod 3, 5 are p=1 mod 3): 125,323,958 tuples | identical root sets to brute force; root-count histogram [0..4] = [46.0M, 43.6M, 29.6M, 1.77M, 4.30M] | `quartic_validation.txt` |
| Quartic vs reference CZ, 24,000 random + forced-4-root quartics over 8 primes up to 2^61-1 | identical; classes [0..4]=[6706,6065,4476,0,6753] (no 3-distinct-root class arises randomly; it is a repeated-root case, covered by the exhaustive run) | `quartic_validation.txt` |

p>=5 is required (divisions by 2 and 3).

## Measurement (`quartic_measurement.md`, 6000 random quartics per prime)

Two CZ baselines, same counted primitive: **CZ-harness** (the one in the prior
`radical_rootfind` PR: it re-inverts the leading coefficient on every polynomial
reduction, about 90-120 muls each) and **CZ-opt** (monic modulus, no repeated
inversions; a fair, stronger baseline). The radical solver assumes per-prime
precomputation (non-residues, 3-Sylow generator, constants), not counted per
call, as for a decomposition oracle with a fixed p.

p = 2^61-1 (p=1 mod 3):

| group | radical | CZ-harness | CZ-opt | harness/rad | opt/rad |
|---|---:|---:|---:|---:|---:|
| no root (38%) | 1,824 | 19,654 | 4,650 | 10.8x | 2.55x |
| 1 root, type (1,3), fallback (33%) | 4,802 | 19,652 | 4,648 | 4.1x | **0.97x** |
| 2 roots | 2,570 | 51,947 | 7,170 | 20.2x | 2.79x |
| 4 roots | 807 | 101,153 | 15,164 | 125x | 18.8x |
| **root exists** | 3,635 | 38,102 | 6,362 | 10.5x | 1.75x |
| **all** | 2,953 | 31,162 | 5,718 | 10.6x | **1.94x** |

Across the six primes (1e6 .. 2^61-1) vs CZ-opt: no-root 1.4-2.7x, root-exists
1.2-1.75x, all 1.3-1.9x; vs CZ-harness: 3.0-10.6x. The **(1,3) class is a small
net loss (0.71-0.97x)**: it pays the failed resolvent attempt on top of the same
CZ step. Four-root polynomials gain most (3.8-18.8x vs CZ-opt) because
everything is radical there.

Quadratic recheck (same file, `measure_quad`): radical vs CZ-harness 20-256x but
vs **CZ-opt only 5.4-20x** (p=2^61-1: no-root 7.1x, root-exists 19.9x). The earlier
13-113x quadratic headline was against the inversion-heavy harness CZ; the
fair-baseline gain is smaller, still large.

## Verdict

- Cube root (F_p and F_{p^2} AMM): validated, no brute fallback. The stated stop
  condition (cannot validate F_{p^2} cube root) did not trigger.
- Quartic radical: validated exactly. Speed: ~1.9x fewer field muls than a fair CZ
  overall at 61 bits (3.2-10x vs harness CZ), but **not** the 50-400x predicted:
  the prediction ignored the (1,3) class (about 1/3 of polynomials, no gain) and the
  inversion-inflated baseline. For the IC oracle that only needs "does a root
  exist and which", ~2.5x on the no-root path is the typical gain.
- Not a wall-time claim. Novelty check (`research-kb`) and the
  `controlled-experiment` protocol are still owed before any landed claim.

## Port

`src/cryptanalysis/prime_radical_root.rs` (`RadicalRootCtx::new(p)`,
`roots(&[u64]) -> RadicalRoots{roots, frobenius_fallback}`, degrees 1..4,
p in [5, 2^61-1]). The (1,3) class calls `prime_root_trace::solve` (validated
CZ). Tests: exhaustive cube roots, exhaustive deg 1-4 over p in {5..17} vs brute
force, random large instances vs `prime_root_trace::solve`.

## Reproduce

```bash
cd prototype && rustc -O radical_quartic.rs -o rq
./rq cube      # ~2 s
./rq quartic   # ~4.5 min on 4 cores (125M tuples)
./rq measure   # counted-mul tables
```
