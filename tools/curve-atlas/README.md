# Curve Atlas

A buildless, browser-local explorer for explicit elliptic isogenies, higher-genus covers and induced Jacobian maps over small prime fields. Open `dist/index.html` through any static HTTP server. No external computation service is used.

For a single-file version, run `node build-standalone.mjs` from this directory
and open [curve-atlas.html](curve-atlas.html) directly in a browser. The generated
HTML is included. See [INTEGRATION.md](INTEGRATION.md) for repository context,
sources, producer-check scope and unavailable validation gates.

Run verification with `node checks.mjs`. The checks verify rational map equations over finite fields, equality of point counts across computed isogenies, genus examples, and rejection of invalid inputs.

## Scope

For a nonsingular short-Weierstrass elliptic curve over F_p (5 ≤ p ≤ 499), enumerate rational degree-2 kernels to depth 0–2 using Vélu quotients. Nodes are coefficient models, not isomorphism classes. Covers use v^m = f(u^n+c) with m in {2,4}, n at most 6, f(c) nonzero and p not dividing mn. Their maps are (u,v) ↦ (u^n+c,v^(m/2)). All genera refer to smooth projective models. The m=4,n=1 family is a nonhyperelliptic genus-3 plane quartic. Hyperellipticity of other m=4 examples is not determined.

Induced pullback and norm maps are structural divisor-class maps; this tool does not compute Jacobian coordinates. Their composition on E is multiplication by the covering degree. They are not isogenies from E onto higher-dimensional Jacobians.

This is not an exhaustive classification of maps, covers, correspondences or cohomological transfers. Nonconstant maps of smooth curves run from these higher-genus covers down to E. Export buttons provide a machine-readable JSON graph and a standalone SVG.

## Files

- `dist/math.mjs`: reusable pure computation module (`build`, `points`, and finite-field helpers).
- `dist/app.mjs`: interactive graph, inspector, controls and exports.
- `checks.mjs`: mathematical validation.

No build step or package install is required. Fonts have local fallbacks if Google Fonts is unavailable.
