# Direct duplication bound for the fixed xedni curve

For `E/ℚ: y²=x³−47x²+751x−3997`, the duplication forms and an exact
interval certificate give the uniform comparison

`|h(2P)−4h(P)| ≤ 53 log(2)` and `|H(P)−h(P)| ≤ (53/3) log(2) < 14`,

where `h(P)` is the logarithmic projective height of the reduced `x` fraction,
`h(0)=0`, and `H(P)=lim h(2^nP)/4^n`. The second inequality follows from the
checked geometric-tail theorem in the [canonical-limit packet](../xedni_canonical_height_limit_20261122/README.md).
This derives the numerical comparison needed by the existing 130-bit
fixed-curve exclusion without invoking the general Silverman estimate.

## Exact argument

For reduced `x=A/B`, put `M=max(|A|,|B|)` and define the homogeneous
duplication forms

```
F = A⁴ − 1502A²B² + 31976AB³ − 187435B⁴
G = 4A³B − 188A²B² + 3004AB³ − 15988B⁴.
```

The tangent formula gives `x(2P)=F/G` as a projective fraction, including
`G=0` for a doubled identity point. When `B=0`, `P` is the identity and its
doubling defect is zero. The coefficient sums of both forms are below `2¹⁸`,
so `max(|F|,|G|)<2¹⁸M⁴`.

The exact archimedean certificate proves
`max(|F|,|G|)≥M⁴/1024`. For `|A/B|≤128`, its 91 closed rational intervals
cover `[-128,128]`; each lists a signed form whose Bernstein coefficients
are all at least `max(1,|left|,|right|)⁴/1024`. Bernstein basis functions
are nonnegative and sum to one on each interval. Outside this range,
`|F(A,B)|≥(3/4)|A|⁴` follows from the exact tail inequality checked by Lean.

The homogeneous Bézout identity in `Bounds.lean` is

`U(A,B)F(A,B)+V(A,B)G(A,B)=R B⁷`, with `R=8038359040000<2⁴³`.

For coprime `A,B`, every common divisor `d` of `F,G` is coprime to `B`:
modulo `B`, `F≡A⁴`, and any prime dividing both `d` and `B` would divide
`A`. Since `d|R B⁷`, Euclid's lemma gives `d|R`. Thus reducing `[F:G]`
removes at most a factor `R`; combining the upper and lower bounds yields
`−53 log(2)≤h(2P)−4h(P)≤18 log(2)`.

## Verification and proof map

| Result | Evidence | Status |
| --- | --- | --- |
| Forms match fixed-curve doubling | `check_sage.py`, 30 signed multiples with the checked launcher | Independent finite replay; general point identity not yet in Lean |
| Homogeneous Bézout identity, coefficient and tail inequalities | `Bounds.lean`, pinned build and `#print axioms` | Lean checked |
| Archimedean lower bound | `certificate.json`, exact Horner generator and independent exact Bernstein auditor | Exact computational certificate; Lean checker open |
| Common-divisor bound `d|F,G ⇒ d|R` for primitive `A,B` | `common_divisor_dvd_resultant` in `Bounds.lean` | Lean checked, including coprimality bridge |
| Uniform doubling defect and numerical comparison | Above inequalities and the canonical-limit geometric tail; Lean checks `(53/3)log 2<14` | Mathematical derivation; full fixed-point Lean theorem open |

The polynomial, divisibility and numerical Lean declarations use only standard
foundations in their `#print axioms` output. The packet is **partially
formalized** until the interval certificate and general duplication map are
connected to the actual `WeierstrassCurve.Affine.Point.naiveHeight`.
The canonical-limit packet already proves quadraticity for integer multiples.
Mordell–Weil generation and rational-to-finite reduction remain separate
obligations for a complete fixed-curve exclusion theorem.

Replay from this directory:

```sh
python3 generate_certificate.py
python3 audit_certificate.py
sh build.sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info
/Volumes/SSD990/cryptanalysis/sage -python /Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119/research/exotic_ecdlp_20261021/xedni_duplication_height_bound_20261122/check_sage.py
```

The recorded runtime and result files correspond to the exact `check_sage.py`
in this packet. The float scan in `protocol.json` was only a pre-freeze pilot;
the 91-interval certificate is the promoted arithmetic evidence.
