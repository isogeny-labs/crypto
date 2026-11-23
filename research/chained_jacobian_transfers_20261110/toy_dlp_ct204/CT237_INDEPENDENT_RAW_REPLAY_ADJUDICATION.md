# CT237 — independent Python raw relation and full Q73 replay (2026-11-19)

**Status: hosted bounded replay PASSED; [pcc:unverified].** This independently reconstructs the already-frozen CT235 native Rust relation stream in Python; it does **not** independently time native Rust, and is not a growing-prime-field attack.

## Reproducibility and custody

- Source branch `research/ct204-odd-cover-toy-dlp-20261118` (draft [#4715](https://github.com/isogeny-labs/crypto/pull/4715)); exact-source replay [GitHub CI #44570543398](https://github.com/isogeny-labs/crypto/actions/runs/44570543398), job `135724047056`, **success** from `a2e89b1c1840e5124c68bee099d1584e1180adf4`.
- [Replay implementation](ct237_independent_raw_relation_replay.py) imports only the separately authored [CT223 Python polynomial/Cantor arithmetic](ct223_extension_norm_oracle.py) as its Jacobian arithmetic; it does **not** call the original native Rust Jacobian or read Rust relation matrices.
- Frozen native [CT235/236 panel](ct235_native_panel.jsonl) remains **unchanged** (16 genuine unique records, four per basis). The independent replay read only the public curve/offset/target specifications and that panel **after** computing the relations.
- Exact checked command:
  ```sh
  cd research/chained_jacobian_transfers_20261110/toy_dlp_ct204
  python3 ct237_independent_raw_relation_replay.py ct235_native_panel.jsonl
  ```

## Finite exact replay result

Fixed source `E/F101: y²=x³+x+46`, `G=(2,37)`, `r=109`, public `Q=(19,37)`; fixed genus-4 cubic cover `C:y²=u⁹+u³+46`. For each accepted fully split degree-4 divisor the Python replay checks, *without using discrete logs*, the source group identity

```text
sum(norm(C_support_points)) - sum(norm(offset_points)) == [3k]G
```

where `norm(u,v)=(u³,v)` and `k` is the public relation-walk multiplier. It independently rebuilds weighted two-large-prime elimination, factor-base Gaussian pivots, recovered large-prime logs, and the sole public-target discrete logarithm.

| Signed basis | Raw accepted group identities checked | Rank | Walk candidates | LP pivots/known logs | Independent recovered Q |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 24 | 66 | 24/24 | 2,882 | 28/28 | 73 |
| 36 | 60 | 36/36 | 1,596 | 17/17 | 73 |
| 48 | 62 | 48/48 | 1,421 | 6/6 | 73 |
| 54 | 62 | 54/54 | 1,421 | 0/0 | 73 |

- **250/250** raw accepted relations were independently reconstructed and verified by actual elliptic point arithmetic, **four** factor-base systems independently solved, and **four** Q73 public solves independently reproduced; each correct scalar checked by `[73]G=Q` but scalar 73 was not supplied to the target solver.
- **208/208** applicable counter, rank, LP, source-log and target fields matched the 16 frozen native records (13 per record); no timing is treated as independently observed.
- Both GitHub mutation controls were **correctly rejected**: corrupted basis rank and omitted final measurement. Hosted log contains expected Python ValueError traces for those two deliberate negative controls, followed by `CT237_MUTATION_CONTROLS_PASS`.
- Producer-local Linux Python 3 check also passed prior to publication. Hosted runner Python 3.14.4 reproduced the result. No native/PCC signature or second independently authored Python validator beyond CT223.

## Next performance experiment

The validated F101 solver remains **16,404–38,222× slower** than direct same-job tiny BSGS in CT236's median paired native measurements; CT237 did not change these timings. The next lead is norm-fiber compression when `p ≡ 1 (mod 3)`, which can shrink the rational projected factor base by identifying the three roots under `u ↦ ζu`. Any candidate must still be measured as a fully cold single-target solver on the **same exact source curve** against direct rho/BSGS/frontier IC, with transfer/setup/relations/LA/memory and uncertainty.

The original 695/701-record authenticated norm-point reuse and physical `C169,C13a,C13b,N2_actual,D` remain blocked by original data custody and authorized hardware/pricing. No paid EKS/Spot workload, original ledger change, protected merge or PCC promotion was performed.
