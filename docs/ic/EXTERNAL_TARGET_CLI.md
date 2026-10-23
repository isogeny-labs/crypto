# Explicit public-target IC commands

`ic` and `icx` accept caller-supplied public curve, generator, and target points for
two bounded field families. Both commands reject malformed curves and points
before relation collection. A successful result includes a scalar checked by
an independent multiplication of the supplied generator.

## Prime field

```sh
ic solve-prime --p 271 --a 1 --b 19 --n 281 \
  --gx 3 --gy 7 --qx 231 --qy 54 --seed 42 --json
ic solve-prime-file --input docs/ic/fixtures/prime271_target_input_20261023.json \
  --seed 42 --rho-steps 0 --json
```

`solve-prime-file` is available through both `ic` and `icx`. It accepts a
local JSON path or indexed `store:NAME` and `sha256:DIGEST` reference with the
eight integer fields `p`, `a`, `b`, `n`, `gx`, `gy`, `qx`, and `qy`; unknown or
missing fields are rejected. The file is capped at 64 KiB and the report
records its SHA-256 and byte count in `input_artifact`. The command calls the
same field, subgroup, relation, isogeny-route, and scalar-verification code
as the flag-based `solve-prime`. Relation budgets, `--seed`, `--rho-steps`,
`--route-file`, and `--route-id` remain CLI options. The public target file
passed live Setec S3 upload, HEAD, downloaded-byte verification, fresh-cache
solves in both binaries, pull, and exact-object cleanup; see the
[`PRIME_TARGET_FILE_S3_ACCEPTANCE_20261023.json`](../store/PRIME_TARGET_FILE_S3_ACCEPTANCE_20261023.json)
receipt.

Both `ic` and `icx` can save any report with `--out PATH`. Add
`--store-name NAME` to publish that new file to the configured artifact store
in the same invocation. The name must be unused, the output path must not
exist, and the remote must be configured before the solve starts. A successful
publication prints the indexed SHA-256 and byte count to stderr. A bounded or
error report can also be published so its limits remain available for review.
For example:

```sh
CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/ic/v1/runs \
ic solve-prime --p 271 --a 1 --b 19 --n 281 \
  --gx 3 --gy 7 --qx 231 --qy 54 \
  --out prime271-report.json --store-name ic-reports/prime271/run-001 --json
crypto store verify ic-reports/prime271/run-001 --remote --download
crypto store pull store:ic-reports/prime271/run-001
```

The remote prefix is an example schema; choose a project-owned prefix and
configure the same store index for the solve, verify, and pull commands.

The example target is `[190]G`. The command requires a prime `p` in
`5..=1000000`, a nonsingular short-Weierstrass curve, a prime subgroup order
`n` in `3..=4000000`, canonical coordinates, and `[n]G = [n]Q = O`.
When `n` also divides the cofactor, it checks exact membership in `<G>`;
an order-`n` point in another subgroup is rejected.
It counts `#E(F_p)` and records the exact cofactor before collection.
Relation collection is limited by `--fb-size`, `--extra-relations`,
`--max-trials-per-relation`, and `--max-relation-attempts`. The JSON report
separates factor-base construction, relations, linear algebra, and final
verification, and includes a same-target Pollard-rho reference unless
`--rho-steps 0` is set. `--seed SEED` makes the IC relation sampler
reproducible and records the seed in `config.seed`. With the same binary,
parameters, and seed, relation counts, trial counts, and the verified scalar
replay. Wall-clock stage timings remain host-dependent. Without `--seed`,
relations use OS randomness. The optional same-target rho reference has its
own sampler; use `--rho-steps 0` when comparing IC reports for relation replay.
The public [seed replay receipt](fixtures/prime_seed_replay_20261022.json)
records two release-CLI runs each for `ic` and `icx`, with and without a
certified isogeny route. All four pairs matched the scalar and relation
counters after excluding host-dependent timing fields.
The verified public solve output used for the S3 integration check is frozen
as [`prime271_solve_report_20261018.json`](fixtures/prime271_solve_report_20261018.json).
Its SHA-256 is recorded in the linked
[`IC_REPORT_S3_ACCEPTANCE_20261018.json`](../store/IC_REPORT_S3_ACCEPTANCE_20261018.json)
receipt; the S3 acceptance object was deleted after the fresh pull check.

### Certified isogeny route

`solve-prime` can transport the supplied `G` and `Q` along one root-origin
`IW1` route before collecting IC relations:

```sh
ic prime-route-generate --p 271 --a 1 --b 19 --n 281 \
  --gx 3 --gy 7 --qx 231 --qy 54 \
  --degree 11 --max-curves 3 --seed 1 --json \
  > /tmp/prime-route.json
ic solve-prime --p 271 --a 1 --b 19 --n 281 \
  --gx 3 --gy 7 --qx 231 --qy 54 \
  --route-file /tmp/prime-route.json \
  --route-id IW1E11h1h1a619f4ab250 --json
```

`prime-route-generate` is available through both `ic` and `icx`. It accepts
the same public curve, generator, and target coordinates as `solve-prime`,
counts and validates the group, runs a bounded certified walk, and
independently verifies the generated `IW1` graph before emitting its route
file. The degree must be prime in 2–31 below `p-1` and distinct from the
DLP subgroup order; `--max-curves` is 2–64 and `--max-depth` is 1–64
(default 1). Degree 2 follows rational 2-torsion through the requested depth.
Other degrees use the modular-polynomial walk. Routes may
contain several checked edges. A walk with no certified
rational route exits unsuccessfully. The command may return several route
IDs; select one from the emitted `routes` array. The p=271 command above
returns two verified edges and two routes, including the shown route ID.
For a rational degree-2 route, use the public `p=101`, `#E=106`, `n=53`
target:

```sh
ic prime-route-generate --p 101 --a 2 --b 7 --n 53 \
  --gx 73 --gy 57 --qx 20 --qy 88 --degree 2 --json \
  > /tmp/prime-route-2.json
ic solve-prime --p 101 --a 2 --b 7 --n 53 \
  --gx 73 --gy 57 --qx 20 --qy 88 \
  --route-file /tmp/prime-route-2.json \
  --route-id IW1E2x1h3844094bf86c --rho-steps 0 --json
```

