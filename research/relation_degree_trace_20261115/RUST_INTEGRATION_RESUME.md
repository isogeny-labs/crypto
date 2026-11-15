# Resume: native certified-relation integration

The current source is 0eb68e7d47b34793b3700ad3619a573e848508cb, built and
qualified through the real root Rust library. Read INTEGRATION_REPORT.md and
integration-development/rust-{build,pre-call,post-call}-03.json before
repeating work. The immutable round-03 executable SHA-256 is
1ab308935d518437f009684e8bc1ed3c57236186348e2473d7c227fe472afd6b.
The three qualifier archives and earlier failures are retained.

The opt-in public boundary is `f4_fp::certified`; source is shared through
crates/certified-relations, and the original study reexports it. The final
native API qualifier passed five tests; shared normal/counter suites passed
27/28 tests; study compatibility suites passed ten tests each. Strict clippy
passed for the shared crate, study, root library and focused native qualifier.
These are producer development checks, not signed execution evidence
`[pcc:unverified]`.

Input-order expanded proofs still hit the unchanged budget. Lean containment
checks plus breadth-first coordinate waves make the fixed seed-7 six-cycle
complete on both fields. Maximum dependency depth is 3 and the largest stored
final-basis proof has 8336 terms. Neither is a measured solving-degree or
runtime improvement. Claim PCC-20261115-ce42a6 pins the final post-call receipt
and remains unverified `[pcc:unverified]`.

No scientific timing is in flight. The checkpoint gate lacks S3 upload
transport. After that external access is available, freeze a distinct root-API
runtime protocol using actual optimized F4 at the 31-bit field and a supported
mature solver at the 53-bit field. Charge field admission, conversion,
construction and matched checks; retain original equations and basis equality.
Existing QUOTIENT_SPEED_PROTOCOL.md concerns the older standalone comparison;
do not relabel its denominator or reuse its timing as root-API evidence.

Publication uses codex/native-certified-relation-integration-20261115, stacked
in [PR #4349](https://github.com/isogeny-labs/crypto/pull/4349) on the existing
relation-degree-trace PR #4228. The parent advanced while the
qualified source was being tested. Do not rebase, choose a ledger side or
rewrite frozen claims to resolve that divergence. The pinned LEADS.md has been
restored; evolving older notes are LEAD_STATUS.md. LEAD_RESUME.md describes
the earlier prototype state and is historical.
