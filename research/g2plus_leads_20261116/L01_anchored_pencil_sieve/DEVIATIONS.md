# Deviations from PROTOCOL.md / SPEC.md (executor)

`PROTOCOL.md`, `SPEC.md` and `frozen.sha256` were not edited (`sha256sum -c frozen.sha256` is OK after the runs).
No threshold, arm, prime, seed rule, count rule, output field or decision rule was changed.
No repository hook refused any command; `raw/REFUSALS.txt` was not created.
No instance size was reduced; no run hit the CPU-time guard (every record has `status: "ok"`).

1. **P_2lp: repeated residual root at a bucket abscissa is rejected, not a gate failure.**
   SPEC 7.8 step 4 says any residual root in S raises `GateFailure("G5 S-root")`. A fibre of the degree-(g+1) map
   with a double root at a bucket point has that abscissa both in the bucket and again as a root of the quotient
   (PROTOCOL section 1 (iii): "a fibre with a repeated root ... is not collected"). Taken literally the gate would fire on
   such a fibre. Implemented: if a residual root equals a bucket abscissa the candidate is rejected and counted in
   `repeat_rejects` (new field of `L01.lp2/v1`); a residual root in S that is NOT a bucket abscissa still raises
   `GateFailure("G5 S-root")` (never fired). Observed `repeat_rejects` over the 12 P_2lp runs: total 15
   (g=2 p=4099: 1,0,1; g=2 p=16411: 0,1,2; g=3 p=4099: 1,5,2; g=3 p=16411: 0,1,1).
2. **V2 on B relations above p = 4099.** PROTOCOL G2 says "every B relation passes V2"; SPEC 7.7 applies V2 to every
   relation only at p <= 4099 and to about 200 sampled relations above. Implemented SPEC 7.7 for V2 (counts in
   `v2_checked`). In addition every B relation at every p passes an uncounted polynomial check `v1_walk`
   (u == prod (x - x_k), v(x_k) == y_k, y_k^2 == f(x_k)); counted in `v1_checked`. P relations: V1 on every relation, V2 per 7.7.
3. **V2 sampling in the 2LP arms.** SPEC 7.7 uses `N_target`; the 2LP arms have no `N_target`, so `target_cycles`
   (nb + 8 + nb//100) is used in the sampling threshold `1000*200/target`; all relations are checked at p <= 4099.
4. **G = [h] D0 in the gate.** SPEC 9.2 says "first random_divisor D0 with mul_ltr(D0,h) != identity; check mul_ltr(G,r)==identity";
   G is taken as `mul_ltr(D0, h)` (the only reading under which `G` has order r).
5. **`seeds_tried`** in the gate record is the selected curve seed itself (seeds 1..cs examined, including non-squarefree ones).
6. **`setup_mul`/`setup_inv` of the relphase records.** P arms: character table, square-root table and base enumeration
   (`table_setup_mul` has the same number for every arm). B_lab: its own 16 jumps and two restart bases (this is what SPEC 8.1 lists
   as B setup); the table setup is reported in `table_setup_mul` for B_lab too. Neither is in any FPM.
7. **`walk_add`** counts every walk step (one generic Cantor call), including the rare step where `cur == M[j]` (tallied as a
   doubling by `ops` but still one walk step); `strong_fpm` prices every walk step at 35.
8. **Row file for P_naive1** is not written (SPEC 8.2 lists rows files for P_full and B_lab only); its row statistics and digest are computed.
9. **Inversion counting inside native routines** is `inv += 1` as in SPEC 6 (not Fermat multiplications); Cantor `k_inv` also `inv += 1`, as SPEC 5.
10. **Point counting speed-up (uncounted, gate only).** `N_3` is computed by summing over Frobenius orbits of (b, c) pairs
    (weight 3 for non-fixed orbits) instead of all p^3 elements; verified against the brute-force hand values of unit test 3
    (the hand values (7,3,1) and (7,3,2) use it) and consistent with the order check `[#J] D = 0` on 16 divisors in every gate.
