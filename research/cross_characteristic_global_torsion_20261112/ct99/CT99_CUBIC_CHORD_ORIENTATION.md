# CT99 — A scalar-blind online *chord* orientation for the fixed cyclic seven-torsion cubic

**2026-11-14. Scientific standing: [pcc:unverified].** Exact global coordinate identity, bounded toy fixture. This is an application of the classical chord-and-tangent group law, **not** a novel general ECDLP algorithm. It supplies the missing oriented (y) point coordinate without executing CT98's input-scalar extraction. It does **not** erase the input log recoverable from CT97's rational x-map representation, nor does it establish any cold cost advantage.

## Literature/previous work read

- Repository CT80–CT94: 11a1 global seven-division polynomial, selected 37-adic and binary places, CT87 cubic, CT90 integral CRT projection, CT91 complete binary group law, CT94 degree-24 binary extension requirement.
- CT95 degree nonmembership, CT96 full tower, CT97 scalar-blind **x-only** cubic splitting and source-root selection, CT98 global oriented multiplication-map certificate which explicitly reads `m` from a leading coefficient. New method below avoids that specific scalar read.
- Silverman, *The Arithmetic of Elliptic Curves*, Ch. III, elliptic group law: https://www.math.brown.edu/johsilve/AECHome.html ; Zwegers, *An Elementary Approach to the Group Law on Elliptic Curves* (2024), https://arxiv.org/abs/2401.02346. This is classical collinearity, not new theory.
- The dedicated research-KB MCP and the repository CLI wrapper are not exposed in this connector runtime. Fallback was primary repo reports and GitHub code search across isogeny-labs/crypto, cryptanalysis and crypto-autoresearcher. Neither a negative search nor this case establishes novelty.

## Precise proof obligation and construction

Consider the actual CT80 curve

[
E: Y^2+Y=f(X)=X^3-X^2-10X-20.
]

Write (T=(alpha,Y)) for the fixed globally oriented seven-torsion generator. The fixed CT87 cubic over the degree-eight subgroup field is

[
h(Z)=Z^3-sZ^2+e_2 Z-e_3,
]

and its three roots in the degree-24 x-point field (L) are (alpha,eta,gamma). The construction of (s,e_2,e_3) is charged as cold setup. Algebraically these roots are (x(T),x([2]T),x([3]T)); **the online procedure is never passed a multiplier for an input target**.

Put (D=4f(alpha)+1=(2Y+1)^2), (M=2alpha(s-1)-e_2-10). The two decisive **exact quotient-ring identities** are

[
h(alpha)=0,qquad M^2=(s-1)D.
]

They are computed over (mathbb Q[X]/(psi_7)), not inferred from matching the source or binary images. The second identity follows independently from the classical fact that (T,[2]T,-[3]T) are collinear. Define

[
lambda=rac{M}{2Y+1},quad
mu=Y-lambdaalpha.
]

Now (lambda^2=s-1) and (lambda(2Y+1)=M). Expanding (f(Z)-(lambda Z+mu)^2-(lambda Z+mu)), its (Z^2), (Z) coefficients are respectively (-s,e_2); the constant coefficient is (-e_3) because (h(alpha)=0). Thus **the full exact polynomial identity** is

[
oxed{f(Z)-(lambda Z+mu)^2-(lambda Z+mu)=h(Z).}
]

In particular, for **any** of the three already-recognized global x-roots (eta), the element

[
H_eta = rac{M(eta-alpha)}{D}in L,qquad
oxed{Y_eta=H_eta+(1+2H_eta)Y}
]

is an oriented global ordinate on the curve. **There is no division by two**, and the only denominator (D) is a unit at the fixed source and binary places. Therefore the same algebraic element specializes legally in both characteristics. The inverse is (-1-Y_eta).

This establishes a global coordinate lift in the degree-48 point field (K=L(Y)) for all three x-roots, entirely by the common relative cubic and its chord geometry rather than scalar-conditioned rational multiplication maps. The choice of sign for a public ((x,y)) is made by comparing its **source** reduction with the two curve ordinates, not by reading its discrete logarithm or choosing a target prime.

## Two explicit specialization controls

Source place (alphamapsto9, Ymapsto4) over (mathbb F_{37}), same fixed cubic: (s=27,e_2=30,M=21,D=7) and (lambda=27). The line is

[
y=4+27(x-9)pmod {37}.
]

At (x=9,27,28) its (y)-values are **4,9,36**; inverse values are **32,27,0**. Source orientation uses only this line and the public point, not an input multiplier.

