# CT329 result — account for the original F103 BSGS's executed EC additions

2026-11-23 UTC. **Completed scoped source-pin/native correctness audit**, not an ECDLP performance result or signed independent PCC. [pcc:unverified]

## Executed result and exact boundaries

Source Git blob: `tests/ct238_native_cubic_norm_compression.rs@234e14f2aea35b8ef3203065a5e2468aae9eb71f`. The original `direct_bsgs`, `ec_add`, `ec_mul`, rho and CT241 collector are **unchanged**; `ct329_compose.py` composes a dependency-free test translation unit from their original source and the separately authored counted BSGS wrapper. The source-gate rejects a changed source before producing the translation unit.

[Exact-head CT329 hosted native Actions #45134941694](https://github.com/isogeny-labs/crypto/actions/runs/45134941694), job `137558879089`, **completed success** at branch commit `7e134c4db560c66172f73cb377000a005b4ddafb`, Actions merge-checkout `44c4ddf261512213f9639725100f17ab3f050353`. Hosted `rustc 1.100.0`. Generated translation unit SHA-256 `49e98be54d0138c00e6909023468193896c317949a22627e5b9518e10c60e2b9`, appended Rust audit SHA-256 `a6ed287cb1b634bc228bbe193fd9f239cfa53582b1b54efa1bb12fad9c024dae`. The full workflow log is linked from Actions; the source-mutation negative was rejected and both compiled Rust tests succeeded.

### Exact public Q73 operation counts

| Executed group-add boundary | Original partial `ops` | New instrumented executed count |
|---|---:|---:|
| Build 11 baby table entries | 11 | 11 |
| Build giant step `[-11]G` (4 doubles + 3 adds) | omitted | 7 |
| Perform six giant steps before hit | 6 | 6 |
| Verify `[73]G=Q` (7 doubles + 3 adds) | omitted | 10 |
| **Total EC additions within original BSGS** | **17** | **34** |

The old `ops=17` is correct **for its stated partial code boundary**, but not as a count of all executed `ec_add` in that same BSGS routine; this audit identifies the omitted 17. **Neither the original measured BSGS elapsed time nor CT241 frozen rows change.** The original elapsed clock already includes the omitted calls. This is a measured/source-checked *accounting correction*, not a 2× solver throughput improvement.

- Native compiled Rust original-BSGS vs counted-wrapper comparison: **109/109 public F103 logs** agree with each other and the independent `[k]G=Q` scalar check; every off-by-one forged scalar rejected; counted additions range **18–39** for the 109 targets. Native `CT329_Q73_COUNTS original=17 baby=11 step=7 giant=6 verify=10 total=34` and `CT329_CORRECTNESS_PASS logs=109 min_total=18 max_total=39` observed. The same binary also passed the earlier original-source CT327 rho 109-target test and 110 negative/forged-scalar assertions: **2 Rust tests passed / 0 failed**.
- Separately authored Python EC implementation counted **109/109** source BSGS logs and predicted the same Q73 counts, minimum and maximum. Normal Python and Python `-O` outputs matched byte for byte. This same-producer, different-language check is **not independent signed validation**.
- The Q73 diagnostic rho's existing 109-target all-log source test reports 48,147 total EC additions across targets, **not a Q73-only count**. Do not divide aggregate counts by one source log or compare their ratio with Q73 BSGS as a measured full attack cost.

## Mathematical transfer filter (no new speed proof)

For the explicit separable degree-three map `pi:C/F103 -> E/F103` defined by `pi(u,v)=(u^3,v)`, `C:v^2=u^9+u^3+24` and `E:y^2=x^3+x+24`, standard norm/pullback composition is `pi_* pi^*=[3]` on `E`. Thus on a cyclic subgroup of order 109, `pi^*` is injective; `[73]pi_*` recovers the source since `3*73 = 219 ≡ 1 (mod 109)`. The transferred log equals the source log for the image subgroup. This is a structural equality, **not** a change in 109-subgroup generic rho complexity, an isogeny into another characteristic, or a new complementary-factor transfer. The CT224/225 `Hom(E,B)=0` analysis concerned a **different F101 curve**, and is not extrapolated to this model.

## Reproducibility and unresolved gates

Reproduce from the [CT329 protocol](CT329_BSGS_GROUP_COST_BOUNDARY_PROTOCOL.md) and [workflow](../../../.github/workflows/ct329-f103-bsgs-cost-boundary.yml):

```sh
python3 -B research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct329_independent_oracle.py
python3 -B research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct329_compose.py \\
  --source tests/ct238_native_cubic_norm_compression.rs \\
  --append research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct329_full_bsgs.rs.inc \\
  --out /tmp/ct329-source.rs
rustc --edition=2021 -O --test /tmp/ct329-source.rs -o /tmp/ct329-tests
/tmp/ct329-tests --test-threads=1 --nocapture
```

The isolated build does not exercise the **original root crate**, Cantor solver, original cold IC/rho/BSGS timings, full source-group setup/validation symmetry, physical field operation counters, peak memory or any 695/701-record native Jacobian replay. The inherited `src/cryptanalysis/binary_hyperelliptic_index_calculus.rs` root syntax remains another owner's blocker for full CT327 cold triplets; do not rewrite it in this lane. The original six S3 objects, cost16 custody gap, real `C169/C13a/C13b/N2_actual/D`, fully charged Prime31 transfer benchmark, strongest matched rho/BSGS/IC, repeated same-host uncertainty, cross-host replay and independent signed PCC remain unresolved. All paid/EKS, protected-merge and read-only vault holds remain binding. [pcc:unverified]

## Actual roles, ownership and next decision

No native agents/subagents are exposed in this runtime. One coordinator used parallel read-only connector checks for mathematics (separable map/injective kernel and F101-only obstruction), Rust (pinned original F103 functions and unmodified collector ownership), and validation (CI/source SHA, changed-source rejection, independent Python finite EC oracle, counted boundary). These were **not three independently executing workers**, and no elapsed-time acceleration is claimed. Only the coordinator wrote in the existing [draft #4973](https://github.com/isogeny-labs/crypto/pull/4973). The shared norm-reuse/cost16 owner [#4990](https://github.com/isogeny-labs/crypto/pull/4990), [cryptanalysis #1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523) and original frozen fixtures were not modified.

Next: root owner restores compile → preserve the preregistered three Q73 **actual cold IC/rho/BSGS** triplets (only after checking job reservations), with all native setup, transfer, linear algebra, recovery, verification, RSS and operation counters; independently replay and compare with strongest relevant rho/BSGS/IC. If original custody authorization reopens, prioritize native 695/701 and physical norm-point reuse prices. Do not claim a speedup from this standalone accounting audit.
