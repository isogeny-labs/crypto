#set page(paper: "us-letter", margin: (x: 0.72in, y: 0.68in))
#set text(font: "DejaVu Sans", size: 9.2pt)
#set par(justify: true, leading: 0.55em)

#align(center)[
  #text(size: 13.5pt, weight: "bold")[
    Necklace-Reduced x-Only Sibling $k$-Sum Tables with Free Setup Collisions on Koblitz and Binary Curves
  ]
  #v(3pt)
  #text(size: 8.8pt, fill: rgb("#374151"))[
    Study `research/koblitz_frob_ksum_sib_20261118` · Frozen commit `bac5b145b5` · Binary `0dd4f1894374…` · Date: 2026-11-18
  ]
]

#v(6pt)

== 1. Construction, Scope, and Assumptions
This study implements and evaluates five strictly counted improvements for characteristic-2 index-calculus decomposition oracles in `src/cryptanalysis/ic_boundary.rs`, `src/cryptanalysis/ic_framework/plugins.rs`, and `src/cryptanalysis/ecbench/{methods,work}.rs`:
+ *Binary x-only sibling pairs (`BinaryGroup::sibling_pair`):* computes both $x(P + J)$ and $x(P - J)$ on $y^2 + x y = x^3 + a x^2 + b$ over $FF_(2^n)$ from one shared field inverse $(x_P xor x_J)^(-1)$, charged as `1 GroupAdd + 1 sibling_second_points` (matching exact `bitop/v1` gate counts). This enables `sib=1` on both `ksum-frobenius` and generic `ksum`.
+ *Orbit-layout cyclic necklace canonicalisation (`ksum-frobenius:layout=1`):* indexes orbit members in $O(1)$ on `compact-orbit-scan` bases (zero build Frobenius maps or base lookups) and restricts leading-orbit rotations to the cyclic minimum-gap domain ($"rot"_1 <= floor(n/2)$, $"prev" + "rot"_1 <= "rot"_s <= n - "rot"_1$), removing rotational duplicates while retaining 100% of distinct orbit keys.
+ *Dead-end pruning (`prune=1`) and Montgomery leaf batches (`sib=1`):* skips partial sums unable to reach $m$ summands and batches leaf inversions via `Gf2::batch_inv` (chunks up to 256), cutting setup `GroupAdd` to $approx 0.52$ per stored orbit entry.
+ *Target-free setup collision harvesting (`free=1`) and analytical sizing (`scale=X`):* emits group-checked free orbit rows from colliding table keys (after a counted $ZZ[tau]$ mod-$r$ identity filter) and caps stored unique entries at $E = max(1, "round"(X sqrt(r / n)))$. With $M = (r-1)/(2n)$ signed Frobenius orbits, $E = X sqrt(r/n)$ yields scale-invariant expected collisions $n E^2 / r = X^2$ and asymptotic group cost $EE[S(w=0)] approx 1.17 / sqrt(n)$ at $X = 1.35$.

*Track and boundary:* Generic / Frobenius-orbit decomposition track ($tilde(Omega)(sqrt(r/n))$ on Koblitz curves, $tilde(Omega)(sqrt(r))$ on generic binary curves); cannot beat Pollard rho asymptotically in exponent. All flags default off so existing replay sentinels remain bit-identical. All 9 official sessions ran on clean commit `bac5b145b51f93346cfa796ff13c27d9a734c64c` (`git_dirty: false`, `ECBENV3h348f48ffd10a`, `ECBHW1hf5f6005c788f`, isolation `L0`), with 100% of runs verified (`[k]G == Q`), 100% of measured records reproduced identically under `ecbench verify --replay-all`, and `bitop/v1` `complete: true` (`unpriced: []`).

== 2. Cohort K: Held-Out Koblitz Ladder Results (15.4 to 44.3 bits)
One fixed recipe (`cand-kf4-c2-free`: `columns=2, generator_orbit=1`, `ksum-frobenius:m=4,layout=1,prune=1,sib=1,free=1,scale=1.35`, zero per-curve tuning) strictly beats every baseline IC arm (`inc-gen-layout-c*` and `inc-kf3`) across the entire `AGENTS.md` §5a pricing band $w in [0, 1]$ on all 6 curves (`ecbench band --ranks`):