The degree-2 constructor enumerates rational roots of the cubic defining
2-torsion, checks each linear kernel polynomial and Vélu codomain, and emits
root-origin routes. The verifier independently checks the recorded kernel,
codomain, curve identities, and point map. The command above generated one
checked edge; `solve-prime` verified the route and scalar 5 on both curves.
The generated route passed a live Setec store push, HEAD, remote verification,
fresh-cache pull, `solve-prime` replay through `store:`, and exact-object
cleanup. Its isolated object key and checks are in
[`GENERATED_PRIME_DEGREE2_S3_ACCEPTANCE_20261022.json`](../store/GENERATED_PRIME_DEGREE2_S3_ACCEPTANCE_20261022.json).
For a two-edge degree-2 route, use the public p=107 curve with counted order
124, subgroup order 31, generator `(11,37)`, and target `(16,61)`:

```sh
ic prime-route-generate --p 107 --a 3 --b 5 --n 31 \
  --gx 11 --gy 37 --qx 16 --qy 61 --degree 2 \
  --max-curves 8 --max-depth 2 --json > /tmp/prime-route-2-depth2.json
ic solve-prime --p 107 --a 3 --b 5 --n 31 \
  --gx 11 --gy 37 --qx 16 --qy 61 \
  --route-file /tmp/prime-route-2-depth2.json \
  --route-id IW1E2x2h0d74657d90d7 --rho-steps 0 --json
```

The generator verifies every rational 2-torsion kernel it selects and
transports the public generator to each new model before expanding it. The
solver rechecks the two-edge path and scalar 5 on its source and destination.
The generated two-edge degree-2 route passed a live Setec S3 upload, HEAD,
remote verification, fresh-cache pull, `solve-prime` replay through `store:`,
and exact-object cleanup. The isolated key and checks are in
[`GENERATED_PRIME_DEGREE2_MULTIHOP_S3_ACCEPTANCE_20261022.json`](../store/GENERATED_PRIME_DEGREE2_MULTIHOP_S3_ACCEPTANCE_20261022.json).
For a two-edge route on the p=271 target, set `--max-depth 2 --max-curves 16`
and select `IW1E11h2hd708dae2b716` from the resulting route file. Both edges
are reverified during `solve-prime`; the public scalar replays on the second
destination curve. The generator emits only root-origin route IDs that
`solve-prime` can consume, while retaining and verifying the full graph.
The two-edge route also passed a live Setec S3 upload, HEAD, remote verify,
fresh-cache retrieval, `solve-prime` replay through `store:`, and exact-object
cleanup. The isolated key and checks are in
[`GENERATED_PRIME_MULTIHOP_S3_ACCEPTANCE_20261022.json`](../store/GENERATED_PRIME_MULTIHOP_S3_ACCEPTANCE_20261022.json).
The generated route also passed a live Setec store push, HEAD check, remote
verification, fresh-cache pull, `solve-prime` replay of scalar 190, and
exact-object cleanup. Its isolated key and checks are recorded in
[`GENERATED_PRIME_ROUTE_S3_ACCEPTANCE_20261022.json`](../store/GENERATED_PRIME_ROUTE_S3_ACCEPTANCE_20261022.json).

`--route-file` also accepts an indexed `store:NAME` or `sha256:DIGEST`
reference. The solver verifies and fetches a missing route file through the
configured content store before checking the graph and transporting points.
For example, after publishing the fixture as
`isogeny-routes/public-prime271/walk-11/routes.json`, replace the local path
above with
`store:isogeny-routes/public-prime271/walk-11/routes.json`.

The fixture was enumerated by `isogeny_walk walk` with `--primes 11`,
`--max-curves 3`, and `--max-depth 1`; the two edge certificates also passed
`isogeny_walk verify`. The IC command independently counts the starting
curve order, re-verifies every graph node, edge, and route ID, constructs each
point map from its certified kernel polynomial, checks the codomain and
isomorphism, and tests that each map commutes with point doubling. It rejects
an edge that kills the selected subgroup. Its JSON receipt records the route
hash, both transported points, and validation counts. Any scalar found on
the destination is checked on both the destination and the original public
curve. The Pollard-rho reference stays on the original `G,Q` pair.

A second live acceptance enumerated this public custom curve with the
release `isogeny_walk` binary, published the three-curve walk under a unique
Setec S3 prefix, confirmed the completion marker with HEAD, fetched all three
files with hash verification, and independently verified both fetched edges.
`ic solve-prime-file` consumed the fetched route and verified scalar 190 on
the original curve. The four test objects were then removed; see
[`ISOGENY_WALK_CUSTOM_S3_ACCEPTANCE_20261023.json`](../store/ISOGENY_WALK_CUSTOM_S3_ACCEPTANCE_20261023.json).
For an on-demand S3 existence check, use
`isogeny_walk exists --from s3://BUCKET/PREFIX --run RUN_ID`. It HEADs the
authoritative `complete.json` marker and returns JSON with `exists`, byte
count, and S3-reported SHA-256. Use `--shard I --of N` or `--collected --of N`
to check trait outputs; a missing marker reports `exists: false`.
Add `--check-objects` to read the marker and HEAD every referenced object.
Its `object_check` reports `pass`, `incomplete`, or `no_marker`, with missing
and mismatched keys. This compares S3 byte counts and stored SHA-256 values;
`fetch` checks downloaded and decoded bytes before publishing local files.
The [live object-check receipt](../store/ISOGENY_WALK_OBJECT_CHECK_S3_ACCEPTANCE_20261023.json)
records all three statuses, including a missing object and a replacement with
different stored bytes under a unique Setec prefix.
The [live S3 receipt](../store/ISOGENY_WALK_EXISTS_S3_ACCEPTANCE_20261023.json)
records false, true, and false across publish and exact-prefix cleanup.
`isogeny_walk fetch` writes to a new output directory only after every
downloaded object passes its stored and decoded SHA-256 checks. A missing or
corrupt object leaves no output directory, and an existing output path is
rejected rather than overwritten.
The [live acceptance receipt](../store/ISOGENY_WALK_ATOMIC_FETCH_S3_ACCEPTANCE_20261023.json)
records a complete three-file fetch, existing-directory rejection, and a
missing second object that leaves neither an output directory nor staging files.

## Cubic extension field

```sh
ic solve-cubic --input docs/ic/fixtures/cubic_f7_public_target.json \
  --seed 1 --max-residuals 100000 --json
```

