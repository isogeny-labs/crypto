# Constructor exclusion: checked algebraic core and remaining bridge

Lean 4 v4.34.1; no external packages. `lake build` checks the actual module.
The official Linux archive's release-API SHA-256 is
47bf4bbd78f70c2e9670598ab7124d92b6efb7330ff33e5fbb4030f6fd72e4e4.

| Readable argument | Lean declaration | Exact scope/status |
|---|---|---|
| Expand f(a+u)-f(a) | `quartic_difference_identity` | checked, every commutative ring; the IDC certificate points to this theorem |
| Differences of quartic roots satisfy u^4+c1*u=0 | `root_difference_linearized` | checked, every characteristic-two commutative ring |
| A fixed kernel translation has a two-cycle | `fixed_translation_two_cycle` | checked, characteristic two and additive f, explicit fixed translation |
| Return times three and four force a fixed point | `cubic_and_quartic_cycles_force_fixed` | checked, arbitrary function and element |
| Fixed cube-root multiplier gives a three-cycle | `fixed_cube_multiplier_three_cycle` | checked, multiplicative f and explicit fixed k with k^3=1 |
| A quartic root cannot have a four-cycle | `quartic_root_two_cycle` | checked, characteristic-two field; additive/multiplicative f, fixed c1, every cube root of unity fixed, f^4=id, both a and f(a) roots |
| Specialize f to q-Frobenius on F(q^4), show cube roots fixed when even m, and infer reducibility | none | readable finite-field argument only; not yet formalized in this dependency-free project |

The final checked theorem includes the zero root-difference case and every
c1, including c1=0. For u!=0, u^3=c1. Put k=f(u)/u. Multiplicativity and
fixed c1 give k^3=1. The explicit cube-root hypothesis fixes k, giving
f^3(u)=u. Together with f^4(u)=u this fixes u, and characteristic two gives
f^2(a)=a. The finite-field specialization and irreducibility bridge remain
explicit obligations; do not call the whole exclusion a completed Lean theorem.

`#print axioms` reports only propext, Classical.choice and Quot.sound,
or no axioms for the arbitrary-function lemma. No sorry, custom axioms or
native_decide. The repository proof gate audits transitive axioms separately.
`lean4checker --fresh` is unavailable on this host; no fresh-checker claim.
