# MP-symmetry-1: result (engineering, constant factor)
Outcome: SUPPORTS (the narrow pre-registered A0 claim). The null arm has NO POWER.
- A0 (Z/3 block permutation, unchained binary S_4, l=4, K_0-like y^2+xy=x^3+1): 24/24 UNSAT instances (8 each at n=11,13,17) give c*_full = c*_triv = 9.
  The span is G-stable on all 24, and W_c at c* gained 0 new dims from sampled g-images.
  At c*: rank_triv/rank_full = 0.343 (n=11), 0.325 (n=13), 0.335 (n=17), inside [0.30,0.40]. Rows and cols both shrink to about 1/3 (orbit-representative rows R(m f) over a degree-filtered basis), so memory is about 1/9.
  Counted word-XOR ratio full/triv = 22.8-24.7, above the |G|^2=9 prediction (the extra comes from the incremental-echelon cost).
- A-null (x1,x2,x3 in 3 independent subspaces, so the span is not G-stable): 4/4 at n=11 give c*_full = c*_triv = 10. The orbit-sum map gave no false early certificate. The test therefore does not discriminate: agreement under broken symmetry means 100% agreement in A0 is not evidence for the theorem beyond implementation correctness. The theorem itself (|G| odd, G-stable W_c) is a textbook corollary (Faugere-Svartz 2013, Gatermann-Parrilo 2004, A2-11), with no novelty.
- Side observation: under symmetry breaking (independent V_i), c* rises from 9 to 10 at n=11, l=4.
- NOT RUN: A1 (l=5) and Arm B (C_n barrel, not defined in the brief), because of the CPU budget (about 18 CPU-min used). The barrel net saving is unmeasured. Arm Z (per-block cokernel, 3 sigma) was not run.
- Caveats: c* is constant at 9 for n=11..17 at l=4, so it says nothing about growth of d_F. The word-op count is for this python echelon, not M4RI/F4. B*D^2 is unchanged.
- Next step: run A1, l=5, trivial block only, c<=10, at n=11,13 (about 1/9 memory makes it feasible), and l=6 for the (4,5),(4,6) cells. Define the barrel explicitly and measure its net saving including the selectors.
Files: PROTOCOL.md, run.py, raw_*.json, log_*.txt, RESULTS.json.