The input describes `F_p[t]/(t³-c)` and `y²=x³+ax+b`, with field elements
as three canonical coefficients in increasing powers of `t`. The included
fixture has `p=7`, `c=2`, full curve order `347`, and target `[5]G`; its
coordinates and curve order were also computed independently with direct
extension-field arithmetic and point enumeration. The command requires prime
`p` in `7..=101`, a noncube `c<p`, prime `n` in `3..=2000000`, a nonsingular
curve, and valid subgroup points. It counts `#E(F_{p³})`, requires `n` to
divide that order, and requires the cofactor `h = #E/n` to be invertible modulo
`n`. There is no separate cofactor size limit within the stated field and
subgroup bounds. It samples cofactor components, verifies relations in
the full group, then projects each relation by `[h]` before linear algebra.
The projected factor base must contain a nonidentity subgroup point.
`--input` also accepts `store:NAME` or `sha256:DIGEST`; the artifact store
checks the indexed content hash when it fetches the JSON into its cache.
The JSON input is capped at 64 KiB, and `input_artifact` in the result records
the exact reference, SHA-256, and byte count used for the solve. This binds a
local or retrieved target to the reported scalar and route validation.
The [input-identity acceptance receipt](../store/CUBIC_TARGET_INPUT_IDENTITY_ACCEPTANCE_20261023.json)
records a local and a fresh-cache Setec S3 solve, a routed solve through a
digest reference, remote byte checks, and exact-object cleanup.
It reports the Gaudry-style relation and linear-algebra counters and runs a
same-target rho reference unless `--skip-rho` is set.
The checked `ic solve-cubic` output for this input is frozen as
[`cubic_f7_solve_report_20261018.json`](fixtures/cubic_f7_solve_report_20261018.json).

An earlier public input on a curve of order `344 = 8 × 43` is retained as
[`cubic_f7_cofactor8_input.json`](fixtures/cubic_f7_cofactor8_input.json).
Before the full-order check was added, its bounded run used 100,000 residuals
and found zero decompositions; the exact report is
[`cubic_f7_cofactor8_bounded_20261018.json`](fixtures/cubic_f7_cofactor8_bounded_20261018.json).
That observation does not establish that no decomposition exists. The updated
CLI rejects the input before relation collection because all three factor-base
points project to the identity in its order-43 subgroup. The cofactor-12
fixture [`cubic_f7_cofactor12_public_target.json`](fixtures/cubic_f7_cofactor12_public_target.json)
has four nonidentity projected base points and recovers its externally
specified scalar with full-group relation checks and subgroup replay. Its
charged run and same-target rho reference are frozen in
[`cubic_f7_cofactor12_solve_report_20261018.json`](fixtures/cubic_f7_cofactor12_solve_report_20261018.json).
A separate live S3 test fetched this input into a fresh cache through the
`solve-cubic` command and verified scalar 5; its upload, HEAD, remote check,
and cleanup receipt is
[`CUBIC_TARGET_S3_ACCEPTANCE_20261018.json`](../store/CUBIC_TARGET_S3_ACCEPTANCE_20261018.json).

The public order-3 subgroup input
[`cubic_f7_cofactor116_public_target.json`](fixtures/cubic_f7_cofactor116_public_target.json)
uses the same order-348 curve and a coprime cofactor of 116. With seed 1 and
1,000 residuals, `ic solve-cubic --skip-rho` recovers scalar 2, verifies four
relations, and replays the scalar on the supplied points. This exercises the
former 64-cofactor boundary; larger instances remain subject to the field,
subgroup, and relation budget bounds above.

### Checked cubic-field isogeny route

The cofactor-12 input also has a rational order-3 kernel over its specified
cubic field. Both `ic` and `icx` can verify the corresponding graph and
transport the public generator and target before collecting relations:

```sh
ic solve-cubic \
  --input docs/ic/fixtures/cubic_f7_cofactor12_public_target.json \
  --route-file docs/ic/fixtures/cubic_f7_cofactor12_isogeny_route.json \
  --route-id root-to-velu3 --seed 1 --max-residuals 1000 --skip-rho --json
```

The route file can also be an indexed `store:NAME` or `sha256:DIGEST`
reference. Its schema fixes `field_characteristic` and `cubic_nonresidue`,
lists bounded short-Weierstrass `curve_nodes` with canonical three-coefficient
`a` and `b`, and records each edge's source, target, prime degree in
`2..=31`, and affine `kernel_generator`. Each route is a root-origin sequence
of edge IDs. The command verifies every node, edge, and route, checks each
kernel's exact prime order, independently reconstructs the Vélu codomain,
checks the transported point map against doubling and subgroup order, and
counts the destination curve order. A recovered scalar is replayed on both
the destination and the caller's original `G,Q` pair. The rho reference stays
on that original pair. The JSON `route` receipt records the graph hash,
verification counts, destination model, and transported points.
A live run resolved this route through `store:` from an empty cache, recovered
and replayed scalar 5, verified the object in S3, and removed only its isolated
acceptance object. Its nine checks are recorded in
[`CUBIC_ROUTE_S3_ACCEPTANCE_20261020.json`](../store/CUBIC_ROUTE_S3_ACCEPTANCE_20261020.json).

### Cubic-field j graph numeration

Both CLIs can enumerate distinct rational j-invariant neighbours from a
specified cubic-field curve using independently evaluated modular
polynomials:

```sh
ic cubic-graph --p 7 --c 2 --a 3 6 4 --b 4 4 6 \
  --max-ell 5 --max-nodes 32 --seed 1 --json
icx cubic-graph --p 7 --c 2 --a 3 6 4 --b 4 4 6 \
  --max-ell 5 --max-nodes 32 --seed 1 --json
icx cubic-graph --input docs/ic/fixtures/cubic_f7_cofactor12_public_target.json \
  --max-ell 5 --max-nodes 32 --seed 1 --json
ic cubic-graph --p 7 --c 2 --a 3 6 4 --b 4 4 6 \
  --max-ell 2 --include-degree-two --max-nodes 64 --seed 1 --json
ic cubic-graph --p 7 --c 2 --a 0 0 0 --b 1 0 0 \
  --max-ell 2 --include-degree-two --include-multiplicity \
  --max-nodes 8 --seed 1 --json
```

`--input` accepts the same strict public cubic target JSON used by
`solve-cubic`, including a checked `store:NAME` or `sha256:DIGEST` reference.
It supplies `p`, `c`, `a`, and `b` and cannot be combined with those explicit
flags. The graph report records the input reference, SHA-256, and byte count;
point and subgroup checks remain the responsibility of `solve-cubic`.
The [target-file acceptance receipt](fixtures/CUBIC_GRAPH_TARGET_FILE_ACCEPTANCE_20261023.json)
compares both release CLIs with explicit coefficients, records five rejected
inputs, and checks fresh-cache `store:` and digest references against one live
Setec object before exact-prefix cleanup.

