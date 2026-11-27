# Ideas 35–67: bounded execution record

Date: 2026-11-24 (America/Los_Angeles). This record continues the 100-idea screen with arithmetic checks and a readback of historical GPU records. It does not substitute field-integer smoothness for elliptic-curve point decomposition. All arithmetic inputs below are synthetic and public. The checkout was `c86bb64e48a0b621eaa15dd9589e85301e4065fb` when the checks began. Numerical and execution statements are `[pcc:unverified]` producer records. No paid resource was launched.

## Frozen checks and results

The checks address these discriminators: (i) same graph support can have different modular rank; (ii) an ideal reversible oracle can amplify a marked state, but this says nothing about its EC relation circuit cost; (iii) a real solution need not equal a finite-field solution; (iv) a sparse matrix can have maximal tensor-network cut rank; and (v) public group arithmetic can reject a tampered relation directly. The identities below are exact, hand-checkable calculations, with no timing claim.

| Check | Exact output | Reading |
|---|---|---|
| #35–36, #38–39 | `A_rank_F5=3`, `B_rank_F5=2`, `same_nonzero_support=true` for `A=[[1,1,0],[0,1,1],[1,0,1]]`, `B=[[1,1,0],[0,1,1],[1,0,-1]]`. | Their triangle support and every topology derived solely from that support coincide, but modular rank differs. The triangle 1-cycle does not imply a row dependency in `A`. Topology can still guide a heuristic if a coefficient-aware map and measured yield are supplied. |
| #40 | Grover marked probabilities for `N=16`, `M=1`, iterations `0..4`: `0.0625, 0.47265625, 0.908447266, 0.96131897, 0.58170414`. | Amplitude amplification is real for an ideal oracle. The EC relation predicate and reversible implementation were not specified, so this is an oracle-call demonstration only. |
| #42 | `A=[2]`, `b=[1]`: real solution `0.5`; solution modulo `5` is `3`, verified by `2*3 mod 5=1`. | A real/complex HHL state cannot be read as an exact modular logarithm without an additional reconstruction argument and cost. |
| #46 | In `F_23^×`, `2` and `13` both have multiplicative order `11`; the multiplication permutations have the same two 11-cycle spectra. At `B=3`, canonical integer `2` is smooth and `13` is not. | This screens the attachment's multiplicative-field proposal only. It has no `E(F_p)` relation or performance implication. |
| #47 | The 16×16 matrix swapping two 2-bit registers has 16 nonzeros (one per row) and operator-Schmidt rank 16 over `F_5`, the maximum across the 2-bit/2-bit cut. | Sparsity alone does not bound tensor-network bond dimension. This does not rule out low-rank structure in a particular relation matrix. |
| #61–62 | On `y²=x³+x+14` over `F_1009`, `P=(0,425)`, `Q=2P=(991,641)`. `Q−2P=O` checks true; tampered `Q−P=O` checks false. | A public relation can be checked with ordinary group arithmetic. A zero-knowledge design needs a concrete hidden witness or privacy requirement. |
| #55 | For an unsigned 8-bit dot product into signed 32-bit accumulation, `255²·33025=2147450625 ≤ 2³¹−1`, while `255²·33026=2147515650 > 2³¹−1`. For signed magnitude 127 the analogous cap is 133144 products. | Exact tensor-core limb schemes must segment accumulation or widen it; this is an arithmetic bound, not a device-speed measurement. |

Replay the algebra without a research script: `det(A)=2` and `det(B)=0`, with the first two rows of `B` independent modulo `5`; the ideal-oracle marked probability after `t` Grover iterations is `sin²((2t+1)arcsin(1/4))`; `2·3≡1 (mod 5)`; `2^11≡13^11≡1 (mod 23)` and neither base is `1`. The sparse SWAP operator's 16-by-16 unfolding is a permutation matrix, so its rank is 16 over `F_5`. The accumulator boundary follows from `255²·33025=2147450625≤2³¹−1<2147515650=255²·33026`. For the EC row, `425²≡14 (mod 1009)` and the tangent slope `(2·425)⁻¹≡844`, giving `x(2P)≡844²≡991` and `y(2P)≡844·18−425≡641`; hence `Q−2P=O` and `Q−P=P≠O`.

An initial scratch Python check was run before the implementation-language rule in `AGENTS.md` was noticed. That run is excluded from the evidence and from the deliverable; the displayed identities stand on the explicit arithmetic above. No further research run was launched after the S3 checkpoint gate returned HTTP 403.

