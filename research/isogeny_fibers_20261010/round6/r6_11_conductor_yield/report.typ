#set page(margin: 2cm)
#set text(size: 10pt)
= R6.11: relation yield across a prime conductor gap

*Question.* Is the index-calculus class factor $c$ the same on the crater ($"End" = cal(O)_K$) and the floor ($"End" = ZZ + ell cal(O)_K$) of an $ell$-volcano, $ell in {5,7,11,13}$, $p in [2^20, 2^22)$? (Toy test of Conjecture 10′.)

*Verdict: confirmed at toy size; the falsifier did not fire.* 24 instances (6 per $ell$), same $\#E = N$ prime on both levels (certified), $|"Aut"| = 2$, $h = 1$; every crater→floor edge certified by an Elkies kernel and Vélu codomain. Exact census at $m = 2, 3$, $|F| in {60, 80, 100, 200}$: every crater/floor ratio mean lies within 0.5% of 1 and every mean ± 2 sd interval contains 1 (measured).

#figure(image("fig1_ratio_vs_ell.svg", width: 100%), caption: [Crater/floor₀ class-factor ratio vs $ell$; bars mean ± 2 sd over 6 instances, dots per instance.])

#figure(image("fig2_c_over_birthday.svg", width: 100%), caption: [Measured $c$ over the derived birthday reference, every curve; crater and floor interleave.])

*Limits.* At $m = 2$, $|F|^2/2 << N$ forces $c approx 1$ on any curve; $m = 3$ is the informative arm. Plain x-frame bases only: an oracle that uses $"End"(E)$ explicitly is not tested. Toy sizes; $h(D) = 1$ craters; one vertical step. No sub-rho or deployed-curve claim.

Full tables, commands and raw data: `RESULTS.md`, `raw/`.
