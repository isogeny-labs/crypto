# Generalized GPU rho workers for the nine Certicom binary challenges

- **Status:** `partial` — exact ECCp-131/ECC2-131 native arithmetic is
  host-verified; CUDA device replay and the generalized persistent worker
  remain open
- **Kind:** implementation gap; measurement and hardware access gap
- **Affected work:** the nine Certicom binary challenge records in `src/binary_ecc/curve.rs`, the `ecdlp-nist` rho CLI, and the curve-specific `ecc2k130/` GPU campaign; see also [the matched GPU workload problem](ecbench-gpu-matched-workload-contract.md). The in-progress service worktree has a separate, unpublished record at `/Volumes/SSD990-2/cryptanalysis/wt/ecc-challenges-deploy/problems/multi-campaign-worker-and-verifier.md`.
- **Next action owner:** Conductor task `T-4099` for the exact ECCp-131 and
  ECC2-131 arithmetic path; the `cryptanalysis` service worktree owns its
  separate v2 campaign integration
- **Last updated:** 2026-10-18

## Problem

The repository cannot yet run a persistent, independently replayable GPU Pollard rho campaign on all nine Certicom binary challenges because the existing high-throughput client is specialized to ECC2K-130 and the generic binary rho solver retains its distinguished-point table in process memory.

The affected groups are ECC2K-130 and ECC2-131 over GF(2^131); ECC2K-163 and ECC2-163 over GF(2^163); ECC2-191 over GF(2^191); ECC2K-238 and ECC2-238 over GF(2^239); and ECC2K-358 and ECC2-353 over GF(2^359). Each has its own published field polynomial, coefficients, subgroup and target point. Random binary curves cannot inherit a Koblitz Frobenius quotient without a separately valid automorphism. The requested acceptance criterion is an end-to-end worker, durable distinguished-point ingest and collision verification for each exact challenge, with the fastest validated arithmetic selected by matched measurements on its intended hardware.

## Evidence and uncertainty

- **Observed:** `ecc2k130/src/main.cu` dispatches its configured degrees 131, 97 and smaller validation fields; it has no dispatch for all nine challenge records. `src/cryptanalysis/ecdlp_nist/rho.rs` uses an in-memory `HashMap` for distinguished points. The current development host is arm64 and has no `nvcc` or NVIDIA device utility. Certicom's parameter source is https://www.certicom.com/en/curves-list.
- **Cross-repository observation, not an archived release:** On 2026-10-16, the dirty `cryptanalysis/wt/ecc-challenges-deploy` worktree contained a candidate generic CPU/CUDA worker, versioned storage and a disabled-by-default v2 service path. Its earlier 24-curve CPU and L4 arithmetic parity receipts were for a raw-coordinate distinguished-point predicate. A finite control then found that predicate unsuitable on one curve; the replacement hashes the canonical point and requires fresh CPU/GPU controls, a pinned image and a live admission/collision canary. That worktree's problem record owns those service gates. This observation does not certify the revised worker or the released image.
- **Derived or proved:** A walk rate alone omits durable DP transfer, merge, collision replay and verification. End-to-end throughput and a fastest-kernel choice therefore require measurements covering those phases. No mathematical obstruction is established.
- **Suspected explanation:** Unknown whether one field representation and kernel geometry will be best at all five field degrees; this is a measurement question.
- **Missing capability in this repository:** an integrated GPU path and durable DP store for the nine catalog entries. For the separate service candidate, revised-walk controls and release canary remain open. Matched device performance receipts and an external fastest-tool comparison are absent for both paths.
- **Native two-curve arithmetic candidate (2026-10-18):**
  `gpu/certicom131/` now preserves the recovered ECCp-131/ECC2-131
  parameters and vectors and adds one native host/CUDA implementation and a
  stable verifier.  The host verifier passed 256 field-vector sets per curve,
  16 scalar vectors per curve, point identities, and four subgroup checks;
  the final merged-head Rust library suite passed 4,399 tests after two
  tracked sparse fixtures were materialized.  CUDA device replay is not complete: an
  approved 30-minute `g7e.2xlarge` correctness job was rejected before
  instance creation because the AWS GPU vCPU quota is zero.  This establishes
  host arithmetic for these two exact curves only; it does not establish a
  GPU result, a persistent worker, or coverage of the other challenge curves.
  See `gpu/certicom131/PROTOCOL.md` and `gpu/certicom131/RESULT.md`.
