# Native retained-base rank-query census protocol (frozen before execution)

Protocol ID: `N83-RQC-20261115-v1`. This is an exact analytic screen of retained manifests, not a relation search or timed IC comparison. The original requested objective remains a fully charged cold one-target runtime minimum over the requested factor-base and solver combinations.

## Inputs and identities

- October S3 manifest census: `../verification/live-s3-manifest-census-20261025/evidence-01/catalog.json`, SHA-256 `1117ead0dfcc9164a73d83d5ca709ec538528f7042e070ca8d72a5e188487ade`, 77 content-addressed objects, 50 primary `a0` and 27 diagnostic `a1`.
- Later full direct-subgroup D18 export: `../verification/d18-full-orbit-20261025/manifest.json`, SHA-256 `1d3650eabc4662d8f2597498d7ac92b7cd0a4134ac5f8155aaa49c7af3407dce`, one primary object.
- Historical six-case screen regression: `../pilot-01/rank-query-screen.json`, SHA-256 `3b0d9424a2809ee865b67614460a348bf70ae884d426b1732296638cec052276`.
- Primary curve `icv1-f2m83-tm6151469093347-debefd74`, order `2417851639230796216685689`. The `a1` order `8569786107849059` is a separate diagnostic.

## Calculation and acceptance

For each of the 78 source-claimed distinct base objects and each `m=2,3,4,5,6`, take `B` from its declared point count and `K` from its declared orbit columns. Calculate `M=binomial(B+m-1,m)` exactly with arbitrary-width integers. For uniform subgroup queries and an oracle yielding at most one independent full-smooth row per query, set `p<=min(M,r)/r`. The necessary query count before the Markov ceiling can reach one half is `q_min=max(K,ceil(K*r/(2*min(M,r))))`. Correlated queries are allowed. Check both sides of the threshold exactly. Emit all 390 rows, source digests, and explicit null total runtime. Never rank a total-runtime winner from this screen.

Controls: source hashes and counts must match; duplicate object keys and zero `B/K` reject; a changed source byte rejects; 30 historical arm/size/arity cases reproduce the frozen screen; hand-check small combinatorics and `q<K` cases. The diagnostic arm must never be pooled with primary. Stop on any failed control; preserve its output and correct the implementation before drawing conclusions.

The declared counts are manifest claims, not a fresh semantic audit of all point sets. The bound does not apply to biased targets, multiple independent rows per query, large-prime partial relations, or a solver with a different acceptance event. It prices neither construction, decomposition, matrix rank, descent, verification, nor matched rho. No wall-time performance claim follows. This screen is a prerequisite filter for a later frozen empirical comparison, not a substitute for the user's requested full sweep.
