# EcGFp5 quotient: can a Hermite-form change of order replace msolve's Wiedemann finish? (feasibility)

**Date.** 2026-11-02.

**Question.** This is item 2 of crypto-autoresearcher `KN-OPEN-814b4c`, which is also the first revisit condition of `LEAD-ICP-gfp5-symmetrized-gaudry`. At m = 5, about two-thirds of the per-system cost c of the (Z/2)^4 ⋊ S5 quotient solve is msolve's Wiedemann-based sparse-FGLM finish (`research/ecgfp5_quotient_solve_20261101/README.md` §3). Berthomieu–Neiger–Safey El Din replace that finish with the Hermite normal form (equivalently, the determinant) of a t×t polynomial matrix. They report about 5× at D = 65,536 ([BNS22, Table 2](https://arxiv.org/pdf/2202.09226)). Can that route run on this system, and what does it cost?

**Status.** This is a **dev-scale feasibility test, not a frozen experiment**.

- It is one solve at the dev prime p = 331, on one 4-vCPU / 15.7 GiB host.
- No protocol cell was run, and no c is reported.
- **No improvement is claimed.** The baseline is unchanged: the whole-solve c_cyc from the merged quotient solve (2^46.30–2^46.63), with G1 CLOSE.
- This is a memory-bound null for one implementation on one host. Read it as that and nothing more.

Each number is labelled *measured* (committed record in `dev/`), *derived* (calculated here) or *predicted* (model, not measured).

![Memory and CPU of the HNF-finish dev test](figures/hnf_finish.png)

## 1. Answer

1. **The structural precondition of BNS22 holds at m = 5** (*measured*, `dev/m5p331/hnf.log`).
   - The reduced grevlex basis (|G| = 5,578) has a staircase of D = 65,536 monomials.
   - The staircase splits into t = 4,920 chains in the last variable w, of maximum length 37.
   - Every chain head h of length d has y^d·h as the leading monomial of a basis element (`stab_fail = 0`).
   - So the t×t matrix P ∈ F_p[w] with det P = charpoly(M_w) exists, and its column degrees sum to D.
   - t matches the 4,920 dense columns msolve reported for its multiplication matrix in all three protocol cells (*measured*, `research/ecgfp5_quotient_solve_20261101/results/m5_p*_planted/quot.log`).
2. **Building P is cheap** (*measured*).
   - Streaming 305,355,740 basis terms out of msolve and into P took 118 CPU-s: parse 103 s, staircase 1.4 s, build P 14 s.
   - The msolve finish it would replace costs 20,655 CPU-s (*measured*, p = 4111, merged experiment).
3. **The determinant did not fit in memory** (*measured*, `dev/m5p331/kernel_oom.txt`).
   - PML's `determinant_generic_knowing_degree` (the Labahn–Neiger–Zhou triangular decomposition) was killed by the memory cgroup after about 56 s.
   - At that point it held 13.33 GiB of anonymous memory.
   - That is already above the 12 GiB envelope of the merged protocol. The host has 15.7 GiB.
   - msolve had exited before the determinant started, so the whole host was available.
4. **Why** (*derived*).
   - P holds about t·D ≈ 3.2·10^8 coefficients, which is about 2.4 GiB at 8 bytes per NTL `zz_p`. Peak RSS after build_P was 6.97 GiB, with the parsed tails still resident.
   - Fast polynomial-matrix algorithms (approximant and kernel bases, partial linearisation) hold several working matrices of the same total size t·D. The kill point is already at least 5.5× the P floor.
   - PML's other determinant routes keep that t·D floor:
     - `determinant_via_linsolve` stores a solution vector of t polynomials of degree about D.
     - The evaluation routes need t·deg P + 1 dense 4920×4920 determinants, far more work than Wiedemann.
   - None was run, because none would fit with the margin the kill point leaves.
5. **So the BNS22 finish is not measurable on this host.** It needs a larger-memory host. The EKS route that could provide one is unavailable for arbitrary commands in this session (see `research/ecgfp5_quotient_solve_20261101/README.md`, "EKS"). The question stays open, with its precondition now verified and its memory requirement now bounded below.

## 2. What was built

`src/hnfsolve.cpp` (C++17, NTL 11.5.1, PML `ntl-extras` at commit `a26906b8`) reads msolve's `-g 2` reduced Gröbner basis output, as a file or a FIFO, and does six things:

- It enumerates the staircase B by BFS from 1, excluding multiples of the leading monomials.
- It takes the chain heads (monomials free of the last variable y) and their chain lengths d_j.
- It checks the stability assumption: y^{d_j}·head_j must be a leading monomial.
- It builds P with P[j][j] = y^{d_j}, plus each tail term c·y^e·head_k of that basis element at column k.
- It computes det P (LNZ17, falling back to linsolve), makes it monic, finds the rational roots with gcd(y^p − y, h), and recovers each point from the kernel of P(y_0) at the heads 1, x_1, …, x_{n−1}.
- It checks the planted point and writes JSON with per-phase wall and CPU time and max RSS.

