# Frozen selected-ID and scalar replay of proposal 27

Frozen 2026-11-25 (America/Los_Angeles), draft crypto PR #5165, before this follow-up run. The original [one-hop message-passing producer](EC_GNN_027_PROTOCOL.md) is unchanged: source SHA-256 `83181f42ba9a50aa7afab877fee0d69a3acb828bcfc8d67df25db6930243be1f`, stdout SHA-256 `c3fd4aaeaa2ae0e9304c6cf3594a76a940127a4cfa96765e2b24eed17360d437`. Its stdout reports aggregate relation counts and rank, but not the selected candidate IDs. The objective is to replay those exact IDs under different EC arithmetic and verify their hit labels and modular row ranks. This does not independently reproduce floating-point model scores or training weights.

The repository [#27 control](EC_CLASSIFIER_001_034_PROTOCOL.md), [new producer](EC_GNN_027_PROTOCOL.md), and the shared KB were checked before this follow-up. KB status remains ready at revision `750797a540fb4c38837c67b12df84d5b`, refreshed 2026-11-25T03:19:48Z; search for independent EC selector replay returned related internal selector records but no method that changes this exact scalar audit. [Semaev 2004](https://eprint.iacr.org/2004/031.pdf) supplies the point-relation context; the replay itself uses the elementary cyclic-group identity after anchoring every public point to a generator.

## Frozen two-stage audit

1. Compile a separate [output-only Rust emitter](EC_GNN_027_VERIFY_EMIT.rs), copied byte-for-byte from the original source except for one `println!` after selection that emits each target's ordered 8192 model-selected candidate IDs as `model_ids_uN=[...]`. The unified diff has exactly this added line; the trained model, scores, sort, chosen IDs, exact verifier, and other controls are unmodified. Require the aggregate model step/hit/rank lines to match the original run, allowing only measured time fields to vary. Keep the original raw/source untouched. The emitter is evidence of what the frozen algorithm selected, not independent verification of the learned ranking.
2. Run a separate [PARI/GP qualifier](EC_GNN_027_VERIFY_GP.gp) that parses the eight emitted ID vectors; checks length 8192, uniqueness and range for each target; rebuilds the held-out `E/F_1019:y²=x³+x+20` base/targets using the Rust producer's smallest-nonnegative-y rule; anchors all 16 base and eight target points to the asserted order-1033 generator `G=(0,64)` by bounded scalar search; decodes every ID to `(i,j,k,mask)`; decides the relation through `±f_i±f_j±f_k≡t_u (mod1033)`; reconstructs all selected 24-column signed rows and modular rank. Independently scan the full 8×32768 candidate grid by scalar congruence to determine missed labels and full-grid rank. Stop if selected hits, per-target counts, ranks, coverage, or full-grid baseline differ from the original producer.

The qualifier never calls the producer's point-add routine. It uses exact scalar arithmetic after the 24 point-log anchors, which are feasible only because this is a public order-1033 group. These scalar logs are **verification inputs after model selection**, not features, training labels, or a way to skip exact relation checks in the original producer. The audit cannot certify the original model's floating-point scores, weight-update path, or top-K ordering independently, because weights/scores were not emitted in the original raw. It can certify that the newly emitted selected IDs imply the reported exact group relation labels and rank, conditional on the output-only diff preserving selection. Timings from the emitter are diagnostic only and are not combined with the original workload.

The checkpoint gate remains failed (`.resume/RESUME.md` absent; earlier S3 `InvalidAccessKeyId`). The user's explicit local bypass and renewed public EC instruction admit this bounded local replay; no cloud spend or external target. Retain stderr/exit and any failed or censored attempt; GP may return shell exit zero after parser errors, so empty stderr and completion sentinels are mandatory. No silent source repair. The binary remains local-only under `/tmp`.

Frozen Rust 1.99.1 emitter source SHA-256 `b35ff75d8211e244093ddfefe437efbfd037132314824b9b881f6fb517b6f641`; frozen GP 2.17.3 qualifier SHA-256 `9cd3d9fd2f5f5e3079dd2600657bff8e1f37f2b8a9633b0b10ea1bf5104d9117`. One compile/run from repo root:

```sh
TMPDIR=/tmp rustup run 1.99.1 rustc -O --edition 2021 research/prime_field_100_ideas_20261123/EC_GNN_027_VERIFY_EMIT.rs -o /tmp/ec_gnn_027_verify_emit.bin
/tmp/ec_gnn_027_verify_emit.bin > research/prime_field_100_ideas_20261123/EC_GNN_027_VERIFY_EMIT.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_GNN_027_VERIFY_EMIT.stderr.txt
```

Record both statuses in `EC_GNN_027_VERIFY_EMIT.exit.txt`. On a valid emitter, invoke the GP qualifier once:

```sh
gp -q < research/prime_field_100_ideas_20261123/EC_GNN_027_VERIFY_GP.gp > research/prime_field_100_ideas_20261123/EC_GNN_027_VERIFY_GP.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_GNN_027_VERIFY_GP.stderr.txt
```

Record shell status immediately in `EC_GNN_027_VERIFY_GP.exit.txt`. No result is admitted at this freeze point.

## Observed replay

The output-only Rust emitter compiled and ran once with statuses `compile=0`, `run=0`; runtime stderr was empty. Rust emitted the same nonfatal `unused variable: label_ms` warning as the frozen source. Its stdout contains exactly eight `model_ids_uN` vectors with 8192 IDs each and is 438,641 bytes. A literal diff of every `arm_step` and `arm_summary` line against [the original stdout](EC_GNN_027.stdout.txt) was empty; time-only fields were excluded. The one-line source diff and this aggregate equality support that the emitter exposed the original deterministic selections, but do not prove the floating-point score values independently. The emitter binary remains under `/tmp`, not staged.

The frozen GP qualifier ran once with shell exit `0`, empty stderr, and a completion sentinel. It anchored all 24 held-out points to the order-1033 generator by point multiplication, checked each emitted vector's length/uniqueness/range, then used scalar congruences to label every selected ID and the full grid. Its results matched the original producer exactly:

| Held-out target x | Model-selected exact relations | Full-grid exact relations |
|---:|---:|---:|
| 38 | 0 | 21 |
| 39 | 6 | 15 |
| 41 | 6 | 33 |
| 46 | 0 | 18 |
| 49 | 0 | 34 |
| 53 | 18 | 54 |
| 54 | 21 | 48 |
| 56 | 0 | 0 |
| **Total** | **51** | **223** |

The 65,536 selected IDs yield target coverage 4/8 and modular rank 9/24; the complete candidate grid yields coverage 7/8 and rank 22/24. Thus the independent scalar replay confirms 172 missed exact relations and rank loss 13 for the selected set. This is a genuine independent arithmetic/rank check of the emitted IDs, distinct from the original producer's point-add verifier. It does **not** independently replay the original model's SGD weight updates, floating-point scores, or top-K ordering from the original raw: those internal values were not printed. Its selection-identity inference rests on the output-only one-line source diff and identical arm metrics. Both run outputs remain `[pcc:unverified]` pending outside review.

| Preserved artifact | SHA-256 |
|---|---|
| [Emitter source](EC_GNN_027_VERIFY_EMIT.rs) | `b35ff75d8211e244093ddfefe437efbfd037132314824b9b881f6fb517b6f641` |
| [Emitter stdout](EC_GNN_027_VERIFY_EMIT.stdout.txt) | `02e7e8517aeaf3da60e2b23c0b4d70225828f21488d4c099d90a194425eab7f8` |
| [Emitter runtime stderr](EC_GNN_027_VERIFY_EMIT.stderr.txt) (empty) | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [Emitter exit](EC_GNN_027_VERIFY_EMIT.exit.txt) | `6ae53545e4b9562543543c003678fd596b448a77bc6940dfe87010b843763960` |
| `/tmp/ec_gnn_027_verify_emit.bin` (local only) | `f217e2755d761e775f4062cdebff993c38bd9be883658a67063757d4e5f98df6` |
| [Qualifier source](EC_GNN_027_VERIFY_GP.gp) | `9cd3d9fd2f5f5e3079dd2600657bff8e1f37f2b8a9633b0b10ea1bf5104d9117` |
| [Qualifier stdout](EC_GNN_027_VERIFY_GP.stdout.txt) | `0cfe33e065ed77355c10e0bec9f5f00924cfa2c8fbba1148af32e4d1d7cf9b8b` |
| [Qualifier stderr](EC_GNN_027_VERIFY_GP.stderr.txt) (empty) | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [Qualifier exit](EC_GNN_027_VERIFY_GP.exit.txt) | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |
