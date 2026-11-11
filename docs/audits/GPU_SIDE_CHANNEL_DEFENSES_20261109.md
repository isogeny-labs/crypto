# GPU side-channel defenses and an audit of CUDA `clmad` — 2026-11-09

Desk study. Nothing here was run on a GPU. Every statement is an assertion
`[pcc:unverified]` unless a file and line are given. Literature items marked
"unverified" were seen only in secondary sources or recalled, not read.
The research-kb catalog was not available in this session, so no
knowledge-base check was made.

Scope (owner, 2026-11-08): attack and benchmark kernels stay variable-time.
Defenses below matter for (a) protecting our results on shared GPUs and
(b) any future GPU code that handles real secrets.

## 1. Defenses by channel

| Channel | Representative attack | Defense available to us | Gap |
|---|---|---|---|
| Secret-indexed loads, coalescing timing | Jiang et al. HPCA 2016 (unverified); Trident, HPCA 2021 | Table-free or bitsliced code; `clmad` instead of tables | Hardware fixes (BCoal, TridentShield) are not deployable on rented parts |
| Co-tenant contention (SM, L2) | Rendered Insecure, CCS 2018 (unverified) | One tenant per GPU, or MIG; never MPS or time-slicing between tenants | No independent analysis of MIG isolation found |
| Multi-GPU interconnect | NVBleed, arXiv:2503.17847 | Do not share NVLink-connected sets | No published defense |
| Leftover memory | LeftoverLocals, CVE-2023-4969 | Scrub before free (cryptanalysis #1359) | SIGKILL, driver reset, checkpoints on disk |
| Counters and telemetry | Rendered Insecure; NVBleed | `NVreg_RestrictProfilingToAdminUsers=1`; no NVML in tenant containers | Default and container behaviour per driver not confirmed |
| Power and EM | BarraCUDA, arXiv:2312.07783; Kraken, arXiv:2603.02891 | None needed for rented GPUs (needs proximity) | No GPU masking literature found |
| Rowhammer on GPU memory | GPUHammer 2025 (venue unverified); GPUBreach, GPUThor (unverified) | System-level ECC; verify results | Integrity risk to our own computations |
| Confidential computing | H100 TEE (unverified this session) | Closes host and bus reads | Does not close timing or contention inside the GPU |

## 2. Ideas not found in the literature (speculation)

1. A constant-time checker over PTX or SASS: flag secret-dependent branches,
   secret-indexed loads and variable-latency operations. The `ct_dynamic_audit`
   harness has no GPU counterpart.
2. A warp-uniformity proof: if every lane of a warp takes the same path,
   divergence cannot carry a secret. Checkable from SASS predicates.
3. Result verification as a fault defense. Distinguished points are cheap to
   re-walk on a CPU; a sampled re-walk detects Rowhammer flips and miscompiles
   alike. `ec2k-gpu walk --verify` already exists and normally runs at 0.
4. A leftover-memory canary: a kernel that reads fresh allocations at start-up
   and reports non-zero pages, run on every rented GPU before a campaign.
5. Blinded walk state: store walk coefficients XORed with a per-run key held
   only on the host, so residue on the device is useless without it.
6. dudect-style two-class timing on GPU kernels in CI, using `%clock64` inside
   the kernel to avoid launch jitter.
7. Checkpoint encryption at rest, closing the on-disk gap `docs/GPU.md` lists.

## 3. Audit: `clmad` (CUDA 13.3, PTX ISA 9.3)

`clmad` is an instruction, not a library: a 64x64 carry-less multiply with a
64-bit XOR addend, `.lo` and `.hi` forms, sm_80 and newer. NVIDIA's
announcement (2026-07-15) makes no statement about timing, constant time or
side channels. No public source found states its latency is data-independent.

| # | Finding | Evidence | Severity for us |
|---|---|---|---|
| A1 | No vendor timing guarantee. Data-independent latency is unstated. | NVIDIA blog; PTX docs section 9.7.1.5 not read in full | Matters only for secret-handling code (GHASH keys, HQC, BIKE). Not for our walks |
| A2 | Our own measurement says issue rate does not vary with operand width. | `cryptanalysis/ecc2k130/README.md:192-194` | Supports data independence, but it measured throughput, not a two-class secret test. Not a proof |
| A3 | Public report: `ptxas` 13.4.92 on sm_120 miscompiles a `clmad` whose multiplicand folds to the constant one, at -O2 and -O3. The high half reads an unwritten register, so the wrong bits vary between runs. Reported to NVIDIA 2026-10-19 as bug 6955062; no fix seen. | github.com/robik75/ptxas-clmad-constant-operand (third party, not reproduced here) | Correctness, and a possible leak of stale register contents into outputs |
| A4 | Our exposure to A3 looks low. The batched-inversion chain starts from the first factor, not from one (`ecc2k130/include/packedkernels.cuh:513`). All `clmad` call sites take run-time operands (`include/packed131.h:189-190, 252-257, 293, 502-511`). Documented builds use CUDA 13.3.x, not 13.4. | grep of this checkout | Low now. Rises if a toolkit upgrade or a new kernel introduces a constant operand |
| A5 | Builds use `-O3` with no pin on the `ptxas` version. | `ecc2k130/Makefile:200` | An upgrade to 13.4.x would land silently |
| A6 | Fallback paths differ in timing character. `clmulTiny` and the software product are masked shifts; host paths use PCLMULQDQ or PMULL. | `include/packed131.h:286-303`, `include/hostclmul.h` | None for walks; note for any reuse on secrets |
| A7 | The squaring path uses `clmad(x, x)`. Both operands are the same register, a pattern a compiler can special-case. | `include/packed131.h:786-800` | Unknown. Covered by the square test in `runner/src/testpackedcuda.cu:204` |

Recommended, in order:
1. Record the `ptxas` version in every run manifest and refuse 13.4.92 for
   `clmad` builds until NVIDIA closes bug 6955062.
2. Run the device-versus-host tests (`make test-clmad`, `testpackedcuda`) on
   every new toolkit before a campaign, at the production optimisation level.
3. Reproduce the third-party repro on our RTX PRO 6000 class card. A confirmed
   or refuted repro is a small publishable result either way.
4. If GPU code ever handles a secret through `clmad`: run a two-class timing
   test (fixed versus random operands, low versus high Hamming weight) with
   `%clock64`, with a positive control, before relying on it.

## Follow-up (2026-11-10): the audit's open items, run

Measured in `cryptanalysis` PR #1459, study `research/clmad_audit_20261110/`
(protocol frozen before data; record `benchmarks/records/20261110T0408Z-clmad-audit.json`;
all `[pcc:unverified]`, one run per arm, no independent replay yet):

- **A3 reproduced, and wider than reported.** The public reproducer
  (github.com/robik75/ptxas-clmad-constant-operand, run unchanged on RunPod
  RTX PRO 6000 pods) fails at `-O2`/`-O3` and under driver JIT for
  N = 14, 15, 18, 19 and passes at N = 16 and at `-O1`/`-O0`, under **both**
  `ptxas` 13.4.92 and 13.3.73. The defect is not specific to 13.4 Update 2.
- **A4 holds.** An independently written Karatsuba reproducer folds the same
  constant-one step (6 fewer `CLMAD` in SASS) and still computes correctly on
  both toolkits; the defect is sensitive to the surrounding code. The
  `ecc2k130` device tests pass on both toolkits with the production defines.
- **A5 closed.** Both `ecc2k130` Makefiles read the `nvcc` version and refuse
  a `clmad` build on a `BAD_PTXAS` entry unless `ALLOW_BAD_PTXAS=1`; the GPU
  client prints `nvccVersion` in its `devices` and `bench`/`walk` JSON. The
  default list is empty, since the affected versions include the production
  toolkit and the kernels pass their device tests on it: the tests on the
  toolkit in use are the guard.
- **A2/A7 measured.** A two-class timing test (dependent `clmad` chain,
  `clock64`, eleven operand classes, positive and negative controls, shuffled
  launch order) found no operand class distinguishable for `clmad.lo` or
  `clmad.hi` on an RTX 5090 (55.08 cycles per operation, one warp) or an RTX
  PRO 6000 (63.53), nor at two blocks per SM (about 255.9). A bound at that
  sample size, not a constant-time guarantee.

## Sources

NVIDIA blog on clmad: https://developer.nvidia.com/blog/building-faster-cryptography-with-carryless-multiplication-in-nvidia-cuda-13-3/ · [pcc:unverified]
PTX ISA: https://docs.nvidia.com/cuda/parallel-thread-execution/index.html ·
ptxas repro: https://github.com/robik75/ptxas-clmad-constant-operand ·
LLVM NVPTX clmad: https://github.com/llvm/llvm-project/pull/216412 ·
FAEST on GPUs: https://eprint.iacr.org/2026/2489 ·
NVBleed: https://arxiv.org/abs/2503.17847 ·
BarraCUDA: https://arxiv.org/abs/2312.07783 ·
Kraken: https://arxiv.org/abs/2603.02891 ·
LeftoverLocals: https://blog.trailofbits.com/2024/01/16/leftoverlocals-listening-to-llm-responses-through-leaked-gpu-local-memory/
