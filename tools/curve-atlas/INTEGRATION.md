# Integration and evidence boundaries

This PR packages the existing Curve Atlas browser tool. Its scientific coverage
is partial: rational 2-isogenies and two explicit families of cyclic covers over
prime fields 5 through 499. All Jacobian arrows are structural descriptions;
there is no implemented divisor-class transfer, kernel certificate, full
Jacobian equation, subgroup-preservation check or destination solver.

The user's requested deliverables are an elliptic transfer graph, emphasis on
hyperelliptic and higher-genus targets, and a PR in isogeny-labs/crypto. The graph
and explicit cover families are implemented; browser behavior remains untested
in this environment. An enumeration of *all* transfers is not implemented;
the UI and exported graph state the finite search family and bounds.

## Sources checked

- [Repository cover construction](../../docs/curves/COVERS.md): the n=2, m=2
  case already has a native construction. This tool presents a separate small-field
  interactive exploration and does not replace that certificate implementation.
- [Higher-genus routing](../../.agents/skills/ecdlp-surfaces/references/higher-genus.md):
  distinguishes curve covers from induced Jacobian maps, divisor arithmetic,
  subgroup injectivity and costs. The graph keeps those obligations separate.
- [Existing isogeny CLI](../isogeny-cli/README.md): native catalog and replay
  support remains the interface for larger-field isogeny work.
- [Sage isogeny documentation](https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_curve_isogeny.html):
  inspected the documented Vélu construction, rational maps and codomain
  invariants. The implementation specializes the rational degree-2 kernel.
- Sources checked (KB): **KB unavailable**. No research-kb MCP tools were
  loaded; `scripts/research-kb status` returned 127 (no executable). Repository
  sources above provide the fallback review. No novelty claim is made.

## Local software checks

`validation.log` retains this integration's commands, outputs and exit statuses.
These are producer checks, not signed PCC standing or independent execution
proofs. Any reported outcome is an unverified assertion. [pcc:unverified]

The mathematical checks enumerate affine points for bounded small-field cases,
test displayed Vélu rational-map equations away from their exceptional
denominators, compare source/codomain elliptic point counts, and check fixed
genus and invalid-input cases. They do not address extension at exceptional points,
classify all maps, or test Jacobian arithmetic. Cover-map point tests use c=0;
nonzero shift behavior is not exhaustively tested.

No Rust implementation is changed. The requested `cargo test --release --lib`
gate could not run because Cargo is not installed in this runtime. Conductor
is also absent; the reserved scope is recorded in today's session log. Visual
QA remains untested because the Sites browser helper was unavailable in the
build environment. No performance measurement, benchmark, paid cloud run,
catalog promotion or leaderboard update is part of this integration.

## Reproduce

```sh
node tools/curve-atlas/checks.mjs
node --check tools/curve-atlas/dist/math.mjs
node --check tools/curve-atlas/dist/app.mjs
node tools/curve-atlas/build-standalone.mjs
```

Open `tools/curve-atlas/curve-atlas.html` directly in a browser. For source
development, serve `tools/curve-atlas/dist/` with a static HTTP server.
JavaScript is used for browser-local interaction; no Python runtime is added
to the application's computation path. Fonts have system fallbacks.
