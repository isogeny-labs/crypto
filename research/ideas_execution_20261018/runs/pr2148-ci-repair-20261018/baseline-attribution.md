# Five-check attribution, crypto PR 2148

Read-only review at head `eaedaf1c132ed1f1ff101edd10c75369ab5e2dfe` against base `b7cafe04485eb97d873dffef698e7337cfb38cfe`. Source/tree comparisons used `GIT_NO_LAZY_FETCH=1`. No repository/Git mutations, test runs, research computations, or benchmark reproductions were performed. Only this attribution record was written under `/tmp`.

| Check/job | Attribution | Concrete evidence |
|---|---|---|
| certificates / 117628500480 | Pre-existing deterministic stale-count assertions | Log lines 3547-3563: `src/bin/hyperelliptic_cover.rs:985` expects 2100 catalog models but reads 2572; line1123 expects134 checked binary representations but finds140. Source, registry and cover graph blobs are identical on base and head. |
| canonical identities and upstream mirror / 117628500268 | Change-caused metadata incompatibility | Log lines314-355: two negative-control tests fail. New four Fp4 entries violate unchanged schema/validator, unchanged mirror lock no longer hashes curves.yaml, and four non-null model slugs are absent from unchanged model registry. |
| backlog lab / 117628500207 | Pre-existing deterministic fixture/contract failures | Log lines974-1086: concurrent-claim test expects unclaimed shard1 although unchanged seed card declares count1; unchanged blocked quintic card has unbound `{artifact}` in outputs.verify, causing both committed-card validation and unchanged publication tests to fail. |
| IC end-to-end / 117628500013 | Observed timing-only gate failure; no change-caused algorithm regression demonstrated | Log lines1052-1065: all three rungs have identical counters,32/32 verified IC and rho answers; only n31 wall ratio0.029 misses frozen floor0.031. Workflow, benchmark script, frozen reference and ic binary source blobs unchanged. All changed src hunks are tests or a comment. Environment/timing variation is plausible; this review does not establish its cause. |
| ecbench harness / 117633871401 | Pre-existing incomplete committed sessions | Log lines999-1008 lists four old gaudry scratch sessions with `No such file or directory`. Each base/head tree contains only plan.json, records.jsonl, session.json; required host.json/spec.json are absent in both. Audit implementation and workflow/script unchanged. New oracle session is not among failures. |

## Workflow commands and source proofs

`certificates` here belongs to `.github/workflows/curve-covers.yml:92`, not the separate Jacobian certificates workflow. Its failing command is `cargo test --bin hyperelliptic-cover` (line109), after the hyperelliptic and cover-transfer controls passed. `src/bin/hyperelliptic_cover.rs:888` embeds docs/curves/registry.json and line889 embeds docs/curves/cover-links.yaml. Exact base/head common blobs:

- wrapper source: `5041317bcff0bc0c676530839ed489c6478cfc62`
- model registry: `e0c176272b03cbe15f3a3c24b78d23e826ca8d51` (2572 rows on both revisions)
- cover graph: `751bfa2a9da16e70efa30c2e044eff50d7854114`

Semantic workflow `.github/workflows/ic-semantic-metadata.yml:42` runs the negative controls before fetching cryptanalysis main. The failure is therefore already local metadata invalidity; this job never reaches upstream byte comparison. The four entries appended at `docs/curves/ic/curves.yaml:254` onward each cause five errors:

1. Missing required trait_status.
2. Missing required representation_links.
3. icv1_identity.status=`verified_constructor_model_not_main_registry_entry` is outside its enum.
4. icv1_identity.model_json is forbidden by that object's additionalProperties=false.
5. `check_curve_math_metadata` (`validate_semantics.py:120`) recognizes coefficients/coefficients_a1_a2_a3_a4_a6/a2/a6 but not the new a/b form, reporting exact coefficients missing.

Those20 errors explain the proposed-link control failure. The full committed-metadata control adds the stale curves.yaml mirror-lock digest and four absent model registry mappings, giving25 errors. Mirror-lock remains `7beabb9634e873a9760e5106af2bc60c096ccb01`; schema remains `35ba7a609851d0aa136d0d029e73e0302101e9fd`. curves.yaml changes from blob `58040903e91756deb29675650f813ebcf2f1b47e` to `e147a9aa797dff2b3c0a5b3d2e5e0058c1dfc943`.

Safe repair considerations, for the owner of fixes: preserve the exact field/curve records and frozen plans. Adding a coefficients member inside curve changes the EC1/UID preimage; do not repair validation by silently changing it. Instead recognize and validate the existing exact short-Weierstrass a/b shape, add the required unknown/null trait metadata and empty representation inventories outside curve, and use the schema's unresolved identity with null slug/full while retaining computed unregistered identity as separate provenance. Any alternative that registers a model must satisfy actual model registry support. The authoritative cryptanalysis metadata/validator change and lock must land first, then the matching crypto mirror and lock; changing only crypto's mirror-lock does not satisfy upstream equality. Old three entries and raw evidence remain untouched.

Backlog workflow `.github/workflows/backlog.yml:146` runs `BACKLOG_REQUIRE_CAIRN=1 cargo test --release --test backlog_cli -- --nocapture` after installing its pinned cairn. Baseline-identical blobs: tests/backlog_cli.rs=`5c8fdcc2f2a74d1802172815e09624d53c46c20d`, src/bin/backlog.rs=`9b7cfd380b0b557a4e3d9425d8f0f5354be0edbe`, one-shard card=`e15dfbd8bca232c459ff7212805618afcee1939d`, quintic card=`7789a2821deb79218e4144b1993ddb64bfa14c72`, workflow=`0c51ddbc2752db9dba3479804259787a3c694503`. At tests/backlog_cli.rs:401 the test explicitly searches for shard1; the card declares shards.count=1 and only m3-n21/shard0. The demo-search TODO failures in the log are a passing negative control, not additional failing tests.

IC end-to-end workflow `.github/workflows/ic-e2e-benchmark.yml:73` calls its checker with wall-ratio tolerance0.5. Baseline-identical blobs: workflow=`c880fb92ca6165286707cedb05a77609c7833482`, benchmark script=`041b8e6e49e6598653dcfbb33c740f717127f4b5`, frozen reference=`ad0cb98522d2aa62ab9d00e3e82ce30951b82bf2`, src/bin/ic.rs=`1db29c48a0a70f8af62602fe3ea1fbdfc4bd3fa7`. No baseline or head benchmark rerun was attempted. Do not change the frozen reference or tolerance merely to remove this red check.

Ecbench workflow `.github/workflows/ecbench.yml:136` discovers every session by its session.json schema, then line405 runs `.github/scripts/ecbench-audit-sessions.sh` with replay0 and allow-interrupted. Audit source `src/cryptanalysis/ecbench/audit.rs:283` reads host.json and line298 reads spec.json; the four failed sessions have neither on base/head. Their trees and the script/workflow/audit blob IDs are unchanged. These are incomplete historical commits, not sparse checkout omissions: git ls-tree itself contains only the same three files per directory. Required immutable sessions should not be rewritten or bypassed to make CI green.

The full `src` diff has only workload.rs, gaudry_quartic.rs and jacobian_transfer/mod.rs; every changed executable hunk is within #[cfg(test)] modules, with workload's only production hunk a comment. Thus the reviewed timing failure has no new production algorithm path attributable to this diff.