The field must be exactly `F_p[t]/(t³-c)` with canonical coefficients,
prime `p` in `7..=101` (necessarily `p ≡ 1 mod 3`), and noncube `c`.
Supported odd prime degrees are 3, 5, and 7 when `p > degree + 1`.
`--include-degree-two` also includes degree 2 when `--max-ell >= 2`; the
default remains the odd-degree graph. Both `ic` and `icx` accept the flag.
`--max-nodes` is bounded to 1–64;
`truncated` records when that limit omits a discovered neighbour. Nodes
are distinct j-invariants in the exact cubic field. Edges record distinct
target j-invariants per degree and pass a second modular-polynomial
evaluation. `--include-multiplicity` adds the root multiplicity of each edge
in Φ_degree(source j, Y), counting repeated roots at special j values. It
does not identify a distinct kernel for each repeated root. The default JSON
omits this field so existing consumers keep the same schema. This numeration
does not include kernel generators. In the j=0 example, the degree-2 edge to
j=2 has multiplicity 3. To transport an IC target, supply a separately
certified `--route-file` as described above.

The public p=7 command above completed without truncation: 6 j nodes and
18 distinct degree-labelled directed edges. Both `ic` and `icx` returned
the same JSON data. A separate exhaustive test evaluated every candidate
target j over all 343 field elements for both degrees and found exactly the
reported neighbours. The frozen CLI output is
[`cubic_f7_j_graph_public_20261021.json`](fixtures/cubic_f7_j_graph_public_20261021.json),
SHA-256 `4e07e7c2fc117ecf8deec4916ac9d0178325f5118464ae5881c980c0ab2069de`.
A separate `--max-ell 2 --include-degree-two` run on the same source returned
4 j nodes and 6 directed degree-2 edges without truncation. Both release
CLIs returned identical JSON, and an exhaustive scan over all 343 field
elements found exactly those degree-2 neighbours for every retained node.
A p=97, c=2 run with degree 3 and a two-node cap completed in 1.144 seconds
on the local host and correctly marked its output truncated; that check
validates command execution at the upper prime range, not complete graph
coverage there.

### From a cubic j neighbour to a checked IC route

For a neighbour reached by a rational prime-order kernel, either CLI can
construct the exact one-hop Vélu route file consumed by `solve-cubic`:

```sh
ic cubic-route-generate \
  --input docs/ic/fixtures/cubic_f7_cofactor12_public_target.json \
  --degree 3 --target-j 3 5 0 --seed 1 --max-samples 256 --json \
  > /tmp/cubic-route.json
ic solve-cubic \
  --input docs/ic/fixtures/cubic_f7_cofactor12_public_target.json \
  --route-file /tmp/cubic-route.json --route-id root-to-velu3 \
  --seed 1 --max-residuals 1000 --skip-rho --json

icx cubic-graph \
  --input docs/ic/fixtures/cubic_f7_cofactor12_public_target.json \
  --max-ell 5 --max-nodes 32 --seed 1 --out /tmp/cubic-j-graph.json --json
icx cubic-route-generate \
  --input docs/ic/fixtures/cubic_f7_cofactor12_public_target.json \
  --degree 3 --graph-file /tmp/cubic-j-graph.json --target-node 1 \
  --seed 1 --max-samples 256 --json > /tmp/cubic-route-from-node.json
icx solve-cubic \
  --input docs/ic/fixtures/cubic_f7_cofactor12_public_target.json \
  --route-file /tmp/cubic-route-from-node.json --route-id root-to-velu3 \
  --seed 1 --max-residuals 1000 --skip-rho --json

ic cubic-route-generate \
  --input docs/ic/fixtures/cubic_f7_cofactor12_public_target.json \
  --degree 2 --seed 1 --max-samples 256 --json \
  > /tmp/cubic-route-2.json
ic solve-cubic \
  --input docs/ic/fixtures/cubic_f7_cofactor12_public_target.json \
  --route-file /tmp/cubic-route-2.json --route-id root-to-velu2 \
  --seed 1 --max-residuals 1000 --skip-rho --json
```

`--graph-file` also accepts a checked `store:` or `sha256:` reference. With
`--target-node`, it requires a direct edge from graph root to the selected
node at `--degree`, one route hop, and the same field and source curve as the
target. A graph generated from `--input` must carry the target artifact's
SHA-256. The modular-polynomial edge is checked again before a rational
kernel search. A j-neighbour without a suitable rational kernel can still
exhaust the stated search budget; the graph alone is not a point map.
`solve-cubic` checks that a graph-selected route records the SHA-256 of the
exact target artifact it reads, along with the selected destination j and
degree.
The [graph-node route acceptance receipt](fixtures/CUBIC_GRAPH_NODE_ROUTE_ACCEPTANCE_20261023.json)
records both CLI outputs, verified scalar 5, rejection cases, and an isolated
S3 upload, remote check, fresh-cache retrieval, solve, and exact-object cleanup.

The generator counts the source curve order, checks the supplied public
points and subgroup, samples a rational kernel of the requested prime
degree in `2..=31`, and checks the Vélu codomain, point maps and subgroup
before returning a route. `--target-j` selects an exact j-invariant from
`cubic-graph`; omit it to accept the first checked rational-kernel edge.
The field and subgroup bounds match `solve-cubic`; `--max-samples` is
1–10,000. A bounded search that does not find a matching rational kernel
exits unsuccessfully. The constructor handles edges with a rational kernel
generator; other j-neighbours remain outside this route format.

