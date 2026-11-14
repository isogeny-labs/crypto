# L1 — Frobenius- and negation-closed non-linear factor base: CLOSED-BY-PRIOR

Lane report from a prior-art audit, saved by the main session because the subagent could not
write files. Nothing was measured, so this lane has no PROTOCOL.md and no data. All statements
`[pcc:unverified]`. research-kb and the lead atlas were unavailable, so this lane makes no
novelty claim. The main session re-read two of the cited figures from source and both match:
`ecbench_frobenius_orbit_vs_rho_20261014/README.md` lines 7, 79 and 115, and
`ic_leads_20261010/RESULTS_ROUND2.md` lines 76–77.

## The owner's claim, part by part

1. **"Weil descent needs a subspace base, and at n=131 no stable subspace of useful size [pcc:unverified]
   exists."** Correct. The repos already say it:
   - `cryptanalysis:ecc2k130/runner/research/candidates/goal22/codegen/decomp.py`, header.
   - `cryptanalysis:experiments/frobenius-quotient-m4/README.md`.
   - `crypto:research/koblitz_symmetrised_e2e_20260927/README.md`, which finds no proper
     stable subspace at n = 13, 19, 29, 37.
2. **"Only Gröbner-free methods can use a Frobenius-closed base."** Partly wrong. Algebraic
   and SAT oracles can use a closed base by making the Frobenius rotation of each point an
   unknown in the system, one system per rotation:
   - `crypto:research/ic_leads_20261010/RESULTS_ROUND2.md` L5 (algebraic form): 1.79× hits
     per system at n=23.
   - `crypto:research/pdp_split_slots_20261012/README.md`: the closed base costs 0.35–0.54×
     the standard base in ecbench.
   - `crypto:research/frobenius_quotient_sat_20260930/`: the same idea in SAT, with one-hot
     rotation variables.
   - msolve solved the Hamming-weight base directly at n=11, degree 4:
     `crypto:research/ic_mega_pass_20261031/experiments/L6-frobenius-stable-nonsubspace-bases/`.
3. **"Nobody tried it."** Wrong.
   - The base closed under Frobenius and negation, built as a union of point orbits
     (`compact-orbit-scan`), is in ecbench. It was measured end to end, with every cost
     priced, against rho on 8 Koblitz curves from n=13 to n=41, including n = 13, 19, 29 and
     37, where 2 is primitive mod n: `crypto:research/ecbench_frobenius_orbit_vs_rho_20261014/`.
   - The Hamming-weight base is built in goal22 `indexcalc_e2e.py`, and in ecbench as
     `onb-weight:w=W` (`crypto:src/cryptanalysis/ic_framework/plugins.rs`).
   - ECC2K-130 weight-4 census, 44,824 columns of signed Frobenius orbits:
     `cryptanalysis:experiments/ecc2k130-normal-weight4-base-20261009/`.
4. **The negation half of the 262×.** Already taken: `subgroupBase` in `indexcalc_e2e.py`
   makes one column per signed Frobenius orbit, and `compact-orbit-scan` counts signed orbits.

## The main session's contrarian hypothesis, factor by factor

- **(a) Decomposition probability at matched size: shape makes no difference.**
  - Yield per point sat at the random-set null (ratio 0.94–1.13) for cyclotomic, Hamming and
    random-subspace bases (ic_mega_pass L6, with its audit).
  - In table (MITM) form at equal points and columns, n=23: 1.53 trials per relation for the
    orbit union against 2.61 for the invariant subspace.
  - Trap: in a type-II ONB, Tr(x) = HW(x) mod 2. For a=0 curves, odd-weight abscissae are
    never in the prime subgroup, so about half of a "weight ≤ w" base is dead. goal22 already
    filters for this.
- **(b) Relations needed and linear algebra shrink by the orbit factor 2n, not per trial.**
  - At equal columns: 30.5 relations needed against 27.5.
  - `frobenius-quotient-m4`: rho already collects the same constant, as √(2·131).
- **(c) Per-trial cost depends on the solver.**
  - Table/MITM: identical, 47,565 table additions in both arms. Building the orbit base costs
    2,870 GAE against 182, paid once.
  - SAT/Gröbner: the non-linear base costs more per trial. The frozen paired n=13 SAT run
    (`frobenius_quotient_sat_20260930/paired_fixed_budget_20261011`) needed 1.38–1.61× more
    CPU per new independent relation. That run used the legacy Python harness and is
    provisional.
- **(d) End to end, fully priced.**
  - Orbit base against the negation-only subspace base: 0.51, 0.23, 0.15 and 0.065 at
    n = 13, 17, 19, 23.
  - Orbit base against signed-Frobenius rho: worse on all 8 curves, by 1.6× to 18×. The gap
    grows with r (β = 0.165 [0.117, 0.213] per ln r), so the base buys a constant and does not
    change the exponent.
  - Distance remaining to matched rho: 18.16× [15.67, 21.55] at n=41, 3.85× [3.04, 4.82] at
    n=23.
  - §5a band: no counter was left unpriced, so the 0, 0.1 and 1 GAE pricings give the same
    numbers.

## Bottom line

The Frobenius-closed non-linear base already pays off in the incumbent MITM pipeline: fewer
columns, the same yield per point, and the same table cost per trial. Index calculus is still
1.6–18× above rho-frob with it, and the gap grows with size. Under SAT or Gröbner solvers,
the non-linear base costs more per trial.

## Open thread (not run)

A cleanly size-matched SAT/Gröbner comparison for m ≥ 3, weight base against a random subspace
of equal size, at an n where 2 is primitive mod n, counted in conflicts. It is unlikely to
change the verdict: the table oracle already takes the gain, and algebraic oracles cost 50–100×
the table at these sizes. The right vehicle is a native ecbench arm (`onb-weight` with a SAT
oracle against `binary-subspace`).
