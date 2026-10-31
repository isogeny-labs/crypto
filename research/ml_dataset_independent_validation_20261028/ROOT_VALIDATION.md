# Independent root-set software validation

The consumer now independently checks completed distinct-root certificates from
prime/root/m=2 and binary/root/m=2 datasets. Its polynomial arithmetic computes
`monic gcd(f, X^q-X)` and the product of the declared distinct linear factors;
it does not import the producer root finder. The mathematical source and claim
limits were recorded before implementation in
[ROOT_CERTIFICATE_DESIGN.md](ROOT_CERTIFICATE_DESIGN.md).

The runtime implementation is `25e863fe6075d7bac795ee62680b72c123ba0f33`.
Source `a39594756923822f3705b4e5a916bb9321dacba6` adds frozen software inputs,
documentation and a stronger sealed-fixture mutation test. Its production
`native/src` files are unchanged. Merge `8c94639876d0281985e4ba562950083a707e835b`
incorporates the parent feature branch without changing any file bytes.
The subsequent evidence handoff does not change native source.

## Completed checks

| Software acceptance check | Observed result |
| --- | --- |
| Standalone release tests | 88 unit + 7 integration passed, zero failed |
| Strict all-target release Clippy, formatting, release build | Passed |
| Sealed producer fixture acceptance | 51 cells; 30 completed distinct-root sets checked |
| Independent positive group certificates in those cells | 14 |
| Censored cells / censored native calls | 11 / 8, preserved separately |
| Binary coverage | Binary and Koblitz; n=5 and n=6; both twists |
| Base contracts | Legacy v1 and explicit-policy v2, including unsupported cells |
| Rehashed root/input substitutions | Rejected for prime, binary and Koblitz |
| Static routing exports with pinned producer replay | 51 rows; root summaries remain labels |
| Previous sealed 72-cell corpus re-audit | 5 completed prime root sets and 22 positive certificates |
| S3 archive fresh readback | 1,738 internal hashes passed; restored acceptance report byte-identical |

The root substitutions omit a genuine root or add canonical zero to an empty
list, updating both redundant call/certificate artifacts and their manifest
hashes. They cannot be rejected merely as stale transport hashes. Native tests
also cover small-field exhaustive root oracles, repeated roots, constants,
zero/all-field semantics, invalid fields, malformed completion/censoring,
public/config/domain/outcome substitutions and a degree-eight prime polynomial
at p=2^61-1. Large-field validation does not enumerate the field.

The required root-library gate is reused through exact tracked-file equivalence:
4,987 passed, zero failed and 122 ignored at
`8d55079e37f2c5f2f506edf20670f018354edd56`. All tracked files outside this
standalone consumer research subtree are unchanged from that gate's source;
`src`, Cargo manifests, `build.rs`, tests, examples and benches were also
compared explicitly. The current standalone consumer has its own 95-test gate.
No root-library rerun is represented as having occurred. Prior root receipts
remain in [the policy evidence](evidence/policy-compatibility-20261028/root-validation.json).

Two read-only source reviews found no actionable issue: an independent source
assessment and an additional binary-contract review by the producer implementer.
Neither review reran the tests or established remote CI/review approval.

## Frozen fixtures and retained failures

The pinned producer source is
`d79f658f7a0a83e2f75f4cb6f0d1f7542d0e6b83`; its copied executable SHA256 is
`c01e024156067799356f590a0c3ac749da509305bd569e26c9bf084d5402d04a`.
All metadata preflights reported clean source and zero generated cells.
Inputs and bounded drivers are in [root_validation/](root_validation/).
Each cell has a five-second limit, 512 MiB memory and 8 MiB trace cap; no cloud
worker or dataset-publication flag is used.

The first frozen 48 attempts comprise 12 legacy high-cap, 24 explicit-policy
high-cap and 12 legacy low-cap cells. The work limits are 5,000 and 16.
Four rows retained setup errors for the same n=6 binary/untwisted curve with
no odd prime subgroup. The first coverage assertion failed on that missing
branch. An additive two-cell seed2 supplement preserved another setup error:
changing the target-sampling seed did not change the separate curve-seed axis.
The second coverage assertion therefore also failed. Both logs and all original
rows remain in the archive. A subsequently frozen single curve_seed1 cell
completed the missing branch. It selects shard_index2/shard_count4 from a
four-cell plan and attempts exactly one cell; the other three plan cells were
not requested. No data was overwritten or silently rerun.

Across all 51 retained attempts, producer statuses are 14 verified, 17 partial,
11 budget-exhausted, five error and four unsupported. Those producer labels are
not 51 independently verified mathematical results. Thirty completed **calls**
have independently checked root sets; fourteen positive **certificates** have
checked group equations. Root-set verification is not an UNSAT or cost proof.

## Durability and replay

All raw cells, frozen inputs, copied executables, source snapshot, source hashes,
failed and successful validation logs and exported rows are archived at:

`s3://setec-astronomy-artifacts/research/ml-dataset/validation/20261028/consumer-root-certificates/40d88d8d0bdfbe57ab8c201970d4ced5c49792c86eea552067e44726921c4809/validation.tar.gz`

The archive is 12,750,951 bytes, SHA256
`40d88d8d0bdfbe57ab8c201970d4ced5c49792c86eea552067e44726921c4809`.
It was conditionally created, freshly downloaded using its ETag, checked for
length and SHA256, and compared byte-for-byte. After extraction, all 1,738
internal file hashes passed. The retained saved-fixture test then re-audited the
restored 51 cells and its report exactly matched the original report.

Extract the archive into a new directory. In its
`ml-consumer-roots-validation-20261028/` root, run
`sha256sum -c archive-files.sha256`. The independent test takes the five sealed
run directories in `ML_DATASET_ROOT_FIXTURES` (colon-separated) and a nonexistent
report path in `ML_DATASET_ROOT_REPORT`; invoke the retained `saved-roots-test`
with `--ignored --nocapture`. This launches no producer and generates no cells.
The directories are `fixtures-01/{legacy-complete,policy-complete,legacy-censored}.run`,
`supplement-01/run` and `curve-supplement-01/run`.

Exact JSON receipts, test logs, source reviews and scoped results are in
[evidence/root-certificates-20261028/](evidence/root-certificates-20261028/).
The controller archive roundtrip does not establish worker-role publication,
completion-receipt fetch conformance or production retention guarantees.

## Remaining boundaries

Prime v1 binds the mathematical fields present in its certificate; unlike the
binary certificate, it has no whole-public/base/config hash. Arbitrary prime
annotations are outside that binding. The checker deliberately leaves all
intermediate event identities, execution completeness, total operation cost,
subgroup orders, factor-base construction completeness, UNSAT, fall degree and
producer-training-export equivalence unverified. Completed calls preceding
censorship retain per-call labels without promoting the whole cell.
The separate consumer static views v1/v2 and their split rule are unchanged.
These small software fixtures are not the intended larger training corpus or
new scientific observations. The original USD200 total ceiling is unchanged.
