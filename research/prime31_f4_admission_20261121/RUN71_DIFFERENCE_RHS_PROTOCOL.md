# Prime31 Run71 — source-bound target-difference RHS rank and stop gate

Date: 2026-11-22 UTC. Registered *before accepting execution evidence*; current status: pending original Rust Actions. This is a **GENERIC CONTROL**, not a new polynomial relation oracle, a complexity theorem, or evidence of an index-calculus advantage.

## Mathematical invariant

On the existing ordinary cofactor-one E/F_(2^31-1), order n=2147446831, public G, 677-element signed factor base and 88 group-law-verified homogeneous rows, let L_i=log_G(P_i). For each unknown-index public target Q_t and four separately verified signed triple decompositions Q_t+[j_(t,r)]G=Σ c_(t,r,i) P_i, the equation

Σ (c_(t,r,i)-c_(t,0,i)) L_i = j_(t,r)-j_(t,0) (mod n)

has a known RHS without knowledge of log_G Q_t. All **96** candidate differences (32 targets × 3) must be independently replayed using the original Rust group law before being added to the exact modular rank system. Rank is not presumed to be 96.

Under an authentic pair-RHS signed-DSU reduction L_i=s_i·U_root+b_i, free and difference rows are specialized into the same unknown-root columns and reduced with exact modular RREF (not an equation-count proxy). A candidate log is accepted only if its affine form reduces to zero unknown columns, followed by original group proof [x]G=Q, secret-only harness comparison and independently computed BSGS.

## Frozen paired protocol

- Reuse the original Rust crate from `research/prime31_ic_20261115/native`, including the Run69 `Arithmetic`, `Base`, RHS signed DSU, verified group-law rows, and `target_values`. Do not rebuild EC arithmetic.
- Use the exact Run69/70/70B deterministic 32 public Q cohort and the Run70B K=4 public certificate order, with original `0x68B677A1D1` PRNG seed, B677 and 916469-key GENERIC reference pair table.
- Compare two arm-specific *exact query-stop counts on the identical relation stream*: K4 readout-only (existing Run70B control) and K4 plus 96 true-RHS difference rows, checking only at multiples of 32 actual pair hits. No internal scalar or secret target index enters candidate extraction, rank or stopping logic.
- Source-bound K4 reference MUST replay `2,528,933` real public shifts / `1,088` authentic hits / `546` independent pair-DSU rows with 32 verified target logs. The difference arm is allowed to equal, improve or be censored. Do not claim a saving until measured.
- Report initial rank(88 free rows) and rank(88 free + 96 differences), projected free and augmented ranks at the augmented stopping point, all failed shifts and leaf probes, exact number of signed group proofs and 32/32 verified logs.
- Report setup, witness, **joint two-arm walk** and total wall time, plus same-process shared BSGS; the joint loop deliberately waits for the slower arm and **MUST NOT** be described as a cold difference-only solver. For an actual cost advantage, implement and benchmark a separate early-stopping cold arm with its own setup and extraction charges.
- `cargo test --locked --release --lib` and `cargo run --locked --release --example prime31_run71_difference_rhs` are the source-linked Rust 1.100.0 commands in workflow `.github/workflows/prime31-run71-difference-rhs.yml`. Failure or timeout is a censored result. Record SHA and link.
- Next panel: K=1/4/16/32, T=1/32/128, independent public seeds and negative controls; then replace BOTH generic pair table and triple enumeration with the public-only nonenumerative polynomial extraction owner (#4692/#4812/#4822).

## Prior lead check and ownership

- Run65 eight-shift correlation benefit negligible; avoid more near-neighbor correlation tuning.
- Run68 true-RHS all-base solve, Run69 early quotient stop, Run70/70B K4 public multiple-certificate readout and fully charged **7.2× slower-than-BSGS generic control** established the reference paths.
- #4809 quadratic S3 projective/membership primitive is a reusable arithmetic component, not the blind RR9 oracle.
- #4812 transformed-slot Horner/F3 and #4822 sparse 4-variable Krylov have source-verified polynomial identities/positive tests but no qualified blind unknown-index extraction throughput. #4692 has 46 positive / 18 negative registered gate and external checkpoint blocker.
- #4919 B8 public-target candidate is a separate new extractor lane; #4921 FASS symmetric S3 is a *proposal*, unbenchmarked, and its balanced-pair join is generic.
- Literature: Semaev (2004), `https://eprint.iacr.org/2004/031`; Amadori–Pintore–Sala (2018), `https://doi.org/10.1016/j.ffa.2018.01.009` (one-relation strategy already explored). The target-difference identity is elementary; **no novelty or asymptotic claim**. Research-KB MCP unavailable in this runtime; original repo PRs/AGENTS.md and primary bibliography serve as fallback.

**Success metric:** strictly more independent authentic RHS rank, fewer fully witnessed public relation queries and verified complete target logs per *fully charged* solver cost. A faster original-Rust generic reference alone does not meet the user's non-generic/index-calculus requirement.
