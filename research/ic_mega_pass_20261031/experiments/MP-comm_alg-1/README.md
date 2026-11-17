# MP-comm_alg-1 — result (class: theory/measurement, no B*D^2 change, novelty_unverified)

Outcome: SUPPORTS the restricted pre-registered claim at d=3 only. Says nothing about d_F growth (lever 1 stays open).
Known background, not claimed here: bilinear form = falldegree.tex Thm reform; k+2 = Thm upper (FSS 2011); Frobenius collapse = Kosters-Yeo / THM_SEMAEV_FALL1 L1; low_3=3n = treatise Lemma deg3.

Code validity: system equals ffd_semaev_core.build_system (n=7,9,10). Arm A' (random L1,L2,L3) equals A exactly in every cell (29/29).

d=3, k=ceil(n/2), n=17..45, 20 draws (low_3 = dim W_3 ∩ B_{<=2}; sd=0 unless stated):
- A (Semaev mu_V), C1 (mu_W other subspace), C2 (Lr∘mu(W1xW2)): low_3 = 3n exactly, every draw. Excess vs B = 2n, fit slope 2.000 ± 6e-16, intercept 0; extrapolation n=131: e_3 = 262.
- B (dense random), D2 (F_2-rank 2n), D4 (F_2-rank 4n): low_3 = n exactly (no falls at d=3).
- D1 (random F_2-rank n): mean 3n + 2..15 (sd 2..40); slope 1.80 ± 0.27. Differs from B at >6 sigma, from A at 2..6 sigma (above A).
- Off-diagonal k: k=17,24 (n=25,35) same as diagonal (3n). Small k (n=25,k=9; n=35,k=12; n=45,k=15): k=9,12 cells give 1 in W_3 in 95-100% (A,C) vs 0% (B); k=15 at n=45 gives 3n.
- n=9..13: 1 in W_3 in 15-45% for A/C/D1, 0% for B.

d=4, n=9..17, 10 draws: n<=15 all arms indistinguishable (|z|<1.8 on low_4); n=17: A,C1,C2,D1 low_4 ≈ 985-986, B,D2,D4 = 323. 1 in W_4 frequency: A 0.5/0.4/0.2/0.4/0.1 vs B 0.2/0.1/0.1/0.1/0.0.

Reading: the d=3 low-degree falls are fixed by low K-bar-rank (Galois-stable controls match A exactly), but a random F_2-rank-n tensor with no Galois structure produces them too (ℓ^2=ℓ in the Boolean ring). F_2-rank 2n destroys them although mu_V itself has F_2-rank >= 2n-1. So the controlling quantity is "rank n over some extension", not F_2-rank, and not the curve. This is close to a relabelling of the Frobenius collapse.
Caveats: d_F not measured (cap d<=4, n<=17), so nothing about growth. No msolve cross-check, no static d_NS, no semi-regular corank or random_matched null (cut for budget). Total CPU 1070 s.
Next step: compute low_4 / 1 in W_4 for A vs C vs D1 at n=19..29 with a compiled (M4RI-style) eliminator, and check whether the d=4 profile depends only on K-bar-rank. If it does, the d_F question becomes a statement about Boolean bilinear systems of K-bar-rank n.
Files: PROTOCOL.md, tensor_closure.py, run.py, analyze.py, check_vs_core.py, cells.txt, raw/*.json, RESULTS.json.
