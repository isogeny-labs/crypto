# VERIFY: syzygy-structure (independent skeptic), 2026-11-12

Every result statement here is `[pcc:unverified]`. I only wrote this file. I changed no source, created no pod and ran nothing on RunPod.

## Verdict

**I agree, with caveats.** I re-derived the measured numbers with my own code and got the same values. That covers dim I_d, dim K_d, dim R_d, R_{e_min} = RM(e_min+1), feasibility, the excess, the solution counts and |Z| in all 6 cells and all 43 a6 instances. I also checked the n = 15 s1 symmetry and translation numbers. I found no number that differs. The caveats below are about what the numbers mean and what I could not check. They do not refute anything.

## What I did

Instance parameters come only from the shared `lfd_small.make_instance`: poly, a2, a6, z, V. The a6 values for the scan were read from the raw `a6scan.jsonl`. Everything else is my own code in the scratchpad. I wrote it from my own derivation:

`S(A, A+C) = A^4 + (C^2+z)A^2 + zCA + z^2C^2 + a6 = Phi_C(A) + h_C`

- My code uses its own field multiply and trace.
- It uses a different column layout from the producer's: `t*n + j` instead of `j*nmons + t`.
- It eliminates on the lowest-bit pivot, where the producer pivots on the highest bit.
- dim I_e = ncols - rank(rows of (I)).
- dim K_e = ncols - rank((I) + h rows).
- dim R_e = rank_full - rank_I.
- `R_e ⊆ RM(e+1)` holds because h_C is F2-affine in the coordinates of C. So R_e = RM(e+1) exactly when dim R_e = M(k, e+1).
- Feasibility (1 ∈ R_e) is tested as an inhomogeneous consistency check.
- Solutions and Z are counted by brute force over V x V.

Scripts, in `/tmp/claude-0/-home-user/5d26ea47-4c79-5b59-bd9c-e47e6411a571/scratchpad/v/`:

| script | sha256 | what it does |
|:--|:--|:--|
| `indep.py N SEED E [A6]` | `6d6c2216…6344` | dims, feasibility, solutions and Z |
| `sym.py N SEED E` | `594102fb…1221` | partial H_b check: Frobenius, multiplication by x, translations |

Commands:

```
python3 indep.py {15 1 2|15 1 3|15 6 2|15 6 3|17 1 2|17 1 3|17 8 2|17 8 3|19 4 3|19 4 4|19 7 3|19 7 4}
python3 indep.py N SEED E A6  for all 43 rows of a6scan.jsonl (E = 3 at n = 15, 17; E = 4 at n = 19)
python3 sym.py 15 1 3
```

Each job was single-threaded and took 2 to 98 s, using under 1 GB.

## Re-derived cell numbers (all match cells.jsonl / RESULT.md table / ../grid.jsonl)

| n | seed | e | dim I | dim K | dim R | M(k,e+1) | R = RM(e+1) | feasible | excess = dim K - M(k-1,e-1) | s |
|--:|--:|--:|--:|--:|--:|--:|:--|:--|--:|--:|
| 15 | 1 | 2 | 37 | 8 | 29 | 93 | no | no | 0 | 2.0 |
| 15 | 1 | 3 | 309 | 146 | 163 | 163 | yes | yes | 117 | 2.2 |
| 15 | 6 | 2 | 38 | 8 | 30 | 93 | no | no | 0 | 1.9 |
| 15 | 6 | 3 | 310 | 147 | 163 | 163 | yes | yes | 118 | 2.3 |
| 17 | 1 | 2 | 46 | 9 | 37 | 130 | no | no | 0 | 8.0 |
| 17 | 1 | 3 | 343 | 87 | 256 | 256 | yes | yes | 50 | 8.8 |
| 17 | 8 | 2 | 46 | 9 | 37 | 130 | no | no | 0 | 7.7 |
| 17 | 8 | 3 | 344 | 88 | 256 | 256 | yes | yes | 51 | 9.3 |
| 19 | 4 | 3 | 374 | 46 | 328 | 386 | no | no | 0 | 35.1 |
| 19 | 4 | 4 | 1802 | 1164 | 638 | 638 | yes | yes | 1034 | 45.9 |
| 19 | 7 | 3 | 375 | 46 | 329 | 386 | no | no | 0 | 33.7 |
| 19 | 7 | 4 | 1803 | 1165 | 638 | 638 | yes | yes | 1035 | 46.8 |