At CT90's **selected** characteristic-two prime, (alpha=2, s=1545, e_2=1906) in the polynomial-basis (mathbb F_{2^{12}}). Since (Dequiv1pmod2) and (Mequiv e_2pmod2),

[
lambda_2=e_2=1906,qquad
Y_{eta,2}=Y_2+1906,(eta_2+2).
]

The binary square check (1906^2=1544=s+1) is independent. The six oriented source outputs, in the CT84 tower (mathbb F_{2^{12}}[Y_2]/(Y_2^2+Y_2+12)), are:

| Source public point | Binary x | Binary ordinate ((u,v)) |
|---|---:|---|
| (9,4) | 2 | (0,1) |
| (9,32) | 2 | (1,1) |
| (27,9) | 17 | (77,1) |
| (27,27) | 17 | (76,1) |
| (28,0) | 1562 | (2879,1) |
| (28,36) | 1562 | (2878,1) |

All are the same compatible-point outputs independently obtained through CT98's *different*, scalar-reading construction. The new algorithm **never computes a candidate scalar** during orientation; its own source and binary group operations are only validation controls.

## Independent regression and strict scope

- `ct99_cubic_chord_orientation.py` imports CT97's actual algebraic cubic roots and increasing-degree QQ nullspace reconstruction, verifies both **global** polynomial identities, checks that (H+(1+2H)Y) obeys the full global curve equation for every root in (mathbb Q[X]/(psi_7)), checks the selected source and binary specializations, and orients all six public points. No multiplication map leading coefficients are examined or scalar index is provided.
- Independently written `ct99_cubic_chord_independent.js` checks source cubic coefficients, both Frobenius-compatible chord square identities, source/binary on-curve equations and **all 49 ordered addition pairs** in the seven-element cyclic group (including the neutral point). It does **not** independently discover the global cubic roots or recompute the 24-dimensional QQ quotient identity.
- Scoped workflow runs normal and optimized Python and Node modes, compares complete canonical JSON outputs and prints the digest. GitHub CI is a separate gate; signed PCC and repo-wide native Conductor verification remain separate.
- Negative source x controls and fixed target-prime selection are inherited from the **actual** CT97/CT90 imports; this test does not retarget the binary place after examining the input.

From repository root:

```bash
D=research/cross_characteristic_global_torsion_20261112/ct99
python3 -m pip install 'sympy==1.14.0'
python3 -B "$D/ct99_cubic_chord_orientation.py" >/tmp/ct99-py.json
python3 -O -B "$D/ct99_cubic_chord_orientation.py" >/tmp/ct99-pyopt.json
node "$D/ct99_cubic_chord_independent.js" >/tmp/ct99-js.json
node --no-opt "$D/ct99_cubic_chord_independent.js" >/tmp/ct99-jsopt.json
cmp /tmp/ct99-py.json /tmp/ct99-pyopt.json
cmp /tmp/ct99-py.json /tmp/ct99-js.json
cmp /tmp/ct99-py.json /tmp/ct99-jsopt.json
sha256sum /tmp/ct99-py.json
```

## Important cryptanalytic boundary and cost obligations

This is a **precomputed order-seven cyclic cubic**. It makes online coordinate selection scalar-blind as an executed algorithm, but CT87 **constructed the cubic using explicit small multiples** of the global generator, and CT97's exact rational representations reveal (|m|) for these three roots as shown in CT98. The procedure has not separated the construction cost from solving the public ECDLP. Precomputing all nonzero seven-torsion points is already a complete trivial-sized discrete-log table; collinearity does not evade this observation.

Cold cost must include constructing the global division polynomial, its degree-eight subgroup field, the degree-24 x-field and degree-48 oriented field, the cyclic cubic and discriminant, the rational linear algebra/root selection, source and binary prime compatibility, and finite-field arithmetic over (mathbb F_{2^{24}}), followed by **actual** target log solving and final source verification. A fair source BSGS takes ceiling square-root seven table scale, so no advantage is plausible from this fixture alone. This is an explicit tiny correctness test, not an extrapolated complexity claim.

Next decisive experiment: a second independent prime/curve fixture and a certified algorithm recovering globally oriented roots from a non-pretabulated relative polynomial with bounded setup and height costs. For larger (ell), the CT96 (Theta(ell^2))-dimensional dense field remains a representation route cost, and no generic efficient relative cubic is established. [pcc:unverified]