## Item-by-item pursuit and exact next gates

The source-only scan `rg -i -l TERM src gpu` was run for each named technique in this checkout. It found no EC index-calculus implementation for persistent homology, sheaf propagation, Mapper, spectral clustering, quantum walk/HHL/VQE, annealing, memristors, optical computing, GPUDirect, or serverless sieving. This is a checkout-local source scan, not an absence claim about all repositories or literature. The existing EC large-prime filter and black-box solver are described in [`docs/ic/ECDLP_RESEARCH_TECHNIQUES.md`](../../docs/ic/ECDLP_RESEARCH_TECHNIQUES.md); they are the controls below.

| # | Work performed and result | Next discriminating gate or blocker |
|---:|---|---|
| 35 | Triangle counterexample above rejects support-only cycle extraction. | Give a coefficient-preserving map from a persistent 1-cycle to a checked point relation; compare existing multi-large-prime cycle closure. |
| 36 | Same-support `A/B` pair has the same support-derived Euler characteristic at every support-only filtration level but different `F_5` ranks. | Supply a filtration using coefficients and show its stop signal predicts exact rank and target coverage. |
| 37 | Source scan found no sheaf restriction maps for the EC sieve. | Specify stalks, restrictions, and a global-section-to-verified-relation map; compare ordinary distributed joins. |
| 38 | No Mapper embedding/cover found; `A/B` shows support topology does not determine rank. | Freeze an embedding, cover and partition; measure independent verified rows per total time against random partition. |
| 39 | The triangle support is connected, so spectral partitioning cannot produce an exact block diagonal matrix without cutting an edge. | Use a fixed recorded matrix and count cut nonzeros, Krylov communication and full solve cost versus existing sharded SpMV. |
| 40 | Exact ideal-oracle `N=16` calculation above demonstrates amplification. | Define a reversible EC decomposition oracle and include its gates, memory, success probability and readout in the comparison. |
| 41 | No quantum-walk state graph or marked predicate supplied or found. | Provide transition, implementable reflection/oracle and hitting-time bound for verified EC relations. |
| 42 | Scalar real-versus-`F_5` counterexample above. | Construct exact modular output and charge tomography/readout, or switch to a finite-field quantum linear algorithm with a verified output contract. |
| 43 | No Ising encoding or qubit Hamiltonian supplied or found. | Encode one fixed EC point-decomposition instance, show zero energy iff a group-checked relation, and count logical/physical resources. |
| 44 | Same missing exact encoding as #43; no annealer access was identified in this local environment. | Give quadratization, penalty gap, hardware graph and verified yield per run versus classical F4. |
| 45 | No modular Clifford preconditioner supplied. Any invertible left/right transformation preserves rank but not automatically sparsity or Krylov time. | Specify an exact `F_r` operator and compare matrix fill and total solve. |
| 46 | Same-spectrum, different-smoothness pair above rejects order-only filtering. | A separate factorization or decomposition oracle is required and must beat direct checking. |
| 47 | Sparse SWAP achieves maximal operator-Schmidt rank above. | Compute cut ranks/bond dimensions on actual frozen relation matrices, including decomposition ordering and full exact solve. |
| 48 | No zero-energy iff certified-relation objective supplied or found. | Implement one exact objective and compare verified yield per compute unit to existing search. |
| 49 | No memristive grid interface found; analog output has no specified exact modular reconstruction. | Supply modulus/precision/error budget and independently verified full solve on accessible hardware. |
| 50 | No candidate-to-flow feature or equation supplied or found. | Define a prespecified score and test held-out verified relation enrichment against uniform candidates. |
| 51 | No soliton-to-point-relation map supplied or found. | Give an explicit map and test its checked output on a fixed curve. |
| 52 | No spin encoding of the exact modular kernel supplied. | Demonstrate zero-energy iff nonzero modular kernel vector, then charge decoding and verification cost. |
| 53 | Local machine has Apple M4 Pro/Metal 4, no FPGA toolchain or device discovered in this pass. Existing block Wiedemann/SpMV is the CPU control. | Identify FPGA target, exact modular kernel, device access and full operator throughput. |
| 54 | No local `nvidia-smi` or `nvcc`; historical A100 rootfind and kernel records inspected below. | Compare asynchronous queue against recorded synchronous path on identical frozen candidates, including transfers and full verified relation rate. |
| 55 | Accumulator-length bound above; no local CUDA compiler or tensor-core GPU. | Exact limb/carry oracle and A/A then ABAB device measurement against integer CUDA. |
| 56 | No optical matrix device, precision contract or interface identified. | Show exact `F_r` reconstruction and full modular solve, with correction/verification cost. |
| 57 | Historical GPU audit identifies static 48 KiB rootfind shared allocation and runtime-indexed local-memory arrays; no new device profile here. | Profile current kernel memory stalls and compare layouts on the same workload, with full solve cost. |
| 58 | No processing-in-memory device/interface found. | Specify exact relation primitive and hardware path, then compare verified relations/J including transfer. |
| 59 | No neuromorphic device/interface or spike-to-relation decoder found. | Independently verify decoded rows; compare yield/J and false-positive cost. |
| 60 | No local NVIDIA device or GPUDirect-capable storage path; historical records show GPU startup/IPC and matrix costs can dominate. | First show NVMe-to-GPU transfer dominates a frozen full run; then compare direct-storage pipeline. |
| 61 | Synthetic public EC row above accepted; changed coefficient rejected by direct arithmetic. | If incentive/proof is needed, state network threat model and compare direct verification with proposed proof overhead. |
| 62 | Same direct-verification control as #61. | Specify which witness must be hidden and produce a proof circuit whose verification is cheaper than the direct check under that requirement. |
| 63 | No private input or MPC protocol specified for this public synthetic instance. | Identify parties, secrets, leakage and complete modular matrix protocol, with communication and wall cost. |
| 64 | The public field and public relation require no encryption in the checked instance; no FHE sieve circuit supplied. | State a genuinely private input and measure exact encrypted predicate, bootstrapping and verification cost. |
| 65 | Content hashing would authenticate bytes but cannot reduce the arithmetic rank of `A/B`; no IPFS matrix benchmark supplied. | Compare cached and cold block matvec latency, full solver wall and storage costs on identical bytes. |
| 66 | Source scan found unrelated federated-learning references but no EC sieve update protocol. | Freeze a held-out time-ordered relation stream and compare central versus federated tuning, including communication. |
| 67 | No serverless relation worker found; the experiment needs a costed candidate generator first. | Compare verified rows per dollar and complete wall time with persistent workers, charging startup and transfers. |