#table(
  columns: (auto, auto, auto, auto, auto, auto, auto, auto),
  align: (left, right, left, right, right, right, right, right),
  inset: 4pt,
  [*Curve (session)*], [*$log_2 r$*], [*Strictest IC inc.*], [*Inc. $S(0 / 0.1 / 1)$*], [*`cand-kf4` $S(0 / 0.1 / 1)$*], [*Speedup $w=0$ \[95% CI\]*], [*Speedup $w=1$ \[95% CI\]*], [*Wall ratio*],
  [`icv1-f2m29-tm40309-30c52b96` (`ECBS1h9130ca664e3d`, $n=64$)], [15.37], [`gen-layout-c2` \ (`kf3`: 7.72)], [1.052 / 1.675 / 7.284], [0.665 / 1.244 / 6.455 \ (`kf5-c1`: 0.327/0.594/3.003)], [*1.58×* \[1.54, 1.62\] \ (`kf5`: *3.22×*)], [*1.13×* \[1.12, 1.14\] \ (`kf5`: *2.43×*)], [1.06× \ (`kf5`: 1.20×)],
  [`icv1-f2m23-t5197-69e76b73` (`ECBS1hb7bd5c42674b`, $n=64$)], [21.00], [`gen-layout-c6`], [1.012 / 1.352 / 4.412], [0.522 / 0.728 / 2.581], [*1.94×* \[1.65, 2.28\]], [*1.71×* \[1.55, 1.89\]], [1.39×],
  [`icv1-f2m23-tm5197-1f85e9e1` (`ECBS1hb366f3156d09`, $n=64$)], [22.00], [`inc-kf3`], [1.044 / 1.363 / 4.230], [0.445 / 0.621 / 2.201], [*2.34×* \[1.82, 3.02\]], [*1.92×* \[1.62, 2.28\]], [1.73×],
  [`icv1-f2m37-tm534059-32aad96b` (`ECBS1h3f13fef4265b`, $n=64$)], [27.78], [`gen-layout-c13`], [0.765 / 0.964 / 2.753], [0.326 / 0.435 / 1.414], [*2.34×* \[1.98, 2.77\]], [*1.95×* \[1.73, 2.19\]], [2.34×],
  [`icv1-f2m41-tm2308219-7f48b14a` (`ECBS1h764e82418bf0`, $n=48$)], [39.00], [`gen-layout-c90` \ (`c158`: 2.223)], [1.964 / 2.364 / 5.959], [0.322 / 0.425 / 1.346], [*6.09×* \[5.32, 6.97\]], [*4.43×* \[4.00, 4.91\]], [*7.14×* \ (366→56 ms)],
  [`icv1-f2m53-tm56619371-dac20a85` (`ECBS1h0a25307fd6a4`, $n=32$)], [44.26], [`gen-layout-c220`], [2.856 / 3.431 / 8.602], [0.241 / 0.314 / 0.968], [*11.84×* \[10.00, 14.03\]], [*8.89×* \[7.84, 10.08\]], [*14.75×* \ (4334→266 ms)],
)

#figure(
  image("fig-band-speedup.svg", width: 98%),
  caption: [Measured Cohort K paired speedup across the §5a band (left) and cold single-target $S(w=0)$ scaling vs matched `rho.signed_frobenius_strong` (right). Derived ratios computed by `ecbench compare --resamples 4000 --seed 20261118`.],
)

In `bitop/v1` gate counts (`ecbench work`), `cand-kf4-c2-free` reaches *3.31×* the information-theoretic floor on 39-bit `icv1-f2m41-tm2308219-7f48b14a` (down from *18.21×* for `inc-gen-layout-c90`, a *5.50×* gate reduction, and below `rho-strong`'s *3.41×* because x-only sibling leaves avoid per-step Artin–Schreier solves) and *2.76×* the floor on 44.3-bit `icv1-f2m53-tm56619371-dac20a85` (down from *30.11×* for `inc-gen-layout-c220`, a *10.93×* gate reduction, vs *2.99×* floor for `rho-strong`). On `icv1-f2m29-tm40309-30c52b96`, `cand-kf5-c1` eliminates the prior 5.3× `kf3` regression (`7.719 -> 0.327`, *23.6×* speedup) and strictly outranks `rho-strong` across $w in [0, 1]$.

== 3. Cohort B (Generic Binary `sib=1`) and Disclosed Null Algebraic Pilot

#figure(
  image("fig-bitop-and-binary.svg", width: 98%),
  caption: [Left: Cohort K `bitop/v1` gate ratio to floor. Right: Cohort B paired cost ratios $R_w$ of `cand-ksum5-sib-free` against the strictest 2026-11-15 `ksum:m=4|5,prune=1` baseline on Table B primary binary curves ($n=64$ pairs/curve).],
)

On Table B's primary random-binary curves (`ECBS1h3ca155cc5657`, `ECBS1hc4213df6893f`, `ECBS1h022bf6240f56`), `BinaryGroup::sibling_pair` (`cand-ksum5-sib-free`) strictly outranks `inc-ksum4-prune` across $w in [0, 1]$ on all three curves and reduces counted cost against the strictest `ksum4/5-prune` arm at $w = 0$ and $w = 0.1$ on every curve (`B21`: $R_0 = 0.894$ \[0.861, 0.927\], $R_(0.1) = 0.931$ \[0.895, 0.967\]; `B24`: $R_0 = 0.791$ \[0.746, 0.837\], $R_(0.1) = 0.824$ \[0.775, 0.873\]; `B27`: $R_0 = 0.780$ \[0.773, 0.787\], $R_(0.1) = 0.824$ \[0.818, 0.830\]; L0 wall speedups 1.14×–1.17×). Under the strict preregistered H2 all-weight CI rule at $w = 1.0$, larger $N=10$ vs $N=9$ (`B21` $R_1 = 1.086$, `B24` $R_1 = 0.959$ \[0.896, 1.024\]) and `free=1` native checks at matched $N=12$ (`B27` $R_1 = 1.000$ \[1.000, 1.000\]) leave $w = 1.0$ upper CIs $>= 1.0$; using `sib=1` without `free=1` at matched $N$ keeps $U$ identical to `ksum5-prune` while retaining the full leaf-add halving.

*Disclosed negative pilot (algebraic track, not shipped):* Replacing leaf Macaulay reductions by Gray-code quadratic enumeration (`k <= 16`) in `pdp3-koblitz` (`seed 999001`) cut Macaulay reductions 38% (`9,979 -> 6,159`) on `icv1-f2m17-tm101-00378d4e`, but $2^(11)$ leaf sweeps cost `15.3M` word XORs vs degree-3 refutation's `~13.7M` (`S: 1,419 -> 1,431`; `icv1-f2m19-t797-b6cf2467` `S: 5,274 -> 5,204`). Next algebraic step: defer linear $U$-elimination until all bits of a chained summand are assigned so `InheritedBases` avoids the 62 shallow degree-3 Macaulay rebuilds. *Canonical graphs checked and left unchanged:* `docs/ic/LEADERBOARD.md`, `docs/index-calculus-scoreboard.html`, and `docs/ic/progress-timeline.json` (unchanged pending second-host replay validation per `docs/VALIDATION_GATE.md`).
