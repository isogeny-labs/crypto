# Campaign admission v2 — additive review and software validation

Final private unapplied patch SHA256116d6acb4095e6b256555589898d6c643dada68502f866b5ffa94a4248dc2788. V1 patch, review, sources and every validation attempt remain retained. Exact proposed hashes are in campaign-admission-proposal-v2/PROPOSAL-RECEIPT.json.

## Root finding corrected

V1 admitted/rejected at computation status entry and fresh benchmark entry, so an expired reservation could suppress available terminal evidence. V2 reads an already-created computation Pod before requesting new admission, retains terminal supervisor/owner evidence, then rejects continuation. An expired request enters evidence-only retention before returning failure. Fresh expiry retains exact existing Job JSON under a digest-addressed name; it does not invoke verify, replay, compare, payload generation or an unbounded MCP status subprocess. The fresh-owner/artifact collector is a separate concurrent adapter, explicitly not implemented or claimed complete here.

Every expiry collector checks the original absolute22:08:40 cutoff. Before each existing kubectl read it requires at least185 seconds remaining, accounting for that transport's180-second timeout and5-second forced-kill grace. If insufficient time remains, it retains the original admission failure with a collection-failure finding rather than beginning the call. This is not an extension of any deadline. There is no retry loop in the evidence-only branch. Local filesystem stalls and actual infrastructure teardown are not proven bounded by these checks; the earlier admission-versus-actual-cost limitation remains.

Start, restore, continue, checkpoint and verification still require admission after available terminal custody. The prospective next restore reservation is charged before archive verification; if verification fails, that charge is retained. Existing resource reservation digests remain checked before further work. No refund or balance reset was introduced.

A fixture uses an actually expired Reservation to exercise the same retain_then_reject helper used in production, checking retained evidence, preserved original failure, combined retention error, and the185-second collector boundary. This is a software ordering fixture, not a live Kubernetes timeout test.

## Exact validation corpus and retained failures

The first private V1 corpus passed33 tests but used owner helper8e06c043..., not the root's final88ea2cf7... helper. Its result qualifies only that recorded source manifest. Strict Clippy then rejected the eight-argument reserve API; V2 groups request deadline and observation time in AdmissionWindow. An intermediate V2 corpus passed34 tests with that same older owner helper; it likewise does not qualify the final corpus.

The final copied corpus was reconciled against every taskq entry in root research/pencil_lifecycle_20261030/integration/transport-source-hashes-v2.json before overlaying the four campaign files. It uses final owner_observation.rs SHA88ea2cf74b7a972de60c5e143f2f41f593e216fa46149441844c63f5764efb0c, supervisor f8bae891..., contract basefaa8ea98..., and cloud base77a5e790.... Root baseline33 tests plus five campaign fixtures gives38 tests.

The first integrated run passed38 tests; strict Clippy caught one redundant fixture field spelling (`now: now`). That failure and pre-lint patch were preserved. After correcting only that spelling, the exact final source passed:

- `cargo test --locked --offline`:38 tests passed, zero failed; no ignored or measured tests.
- `cargo clippy --locked --offline --all-targets -- -D warnings`: exit0.

Both final receipts bind source manifest SHA9d555f62905c4e821933e43832cc12aca204e44bcc8e2fc6083e7eb8192370de. Their logs and receipts are campaign-admission-validation-v2-integrated/{test-02,clippy-02}*. VALIDATION-SUMMARY.json identifies all historical receipts, including the failures. Targets were private, independent of root's build; all Cargo calls were locked/offline and individually time-bounded. Tests used synthetic/fake subprocess fixtures and local mock transports; no cloud service or mathematical solver was executed.

## Unchanged limits

This qualifies only the copied controller software corpus, not a deployed image, root whole-repository build, physical allocation cutoff, measured worker cost, scientific result, or experiment admission. The optional JSON request-schema addition remains separately reserved/unapplied; this patch does not silently edit it. The fresh owner adapter and its deadline-aware evidence collection remain separate integration work. Exact campaign policy, ledger UID, reconciled prior charge and external enforcement bindings are still required; no production policy or initial zero balance was manufactured. Original25200 seconds and22:08:40 UTC are unchanged.
