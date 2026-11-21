# A full coefficient-six CRT relation surface

For the fixed rational points \(P=(2,2)\) and \(Q=(-16,17)\), this packet computes the exact necessary surface for \(P+6Q=\mathcal O\) throughout the coefficient class

\[
k_1=-3+77u,\qquad k_2=-11+77v,\qquad k_3=1+77t,\qquad (u,v,t)\in\mathbb Z^3.
\]

The Weierstrass coefficients are \(a_1=47k_1\), \(a_2=47k_2\), \(a_3=47k_3\); \(a_4,a_6\) are uniquely reconstructed from the equations for \(P,Q\). The construction preserves the local bridge's congruence conditions at 7 and 11. Over \(\mathbb Q(u,v,t)\), the primitive numerator of \(x(6Q)-x(P)\) has total degree 28 and factors into irreducible degree-13 and degree-15 polynomials, each with multiplicity one. The degree-13 factor lies on the inverse-point y branch; the degree-15 factor lies on the same-y branch. Thus, at a nonsingular specialization where the rational formulas are defined, the degree-13 factor is the generic relation component.

The degree-13 factor has 436 terms over \(\mathbb Z\), but reduces modulo 47 to

\[
F_{13}(u,v,t)\equiv9+38u+19v+30t\pmod {47}.
\]

Consequently, every integer zero of this factor satisfies that affine congruence. It is a first-lift filter on the full coefficient lattice; it has solutions and therefore does not close the integer-point problem. The earlier 25-cell grid is an independently checked specialization of the same three-variable surface.

## Reproduction and evidence

The frozen construction is in `protocol.json` and `run_sage.py`; the factor sign test is in `factor-protocol.json` and `classify_factors.py`. Both jobs used the checked `/Volumes/SSD990/cryptanalysis/sage` launcher and the runtime receipt `sage-runtime-info.json`. The surface process finished in 3.3 seconds and the factor classification in 6.2 seconds under their recorded 300-second caps. The complete 3,545-term numerator and 3,289-term denominator, their digests, two factors, event log, and execution results are retained in this directory. `audit.py` independently multiplies the sparse factors to the full numerator, verifies all 25 prior numerator and denominator specializations, and checks that Lean's 436 coefficients equal the saved degree-13 factor. Its `audit-result.json` is `PASS`.

Run the exact checks from the worktree root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info
gtimeout 300 /Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_bridge6_lattice_surface_20261121/run_sage.py
gtimeout 300 /Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_bridge6_lattice_surface_20261121/classify_factors.py
sh research/exotic_ecdlp_20261021/xedni_bridge6_lattice_surface_20261121/formal/build.sh
python3 research/exotic_ecdlp_20261021/xedni_bridge6_lattice_surface_20261121/audit.py
```

The pinned Lean toolchain and Mathlib manifest are in `formal/`. `formal/build-output.txt` records successful builds and `#print axioms` for all reported theorems; each shows only `propext` and `Quot.sound`. `formal/SignSplit.lean` proves the structural two-branch implication for equal x-coordinates on a Weierstrass equation over a domain. `formal/Residue.lean` contains the exact 436-term factor and proves both its reduction modulo 47 and the necessary congruence for every integer zero. The Lean factor identity is tied to the saved sparse polynomial by `audit.py`.

| Mathematical result | Lean declaration | Hypotheses and status |
| --- | --- | --- |
| Equal x-coordinates split into same-y or inverse-y branch | `XedniBridge6LatticeSurface.same_x_weierstrass_sign_split` | Any commutative domain, common Weierstrass right side; checked |
| The saved relation factor reduces to an affine polynomial mod 47 | `XedniBridge6LatticeSurface.relation_factor_mod47` | Exact 436-term `terms`; checked, coefficient match audited |
| Integer roots satisfy the mod-47 first-lift condition | `XedniBridge6LatticeSurface.necessary_congruence` | `evalInt terms u v t = 0`; checked |

The rational-function construction, irreducible factorization, and identification of the degree-13 factor with the group relation are Sage computations with the independent product and specialization audit described above; they are not yet Lean theorems about elliptic-curve points. Exceptional specializations at denominator zeros or singular curves require separate classification. The next step is to use this affine filter with additional primes to search or exclude integer points of the degree-13 surface, then replay any surviving rational group relation directly.
