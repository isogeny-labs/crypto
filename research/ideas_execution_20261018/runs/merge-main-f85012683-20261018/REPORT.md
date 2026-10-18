# PR 2148 main integration

Status: all required integration validation passed. This is an integration check, not a new cryptanalytic measurement or performance result. The validated resolution is recorded as an ordinary two-parent merge; root owns push and PR follow-up.

The merge combines feature parent `9ba11933fb44cb373e1c4ac96986096847ce4fce` with main parent `f8501268305c0c84d0e077a4ab9054c07ff74540`, using an ordinary Git merge. No frozen campaign evidence conflicted. Prior conflict-stage bytes and hashes are retained in `prior.tar.gz` and `prior.sha256`.

Four source conflicts preserve main's arithmetic/routing and the feature's regression controls. Main's public-target laws for Fp4Torsion and F2m4 are retained, and the regression test now checks deterministic nonidentity subgroup targets. Fp4Translation still has no public-target law. Git's automatic merge additionally produced duplicate constructor arms and a duplicated `fbm` import; both were removed before the successful final CLI build. Existing frozen oracle sessions retain their captured older-source limitations and are not relabeled as runs of this merged implementation.

The final trait file has 2,748 rows, covers all 2,572 current registry records and all manifest sources, retains every one of the feature parent's 2,551 and main parent's 2,748 slugs, and has six valid source input hashes. The four campaign Fp4 crosswalk entries are aliases with ICV1 identities; the native parser skips their arithmetic, and no additional trait computation is inferred from those alias registrations. Native full parsed-source comparison proves that the sole source-text difference is invisible to this trait parser. The audited incremental candidate changes only the recorded crosswalk digest in main's generated data. Its SHA256 is `b5bc649da40a5c143aa814414ade1736e802ffafffeeabe42567ffea2a018c26`. The mandatory separate complete native regeneration and full-byte comparison passed, after which the identical working-tree file was staged and all merge conflicts resolved. This uses the repository's generator as the reference; it is not a claim of two independent mathematical trait implementations.

| Required gate | Status and evidence |
|---|---|
| Fresh merged release curve_traits/ecbench and ic builds | PASS; final build exits 0; failed setup/automatic-merge attempts retained |
| Native six-input/source coverage and both-parent retention | PASS; `traits-coverage.json` and `crosswalk-equivalence.json` |
| Full native trait regeneration and byte comparison | PASS; exit 0, `logs/crypto-pr2148-traits-check.log` |
| Touched example tests/builds | PASS; 7 tests, 0 failed; two additional examples compile with 0 unit tests |
| Full release library suite, explicit network/local TCP access | PASS; 4,382 passed, 0 failed, 117 ignored, exit 0; `logs/crypto-pr2148-lib-tests.log` |
| Source diff whitespace check | PASS; historical raw-log/SVG/vendor whitespace is preserved rather than rewritten |
| Merge conflict resolution | PASS; no unmerged paths; root owns push and PR follow-up |

Conductor is unavailable (exit 127); the exact attempted check and the parent's explicit merge authorization are recorded. The initial missing-promisor-object fetch, read-only default Cargo-cache failure, intentionally interrupted build, and duplicate-import build rejection are retained as unsuccessful setup/integration attempts. None is reported as a completed scientific run. The full command sequence and exact toolchain/environment are in `COMMANDS.md` and `environment.txt`.

The final randomized tiny-prime parameter-generation test was retained and completed successfully. The 117 ignored tests remain unexecuted, as in the prescribed standard release-library suite. Validation does not close unimplemented campaign research requirements or establish any cryptanalytic security/performance conclusion. Two root-owned status documents (`EXECUTION.md` and `cloud_status.json`) were edited separately during validation and are deliberately excluded from this merge staging; none is a compiled or generated-input dependency.
