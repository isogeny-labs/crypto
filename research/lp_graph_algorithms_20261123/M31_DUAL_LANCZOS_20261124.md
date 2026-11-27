# M31 dual Gram: checked scalar Lanczos fixed-RHS reference

Status: deterministic fixed-input algebraic reference, [pcc:unverified]. Not a novel Lanczos algorithm, native execution, timing comparison, or ECDLP solve.

## Custody and scope
Scope notice #5122 comment 7133186291 before computation. Audit branch parent 1050c8f01372c986fcc6bc8296cd59b18cb8f858. Same ten-query B677 capture at 577d6c2c1c4a527604d965e6fb62c44df7902f36:research/prime31_graph_ecdlp_20261123/evidence/mitm-b677-control-01/ten-query.json (blob 7fd27b9888058399d8814cef373d373243e6a1e5), degree-three tape blob 72f2a2240994e3436ab8e5a1910a31cf1949bdcf, symmetric reference blob ec445ea06003de5e29a8b42b13fa459b65586854. Checked all three blob identities. Curve p=2147483647, a=847514846, b=2087444724, group/subgroup order r=2147446831, h=1, G=(625037715,2100627251), public Q=(976979610,1228979761). Arithmetic over F_r. Historical seed 0x7033315f6d340101 unchanged. Study m31-dual-lanczos-v1 has no new PRNG, seeds, independent replicates or randomized measurements; original capture reused intentionally. Historical producer binary/rejection trace gaps remain; not a qualified general A/B. No timing, paid compute, CI dispatch, merge or new collection.

Sources checked (KB): unavailable; current AGENTS, ACTIVE, dated main and branch journals, graph catalog/SOURCES, open #5122/#5120/#5076; bounded cross-repo code and open Lanczos PR searches. Existing Rust prior work: research/ic_gw_nrm_lz_20261115/src/la2.rs blob 146d02f41620bbc313e7bb9f66ba290245018963, scalar A-orthogonal Lanczos with LZ_REUSE identities, originally for kernel extraction on a weighted Gram. Its historical results do not establish M31 cost. Current pq_wiedemann.rs blob b1712946b81487bf5c4c45fdb3e969e9edb70aec. Literature: Coppersmith 1993, https://doi.org/10.1016/0024-3795(93)90235-G, finite-field breakdown motivation (GF(2), not this instance); Thomé 2016, https://arxiv.org/abs/1604.02277, block Lanczos storage and homogeneous systems. Do not duplicate #5120 native implementation or #5076 collection.

## Recurrence and proof
D is the existing 363 x 342 reduced point-constraint matrix, c compressed target functional, A=D^T D. For fixed RHS use w_0=c, w_-1=0, x=0. At step i compute v=A w_i, s_i=w_i^T v. If w_i=0 stop, if s_i=0 refuse, otherwise:
alpha_i=(v^T v)/s_i;
beta_i=s_i/s_(i-1) for i>0, beta_0=0;
gamma_i=(w_i^T c)/s_i;
x <- x+gamma_i w_i;
w_(i+1)=v-alpha_i w_i-beta_i w_(i-1).
The previous inverse is retained. Symmetry and A-orthogonality of preceding nonbreakdown vectors imply v_i^T v_(i-1)=s_i; earlier terms vanish using the three-term recurrence. This inductively makes the nonzero w_i mutually A-orthogonal. The scalar RHS dots t_i=w_i^T c satisfy t_0=c^T c, t_(i+1)=(i=0 ? s_0 : 0)-alpha_i t_i-beta_i t_(i-1), since c=w_0. Exact termination spans the Krylov space, which contains c and is invariant under A. Nonzero s_i give a nondegenerate restriction; the accumulated x solves A x=c on that space. Independently check A x=c, D^T(Dx)=c and original M u=b after reverse tape. A zero denominator or cap is a refusal, not evidence of target identifiability. No assumption of positive definiteness over F_r, no mandatory dense pretest. Use existing checked seeded projection/preconditioning or exact fallback, charging failures.

## Fixed-input execution
One coordinator, no workers. Connector-side exact BigInt reference (not new production code). Rebuild point maps and target coefficients from signed rows and query b values; verify every pivot expression as a map and point-nonzero delta, preserve pre-elimination incidences. Sort live IDs ascending and compare to previous reference. Apply A as D^T(Dv). At each step check recurrence RHS dot directly and, after first step, cross-product identity directly. Bound 342 iterations, stop on zero vector, zero denominator or cap; never reroll. Compute u=D x, reverse tape with u_i=(c_j-sum_{k != i} D_{kj}u_k)/alpha_pivot, and check original signed equations. Raw receipt M31_DUAL_LANCZOS_20261124.json (blob 087e171304a14bc9fcd2faae953820e1bfedf697) retains 342 scalar traces and complete x/u. Initial preflight failed because comparator serialized noncanonical expression ordering; corrected by sorting both lists, all 303 match; no Krylov step executed on failed preflight. No altered input.

| Quantity | Result |
|---|---:|
| Pivot expressions/deltas | 303/303 |
| Nonbreakdown Lanczos steps | 342 |
| End state | zero next vector |
| Exact dot identity comparisons | 683/683 |
| Gram solution vs prior symmetric Wiedemann | identical |
| Lifted multipliers vs prior | identical |
| Gram/reduced/original residual checks | all pass, 645/645 original |

Compared with the prior fixed reference's generation+reconstruction Gram calls (excluding common check), ordinary moment path 1024, symmetric moment path 683, scalar Lanczos 342. The last saves 341 calls vs symmetric, 1,387,188 coefficient contributions at 4068 per call; 49.93% of that component's application count. This is not a runtime A/B. Lanczos also needs vector updates, 342 inversions and dot products; source LZ_REUSE can reduce direct dots and retain previous inverse, but audit-only identity cross-checks were charged separately in the reference. O(n(Z+m+n)) arithmetic plus inversions, O(Z+m+n) working storage without diagnostic trace, same asymptotic class. Native timing may be worse if other costs dominate, or if breakdown/fallback occurs. Capture remains target-nonidentifiable.

Next executable step: #5120 native owner should compare this checked fixed-RHS variant using existing scalar Lanczos code and matrix-free Gram operator against symmetric and independent seeded Wiedemann, with actual native costs and failure paths. Freeze seed manifest, matched input hashes, A/A, replicate panel and timing order before randomized/timing work; fresh held-out M31 instances for general claims. Do not launch competing native runs. Local checkout, cargo/rustc, Conductor, KB and worker tools unavailable; no global admission. Phase elapsed/CPU/RSS unrecorded. Branch-only append, canonical-main reconciliation and independent PCC open. Preserve M31, experiment-seeds/v1, parallel/resource, publication/no-merge and stop/impact requirements.

Impact score: 4/10 — Fixed-M31 scalar Lanczos terminates in 342 steps with exact agreement to prior solution and 645/645 original checks, removing the separate reconstruction pass in this deterministic component reference. Native costs, breakdown rates and complete cold solver advantage remain unestablished.