- **What this evidence does not establish:** It gives no speed ranking for the nine curves and no conclusion about the feasibility of their discrete logarithms.

## What would unblock it

- [ ] Bind all nine exact curve and target records to independently checked generator/target subgroup tests and stable curve IDs.
- [ ] For each curve, pass CPU-reference equivalence of field operations, point operations, coefficient bookkeeping, distinguished-point records, restart and collision verification across ordinary and exceptional cases.
- [ ] Persist and resume DP collection with exact curve/workload identity, zero unexplained drops and independent collision replay.
- [ ] On each intended GPU class, compare candidate kernels with the current valid baseline using the same frozen workload, A/A controls, device safety checks, full phase accounting and uncertainty; retain the measured winner per curve and hardware profile.

These conditions would establish runnable, validated implementations and qualified performance choices at the measured devices and workloads. They would not establish an end-to-end attack-cost advantage over another algorithm without a separate complete cost and success-probability analysis.

## Next discriminating step

- **Action or proof obligation:** First replay `gpu/certicom131/test_cuda.cu`
  on an approved CUDA 13.x RTX PRO 6000 runner and retain the device receipt.
  Then replay the corrected `cryptanalysis` v2 hash-walk on ECC2-131 with CPU
  and CUDA, extend the same frozen controls to the other eight binary
  challenges, and complete its service canary.  The arithmetic verifier does
  not replace those persistent-worker and durable-store gates.
- **Inputs and controls:** the exact ECC2-131 field/curve/subgroup record; an independent binary affine reference; a planted scalar with known answer; the published target as an answer-free collection input; nonempty distinguished-point output, matched kernel baseline and A/A timings.
- **Execution limits and stop condition:** freeze the seed, step count, DP predicate, hardware and maximum process memory before running; stop on any arithmetic mismatch, record drop, sanitizer fault or resource watchdog. These limits bound the validation run, not the broader campaign budget.
- **Outcomes and next actions:** passing replay permits the remaining eight curve replays, service canary and measured tuning; a mismatch localizes a correctness defect; a bounded no-collision run says nothing about the target logarithm; missing hardware or a failed launch is infrastructure evidence and is rerun only after the dependency is restored.
- **Recheck trigger:** the first corrected-walk ECC2-131 GPU replay receipt or a generalized worker design with a verified backend.

## Resolution and history

- 2026-10-16: Opened while integrating the nine published parameter records. GPU and durable campaign support remain unimplemented in this repository for the full set.
- 2026-10-16: Cross-linked the in-progress `cryptanalysis` v2 service candidate and its revised distinguished-point predicate gate. Earlier arithmetic receipts do not release the revised walk.
- 2026-10-18: Added host-verified native arithmetic and frozen known-answer
  replay for exact ECCp-131 and ECC2-131.  The CUDA verifier is implemented
  but its device run was rejected before launch by a zero AWS GPU vCPU quota;
  no device or performance result is recorded.

- 2026-10-22: Single-core CPU baseline for the three 131-bit targets ([study](../research/certicom131_rho_ic_bench_20261022/README.md)): batched-inversion walks on the exact ECC2-131 and ECCp-131 groups with checked arithmetic (8.50 M and 1.23 M steps/s). No CUDA result; the device replay above remains open.

- 2026-10-23: Single-core CPU parity study ([study](../research/certicom131_rho_parity_20261023/README.md)): ECCp-131 AVX-512 walk 15.06 M and ECC2-131 bitsliced walk 11.46 M steps/s against 10.3 M for the ECC2K-130 client in one isolated session, all gates passing. No CUDA result; the device replay above remains open.

- 2026-10-23: CUDA port of both walks ([study](../research/certicom131_cuda_port_20261023/README.md)): per-thread code gated on the CPU against the header arithmetic, kernels compile for sm_80 to sm_90 (and retargeted sm_120). No device parity result and no GPU rate: `walk_cuda` has not run on a GPU. Running it is the next step on the first CUDA machine.

Partially resolved.  Completing the two-curve host arithmetic does not
complete the requested GPU and durable campaign integration.