## Historical hardware boundary and access

The local access command returned `nvidia-smi=null`, `nvcc=null`; `system_profiler SPDisplaysDataType` identified an Apple M4 Pro GPU with 20 cores and Metal 4. No AWS, RunPod, FPGA, optical, PIM or neuromorphic job was launched. The checkout does not contain `gpu/spmv/spmv.cu` at this commit, so the old kernel binary was not rebuilt. These are access/source blockers for fresh measurements, not claims that those systems cannot work.

The [A100 kernel record](../../benchmarks/records/20261112T2313Z-ic-gpu-cards.json) gives `spmv_kernel_s` baseline `0.008175 s`, candidate `0.0005398 s`, `n=5`, equal CPU digest, A/A ratio `0.999`. This is 15.14× at the kernel boundary. The [complete-solve record](../../benchmarks/records/20261115T2002Z-ic-gpu-rootfind-la-r3.json) gives a 39-bit, factor-base-size-4096 arm baseline `1.541 s`, candidate `0.305 s`, `n=6`, matched rho `0.0866 s`, with recorded identity checks. Dividing the recorded candidate by its matched reference gives `3.52`; that is the further complete-wall reduction needed merely to equal this particular reference. At factor-base size 2048, `0.223/0.0866=2.58`. These are derived comparisons within the historical record, not measured speedups from this pass or extrapolations to another curve. The [GPU audit](../../docs/perf/20261112-ic-gpu-audit.md) distinguishes compiler estimates from device measurements. The records remain `[pcc:unverified]` under the repository claim-verification rule.

## Continuation

For #35–39, the next useful step is a fixed relation matrix and coefficient-aware feature extractor; the synthetic support counterexample already rejects topology-only rank inference. For #40–47, a reversible relation oracle or exact modular output mapping is the missing artifact; further ideal-oracle simulation cannot establish an ECDLP gain. For #53–60, a current GPU profile on the exact relation workload and available matching CUDA hardware are needed before another kernel optimization. The independent output gate is the complete single-target cost against the matched rho baseline, with setup, collection, filtering, linear algebra and descent separated.
