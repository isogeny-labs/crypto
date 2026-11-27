# Successor review after the H2 loss

Continue the active local Rust lane PR #5094. H2 cost registration is consumed.
Do not rerun it. Review competing next actions before selecting a new solver.

Sources checked: the existing hypotheses-20261123 H3 derivation; the actual
codimension-one quotient and sparse rank tracker; Faugere–Mou, Sparse FGLM,
author preprint arXiv:1304.1238 §§3.1–3.3; Joux–Vitse, Elliptic Curve Discrete
Logarithm Problem over Small Degree Extension Fields, §2.4's warning about
large-degree field equations; the separate CoverDecomp paper; current
zero_dim_la's existing gcd(f,x^p-x) root filter. The field-equation warning
is from the small-degree summation-polynomial paper, not the CoverDecomp
paper. Do not conflate those papers or claim this mechanism is new.

For rho applicability, read Duursma–Gaudry–Morain's primary author version
§§3–4 and Sage's official invariant formulas. Sources:
https://www.lix.polytechnique.fr/Labo/Pierrick.Gaudry/publis/automorphisms.ps.gz
and https://github.com/sagemath/sage/blob/10.8/src/sage/schemes/elliptic_curves/ell_generic.py.
The web reader returned403 on Morain's page; direct author PostScript
download/conversion worked. Sage is read as a reference, not executed.

KB MCP is absent; wrapper status and exact Frobenius/field-equation query
return no executable. ecdb MCP is absent; read its guide and GET /v1/meta
wrapper refuses without read token/AWS access. Fall back to exact retained
public input and source. No novelty or repository-wide absence conclusion.

Implement one read-only native metadata audit, with no new F4, root finder,
rho walk, DLP solve, timing comparison or target generation:

1. Independently calculate the public model's c4, c6, discriminant and j,
   checking c4^3-c6^2=1728*discriminant. Test known j0/j1728, singular
   refusal and a finite-field Frobenius-fixed-space counterexample first.
2. On the one pinned p1009 public curve, record special-j flags and whether
   coefficients/j are fixed by p^k for k=1,2,3,6. This tests necessary
   applicability conditions, not an exhaustive endomorphism search. Do not
   assume an extension-field curve has the prime-field sixfold automorphism.
3. Read the pinned Run51 raw packet once, emit only public collection keys,
   safe checkpoint counts, matrix dimensions and row counts. Never emit a
   scalar, kernel coefficients, raw target answers or whole raw objects.
   A zero unadmitted rank lower bound is not an exact nullity measurement.
4. Read the new frozen rejection-cost analysis and its original raw summary
   (bound by the same public-input hash; require matching counts in all five
   traced samples) to compute the optimistic
   partial-product ceiling if the entire root stage were free. This is a
   conditional bound on that pair only, not a predicted attack gain.

Inputs: flat public.json hash b3ae4f93974456bdd6a772caf7daa0208d64c17f7341f70ee4187d327fd6462e;
Run51 raw hash6e1db64631838ebe3bdc918a88b160ced5c8362b70aea8814d16aca83368286d;
new derived.json bound by PCC-20261124-773135. Publish source/binary pins
after controls and before the sole public metadata call. Exclusive output;
8GiB virtual memory,120s outer cap, local backend, zero paid compute.
Retain failures. No benchmark wall inference or canonical admission.

Competing hypotheses remain conditional: H3 trades collection tail for a
larger complete homogeneous nullspace plus extra target-useful affine rows;
field-equation filtering must pay for its normal forms and ideal update,
preserve every original Fp root and beat the already implemented univariate
filter. One coordinate does not generally force every coordinate into Fp,
and dimension ker(Frobenius-I) does not count rational points.
