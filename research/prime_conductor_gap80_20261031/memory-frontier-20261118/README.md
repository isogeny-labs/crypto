# Full-phase memory and all-window work audit

The original target is an explicitly evaluated ordinary-curve isogeny with a
certified prime endomorphism-conductor ratio at least 80 bits and repeated
paired endpoint scalar/ECDLP/IC measurements. This is a fixed-source static
qualification; it constructs no new endpoint or map and times no solver.

## Frozen hypotheses and acceptance

The older necessary memory model counted baby roots plus h. Check the
separate phase where h coexists with the monic zeroth giant jet. For baby
chunk b and giant chunk g, the two necessary logical slot counts are
2b+1 and b+2g+2. The latter follows because product_j (Z-x_j)^2 is monic
of degree 2g. This is an additional phase bound; the old model never claimed
its baby-only bound sufficient for admission.

For n=(ell-1)/2, B positive with 2B<=n, G=floor(n/(2B))>=1 and L=n-2BG,
retain the current traversal, including repeated giant contexts. Derive and
check exact all-window lower bounds under a coefficient-slot cap, plus the
minimum over B of max(2B+1,B+2G+2) for a single full rectangle. Label it as
an optimization of this necessary model, not actual RAM optimization.

Native checker must compare the optimizer to exhaustive B for all n=2..2048,
check the integer work bounds at six blocked schedules for every such B,
reject the preceding optimum cap, and retain a fixed root-only-fit/giant-
phase-fail counterexample. Check actual native polynomial lengths/coexistence
on existing supplied fixtures. Compare selected schedule metadata with prior
native outputs. Require zero exits, empty stderr and complete output records.

Pin sources, executables, input/protocol and qualifier before calls. These
are bounded local proof/development checks with a cumulative 120-second cap,
no paid resource, new curve search or performance session. Preserve failures
and all earlier models/receipts. No universal algorithm or impossibility claim;
all objects excluded from the model and the proof assumptions must be stated.

Sources checked: pinned native complete_pipeline.cpp (baby_product, giant
jet construction/finish and coverage), prepared_norm.hpp, original native
CONSTRUCTOR.md for index coverage, prior exact schedule audits, and installed
FLINT coefficient representation. KB unavailable: no tools; wrapper 127.
Use project-native integer/FLINT checks for these different fixtures under
the curve-algebra skill. No Lean/lake executable is exposed.

Results are pending and will be recorded separately in RESULTS.md. Any bound
that fits a cap still does not establish physical fit or launch admission.
