#set page(margin: 1.8cm)
#set text(size: 9.5pt)
#set heading(numbering: none)
= R7.3: conductor-sensitive algorithms across an ℓ-volcano gap

_Second toy test of Conjecture 10′. Native Rust, $p in [2^20, 2^22)$, counted operations only. Source: `examples/r7_3_conductor_sensitive.rs`; raw data `raw/`; full text `RESULTS.md`._

*Hypothesis H.* Some algorithm whose cost depends on $"End"(E)$ shows a measurable crater-vs-floor cost gap on ℓ-volcanoes. *Falsifier.* Every crater/floor ratio CI contains 1 after the automorphism baseline is removed (null), or a CI excludes 1 (report size, cause, and whether it exceeds the bounded factor).

== Verdict (measured)

+ *Iterations and relation yield: null.* Conductor effect after removing $|"Aut"|$: $1.0018$ [0.9986, 1.0050] at $j=0$ and $0.9997$ [0.9966, 1.0030] at $j=1728$ (32 instances each); endomorphism-mixed rho, iterations $1.0001$ [0.9921, 1.0080] (40 instances); plain rho $0.9999$ [0.9939, 1.0061]; class factor at $h(D)=2,3$ craters $0.99985$ [0.9993, 1.0004] (56 instances); transported-base census identical to its source in 120/120 checks. Conjecture 10′ is untested-but-not-contradicted at this scale for these algorithms.
+ *Counted multiplications: the falsifier's second branch fires for one algorithm.* The endomorphism-mixed walk costs the floor $1.46 times$ [1.44, 1.48] the crater, with equal iteration counts; the gap is the cost of evaluating the cheapest explicit endomorphism ($2.72 times$ [2.59, 2.85]: three Vélu maps on the floor, one on the crater; $2.5$ to $2.9 times$ for ℓ = 5 to 13). That walk is $1.22 times$ (crater) and $1.79 times$ (floor) dearer than plain rho and takes no fewer iterations ($0.9993$ [0.989, 1.009]), so the best strategy ignores $omega$ and has crater/floor cost $0.9999$ [0.9945, 1.0055]. A polynomial-in-ℓ constant on an inferior algorithm, not a change in the best attack's cost.

The known automorphism effect is the positive control: ζ-fold rho is cheaper than $plus.minus 1$ rho on the crater by $0.5534$ [0.5511, 0.5559] ($j=0$, ideal $0.5774$) and $0.6820$ [0.6791, 0.6848] ($j=1728$, ideal $0.7071$).

#figure(image("fig1_expA_aut_baseline.svg", width: 100%), caption: [Exp A, 32 instances per $D$, 3000 runs per curve and variant. Squares: aut/pm on the crater (known Aut effect; dotted line is the ideal $1/sqrt(|"Aut"|/2)$). Circles: crater pm / floor pm, the conductor effect with the baseline removed. Triangles: (crater aut / floor pm) times the ideal baseline factor; the offset from 1 is the implementation shortfall of the 6- and 4-fold classes, not a conductor effect. Geometric means, 95% cluster-bootstrap CIs.])

#figure(image("fig2_expB_endo_walk.svg", width: 100%), caption: [Exp B, 40 instances ($h=1$ craters, 380 floors), 600 runs per curve and variant. Left: iterations; right: counted $F_p$ multiplications. Crater/floor ratios for the plain walk and the endomorphism-mixed walk (2 of 16 table slots), and eps/plain on the crater.])

#figure(image("fig3_eps_cost_glv.svg", width: 100%), caption: [Left: measured multiplications per endomorphism evaluation (crater $omega$; floor $psi omega phi$, degree $ell^2 (1+|D|)/4$), one dot per curve; an affine addition costs 32 to 42. Right: GLV split length $log_2 max(|k_1|,|k_2|)$ of the measured eigenvalue, floor against crater per instance (floor minus crater: $+0.15$ bits, sd 0.69, n = 40).])

#figure(image("fig4_expH_class_number.svg", width: 70%), caption: [Exp H, 56 instances, $h(D) in {2,3}$, 12 discriminants, $m=3$, $|F|=200$: crater over mean of its floors, class factor $c$. 5 of 48 per-discriminant intervals exclude 1 (by at most 0.34%, both signs; unadjusted).])

== Methods in brief

Instances: $p=(t^2+|D|v^2)/4$, $ell parallel v$, $t equiv.not 2 (mod ell)$; crater = CM curve of $D$ ($"End"=cal(O)_K$), floors = other $F_p$-roots of $Phi_ell(j_0,Y)$; $\#E=N$ certified on every curve; the DLP lives in the prime-order subgroup, on which every vertical isogeny is an isomorphism. Walk: 16-entry r-adding walk on orbit classes, exact collision bookkeeping, every recovered $k$ verified (1 608 000 runs, 0 wrong, 0 failed). Endomorphisms: explicit Vélu maps, certified on random points (on-curve and additive) with eigenvalues matched ($omega$ from the π-eigenline of $E[n_omega]$; floor $psi omega phi$ acts as $plus.minus ell lambda$). Seeds: the same step-table and start scalars and hidden $k$ on every curve of an instance.

== Limits

Toy sizes, one vertical step, $ell <= 13$, $h <= 3$; endomorphism algorithms only for $h=1$ ($D in {-19,-43,-67,-163}$); endomorphism slot fraction fixed at 1/8; the library `LineOracle` / $F_(p^2)$ ζ-fold and a level-dependent index-calculus oracle were not run; the iteration counts are counts, not timings. No sub-rho claim, no claim about cryptographic sizes or any deployed curve.
