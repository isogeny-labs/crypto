#set page(margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 6.5pt)
#set table(stroke: 0.4pt + gray, inset: 3pt)
#set image(width: 100%)
#show figure.where(kind: image): set figure(supplement: none)
= EcGFp5 m \= 5 quotient: block-Wiedemann finish on the BNS22 chain matrix
<ecgfp5-m-5-quotient-block-wiedemann-finish-on-the-bns22-chain-matrix>
#strong[Date.] 2026-11-02. #strong[Question.] The baseline W0 is the merged quotient solve `research/ecgfp5_quotient_solve_20261101`. Two-thirds of its whole-solve cost is msolve’s change of order plus rational parametrisation. Replace that finish with a block-Wiedemann finish on the BNS22 chain matrix of the reduced grevlex basis. Does the whole-solve cost c fall on the same three systems, with every answer verified?

#strong[Protocol:] #link("PROTOCOL.md")[`PROTOCOL.md`], frozen and pushed in commit `3d3f68d70` (merged \#3297) before the first cell started. There are no amendments.

#strong[Status.] All three frozen cells completed on one 4-vCPU host (tier L). It is a #strong[measured improvement over the per-system index-calculus baseline (W0) at all three primes], so it is a pipeline improvement under §5 and becomes the new per-system baseline. It is an #emph[engineering] change: the same F4, a cheaper finish, the same output. It is not a relabelling: every phase is charged, the eliminant is identical, and the planted point is recovered. #strong[The G1 verdict does not change.] The new c\_cyc lies below the CLOSE line at every prime, but the frozen \[c/4, 4c\] band straddles it. This is a per-system stage cost at toy primes. It is not an end-to-end ECDLP cost, and it makes no security claim about EcGFp5.

Every number below is labelled #emph[measured] (from a committed run record), #emph[derived] (a calculation from those records, done in #link("analyze.py")[`analyze.py`]) or #emph[predicted] (stated in the protocol before the runs).

== 1. Answer
<answer>
#align(center)[#table(
  columns: 8,
  align: (col, row) => (auto,right,right,right,right,right,auto,auto,).at(col),
  inset: 6pt,
  [p], [c\_new CPU-s], [W0 CPU-s], [speed-up], [bar (W0/1.5)], [log2 c\_cyc new / W0], [G1 band], [gates],
  [4111],
  [16,912],
  [31,004],
  [#strong[1.83×]],
  [20,670],
  [45.43 / 46.30],
  [\[43.43, 47.43\], straddling],
  [all pass],
  [262231],
  [18,823],
  [37,036],
  [#strong[1.97×]],
  [24,690],
  [45.58 / 46.56],
  [\[43.58, 47.58\], straddling],
  [all pass],
  [33554371],
  [22,516],
  [38,778],
  [#strong[1.72×]],
  [25,852],
  [45.84 / 46.63],
  [\[43.84, 47.84\], straddling],
  [all pass],
)
]

c\_new is #emph[measured]; speed-up, bar and log2 c\_cyc are #emph[derived] (c\_cyc \= CPU-s × 2.8·10^9).

+ #strong[Correct at every prime] (#emph[measured]).
  - The block-Wiedemann eliminant equals W0’s msolve eliminant coefficient for coefficient (`cmp` of `bw.elim` and `w0.elim`).
  - Its degree is D \= 65,536.
  - The BNS22 stability check passes: 4,920 chains, longest 37, `stab_fail=0`.
  - The planted point is among the recovered rational points: 4, 2 and 2 of them.
+ #strong[Improvement at all three primes.] c\_new is 1.72–1.97× below W0, and under the frozen 1.5× bar at every prime. Under §5 this is a pipeline improvement and the new per-system baseline.
+ #strong[The finish alone is 3.1–4.1× cheaper] (#emph[derived]).
  - Here the finish is the extra msolve CPU for printing the basis, plus hnfsolve, plus bwfinish.
  - Against W0’s change of order plus parametrisation it is 3.15×, 4.12× and 3.26×.
  - The F4 work is unchanged: it is the same msolve binary on the same input.
+ #strong[G1 verdict unchanged.]
  - The point estimates move below CLOSE (2^46) at every prime: 0.57, 0.42 and 0.16 bits below.
  - The frozen \[c/4, 4c\] band still crosses the line, so under §5 each cell is reported as #emph[straddling], not as a verdict change.
  - #strong[Distance remaining to PROMOTE (2^38): 7.43, 7.58 and 7.84 bits] (#emph[derived]). The lead’s atlas `revisit_when` (c\_cyc ≤ 2^38) is not met.

== 2. Where the time goes (#emph[measured], CPU-s)
<where-the-time-goes-measured-cpu-s>
#align(center)[#table(
  columns: 11,
  align: (col, row) => (auto,right,right,right,right,right,right,right,right,right,right,).at(col),
  inset: 6pt,
  [p], [msolve (F4 + basis print)], [W0 F4], [A/A], [hnfsolve], [bw sequence], [generator], [det], [points], [finish total], [finish vs pred],
  [4111],
  [12,655],
  [10,349],
  [+22.3 %],
  [104],
  [3,625],
  [170],
  [73],
  [266],
  [4,153],
  [+6.5 %],
  [262231],
  [14,334],
  [12,981],
  [+10.4 %],
  [134],
  [3,906],
  [194],
  [83],
  [148],
  [4,355],
  [+11.7 %],
  [33554371],
  [15,270],
  [15,325],
  [−0.4 %],
  [126],
  [6,566],
  [198],
  [122],
  [168],
  [7,119],
  [−3.8 %],
)
]

