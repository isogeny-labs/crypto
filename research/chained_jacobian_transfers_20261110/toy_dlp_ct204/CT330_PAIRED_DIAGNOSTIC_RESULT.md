# CT330 — paired F103 Q73 source rho versus direct BSGS

2026-11-23 UTC · **scoped native GitHub Actions execution** · **[pcc:unverified]**

**Result:** on the fixed `E/F103: y²=x³+x+24` / order-109 public `Q=(42,71)`, the original diagnostic 16-jump rho was **slower than the original direct BSGS in all three alternating same-host source-only pairs**. This is **not** a result against the strongest optimized rho; it excludes the transferred complete IC and original 695/701 Jacobian setup. It is an honest, small-field negative finding, not a transfer attack or an asymptotic claim.

## Original-source execution and controls

[CT330 native Actions #45142821120](https://github.com/isogeny-labs/crypto/actions/runs/45142821120), job **137584742816**, **completed success**. GitHub PR branch source commit `37ec11726023e130fd376869f0df7145fb170709`; Actions synthetic merge checkout `ccd5d2b2c50ee846a2bd7ac72196363cf844c391`. Original F103 Rust Git blob **`234e14f2aea35b8ef3203065a5e2468aae9eb71f`**, preserved by SHA-aware `ct328_standalone_source.py`/new `ct330_compose.py`; source mutation was rejected. Generated one-job Rust translation unit SHA-256 `8d92672909f788710d9b0eecef674cae5ab5fe3a21d76de3410c54b11c6bf7c7` and appended test SHA-256 `50649fbfbadf989bc14f24c83932cbbcda55ddfbf0922f92615a45999aa735c9`.

Hosted `rustc --edition=2021 -O --test` compiled original F103 rho and BSGS source plus a separately appended matched test. **2/2 tests passed**: original 109/109 known-answer rho suite and exactly three paired Q73 runs, each independently checked `[73]G=Q`. Five deliberately corrupted output-record variants (wrong scalar, wrong run order, duplicate run, duplicate JSON field, missing terminal) were rejected by the separate Python record-only auditor. This is same-session checking, not signed independent replay.

**Retained failure:** initial [#45142772309](https://github.com/isogeny-labs/crypto/actions/runs/45142772309) failed because the new appendix mistakenly emitted the literal `\\n` rather than a newline before each JSON record. A formatting-only fix was then committed before the successful run; unchanged arithmetic, target, rho parameters and pairing order. The first failed log stays available.

## Exact observed paired nanoseconds

| Round | Order | Original rho cold call (ns) | Original BSGS cold call (ns) | BSGS / rho |
|---|---|---:|---:|---:|
| 0 | rho → BSGS | 14,111 | 1,833 | 0.129899 |
| 1 | BSGS → rho | 10,535 | 1,412 | 0.134029 |
| 2 | rho → BSGS | 10,806 | 1,332 | 0.123265 |

The **median within-pair BSGS/rho is 0.129899**, i.e. the original diagnostic rho took approximately **7.70×** the original direct BSGS wall time in this tiny three-pair panel. Geometric mean BSGS/rho = **0.128988**. The same-process host is not an isolation-controlled timing environment; three trials are **not** a confidence interval, and method setup may warm caches. The rho cold-call timer includes its checked subgroup, its jump-table/seed setup, walk and solution verification; the original BSGS timer includes its baby table, giant step, lookup, and scalar verification.

- All rho calls return 73, tally **332 original-source EC additions**, 2 walk steps, 1 restart, 1 collision, 2 stored points and table capacity 3. Of the 332 `ec_add` calls, the majority are incurred in mandatory original-rho subgroup and jump setup; this diagnostic pays setup even though its walk collides almost immediately. The separate Python 109-target arithmetic oracle is committed as `ct330_oracle.py` (same producer, not a signed verifier); on this source model the 109-target expected total is 48,147 additions, consistent with the previously completed CT328 panel. Exact **new oracle CI** remains a separate gate unless its relevant workflow completes.
- Original BSGS function reports only 17 additions in its **partial** counter; prior [CT329](CT329_BSGS_GROUP_COST_BOUNDARY_RESULT.md) instrumented the same underlying source function to find **34 actual in-function additions** for Q73. This yields a **332/34 = 9.76× addition-count ratio** for the particular diagnostic routines, **not** a measured 9.76× throughput ratio.
- The process-wide `VmHWM` records 2,912–2,916 KiB. This is **not** peak RSS attributable to either solver. Raw CI logs preserve per-round rho setup/walk/verify/unclassified nanoseconds, canonicalization/negation counters, and table allocation lower bounds.

## Reproduction, sources and held full attack

Reproduce within `isogeny-labs/crypto` on a pinned checkout and a Rust-equipped host:

```sh
BASE=research/chained_jacobian_transfers_20261110/toy_dlp_ct204
python3 -B "$BASE/ct330_compose.py" --source tests/ct238_native_cubic_norm_compression.rs --append "$BASE/ct330_paired.rs.inc" --out /tmp/ct330-native.rs
rustc --edition=2021 -O --test /tmp/ct330-native.rs -o /tmp/ct330-tests
/tmp/ct330-tests --test-threads=1 --nocapture 2>&1 | tee /tmp/ct330-native.log
python3 -B "$BASE/ct330_audit.py" --native-log /tmp/ct330-native.log
python3 -B "$BASE/ct330_oracle.py"
```

The protocol is [CT330_STANDALONE_MATCHED_RHO_BSGS_PROTOCOL.md](CT330_STANDALONE_MATCHED_RHO_BSGS_PROTOCOL.md). Prior original CT241 native IC/BSGS [#44592813955](https://github.com/isogeny-labs/crypto/actions/runs/44592813955) and CT329 source-bound audit [#45134941694](https://github.com/isogeny-labs/crypto/actions/runs/45134941694) remain unchanged. Bos–Kleinjung–Lenstra (ANTS IX, 2010) document negation-map fruitless cycles and nonideal practical improvements, so the present `ct327::rho_public` is correctly called a **diagnostic** rather than a strongest optimized reference: https://infoscience.epfl.ch/entities/publication/78564706-286d-4c6a-8a13-25c5d38fe72c.

No authenticated original 695/701 fixture, actual `C169/C13a/C13b/N2_actual/D`, root-native Cantor/IC execution, full Q73 IC-rho-BSGS triplets, same-host strongest optimized rho, cross-target independence, same-host uncertainty, independent execution verifier or signed PCC is supplied. The root library still has an unowned malformed `src/cryptanalysis/binary_hyperelliptic_index_calculus.rs` at Git blob `f001a1b641abd48bb0130c1a9a246e37c28bcc3a`; do **not** fix it in this lane. Other owners retain norm-reuse/cost16 [#4990](https://github.com/isogeny-labs/crypto/pull/4990), Prime31 and existing native benchmark reservations.

## Worker-role and remaining dependency audit

Native child-agent slots, Research-KB MCP and Conductor were unavailable. The single coordinator performed disjoint mathematical (prime-order injectivity and complement-obstruction scope), Rust (exact source construction, Q73 counters) and validation (separately coded Python 109-target oracle, native logfile parsing and mutation controls) work, **without** claiming three independent worker executions, signed validation or aggregate elapsed-time savings. The established mathematical identity `pi_* pi^*=[3]` is a known property of this separable cover and does not imply a cost advantage.

**Next highest-value unblocked action:** after the root owner repairs the unrelated parser break, run the previously preregistered original CT327 *full* IC/rho/BSGS cold triplets under the existing reservation rules. If six-object custody becomes authorized, prioritize the 695/701 source-bound physical norm-point reuse and strongest complete-solve comparator instead. A diagnostic reference losing to direct BSGS is not grounds to abandon investigation of other maps or algorithms.
