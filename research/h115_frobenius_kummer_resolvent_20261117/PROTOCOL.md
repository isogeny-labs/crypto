# H115 Frobenius--Kummer resolvent-moment protocol

Status: frozen before implementation or candidate evaluation. No H115 scalar
label, moment score, selected predicate, decoder result or timing has been
observed.

## Question

Let `E/F_p` be ordinary with prime order `r=#E(F_p)=p+1-t`, let `G` generate
`E(F_p)`, and let `Q=[m]G`. Can a fixed low-order moment of the Frobenius
Kummer-cocycle fiber determine the canonical scalar digit `m mod 6` across
several curves and generator relabelings?

The experiment is deliberately two-gated:

1. an unchanged frozen predicate must be exact on every toy row and its
   scalar-blind radix-six decoder must recover every target;
2. any algorithmic claim additionally needs a uniform way to construct the
   required moment in subexponential time in `log r`, including extension,
   kernel and isogeny-fiber costs.

Passing gate 1 alone is not a generic or subexponential ECDLP algorithm.

## Algebraic mechanism

Write `pi` for `p`-power Frobenius and `V` for Verschiebung. On `E`,

```text
pi V = V pi = [p],       pi + V = [t].
```

Therefore

```text
(V-1)(pi-1) = [p]-V-pi+1 = [p+1-t] = [r].
```

For `R` in the division torsor `[r]R=Q`, its Frobenius cocycle
`c_R=(pi-1)R` lies in the degree-`r` fiber `(V-1)c=Q`. Changing `R` by
`T in E[r]` changes `c_R` by `(pi-1)T`. Because `E(F_p)=E[r]^{pi=1}` has
order `r`,

```text
H = image((pi-1)|E[r]) = kernel((V-1)|E[r])
```

has order `r`. Every element of the fiber is

```text
c0(Q) + H,       c0(Q)=[(p-1)^(-1) mod r]Q,
```

since `V=[p]` on rational points. The original `r^2` division points map onto
this `r`-element fiber with multiplicity `r`. This is an exact representation
compression only; enumerating `H` is still linear in `r`.

## Frozen moments

For nonzero `Q`, the fiber does not contain the identity. In the lexicographically
first monic degree-`d` extension from `CURVES.json`, compute for `1<=j<=8`

```text
X_j(Q) = sum_{C in c0(Q)+H} x(C)^j,
Y_j(Q) = sum_{C in c0(Q)+H} y(C)*x(C)^(j-1).
```

The true fiber is Frobenius-stable, so both sums must lie in `F_p`. All field,
point, Frobenius, `V`, eigenline, fiber and base-field assertions are checked
before scoring.

Define `ratio(a,b)=a/b` when `b!=0`, and `0` when `b=0`. The 54 frozen public
channels are:

- `raw-X-j`, `raw-Y-j`, `ratio-X-j`, `ratio-Y-j` for `j=1..8`, where a ratio
  divides the target moment by the corresponding generator moment;
- `cross-X-j`, `cross-Y-j` for `j=1..7`, equal to
  `M_j(Q)M_{j+1}(G)/(M_{j+1}(Q)M_j(G))` under the same zero convention;
- `mixed-j` for `j=1..8`, equal to
  `Y_j(Q)X_j(G)/(X_j(Q)Y_j(G))` under the same zero convention.

Each field element is converted to its canonical integer in `[0,p)`. One
channel supplies residues modulo 2 and 3. Apply every fixed permutation of the
two residue alphabets, then combine by CRT. Thus the complete family contains

```text
54 * 2! * 3! = 648
```

public predicate IDs. No coefficients, thresholds, interpolation, curve-sized
tables, per-curve formula choice or post-run feature construction are allowed.

## Cohort and acceptance

Use the five exact curves in `CURVES.json`. The first affine point in
lexicographic `(x,y)` order is the base generator, relabeled by multipliers
`1,2,3`. This gives 15 rows and

```text
3 * ((5-1)+(7-1)+(17-1)+(37-1)+(61-1)) = 366
```

nonidentity row-targets. Candidate success requires the same predicate ID to
be exact on all 15 rows. For every row-wise exact ID and every ID in the
15-row intersection, run a scalar-blind radix-six decoder

