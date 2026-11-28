# Full-law local coefficient selector census

**The auxiliary-prime selector returns exactly the residue pair `{2,10}` for 372 of 1,722 ordered inputs, and returns no residue for the other 1,350.** On every one of the 372 selected pairs, eight of the frozen centered CRT curves have a local relation: four at residue 2 and four at residue 10. Across all 32,256 curves, this is 1,488 candidates at each residue and 29,280 with no local coefficient residue modulo 12.

The input law excludes the identity and equal target/generator points on the 43-point group, so its exact coefficient domain is `k=1,…,41`. The two residues lift to seven coefficients in that domain: `{2,10,14,22,26,34,38}`. The actual reduced coefficient lies in the selected set for **72 of 1,722 inputs**, or **72 of the 372 selected inputs**. These are complete finite-law counts for the specified CRT policy, not sampling estimates.

| Stage | Exact count |
| --- | ---: |
| Ordered input pairs | 1,722 |
| Centered CRT candidates | 32,256 |
| Local gate curves at 7 / 11 | 4,032 / 10,752 |
| Local relation tests / generator steps | 177,408 / 162,624 |
| CRT candidates with a local coefficient residue | 2,976 |
| Pairs with a nonempty local residue union | 372 |
| Pairs whose actual reduced coefficient is in the local union | 72 |
| Analysis-only reduced relation tests / generator steps | 74,046 / 72,324 |

The [protocol](protocol.json) and [runner](run.py) were pushed before measurement in `d07f4fc71a4`. They reuse the frozen full-law CRT policy without reading a reduced-relation flag to select curves. For each local gate curve they enumerate the 12 residues using the lifted point pair; for each CRT candidate they intersect the two local sets, then union candidates at the source-pair level. Only after this selector is fixed does the runner enumerate the actual reduced coefficient for analysis. That 43-step-per-pair classification is explicitly recorded as analysis work and is not an available coefficient oracle for a public target. The [result](result.json), complete [pair rows](pair-rows.jsonl.gz), and complete [candidate rows](candidate-rows.jsonl.gz) retain statuses and raw digests.

The independent [checked-Sage audit](sage_audit.py) reconstructs and checks every one of the 32,256 candidate residue sets and all 1,722 reduced coefficients with Sage point arithmetic. Its [result](sage-audit-result.json) passes and matches both raw-row digests. The [runtime receipt](sage-runtime-info.json) records the accepted installed build; stderr is empty. The full-law CRT packet separately audits the curve selection and order-4/order-6 gates.

The [Lean local-period theorem](../xedni_periodic_coefficient_20261120/formal/LocalPeriodicity.lean) explains why the same curve's local tests depend only on `k mod 12` under the order-4/order-6 annihilation hypotheses. The exact `{2,10}` pattern and 72-input coverage are computational census results, not consequences of that theorem alone. They apply to this field, point law, centered CRT representative, and auxiliary-prime gates. A rational-relation experiment on the newly selected residue-10 arm is the next concrete test; it must count the coefficient rule and preserve every zero and excluded pair. The preexisting 2,249,856 local coefficient-residue checks and 32,256 curve constructions remain part of the curve-selection work when assessing total cost.

For a fresh checkout, [hydrate the sharded candidate stream](REPLAY_STORAGE.md) before these frozen replay commands, then dehydrate the temporary whole-file copy afterward. The decompressed row digest is unchanged.

Replay:

```sh
python3 research/exotic_ecdlp_20261021/xedni_local_selector_20261120/run.py > research/exotic_ecdlp_20261021/xedni_local_selector_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_local_selector_20261120/run.stderr
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_local_selector_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_local_selector_20261120/sage_audit.py > research/exotic_ecdlp_20261021/xedni_local_selector_20261120/sage-audit-result.json 2> research/exotic_ecdlp_20261021/xedni_local_selector_20261120/sage-audit.stderr
```
