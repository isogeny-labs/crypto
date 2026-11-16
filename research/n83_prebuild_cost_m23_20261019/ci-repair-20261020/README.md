# PR 2271 CI repair receipts

These are validation and failure receipts, not a new experiment.
The parent head is `90d17b18b3441a0dccf83b160c3f235444b134f2`.
The enclosing commit contains the repaired plan and prose.

`receipts.tar.xz` contains the original three GitHub job logs, the local
plan failure, the native reseal mismatch, the successful plan and fixed-F8
certificate checks, release tests, name lint, actionlint, and upstream
incomplete-session classification checks. `files.sha256` binds each
uncompressed member; `archive.sha256` binds the archive.

## Commands and outcomes

| Command | Outcome |
|:--|:--|
| `cargo test --release --bin ecdlp-audit checked_in_plan_passes` before repair | Expected reproduction: WS1-D2 artifact digest mismatch, exit 101. |
| `ecdlp-audit verify-plan --input docs/ecdlp-audit/ECDLP_AUDIT_PLAN.mock.json --repo-root .` before resealing | Only the canonical plan UID differs; native verifier supplies the required UID. |
| Same native plan verification after resealing | PASS; non-authoritative, promotion disabled, twenty required gates open. |
| `ecdlp-audit verify-fixed-f8-order --input docs/curves/fixed-f8-genus3-order-certificate.json --repo-root .` | Verified. |
| `cargo test --release --lib --bin ecdlp-audit` | 4,401 library tests passed, 118 ignored; 45 audit-plan tests passed; zero failures, exit 0. |
| `python3 scripts/check_curve_names.py` | Initial sparse-input failure retained; after materializing tracked text/manifests, zero problems, exit 0. This is the existing prose/registry metadata lint. |
| `python3 scripts/check_curve_names.py --diff 124ab188cc39d52ee99e979da95b501a3ca34f88` | Zero problems, exit 0. |
| `actionlint .github/workflows/ecbench.yml` | PASS, exit 0. |
| `bash -n .github/scripts/ecbench-audit-sessions.sh` | PASS, exit 0. |
| Audit script with precisely the four retained incomplete directories | All twelve file hashes pass; reports zero complete sessions audited and four incomplete. |
| Same script with one session omitted from the list | Rejects the list/pin mismatch. |
| Same script with a changed expected hash in a temporary copy of the TSV | Rejects the changed hash. Original evidence untouched. |

Cargo used the existing `n83-small-m4-build` target directory, the
`n83-rust-tmp` temporary directory and `CARGO_BUILD_JOBS=2`.
The three backported ecbench files equal upstream commit
`fa31c0ea37f1057c0ac48e6170dd709a2b694bed` from merged PR 2234 byte for
byte. The classification test does not replace GitHub's full historical
session audit.