For the public p=7 source, the command selected destination j=`[3,5,0]`,
returned the checked codomain `a=[3,5,6]`, `b=[4,4,6]`, and its output was
accepted by `ic solve-cubic`. The solver verified the route and recovered
scalar 5 on the source and destination. The
[generated route](fixtures/cubic_f7_generated_route_20261021.json)
SHA-256 is `fe9e14bfe7e53afa681b1fdad0b41be5e96779930309eb06014f4b269e9ebe0f`.
The same route generator is exposed through `icx`; both release CLIs
produced semantically identical route JSON and exited successfully.
For degree 2, both release generators returned the same kernel edge and
codomain; `ic solve-cubic` accepted the generated route and verified scalar 5
on the source and destination. The generator isolates the prime-power
component of sampled points before selecting a kernel, so a group order
divisible by the degree more than once does not silently defeat sampling.
The generated route also passed a live Setec S3 push, HEAD, remote verify,
fresh-cache `solve-cubic` fetch and scalar replay, exact-object cleanup, and
final HEAD 404. The isolated prefix and checks are in
[`GENERATED_CUBIC_ROUTE_S3_ACCEPTANCE_20261021.json`](../store/GENERATED_CUBIC_ROUTE_S3_ACCEPTANCE_20261021.json).
The degree-2 route passed the same live store path under a separate acceptance
prefix. Its object key, digest, fetch, replay, and cleanup checks are in
[`GENERATED_CUBIC_DEGREE2_ROUTE_S3_ACCEPTANCE_20261022.json`](../store/GENERATED_CUBIC_DEGREE2_ROUTE_S3_ACCEPTANCE_20261022.json).

`cubic-route-generate` also accepts `--max-depth` in 1–4 (default 1). Each
hop samples at most `--max-samples` rational kernels of the same prime degree,
checks the mapped subgroup points, and requires a destination model not
already on the path. If `--target-j` is supplied, it applies to the final
hop. A path that cannot be extended within the budget exits unsuccessfully.
For the public cofactor-12 target, both `ic` and `icx` accept a generated
two-edge degree-2 route:

```sh
ic cubic-route-generate \
  --input docs/ic/fixtures/cubic_f7_cofactor12_public_target.json \
  --degree 2 --max-depth 2 --seed 1 --max-samples 256 --json \
  > /tmp/cubic-route-depth2.json
ic solve-cubic \
  --input docs/ic/fixtures/cubic_f7_cofactor12_public_target.json \
  --route-file /tmp/cubic-route-depth2.json \
  --route-id root-to-velu2-depth2 --seed 1 --max-residuals 1000 --json
```

The [generated two-edge route](fixtures/cubic_f7_degree2_multihop_generated_route_20261022.json)
has SHA-256 `36d6222f3f1a00e5effc107ea76febfedd3d118e9f26725e2a21ed1c02ae798b`.
The solver independently verified its two kernels, codomains, and point maps;
the transported scalar 5 replays on the destination and original public
curve. A degree-2 `cubic-graph` run from the intermediate curve includes the
destination j-invariant `[2,0,4]`; selecting it with `--target-j 2 0 4`
reproduces the two-edge route. The same-target rho reference also recovered
scalar 5. A negative
test mutates the second kernel and verifies rejection. These local timings
remain diagnostics under the repository's IC measurement rules.
The generated route passed a live Setec S3 upload, HEAD, remote download
verification, fresh-cache `solve-cubic --route-file store:...` replay with
same-target rho, byte comparison, pull, exact-object cleanup, and final HEAD
404. The isolated prefix and checks are in
[`GENERATED_CUBIC_MULTIHOP_S3_ACCEPTANCE_20261022.json`](../store/GENERATED_CUBIC_MULTIHOP_S3_ACCEPTANCE_20261022.json).

The prime and cubic commands return `bounded_unknown` with a nonzero exit
status if their configured IC budget ends without a verified scalar. These
finite input ranges describe the implemented solvers; they do not certify
performance or coverage on larger curves. The timings are local diagnostics,
not a claim against the repository's full IC-versus-rho measurement gates.

## AES analysis adapters

The direct `crypto aes` block, CTR, CBC, and GCM commands accept local files
or checked `store:NAME` and `sha256:DIGEST` references for both `--file` and
`--key-file`. For example, `crypto aes block-encrypt --file
store:public/plain.bin --key-file store:public/key.bin` reads verified bytes
from the content store. A [live Setec receipt](../store/AES_FILE_S3_ACCEPTANCE_20261022.json)
covers all seven direct AES operations with public inputs, including
authenticated GCM decryption. Use a private store for private keys.

```sh
crypto cryptanalysis aes-attack --technique differential
crypto cryptanalysis aes-attack --technique linear
crypto cryptanalysis aes-attack --technique square
crypto cryptanalysis aes-trail-check --pattern 0001 --pattern 000f
crypto cryptanalysis aes-sbox-table --kind bct --row 1
crypto cryptanalysis aes-sbox-table --kind dlct --row 1
crypto cryptanalysis aes-differential-observed --input chosen-pairs.json
crypto cryptanalysis aes-higher-order-observed --input plaintext-cube.json
crypto cryptanalysis aes-linear-observed --input known-pairs.json
crypto cryptanalysis aes-linear-parity --input known-pairs.json \
  --column 0 --row 0 --input-mask 1 --output-mask 1
crypto cryptanalysis aes-distinguisher --technique impossible --samples 4096
crypto cryptanalysis aes-distinguisher --technique yoyo --samples 4096
crypto cryptanalysis aes-distinguisher --technique mixture --samples 4096
crypto cryptanalysis aes-distinguisher --technique boomerang --samples 4096 \
  --delta-hex 01000000000000000000000000000000
crypto cryptanalysis aes-distinguisher --technique zero-correlation
crypto cryptanalysis aes-cpa --input traces.json
crypto cryptanalysis aes-cache-timing --input isolated-lookup-timings.json
crypto cryptanalysis aes-mitm-observed --input two-round-delta-set.json
crypto cryptanalysis aes-integral-observed --input lambda-set.json
crypto cryptanalysis aes-dfa --input fault-pairs.json
crypto cryptanalysis aes-dfa-schedule --last-round-key-hex 13111d7fe3944a17f307a78b4d2b30c5
```

These commands expose the existing AES-128 two-round differential,
one-round linear, and four-round integral recoveries. `--key-hex` supplies
the analysis instance; `--samples` and `--seed` configure the differential
and linear corpora. Each recovered result is checked against that instance.
`aes-trail-check` accepts one to ten four-digit hexadecimal active-byte masks
(bit `4*column+row`) and reports each ShiftRows/MixColumns column transition.
It enforces a nonzero starting pattern and the zero-column and branch-number
necessary conditions. Passing those conditions does not establish a realizable
exact difference trail or a differential probability.
The rest of the AES modules have separate input models and remain outside
this adapter; `crypto cryptanalysis sbox`, `auto`, `boomerang`, `rectangle`,
`aes-related-key`, and `aes-visual-demo` provide other entry points.
`aes-distinguisher` exposes five further bounded analyses: a four-round
impossible differential, a two-versus-three-round yoyo comparison, a
one-versus-two-round mixture identity comparison with subspace diagnostics,
an exact AES S-box boomerang connectivity table with a two-round fixed-delta
return measurement, and exact zero-correlation mask enumeration on the AES
S-box. The first four use a supplied AES-128
analysis key; the S-box check concerns the public substitution table. Each
command reports its configured sample count and measured or structural
result. These are distinct input models and do not imply an attack on
full-round AES.

