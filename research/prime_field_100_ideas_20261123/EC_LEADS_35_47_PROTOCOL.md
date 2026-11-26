# Local EC controls for ideas 35–47

Date: 2026-11-24, America/Los_Angeles. **Status before invocation:** frozen and unrun. Conductor tasks `T-12258` (source, protocol, raw files) and `T-12266` (immutable qualifier copy). This experiment uses only public synthetic E(F_p) data and the previously emitted #24 V2 relation rows. No cloud resource or third-party target is involved.

## Admission and fixed inputs

`scripts/checkpoint.sh gate` currently fails because `.resume/RESUME.md` is missing. This is a distinct observed failure from the previously reported HTTP 403 S3 checkpoint failure. The user previously instructed “Run locally with bypass” after that failure and has now explicitly renewed the request to run E(F_p) leads in parallel. This protocol admits only this bounded local run under those user instructions. It does not claim an S3 checkpoint or authorize cloud spending.

The immutable input is `S4_MATCHED_V2_STDOUT.txt`, SHA-256 `3accec218c07a2acd2fb433270fa8724db6aee57fef85d0e29e3eb3de16c1f7f`, generated on `E/F_1009: y²=x³+x+14` with subgroup order 1013. The GP template and separately copied qualifier SHA-256 are both `515a6d66e9030f053990a84c5ccea3a664515ac5dedc9540e8779d467a5ffbf9`. Run the qualifier; do not edit it after this hash. GP version is 2.17.3 arm64. The input contains 24 emitted rows for `z=0` and 33 for `z=320`. The existing exact modular rank baseline is 4 and 6, independently read back by `EC_S4_V2_INDEPENDENT_GP.gp`.

## Predictions, controls and limits

1. **#35–39, graph partition:** construct the factor-base co-occurrence graph from nonzero coefficients in the emitted rows. Report active vertices, connected components, distinct rows, and rank of the base-only and augmented matrices. These component counts are descriptive and not predicted. Exact modular rank is the coefficient-aware control. A target column is common to all rows and may reconnect components in the augmented graph; do not infer full-system block diagonalization from factor-base components.
2. **#40, ideal oracle:** from the two fixed `z=0` triples `[3,5,12]` and `[4,5,11]`, enumerate eight signs each, marking a candidate only if its EC point sum equals the printed target. Compare the exact classical marked fraction with rational amplitude-amplification probabilities after 0–4 ideal Grover iterations, checking norm 1. At least two marked candidates are predicted from emitted witnesses. This is an oracle model; no reversible EC circuit, quantum hardware or end-to-end ECDLP cost is tested.
3. **#42, exact linear algebra:** compare the rank of the one-entry matrix `[1013]` over the rationals and modulo 1013; predictions are 1 and 0. This checks that real rank alone does not certify modular rank. Exact rational reconstruction can sometimes reduce modulo the subgroup order; the earlier `[2]` scalar diagnostic is not an impossibility argument.
4. **#45, exact preconditioning:** use a fixed invertible row operation on the 3×3 EC relation-pattern matrix from the earlier graph diagnostic; rank must remain 3. This is a correctness control with no measurement of nonzeros or Krylov time.
5. **#47, tensor cut rank:** construct 16×16 identity and two-register SWAP permutation matrices. Both have 16 nonzeros; their operator-Schmidt cut ranks are predicted 1 and 16 modulo 1013. This tests the claim that sparse storage alone implies low tensor-network bond dimension. It is an abstract matrix control, not a measured rank of the actual EC relation matrix.

One `timeout 60 gp -q < research/prime_field_100_ideas_20261123/EC_LEADS_35_47_QUALIFIER.gp` invocation is allowed. Capture stdout, stderr and shell exit separately. Reject any GP `***` line, nonzero exit, failed guard, changed input/source hash, or missing final sentinel. Outputs are expected to be under 10 KiB; do not retry silently on failure. The run is a structural diagnostic with no wall-time, relation-yield or complete-solver comparison. Preserve any negative or parser result exactly.

## Single local execution and interpretation

The qualifier was invoked once with the frozen input and source hashes. The [raw stdout](EC_LEADS_35_47.stdout.txt) SHA-256 is `94f9b5c51431ed392b6ac599af95dc48b757b86c93a2a0077faae39c90968b73`; [stderr](EC_LEADS_35_47.stderr.txt) is empty (SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`); [exit](EC_LEADS_35_47.exit.txt) contains `0` (SHA-256 `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa`). The final `complete_native_controls` sentinel is present. No input or source hash changed.

| Control | Observed exact value | Limit |
|---|---|---|
| #35–39, `z=0` emitted EC rows | 24 rows; 10 active factor-base vertices; 2 factor-base graph components; 4 distinct rows; base and augmented ranks both 4. | Components are based on support, while ranks use coefficients modulo 1013. The target column connects all rows in the full relation matrix. |
| #35–39, `z=320` emitted EC rows | 33 rows; 12 active vertices; 1 component; 6 distinct rows; base and augmented ranks both 6. | Different targets and relation streams; no causal comparison of topology with relation yield. |
| #40, exact ideal oracle on 16 signed EC candidates | 2 group-equality marked candidates; classical hit fraction `1/8`; ideal Grover probabilities after 0–4 calls: `1/8, 25/32, 121/128, 169/512, 25/2048`. | The mark predicate is evaluated classically here. No reversible EC circuit, quantum device, gate count or DLP solve advantage. |
| #45, invertible row control | `det(U)=1`, rank remains 3 by nonzero determinant before and after the row transform. | No measured fill or Krylov improvement. |
| #47, exact tensor-cut control | Identity and two-register SWAP are both 16×16 with 16 nonzeros; operator-Schmidt ranks modulo 1013 are 1 and 16. | Abstract matrix control; not the actual EC relation matrix. |

The source also **prints** `real_rank_[1013]=1` and `modular_rank_F1013_[1013]=0`, but it does not execute a rank guard for that line. Therefore #42 is only a hand-checkable derivation in this run and is **not admitted as an executed #42 control**. A separate source and raw attempt are required to qualify it. The current run reports no performance measurement and has no signed repository proof-carrying verdict.
