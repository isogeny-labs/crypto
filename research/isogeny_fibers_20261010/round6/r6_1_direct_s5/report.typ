#set page(margin: 2cm)
#set text(size: 10pt)
= R6.1: direct S#sub[5] systems and the `chained-x` plug-in

Toy Koblitz curves only; no sub-rho claim; nothing about deployed curves. Every number below was measured by `examples/r6_1_direct_s5.rs` and the full tables are in `RESULTS.md`.

== Verdict
Mixed: the falsifier is not met, and the hypothesis is partly refuted.

- *Falsifier not met.* No direct-S#sub[5] arm beats the best m = 2 arm. At n = 13 the direct symmetrised S#sub[5] costs S = 7.6e5 against 186 for mitm m = 2. From ℓ = 6 (n ≥ 17) the direct S#sub[5] cannot be Weil-restricted inside a 15-minute cell.
- *Hypothesis partly refuted.* The direct S#sub[5] is dearer per relation than the chain: 1.4e9 against 7.7e6 word XORs at n = 13. It is also not feasible wherever the chain is.
- *Discrepancy explained.* Matched target by target, the panel and the pipeline agree exactly: 396/396 outcomes and XOR counts. The 0.04 against 0.22 hit-rate gap is target sampling (Fisher p = 0.036), not a bug.
- *Plug-in.* The new `chained-x` oracle (library change) runs m = 3 and m = 4 end to end and recovers the planted logarithm. At every n the ordering is m = 2 < m = 3 < m = 4.

== Interpolation
Both S#sub[5] polynomials are unique, with F#sub[2] coefficients:
- the symmetrised one has 182 unknowns, 24 orbits with coefficient 1 and Boolean degree 9;
- the x-frame one has 1 287 unknowns, 44 orbits and Boolean degree 12.

The same code reproduces the library's S#sub[4] terms in both frames. Each polynomial vanished on 200/200 fresh relations at six (n, a) and on 0/200 random tuples.

#figure(image("s_by_arm.svg", width: 100%), caption: [Left: S by arm and n from end-to-end `run_pipeline` runs, with the logarithm verified in every run. Right: word XORs per relation in the m = 4 panel, 24 targets per arm and 0 gate failures.])

== Limits
- n = 15 has no usable curve, and a = 1 exists only at n = 17.
- 6 cells were skipped and are marked NOT RUN.
- The symbolic build is not counted in XORs.
- All wall times are contended.