`aes-sbox-table` exports all 256 rows or one `--row` of four exact public
S-box tables: `ddt` counts XOR difference transitions, `lat` gives signed
linear-approximation imbalances, `bct` counts boomerang switch solutions,
and `dlct` gives signed differential-linear switch imbalances. BCT and DLCT
rows include the trivial zero input or output entries; use nonzero indices
when evaluating attack transitions. The CLI reports the entry definition
and row and column counts with the JSON values.
The complete BCT and DLCT exports passed a live Setec S3 round trip; see
[`AES_SBOX_TABLES_S3_ACCEPTANCE_20261022.json`](../store/AES_SBOX_TABLES_S3_ACCEPTANCE_20261022.json).

The `boomerang` technique requires a nonzero 16-byte `--delta-hex` and a
nonzero `--alpha` input difference at byte zero. It reports the exact public
AES S-box off-diagonal BCT maximum plus the empirical return count for that
fixed delta under the declared two-round model. A zero return count is a
valid measurement, not evidence of key recovery. The command does not choose
each delta from the ciphertexts it just observed.

`aes-mitm-observed` accepts 256 plaintext/ciphertext pairs as
`{"pairs":[{"plaintext_hex":"<32 hex digits>","ciphertext_hex":"<32 hex digits>"},...]}`.
The plaintexts must cover `(i, 0, ..., 0)` for every byte value `i`, in any
order. It applies the two-round AES-128 (last round without MixColumns)
sequence-consistency attack and reports candidates for input-key byte zero,
round-two key byte zero, and the intermediate constant. These are partial
key-byte candidates, not a recovered AES-128 master key.

`aes-integral-observed` uses the same pair schema. Its 256 plaintexts must
cover byte zero exactly once while keeping the other 15 bytes equal across
the corpus. It reports the XOR sum of all 256 ciphertexts and the number of
zero output bytes. A zero XOR sum is the Square integral balance expected
after three full AES rounds with MixColumns in the last round; the command
reports an observed balance, not proof of the underlying cipher or its key.

`aes-higher-order-observed` accepts a complete affine plaintext cube and
reports its observed XOR derivative. The JSON contains
`base_plaintext_hex`, `directions_hex` (1–16 independent 16-byte differences),
and `pairs` with `plaintext_hex` and `ciphertext_hex` for every combination of
the directions. Pair order is arbitrary. The command checks independence,
duplicate and missing plaintexts, and exact cube membership before computing
the derivative. It reports the 16-byte XOR sum and zero-byte count; the input
does not include a key or round count, so the report does not verify which
AES construction produced the ciphertexts.

`aes-differential-observed` accepts caller-supplied data for the reduced
two-round cipher (the last round omits MixColumns):

```json
{"alpha":115,"pairs":[{"p0_hex":"00112233445566778899aabbccddeeff","p1_hex":"73112233445566778899aabbccddeeff","c0_hex":"<32 hex digits>","c1_hex":"<32 hex digits>"}]}
```

Supply at least two and at most 100,000 pairs. Each pair must change only
plaintext byte zero by `alpha`, and its ciphertext difference must have
exactly active positions 0, 7, 10, and 13. The result contains four
last-round key-byte candidates at those positions and reports
`partial_candidate`; this does not identify a whole AES-128 key. The command
requires distinct chosen-plaintext pairs with consistent ciphertexts. Each
reported four-byte tuple must imply the same first-round S-box output
difference across all four active positions for every supplied pair, including
pairs after an individual byte has only one candidate.

`aes-linear-observed` accepts `{"pairs":[{"plaintext_hex":"<32 hex
digits>","ciphertext_hex":"<32 hex digits>"}],"verification_pairs":[...]}`.
It requires 16–100,000 training pairs and permits up to 100,000 independent
verification pairs. Training plaintexts and verification plaintexts must each
be distinct. The one-round model omits final MixColumns. The current
byte recovery routine enumerates S-box input-byte candidates and checks a
constant output XOR across observations; its output is not a LAT-derived
multi-round linear attack. It reports `candidate` without holdout data and
`verified` only when the recovered input key reproduces every distinct
verification pair with the reduced-round cipher. Before either status, it
replays every training pair with the complete reduced-AES key schedule;
per-byte consistency alone can also fit independent output whitening bytes.
An inconsistent training corpus reports `training_model_mismatch` with a null
candidate key and exits unsuccessfully. A holdout mismatch reports
`verification_failed` and exits unsuccessfully. Both observed-data commands
accept local files or checked store references.

`aes-linear-parity` exposes the actual S-box LAT majority-vote analysis on
caller-supplied one-round known pairs. Select a ciphertext byte by `--column`
and `--row` (0–3 each) and nonzero 8-bit input/output masks. The command
requires 64–100,000 distinct plaintext samples and rejects a zero-bias LAT
entry. Its output gives the signed LAT entry, theoretical bias, observed
zero/one equation counts, vote margin, and the candidate parity bit
`a·K0[(column+row) mod 4,row] XOR b·K1[column,row]`; an exact vote tie is
`ambiguous`. This is one bit of effective key material for the stated
one-round model. The command does not use `verification_pairs`, so it rejects
inputs that contain them. Its JSON can be loaded through a checked store
reference as well as a local file.

The release CLI acceptance used the public AES-128 key
`000102030405060708090a0b0c0d0e0f` and 4,096 distinct plaintexts. For
sample index `i` from 0 to 4095, the plaintext is the first 16 bytes of
`SHA-256("aes-linear-parity-public-v1" || i.to_bytes(4, "big"))`. Ciphertexts
were independently generated by pre-whitening with the key, applying the
AES S-box and ShiftRows once, then XORing the AES round-1 key; this is the
declared no-MixColumns one-round model. The compact JSON input uses
`{"pairs":[{"plaintext_hex":"...","ciphertext_hex":"..."},...]}` with
no verification pairs. Its SHA-256 is
`cd4162f785affed42a08ecd47b28248c89f01d7212abb36d46f0f80a65e52197`.
With `--column 0 --row 0 --input-mask 1 --output-mask 1`, the LAT entry was
+12; 2,213 equations evaluated to zero and 1,883 to one. The CLI reported
candidate parity bit 0, matching the public key schedule's bit 0 of input
key byte `00` XOR bit 0 of round-1 key byte `d6`. Binary SHA-256:
`0e704c9a2e3637cd53f4cfde6a3e1927a5204dbbaa3073ece3864b2ee129d15b`;
report SHA-256: `2b91b90f1509eb6fca644d25962603747a5ec0ee91621d3b92b07c967698c031`.
The same input with masks 1 and 2 was rejected because that LAT entry is zero.

