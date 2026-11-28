# CT208 — manifest-complete sparse checkout preflight for native toy transfer

2026-11-18. Continuation of [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715), stacked on #4697. **[pcc:unverified]**

## Exact observed failure

Scoped [CT207 replacement workflow 44307547244](https://github.com/isogeny-labs/crypto/actions/runs/44307547244), native job `134812914985`, completed in failure. The independent `independent-field-oracle` job passed. Native Rust compilation reached `Compiling crypto v0.1.0` at 2026-11-18T12:56:25Z, but failed at 2026-11-18T13:07:29Z **before invoking the toy ECDLP tests**:

```
error: couldn't read `research/full_volcano_graphwide_ic_20261017/execution/summarize.rs`: No such file or directory (os error 2)
error: could not compile `crypto` (bin "volcano-ic-summarize") due to 1 previous error
```

The missing source was independently found at the exact branch path (blob `b42a7dfb2bb096437bd3745642775ccdeafdea7a`). The source is registered by the root Cargo manifest. This is a sparse-checkout defect, not a Jacobian test failure.

## Change

In `.github/workflows/ct205-toy-transfer.yml` the native sparse checkout now explicitly includes the two non-`src/` root-manifest binary entrypoints:

- `research/full_volcano_graphwide_ic_20261017/execution/summarize.rs`
- `research/n83_highk_beyond4405_20261021/native_exclusion_audit.rs`

The first is the observed missing path. The second is another manifest-registered binary outside the existing sparse set; including it now avoids another expensive compilation attempt failing at a missing manifest entrypoint. All 36 explicit root-manifest binary paths were enumerated from the exact branch manifest; these are the only two under `research/`. A new Python `tomllib` preflight checks every `[[bin]].path` for existence immediately after checkout, before Cargo compiles any dependency. This guard does **not** check all transitive `include!/` `include_str!` paths and cannot prove the Rust build will succeed.

The existing native test command, Rust toolchain, 40-minute cap, two Cargo jobs, public toy input, and separate independent oracle are unchanged. No S3 hydration or EKS/Spot usage was added.

## Reproduce and evaluate

```sh
python3 - <<'PY'
from pathlib import Path
import tomllib
manifest = tomllib.loads(Path("Cargo.toml").read_text())
absent = [b["path"] for b in manifest.get("bin", []) if not Path(b["path"]).is_file()]
print(len(manifest.get("bin", [])), absent)
assert not absent
PY
cargo test --locked --release --test prime_cover_toy_complete_dlp -- --nocapture
```

For success, require native test output for all 43 logarithms through the Jacobian, matched direct BSGS, pushforward and wrong-scalar rejection, with exact-head CI evidence. If Rust now fails during compilation or the group-law test, record the actual error without upgrading source assertions to an executed result.

## Boundaries and next decision

This is a **CI engineering improvement**, not a mathematical solver or timing advantage. The authenticated 695/701-record original corpus and native `C169` vs `C13a+C13b`, norm and `D` pricing remain blocked by prior S3 `HeadObject` 403, and separate strongest rho/BSGS/IC complete cold comparisons plus independent PCC are still outstanding. The CT206 sextic complementary-factor obstruction and CT198 cancellation remain in force under their precise hypotheses.

Research KB was unavailable in this connector runtime; prior sources reviewed include merged crypto #3877/#4215, active crypto #4697/#4715, cryptanalysis #1523/#1766/#1770, and the existing CT204–CT207 reports. No novelty claim. **[pcc:unverified]**