```text
R <- [6^(-1) mod r] * (R-[digit]G)
```

until zero, reconstruct the scalar, and independently require `[m]G=Q` for
all 366 targets. No fallback search or generic collision step is permitted.

## Structural checks

For each curve, exhaustively require:

1. the short-Weierstrass discriminant is nonzero; direct point count is the
   stated prime `r`; every nonidentity rational point has order `r`;
2. `d=ord_r(p)`, the frozen extension polynomial is irreducible, and the
   extension contains exactly `r^2` `r`-torsion points;
3. Frobenius on `E[r]` satisfies its characteristic equation and has the
   rational eigenline with eigenvalue 1 and the transverse eigenline with
   eigenvalue `p mod r`;
4. `H=image(pi-1)=kernel(V-1)` has exactly `r` points, where
   `V=[p]pi^(-1)` on the extension torsion;
5. `(V-1)(pi-1)=[r]` on every extension point checked and
   `(V-1)C=Q` for every `C` in every evaluated fiber;
6. cached and scalar-blind online fiber/moment construction agree; each true
   moment is fixed by Frobenius and reduces to `F_p`;
7. all 648 IDs parse, round-trip and are unique; every contingency count and
   decoder trace is internally consistent.

Any mismatch invalidates the run.

## Controls

- **Random-line control:** for the first two curves, choose eight seeded
  non-eigen lines of `E[r]`. For a line `L`, descend its possibly non-rational
  coset moments as `d^(-1) Tr_{F_{p^d}/F_p}` of the same sums. Apply all 648
  predicates with unchanged labels.
- **Rational-line control:** replace `H` by `<G>`, using the same descended
  moments and predicate family. This line's coset is constant as `Q` varies
  and is a wiring/null check, not a candidate.
- **Histogram-matched labels:** on the first two curves, use eight deterministic
  Fisher--Yates shuffles of the canonical `m mod 6` label vector and repeat the
  full 648-way selection.
- **Coordinate isomorphisms:** repeat multiplier-one rows under `lambda=2` for
  all five curves. Raw channels may change; ratio/cross/mixed channel values
  and predictions must be transported and reported separately.
- **Forbidden-oracle positive control:** derive the coefficient of `Q` in the
  exhaustive rational basis only after public moments are fixed, and confirm
  the scoring/decoder pipeline can recover it. This control is explicitly
  ineligible for candidate selection.
- **Ablations:** report raw-only and normalized-only best scores, but neither
  changes the frozen acceptance rule.
- **Tamper rejection:** the independent verifier must reject a changed
  extension coefficient, Frobenius image, fiber member, moment, scalar label,
  predicate ID, raw digest and summary count.

Seeds are SHA-256 domain-separated by control kind, curve slug, generator
multiplier and replicate; take the first eight bytes big-endian and use
SplitMix64. Candidate and controls receive identical selection pressure.

## Cost accounting and interpretation

Record base/extension additions, multiplications, inversions, Frobenius maps,
point additions, scalar multiplications, points enumerated, kernel elements,
moment terms, predicate evaluations, decoder queries, bytes and wall time.
Report separately:

- toy extension enumeration (`p^d` states);
- construction of `E[r]` and `H`;
- per-target `r`-element fiber enumeration;
- moment/channel scoring and decoder work;
- generic division-polynomial baseline (degree `Theta(r^2)`);
- compressed `(V-1)` fiber degree `r`.

Geometric degree is not a field-operation count. The experiment's explicit
fiber enumeration is `Theta(r)` per target, exponentially worse than a
subexponential goal measured in the input length `log r`. Even a perfect toy
digit requires a proved uniformly computable `r^{o(1)}` (equivalently
`exp(o(log r))`) moment shortcut, extension-field construction, and full
decoder cost before a subexponential runtime claim.

## Execution policy

Freeze, proof map, generic Lean identities, producer source and independent
verifier source must be committed and pushed before candidate evaluation.
Measurement uses the repository checkpoint and isolated Isolab policy with a
USD 1.00 cap, 1,800-second deadline and zero scientific retries. If that gate
is unavailable, report the run blocked and register the exact deferred card;
do not substitute a local candidate-scoring run.