`aes-cpa` accepts JSON with `plaintexts` as 16-byte hex strings and
`traces_per_byte` as sixteen equal-length arrays of finite measured values.
`--input` accepts a local path or an indexed `store:NAME` or
`sha256:DIGEST` reference. The store downloads and checks the indexed digest
before analysis; trace JSON is limited to 64 MiB.
Each channel corresponds to first-round S-box leakage for the matching
plaintext byte. It requires 16–100,000 aligned samples and reports the
candidate AES-128 key, peak and runner-up absolute correlations, and the
first-round Hamming-weight leakage model. Pass
`--known-ciphertext-hex HEX` for the first plaintext to verify the full
ten-round AES encryption; without that check the status is `candidate`.
This command analyzes supplied traces. Trace acquisition and alignment
depend on the measured device.

`aes-dpa` accepts the same checked trace JSON and optional ciphertext:

```sh
crypto cryptanalysis aes-dpa \
  --input docs/ic/fixtures/aes_cpa_traces_public_20261022.json \
  --known-ciphertext-hex 69c4e0d86a7b0430d8cdb78070b4c55a
```

For each first-round key-byte guess it partitions traces by each of the
eight predicted AES S-box output bits, then sums the eight squared
differences between the two sample means. The output reports the leading
and runner-up energies in squared trace units. The supplied ciphertext
checks the candidate key with a full AES-128 encryption. Without that check,
the result remains a candidate; trace acquisition and alignment still belong
to the measurement setup.

The release CLI acceptance report is
[`aes_cpa_public_report_20261018.json`](fixtures/aes_cpa_public_report_20261018.json).
It uses 512 noiseless Hamming-weight samples per byte with key
`000102030405060708090a0b0c0d0e0f`. The first plaintext is
`00112233445566778899aabbccddeeff`; for sample `i` from 1 to 511,
plaintext byte `j` is `(37i + 19j) mod 256`. Each trace is the bit count
of the AES S-box output at that plaintext byte XOR the matching key byte.
The input JSON SHA-256 is
`81b50a60c0361b33d7df6fe73cf5827942ce8dd10ff6284e931c97172a9ed2d8`.
The same public trace values are preserved as compact JSON in
[`aes_cpa_traces_public_20261022.json`](fixtures/aes_cpa_traces_public_20261022.json).
That file's SHA-256 is
`636eef3989b37b9db88b43996b5ef841c5fd62eb90870e4ec867b1920a84d0d1`;
the live S3 CPA receipt records the checked upload and fresh-cache replay.
The known-answer ciphertext is `69c4e0d86a7b0430d8cdb78070b4c55a`.

`aes-cache-timing` accepts JSON with `plaintexts` as 16-byte hex strings
and `timings_per_byte` as sixteen arrays aligned to those plaintexts. Each
array must measure one separately instrumented first-round lookup, with a
faster observation when that lookup reaches cache line zero. The command
requires 64–100,000 observations, finite timings, and at least four samples
in every plaintext high-nibble bucket for every byte by default. It returns
one candidate **high nibble** per key byte, the least populated bucket count,
and the gap between the two lowest bucket means. Equal minimum means leave
a byte unresolved. The status is `candidate` or `ambiguous`; neither means
a full AES-128 key was recovered. Ordinary whole-encryption timings do not
satisfy this isolated-lookup input model. The runnable public corpus and
checked release result are
[`aes_cache_timing_public_input.json`](fixtures/aes_cache_timing_public_input.json)
and
[`aes_cache_timing_public_report_20261020.json`](fixtures/aes_cache_timing_public_report_20261020.json).
The corpus has 64 observations, with four samples in every bucket. It uses
an exact zero-noise isolated-lookup model for a public key and is an input
contract check, not evidence about a device's timing leakage.

`aes-dfa` accepts 1–64 correct/faulted ciphertext pairs under a single-byte
fault at the start of AES-128 round 9, before SubBytes. The input is a JSON
object with a `pairs` array. Every entry has `plaintext_hex`,
`correct_ciphertext_hex`, and `faulted_ciphertext_hex`, each exactly 16 bytes
in hex. The command checks that each pair has the expected four-byte
diffusion pattern, intersects round-10 key candidates by affected column,
reverses the key schedule when every column has one survivor, and verifies
the master key by encrypting every supplied plaintext. It also replays each
faulted ciphertext by injecting a nonzero byte fault at the declared round-9
boundary under that key. An unresolved set of
candidates is reported as `bounded_ambiguous` with a nonzero exit status;
contradictory or malformed pairs also fail. The input may be a file path or
a `store:`/`sha256:` artifact reference. The reproducible public input and
result are
[`aes_dfa_round9_public_input.json`](fixtures/aes_dfa_round9_public_input.json)
and
[`aes_dfa_round9_public_report_20261018.json`](fixtures/aes_dfa_round9_public_report_20261018.json).
The input SHA-256 is
`1b1eb355e4c71ed6b5b194987ce03f4a887c2648e7f42dc688c2739edbfee8a1`.
Its 16 correct ciphertexts also match independent OpenSSL AES-128-ECB
encryption under the recovered key. The corpus varies all four injected
source rows in each of the four MixColumns columns; every column has one
surviving round-key tuple. The release report records
`faulted_pairs_replayed: 16` after the recovered master key reproduces every
faulted ciphertext through an injected round-9 byte fault.

`aes-dfa-schedule` reverses a supplied AES-128 round-10 key to the master
key and checks that forward expansion reproduces it. The example round key
maps to master key `000102030405060708090a0b0c0d0e0f`. The output is in
[`aes_dfa_schedule_public_report_20261018.json`](fixtures/aes_dfa_schedule_public_report_20261018.json).

## SHA-family cryptanalysis adapters

