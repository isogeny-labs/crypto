#set page(width: 8.5in, height: 11in, margin: 0.75in)
#set text(size: 9.5pt, font: "DejaVu Sans")
#set par(justify: true)
#align(center)[
  #text(14pt, weight: "bold")[The planted logarithm carried onto the plane quartic at the 56-bit ladder prime]
  #v(1pt)
  #text(8.5pt)[Study `nist_cover_quartic_map_20261116`, amendment S56b (`PROTOCOL_S56B.md`, sha256 `da174929…`) · `[pcc:unverified]`]
]
#v(3pt)
*Instance* (unchanged from S56): $p = 2^56 - 5$, $E: y^2 = x^3 - 3x + 60$, $n = 72057594202405003$ (prime), planted $k = 19837203754012257$, cover $H_15$ and the explicit quartic $C$ of S56.

*Result.* The full chain $E(FF_p) -> J_H (FF_p) -> "Pic"^0(C)$ runs at 56 bits:
- $Psi(phi^*G)$ is well defined: two independent representatives give the same class.
- $Psi(phi^*G)$ is nonzero, and $[n] Psi(phi^*G) = 0$, so the order-$n$ subgroup is preserved.
- $Psi(phi^*Q) = [k] Psi(phi^*G)$: the planted logarithm is carried onto $C$.
- $Psi$ is additive on $G + Q$.
- The wrong-$k$ control fails, as required.

The $Psi$ stage takes 0.04 s with a 48-point pool.

#figure(image("stages.svg", width: 100%))

*Code.*
- `plane_quartic_pic64` is a 64-bit copy of the `Pic^0` arithmetic. It uses `u128` products and step-reduced accumulators, and keeps the original algorithms; the `u32` original is untouched.
- `Transfer64` ports $Psi$ with sampled base points and pool, with no enumeration.
- `split64` finds the points of a split class by factoring.

Regression (21 passed):
- identical canonical keys to the `u32` original at $q = 1009, 65521, 2^31 - 1$;
- the same $Psi$ class as `smith_canonical::Transfer` at $p = 1009$;
- `split64` agrees with `HJac::split` on 400 classes.

*Deviation.* The first run stalled in `proots`, which scans $0..p$, inside `HJac::split`. It produced no $Psi$ output and its log is kept. After `split64` replaced that call, the frozen command was run unchanged.

*Distance to rho (derived).* Rho on $E$ needs about $2.4 times 10^8$ operations; index calculus on $C$ needs about $7.2 times 10^16$. This completes the construction at the requested size; it is not an improvement.

#text(7.5pt)[Evidence: `runs_s56b.json` (`data_s56b.sha256`), `run_s56b.log`, `run_s56b_aborted_proots.log`, `binary_s56b.sha256`, `RESULTS_S56B.md`.]
