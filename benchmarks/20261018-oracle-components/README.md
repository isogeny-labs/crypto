# Oracle, presentation and factorial component controls

The [combined report](report.pdf) and four SVG/150 dpi PNG figure pairs copy the
[source study](../../research/ideas_execution_20261018/oracle_integration/README.md).
Exact trials, failures, replay certificates and source digests remain there.

The current-source and published-release factorial sessions each retained 224
outcomes: 152 verified and 72 exhausted. All 224 deterministic payloads agreed;
the current-source audit replayed all 152 verified runs. This does not admit a
full-cost ranking: the historical cost model and unpriced exact oracle and
bookkeeping remain explicit. The report rejects timing admission on this
shared L0/Tier 0 diagnostic host.

Other component checks include 48 full-oracle root-set gates, partial quadratic
presentation graphs and n83 membership encoding/replay. They do not establish
four complete alternative presentations, a valid fixed-target equivariant F4
replacement, subgroup-useful n83 rank, or a complete IC speedup.

## Additive curve metadata registration

The post-run [native export](registration-native.json) verifies the four exact
constructors p=13,17,29,37, seed=1 against all 32 frozen workloads. Native checks
cover prime p/r, Rabin degree-four field witnesses, nonsingularity, nonidentity
G with [r]G=O, a unique multiple of r in the exact Hasse interval, rational
2-torsion, and all frozen target points. The [metadata receipt](registration-verification.json)
recomputes full EC1QpD4/UID preimages and proves old crosswalk entries unchanged.

| p | Group order | Subgroup order r | Exact new representation alias |
|---:|---:|---:|---|
| 13 | 28,772 | 7,193 | EC1Q13D4Ctau2h0a8c321bb635 |
| 17 | 83,492 | 20,873 | EC1Q17D4Ctau2hd33452c3120a |
| 29 | 708,028 | 177,007 | EC1Q29D4Ctau2hb7e4ca0ac1bb |
| 37 | 1,874,452 | 468,613 | EC1Q37D4Ctau2h78eaf2768f3d |

Only [the IC crosswalk](../../docs/curves/ic/curves.yaml) gains aliases. All four
canonical model slugs are absent from the main `registry.json`; the current
ecbench Fp4 branch still emits `registered=false`, `ec1=null`. Trait parsing
skips explicitly model-linked crosswalk entries rather than inventing arithmetic.
Frozen plans/sessions and the historical report are unchanged by registration.

The initial YAML serialization was refused by the strict native reader before
any trait digest was written. The [separate format correction receipt](registration-yaml-correction.json)
retains that failure: only the four new entries were rendered with indented
sequences and unwrapped scalars; exact hash preimages and old bytes are unchanged.
The traits owner performs the final native dependency-equivalence/freshness check.

## Current metadata erratum (2026-10-18)

The four post-run candidate aliases described above are no longer present in the authoritative IC crosswalk. Their [exact research-only snapshot and correction provenance](../../research/ideas_execution_20261018/fp4_metadata_erratum/README.md) preserve every field/curve preimage and computed identity without claiming canonical model or representation registration. The native registration receipts and historical reports above remain unchanged at their original source bindings.

References above to current source mean the captured run revision. The example has since received formatting and enum ownership maintenance; the [additive source binding](../../research/ideas_execution_20261018/oracle_integration/historical-label-binding-v2.json) retains the exact measured source. Software checks of the revised example do not remeasure these sessions or make their timings applicable to its changed allocation behavior.