- #strong[The sequence dominates the finish (87–92 %).] It is L \= 1,028 dense GEMMs (65,536 × 4,920)(4,920 × 128) in fp64. At p \= 33554371 the operands need two 13-bit limbs, which costs 1.8× the sequence CPU of the one-limb primes, as predicted.
- #strong[Prediction check.] Every finish is within ±30 % of its prediction (−3.8 % to +11.7 %), so there is no prediction failure. Every c\_new is within 10 % of its prediction (15,500, 18,100 and 23,900 predicted).
- #strong[A/A control] (msolve CPU against W0’s F4 CPU, which differs only by the basis print and host drift).
  - It ranges from −0.4 % to +22.3 %, all inside the frozen ±25 %, so there is no host-drift flag.
  - The spread is wide: printing 306M basis terms should cost about the same at every prime, yet the measured excess ranges from about 0 to 2,300 CPU-s.
  - Read c\_new to about ±2,000 CPU-s. Every cell still clears its bar by more than that (by 3,758, 5,867 and 3,336 CPU-s).
- #strong[Memory] (#emph[measured] maxrss).
  - msolve: 4.7–5.2 GiB.
  - bwfinish: 4.85 GiB (one limb) and 9.6 GiB (two limbs).
  - Both are under the 12 GiB envelope. The PML LNZ17 Hermite-form route ran out of memory at 13.3 GiB (`research/ecgfp5_quotient_hnf_finish_20261102`), which is why this finish exists.
- #strong[Wall] (context only): msolve 4,109, 4,515 and 4,809 s. Each cell took 1.5–1.9 h end to end.

== 3. Figure
<figure>
#figure([#image("figures/bw_finish.png")],
  caption: [
    Whole-solve CPU-s and G1 position
  ]
)

- #strong[(a)] Whole-solve CPU-s for W0 and the new pipeline, stacked by phase, with each prime’s improvement bar.
- #strong[(b)] log2 c\_cyc for W0 and the new pipeline, with the frozen \[c/4, 4c\] band against the CLOSE and PROMOTE lines.

Rendered by #link("analyze.py")[`analyze.py`] from the run records only.

