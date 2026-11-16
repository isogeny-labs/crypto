# Pre-registration: block-Wiedemann finish for the EcGFp5 m = 5 quotient (whole-solve c)

Frozen 2026-11-02, before any cell in `results/`. It is committed and pushed before the first
cell starts. Changes after that point are amendments (§9) and never edit this text.

## 1. Question and baseline

- **Baseline W0** (`research/ecgfp5_quotient_solve_20261101`, merged #3267). msolve 0.6.5 `-P 1 -t 4` gives a complete solve of the m = 5 (Z/2)^4 ⋊ S5 quotient: F4, then a Wiedemann sparse-FGLM change of order, then the rational parametrisation. Whole-solve CPU-s, measured:

  | p | W0 whole-solve CPU-s | W0 F4 CPU-s | c_cyc |
  |---|---:|---:|---:|
  | 4111 | 31,004 | 10,349 | 2^46.30 |
  | 262231 | 37,036 | 12,981 | 2^46.56 |
  | 33554371 | 38,778 | 15,325 | 2^46.63 |

  G1 verdict: CLOSE (c_cyc ≥ 2^46) at every prime.
- **Question.** Replace W0's change of order by a block-Wiedemann finish on the BNS22 chain matrix (§3). Does the whole-solve cost c fall, on the same systems, with every answer verified?
- **Scope.** This is a per-system stage cost at toy primes. It is not an end-to-end ECDLP cost and makes no security claim about EcGFp5. No matched rho reference exists for c. The G1 model of `research/ecgfp5_lead_evaluation_20261031` is the context.

## 2. What is fixed

- **Instances.** The three merged W0 m = 5 planted systems, `results/m5_p{4111,262231,33554371}_planted/quot.ms`, read in place. No regeneration.
- **Tools.**
  - `/usr/bin/msolve`, sha256 `c2722288…f759fa`: the same binary and version as W0.
  - `src/hnfsolve.cpp` and `src/bwfinish.cpp`, built by `build.sh`. The source hashes are recorded in the commit freezing this file. PML is `vneiger/pml` at `a26906b8`; NTL is 11.5.1; OpenBLAS is 0.3.26.
- **Host.** The same class of host as W0's tier L: 4 vCPU Xeon at 2.80 GHz, 15.7 GiB. One cell at a time, with nothing else running on the host.
- **Parameters.** b = 128, seed 1, 4 threads. Envelope: 12 GiB and 4 h for msolve (the qsolve caps, as in W0); 12 GiB for bwfinish.
- **Unit.** c_cyc = CPU-s × 2.8·10^9, as in W0.

## 3. Method (one cell)

`run_cell.sh <p>`:
1. msolve `-v 2 -t 4 -g 2` writes the reduced grevlex basis into a FIFO.
2. `hnfsolve --dump-P` reads it, checks the BNS22 stability assumption (exit 3 if it fails), and writes the tail matrix Q of P = diag(w^{d_k}) + Q, with t = #chains and sum d_k = D.
3. `bwfinish`:
   - computes the block-Wiedemann sequence S_i = U M_w^i V (b × b, i < 2⌈D/b⌉ + 4), where M_w = shift − Qᵀ·E_top. Each step is one exact fp64 GEMM (D × t)(t × b), with operands split into 13-bit limbs when (p/2)²·max(t, D) ≥ 4·10^15, which happens at p = 33554371;
   - finds the minimal matrix generator (PML `pmbasis`) and its determinant (PML LNZ17);
   - takes the eliminant as the reversal, which must have degree D;
   - finds the rational roots with gcd(w^p − w, ·), and each point from ker P(w_0).

## 4. Measurements and correctness gates (per cell)

- **c_new** = msolve CPU (`msolve.run.json` user + sys; F4 plus basis print) + hnfsolve CPU (`dump.log`, parse + staircase + dump) + bwfinish `total_cpu_s`. The I/O through the FIFO is inside these CPU totals. No phase is left out.
- **Correctness**, all three required, otherwise the cell is `invalid` and c_new is not reported as a result:
  - (a) the eliminant equals W0's msolve eliminant exactly, coefficient for coefficient (`eliminant_equals_w0=true`);
  - (b) the planted point is recovered (`"planted": "recovered"`);
  - (c) D = 65,536 and the stability check passes.
- **A/A control.** The msolve F4 work is the same algorithm on the same input as W0. Report this cell's msolve CPU against W0's F4 CPU. The difference is the basis print plus host drift. In dev it was +1,137 CPU-s at p = 4111. A difference beyond ±25 % of W0 F4 is flagged as host drift. It does not void the cell, but it is reported next to c_new.
- **Positive control.** Correctness gates (a) and (b).

## 5. Decision rules (frozen)

- **Improvement** at a prime: c_new ≤ W0 whole-solve CPU / 1.5, which is 20,669 / 24,691 / 25,852 CPU-s, with every gate passed. This is the same bar the W0 solver probes used.
- **Pipeline improvement:** improvement at all three primes. It becomes the new per-system baseline (ratchet).
- **G1 verdict on c_new**, per prime, with W0's thresholds: CLOSE if c_cyc ≥ 2^46; PROMOTE if c_cyc ≤ 2^38; otherwise not_measured. The verdict changes only where the c_cyc band [c/4, 4c] of W0's protocol stays on one side of a threshold. Otherwise the cell is reported as straddling.
- A cap stop is reported as `not_measured`, never imputed.

## 6. Pre-registered predictions

These come from the dev run on the p = 4111 dump (`dev/m5p4111/`, not data).

| p | limbs | msolve CPU-s | finish CPU-s | c_new CPU-s | log2 c_cyc | vs W0 |
|---|---:|---:|---:|---:|---:|---:|
| 4111 | 1 | 11,500 | 3,900 | 15,500 | 45.3 | 2.0× |
| 262231 | 1 | 14,100 | 3,900 | 18,100 | 45.5 | 2.0× |
| 33554371 | 2 | 16,400 | 7,400 | 23,900 | 45.9 | 1.6× |

- The predicted G1 verdict is not_measured at every prime, but at p = 33554371 c_cyc is within 0.1 bit of the CLOSE line.
- A miss of more than ±30 % on the finish is reported as a prediction failure.

## 7. Analysis

`analyze.py` reads only `results/m5_p*/` and W0's merged `analysis.json`. It writes `analysis.json` and `analysis.md`, and renders the figure. There is no other analysis path.

## 8. What may not be said

- Not "EcGFp5 is weaker than claimed". c is a per-system stage cost at toy primes.
- Not "the lead is reopened". The atlas `revisit_when` asks for c_cyc ≤ 2^38. A not_measured verdict is reported as a measured improvement over the index-calculus baseline and the distance remaining to that bar.
- Dev numbers are never reported as results.

## 9. Amendments

None yet.