msolve opens its output file once early and closes it before reopening it to write the basis. hnfsolve therefore reopens the FIFO until data arrives, and starts the parse clock at the first byte. Without this, the first dev attempt read an immediate EOF, and msolve then blocked waiting for a reader. Its F4 work was kept, and the reader was restarted by hand. That wait is why `msolve.run.json` shows 7,341 s of wall time against 11,103 CPU-s.

Build:

```sh
PML=…/pml/ntl-extras   # https://github.com/vneiger/pml at a26906b8
(cd $PML && make)       # objects into $PML/lib
ar rcs libpml.a $PML/lib/*.o
g++ -O3 -march=native -std=c++17 -I$PML/include src/hnfsolve.cpp libpml.a -lntl -lgmp -lpthread -o hnfsolve
```

## 3. Validation at m = 3

The m = 3 quotient at p = 4111 (`dev/m3.gb`, msolve 0.6.5 `-g 2` output for `research/ecgfp5_quotient_solve_20261101` cell m3 p = 4111 planted) has t = 6 and |G| = 12. On it:

- hnfsolve's eliminant (`dev/m3.elim`, degree 16) **equals msolve's eliminant exactly**.
- The planted point (3153, 626, 1362, 2434) is the unique rational solution and is recovered (*measured*).

## 4. m = 5 dev measurement (p = 331, `dev/m5p331/`)

| phase | wall s | CPU-s | max RSS GiB |
|---|---:|---:|---:|
| msolve 0.6.5 F4 + basis print (`-t 4 -g 2`) | 7,341 (incl. about 2 h blocked on the FIFO) | 11,103 | 4.66 |
| hnfsolve parse (305 M terms streamed) | 269 (paced by msolve's print) | 103 | 5.02 |
| staircase and stability | 1.4 | 1.4 | 5.02 |
| build P (4920 × 4920) | 14.3 | 14.2 | 6.97 |
| determinant (PML LNZ17) | about 56, then killed | not recorded | **13.33 at kill** |

msolve's F4 at p = 331 ends on the same final basis matrix (5578 × 71114) as the three protocol cells (*measured*, `msolve.log`). This is consistent with the p-independence reported in the merged experiment.

## 5. What this does and does not say

- It **does** say that the BNS22 matrix exists for this system (stability holds) and that forming it costs about 0.6% of the finish it would replace.
- It **does** say that PML's generic determinant needs more than 13.3 GiB at t = 4,920 and D = 2^16. That is above the 12 GiB envelope.
- It **does not** measure the HNF finish's CPU time, so it neither confirms nor refutes the BNS22 model prediction of about 3,000 CPU-s (*predicted*, `KN-OPEN-814b4c`).
- It **does not** change the G1 verdict. CLOSE stands on the measured whole-solve c, so it is not an improvement and not a relabelling.
- Even a finish of zero cost leaves F4 alone at c_cyc 2^44.7–2^45.3. That sits in G1's not_measured band, above PROMOTE (≤ 2^38) (*derived*, merged experiment). So no change of order alone can make this lead PROMOTE.

## 6. Next step

Rerun this exact pipeline on a host with at least 64 GiB:
- at the three protocol primes 4111, 262231 and 33554371;
- under a protocol frozen before the run, with the eliminant checked against `research/ecgfp5_quotient_solve_20261101/results/m5_p*_planted/` eliminants and the planted point recovered;
- with c_new = msolve F4 and print CPU plus hnfsolve CPU, and an improvement bar of W0 whole CPU / 1.5.

Lowering the memory floor is the other route: 4-byte coefficients for p < 2^31, or a determinant that streams P by column blocks. Either is an engineering change to PML, not a parameter.

## Sources checked

- Berthomieu, Neiger, Safey El Din, *Faster change of order algorithm for Gröbner bases under shape and stability assumptions*, ISSAC 2022, [arXiv 2202.09226](https://arxiv.org/pdf/2202.09226): the stability assumption, P with det P = charpoly, Table 2 timings.
- Berthomieu, Eder, Safey El Din, *msolve: a library for solving polynomial systems*, [arXiv 2104.03572](https://arxiv.org/pdf/2104.03572), and [msolve on GitHub](https://github.com/algebraic-solving/msolve) (0.9.5 at `02c06d66` read for its finish; 0.6.5 used for the solves).
- Faugère, Mou, *Sparse FGLM algorithms*, [arXiv 1304.1238](https://arxiv.org/pdf/1304.1238): the Wiedemann-based finish msolve uses.
- [PML](https://github.com/vneiger/pml) source at `a26906b8`: `mat_lzz_pX_determinant.cpp` and `mat_lzz_pX_linsolve.cpp`.

## Files

- `src/hnfsolve.cpp`: the driver.
- `plot.py`: renders `figures/hnf_finish.{svg,png}` from `dev/`.
- `build_report.sh`: renders `report.typ` and `report.pdf` from this README.
- `dev/m3.gb` and `dev/m3.elim`: the m = 3 validation.
- `dev/m5p331/`: `msolve.run.json` (qsolve wrapper record), `msolve.log`, `hnf.log` and `kernel_oom.txt` (the kernel's OOM lines).
