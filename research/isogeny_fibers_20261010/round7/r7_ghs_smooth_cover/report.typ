#set page(margin: 1.8cm)
#set text(size: 9.5pt)
= R7.1: smooth GHS descended curve for m >= 2, and R6.9's relation rate on it
2026-10-25. Stacked on PR \#2502 (R6.9). Source: `RESULTS.md` (same directory), raw data `relrate_descended.json`.

*Result.* The library builds the smooth model of the GHS descended curve $C slash k$ and $Phi: E(K) -> "Jac"(C)(k)$ for $m >= 2$.
On all 13 measured curves: genus confirmed by $N_(g+1)$ predicted from $N_1..N_g$; $Phi$ a homomorphism (260 of 260 checks);
$\#E dot Phi = 0$ and $\#J dot Phi = 0$ (156 checks, 0 failures); the degree-$2n$ Frobenius relation holds with a trace-perturbed negative control at 0.
At N = 12, $(l,n) = (2,6)$, $g = 8$, $q = 4$ (6 curves) the measured fully-split rate is 0.00064 to 0.00261 per step, 0.87 to 1.23 times the enumerated split fraction, and 26 to 105 times Gaudry's $1 slash g!$ (recalled).
R6.9's random stand-ins gave 585 and 858 times; the real descended Jacobians have only 5 $F_4$-points, hence 60 split divisors.
The $(3,4)$, $g = 8$ run did not finish (stopped, cause not isolated). No claim about any deployed curve or any cost against rho.

== Relation rate
#figure(image("fig1_relrate_descended_vs_standin.svg", width: 88%), caption: [Measured fully-split rate against the exactly enumerated split fraction of the walked group, $q = 4$, $g = 8$. 100000 steps per walk; error bars are binomial standard errors of an assumed-independent walk. Orange: R6.9's stand-in curves (recorded values). Source: `relrate_descended.json`, `figure_data.tsv`.])
#figure(image("fig2_ratio_to_inverse_gfactorial.svg", width: 88%), caption: [Measured rate over $1 slash g!$ per curve, $g = 8$, walk in $J_0$ (derived from the recorded rates). Dashed: R6.9's 10x falsifier.])

== Verification summary
#table(columns: (auto, auto), inset: 4pt,
  [Check], [Result],
  [`ghs_smooth` library tests], [5 passed, 0 failed],
  [Genus vs point counts ($N_(g+1)$ predicted)], [13 of 13],
  [$Phi$ homomorphism], [260 ok, 0 failed],
  [Order checks], [156, 0 failures],
  [Frobenius relation gcd degree], [$2n$ (or $2g$); control 0],
  [Closed-form split count = enumeration in $J_0$], [13 of 13],
)
== Not settled
Larger $N$ or $q$; the $q = 8$, $g = 8$ instance; odd-genus $t$-model beyond one curve; $a in.not k$; the factor-base decomposition step; any cost comparison with rho.
