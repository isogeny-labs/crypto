# MP-symmetry-1 protocol (pre-registered 2026-10-31, before any run)
Class: engineering (constant factor). No novelty claim (Faugere-Svartz 2013, Gatermann-Parrilo 2004, A2-11).
## Arms
- A0: Z/3 block permutation of unchained binary S_4 (x1,x2,x3 in one random l-dim V, x4 = target), K_0-like model y^2+xy=x^3+1, l=4, n in {11,13,17}, 8 UNSAT targets/cell (ground truth: exhaustive V^3 evaluation), seeds 1..8 (target/V seed = 1000*n+k).
  Generators: degree-filtered echelon basis of span(descended components) (same basis for both arms of comparison).
  Assert per instance: span G-stable, W_c G-stable at c*_full (rank W_c == rank W_c+gW_c).
  Full: Macaulay rows m*f, deg m <= c-deg f, cols squarefree monomials deg<=c. Triv: rows R(m*f) with m over monomial ORBIT REPRESENTATIVES and f over basis (valid since R(gm f)=R(m g^-1 f), span stable), cols = orbit sums. c = 4..12.
  Metric: c*, rank, rows, cols, counted word-XOR ops in elimination (xors * ceil(cols/64)), incl. orbit build time noted separately.
- A-null: symmetry broken: x1,x2,x3 in three independent random l-dim subspaces (UNSAT targets wrt V1xV2xV3), same orbit-sum computation. n=11,13, 8 targets.
- A1: l=5, n in {11,13}, trivial block only to c<=9; full only n=11, c<=8 (censored: reported as lower bound).
- B (C_n barrel): NOT RUN (barrel not defined in brief; budget) -> reported not_tested.
## Predictions
- true: c*_triv == c*_full on 100% of A0; rank_triv/rank_full in [0.30,0.40]; word-op ratio full/triv >= ~9 (|G|^2) approx.
- false: any c*_triv != c*_full in A0; or orbit matrix >= 0.6x full; or A-null agreement 100% with identical ranks (test has no power -> inconclusive on power).
Budget < 20 CPU-min.