== 4. Method (as frozen)
<method-as-frozen>
+ msolve 0.6.5 (`-v 2 -t 4 -g 2`) writes the reduced grevlex basis into a FIFO. msolve is the same binary as W0’s (sha256 in `results/*/inputs.sha256`).
+ `src/hnfsolve.cpp --dump-P` parses the basis and checks the BNS22 stability assumption. It then writes the tail Q of P \= diag(w^{d\_k}) + Q (t \= 4,920 chains, Σ d\_k \= D).
+ `src/bwfinish.cpp`:
  - computes the block-Wiedemann sequence S\_i \= U M\_w^i V with b \= 128 and seed 1, where M\_w \= shift − Qᵀ·E\_top, so each step is one exact fp64 GEMM through OpenBLAS;
  - computes the minimal generator by PML `pmbasis`, and its determinant;
  - takes the eliminant as the reversal of that determinant;
  - finds the roots by gcd(w^p − w, ·), and each point from ker P(w\_0).
+ `run_cell.sh` chains the three steps per prime. `cards/CARD-1.md` is the run card. The three cells ran back to back from 06:17:55Z to 11:20:02Z, with nothing else running on the host.

== 5. Host, isolation, runs
<host-isolation-runs>
- #strong[Host.] Tier L: this session’s container, 4 vCPU Intel Xeon \@ 2.80 GHz, 15.7 GiB. It is the same host class as W0. There is no core pinning. CPU-seconds are getrusage totals.
- #strong[Runs.] Three cells requested, three completed, no failures, no cap stop.
- #strong[Not in Git.] `P.bin` (1.3 GB per prime) is deleted after each cell. Its sha256 is in `results/*/outputs.sha256`, and it is a deterministic function of `quot.ms`. `w0.elim` is regenerated from W0’s `quot.out` (artifact store) by `src/elim_from_msolve.py`.
- #strong[Dev runs] in `dev/m5p4111/` (b \= 32 and b \= 128 on the p \= 4111 dump) set the predictions. They are not data.

== 6. What this does not say
<what-this-does-not-say>
- #strong[Not "EcGFp5 is weaker than claimed".] c is a per-system Gröbner stage cost at toy primes (12–25 bits), not at EcGFp5’s 64-bit p, and not an end-to-end ECDLP cost.
- #strong[Not "the lead is reopened".] The atlas asks for c\_cyc ≤ 2^38. The new point estimates are 7.4–7.8 bits above it, and the G1 band straddles CLOSE.
- #strong[No leaderboard or `benchmarks/` row.] This stage cost has no matched rho reference (the protocol’s scope, §1) and no replay certificate or validator report. Under the validation gate it stays outside the admitted tables. `benchmarks/20261102-ecgfp5-bw-finish/` keeps documentary copies only, as W0’s does.
- #strong[No solver lower bound.] F4 is now about two-thirds of c, so the next factor has to come from F4 itself (§7).

== 7. Next
<next>
- #strong[The F4 phase is now the bottleneck] (68–76 % of c). The probes of the merged W0 study (weighted F4, Singular `sba`) gave no improvement. The open levers are:
  - #strong[L3]: the u-coordinate quotient ring A(s) + πB(s), and the variable-order arms (`research/ecgfp5_representation_audit_20261102`);
  - a GVW/F5-style signature F4.
- #strong[Cheaper sequence.] b \= 128 is the measured GEMM sweet spot on this host. A modular-BLAS kernel (FFLAS-FFPACK) might cut the two-limb penalty at 25-bit p.
- #strong[Tier-X cell at 64-bit p] once an EKS runner exists. The finish uses multi-limb GEMM there (five 13-bit limbs).

== Files
<files>
- `PROTOCOL.md`: the frozen protocol.
- `src/`: `hnfsolve.cpp`, `bwfinish.cpp` and `elim_from_msolve.py`, built by `build.sh`. `run_cell.sh` runs one cell.
- `cards/CARD-1.md`: the run card.
- `results/m5_p*/`: the run records (exit codes, msolve run JSON and log, dump log, `bw.json` and `bw.log`, both eliminants, sha256 files, timestamps).
- `dev/`: development runs, not data.
- `analyze.py` produces `analysis.json`, `analysis.md` and `figures/bw_finish.{svg,png}`.
- `build_report.sh` builds `report.pdf`.