H_c1 refuted and H_c2 held (6/6): confirmed. At e_min - 1, I_{e_min-1} is larger than M(k, e_min - 1) in these cells:

| cell | dim I_{e_min-1} | M(k, e_min-1) |
|:--|--:|--:|
| n = 15 s6 | 38 | 37 |
| n = 19 s4 | 374 | 176 |
| n = 19 s7 | 375 | 176 |

## a6 scan (43 instances, re-run independently)

All 43 match the producer's rows in dim I_e, dim R_e, feasibility, the solution-pair count and |Z|. The split is 16 unsatisfiable and 27 satisfiable.

- In every unsatisfiable instance, R_e = RM(e+1) and the instance is feasible at e_min(V,z). This is P_B1/P_B2 at the degrees scanned.
- In every satisfiable instance, the instance is infeasible and codim R_e = |Z|.
- R_e ⊆ I(Z) ∩ RM(e+1) is proved. The evaluation of RM(e+1) on at most 4 distinct points has rank |Z| for e+1 >= 4. Together these give P_C1 holding dimensionally in 43/43.
- I also checked that a6scan_v1.jsonl (36 rows) agrees with a6scan.jsonl field by field, with 0 differences. The producer's claim that the second pass reproduced the first holds.

## H_b partial check (n = 15 s1, e = 3)

`sym.py` built a K basis of dim 146 and verified it pointwise on all points.

| map | dim(K ∩ g^-1 K) |
|:--|--:|
| Frobenius | 0 |
| multiplication by x | 0 |
| translations by v_1..v_8 | [0, 29, 29, 0, 29, 0, 29, 0] |

Tr(v_i) = [1, 0, 0, 1, 0, 1, 0, 1]. These match the producer's `translation_stable_dims` and the rule "keeps dim T_e when Tr(v) = 0, else 0". I did not recompute the full stabiliser algebra (S(K) = F2·Id, S(T) dims 211/273). I also did not recompute H_a (dim K ∩ G_e, including the 950/84 split at n = 19). Both of those rest on the producer's code only.

## Caveats / issues (none is a refutation)

1. **The headline "excess formula" is a tautology given H_c2.** x = dim I - M(k,e+1) - M(k-1,e-1) is rank-nullity once R = RM(e+1). The producer says this. The only empirical content is H_c2 (R onto at e_min) and P_C1. Do not cite the formula as an independent finding.
2. **The preregistration order cannot be verified.** The predictions and Addenda A/B/C are all in one RESULT.md, last modified at 20:43:55, after every data file was written (cells 20:22, a6scan 20:34). The directory is untracked in git, so no commit shows the timing. The claimed order is self-attested.
3. **The P_B2 / P_C2 samples are small.** There are 2 to 4 unsatisfiable a6 values per (V,z), and only 2 at n = 19. e_min was only scanned up to d = 4. "e_min is a function of (V,z) alone" is supported at n = 15, 17, 19 and nothing more.
4. **Several conclusions rest on the producer's code alone:** H_a (new top generators 117/117/50/51/84/84, and 950/84), the full stabiliser algebra, and H_a'. My Frobenius and translation spot check agrees with the H_b conclusion at one cell.
5. **Scope gaps** are as the producer stated: no n >= 21, no T-type or planted controls, and no research-kb or ecdb queries. The producer makes no novelty claim.

## Source changes / default behaviour

- `../syz.c` sha256 is `0cccd25f…c0c2`, matching the value in RESULT.md. It is tracked and unchanged against HEAD 2003c4bc (empty `git diff`).
- `syz` rebuilt binary sha256 is `939f3050…849a`, also matching.
- No tracked source file was modified. The only new files are in this directory, which is untracked. The `a6=` override in syzstruct.py is opt-in and confined to the new file. The cells.jsonl a6 values equal the instance a6: my indep.py reproduces the cells using the default make_instance a6, for example 18652 at n = 15 s1.

## RunPod

`GET https://rest.runpod.io/v1/pods` at verification time shows no pod named `bdr-syzygy*` or any other pod for this task. A `bdr-emin-exact-large-n` pod is RUNNING; it belongs to another task and I did not touch it. I created no pods.