```sh
crypto hash sha256 --hex 000102ff
crypto hash sha3-224 --hex 000102ff
crypto hash sha3-256 --hex 000102ff
crypto hash sha3-384 --hex 000102ff
crypto hash sha3-512 --hex 000102ff
crypto cryptanalysis hash-auto --hash sha256
crypto cryptanalysis hash-auto --hash sha224
crypto cryptanalysis hash-auto --hash sha384
crypto cryptanalysis hash-auto --hash sha512
crypto cryptanalysis hash-auto --hash sha512-224
crypto cryptanalysis hash-auto --hash sha512-256
crypto cryptanalysis hash-auto --hash blake3
crypto cryptanalysis hash-auto --hash sha3-224
crypto cryptanalysis hash-auto --hash sha3-256
crypto cryptanalysis hash-auto --hash sha3-384
crypto cryptanalysis hash-auto --hash sha3-512
crypto hash shake128 --output-bytes 32 --hex 000102ff
crypto hash shake256 --output-bytes 32 --hex 000102ff
crypto cryptanalysis hash-auto --hash shake128 --output-bytes 32
crypto cryptanalysis hash-auto --hash shake256 --output-bytes 32
```

The `crypto hash --file` argument also accepts `store:NAME` or
`sha256:DIGEST`. The artifact store checks the indexed content digest when
it downloads the input into an empty cache. This applies to SHA-256,
SHA-3, SHAKE, cSHAKE, and the other unkeyed hashes handled by the command.

`hash-auto` runs a bounded 32-bit-truncated birthday search and a 4,096-trial
single-input-bit differential-bias measurement for each SHA-3 fixed-output
variant. These measurements concern truncated outputs and empirical bias;
they do not establish collisions or differential weaknesses for the full
digests. SHA-3 uses a sponge, so the command records that the
Merkle-Damgård length-extension method does not apply. SHA-256 has a
separate length-extension adapter with explicit prefix length and digest
input. SHAKE128 and SHAKE256 require an explicit `--output-bytes` value in
`1..=64` for `hash-auto`. The bounded analysis treats that many output bytes
as the digest, truncating the birthday search to at most 32 bits, and reports
per-byte differential measurements. The standalone `crypto hash` command
supports longer SHAKE outputs independently.

All fixed-output SHA-2 algorithms exposed by `crypto hash`, plus BLAKE3,
have bounded `hash-auto` birthday and differential reports. SHA-512 and
SHA-256 expose their complete chaining state and also support the explicit
`crypto cryptanalysis length-extension` command for a bare secret-prefix
digest with a supplied secret length. SHA-224, SHA-384, SHA-512/224,
SHA-512/256, and BLAKE3 have no length-extension adapter in this CLI; their
reports make no claim that these constructions resist every extension
technique. The birthday section concerns only a 32-bit truncated output.

The `crypto hash` command also exposes the other built-in unkeyed digest
implementations on text, `--hex`, or `--file` input:

```sh
crypto hash md4 --hex 616263
crypto hash md5 --hex 616263
crypto hash sha1 --hex 616263
crypto hash blake2b --output-bytes 32 --hex 616263
crypto hash blake2s --output-bytes 16 --hex 616263
crypto hash skein256 --hex 616263
crypto hash skein512 --output-bytes 32 --hex 616263
crypto hash skein1024 --hex 616263
crypto hash sm3 --hex 616263
crypto hash streebog-256 --hex 616263
crypto hash streebog-512 --hex 616263
crypto hash whirlpool --hex 616263
crypto hash tiger --hex 616263
crypto hash tiger2 --hex 616263
crypto hash ripemd160 --hex 616263
```

SHAKE requires `--output-bytes` in `1..=1048576`. BLAKE2b and BLAKE2s
default to 64 and 32 bytes and accept lengths in `1..=64` and `1..=32`;
their shorter digests use the algorithm's output-length parameter. Skein-256,
Skein-512, and Skein-1024 default to 32, 64, and 128 bytes and accept
`--output-bytes` in `1..=1048576`. Fixed-output hashes reject that flag.
Keyed BLAKE2, HMAC, KMAC, and SipHash use the separate `crypto mac` command.
cSHAKE128 and cSHAKE256 use `crypto hash` with an explicit output length and
optional hex-encoded function name and customization strings:

```sh
crypto hash cshake128 --output-bytes 32 --custom-hex 456d61696c205369676e6174757265 --hex 00010203
crypto hash cshake256 --output-bytes 64 --function-name-hex 54657374 --hex 00010203
```

When both strings are empty, cSHAKE produces the corresponding SHAKE output.
Other hash algorithms reject `--custom-hex` and `--function-name-hex`.

### Keyed digests and MACs

`crypto mac` reads exact message bytes from a UTF-8 argument, `--hex`, or
`--file`, and reads exact key bytes from `--key-file` or `--key-hex`. Prefer
`--key-file` for private keys so the key is not placed in shell history or
process arguments. Both file options accept local paths or indexed
`store:NAME` and `sha256:DIGEST` references. For example:

```sh
crypto mac hmac-sha256 --key-file key.bin --file message.bin
crypto mac blake2b --key-file key.bin --output-bytes 32 --hex 000102ff
crypto mac kmac128 --key-file key.bin --output-bytes 32 --custom-hex 646f6d61696e --hex 000102ff
crypto mac siphash24 --key-hex 000102030405060708090a0b0c0d0e0f --hex 00
crypto mac hmac-sha256 --key-file key.bin --file message.bin --verify-tag-hex TAG
```

A live Setec S3 acceptance used public message and key bytes to check
SHA-256, SHA3-256, SHAKE256, and HMAC-SHA256 from an empty store cache; see
[`HASH_MAC_FILE_S3_ACCEPTANCE_20261022.json`](../store/HASH_MAC_FILE_S3_ACCEPTANCE_20261022.json).

The command exposes HMAC-SHA-1/256/384/512, keyed BLAKE2b/BLAKE2s,
keyed BLAKE3, KMAC128/256 and KMACXOF128/256, and SipHash-2-4. KMAC requires
`--output-bytes` in `1..=1048576`; keyed BLAKE2 defaults to its full output
and accepts lengths up to 64 or 32 bytes. BLAKE3 requires a 32-byte key and
SipHash a 16-byte key. SipHash prints the 64-bit result as a 16-digit
big-endian numeric hexadecimal value. An empty key is rejected by this CLI.
`--verify-tag-hex` compares a full-length tag in constant time after checking
its encoding and length. It prints `verified` and exits successfully on a
match; malformed, truncated, or different tags fail with a nonzero exit.
