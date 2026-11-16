#set page(width: 8.5in, height: 11in, margin: 0.75in)
#set text(size: 9.5pt, font: "DejaVu Sans")
#set par(justify: true)
#align(center)[
  #text(14pt, weight: "bold")[The novel cover at the 56-bit ladder prime: explicit quartic and checked pullback]
  #v(1pt)
  #text(8.5pt)[Study `nist_cover_quartic_map_20261116`, amendment S56 (`PROTOCOL_S56.md`, sha256 `c7a3ef99…`) · `[pcc:unverified]`]
]
#v(3pt)
*Target.* `find_a3_curve(56, 3)`: $p = 2^56 - 5$, $E: y^2 = x^3 - 3x + 60$, $\#E = n = 72057594202405003$ (prime), $G = (2, 795920743654909)$.

*Result.* All checks of the frozen amendment hold:
- First Smith-positive $gamma$ is 15. That gives the cover $H_15: v^2 = u^7 + 42u^5 + 60u^4 + 630u^3 + 3375u$ and the explicit trigonal map.
- The pullback $phi^*: E -> J_H$ has order exactly $n$ on $G$ and is a homomorphism on the planted pair and on $G + Q$. The wrong-scalar control fails, as it should.
- The canonical quartic $C$ is fitted from 40 sampled image points and verified on *120/120* independently sampled held-out points. The perturbed-$C$ control holds on 0/120.
- $C$ is *certified smooth* by resultants in two charts (both gcds of degree 0) plus the point $(1:0:0)$.

#figure(image("stages.svg", width: 100%))

*Explicit quartic* $C$ over $FF_p$, $p = 2^56 - 5$ (`mons_of_degree(4)` order):
#text(8pt)[`[1, 44858746782548416, 0, 37572650059108326, 60844269658931211, 56389912100593629, 6018521062645660, 29095700753429674, 61315470249296059, 0, 71497452090043392, 29684774564308524, 41647834419698634, 49418112992754898, 398084569319130]`]

*Code change.* The Smith stack was widened from $p < 2^31$ to $p < 2^62$ without changing results below $2^32$:
- `u128` modular products above $2^32$.
- $(p^d - 1)/2$ powers via $(product_(i<d) a^(p^i))^((p-1)/2)$ when $p^d$ overflows.
- A `u64` quartic fit.

Regression: 31 existing tests pass (Smith's Example 1 among them), and 34 with the new controls (Fermat smooth, nodal rejected, compose consistent). `clippy -D warnings` reports no errors.

*Not run at 56 bits.* $Psi$ into $"Pic"^0(C)$ and the planted-log recovery on $C$ need the `u32` `plane_quartic_pic` port. $\#C = \#H$ is not countable at this size.

*Distance to rho (derived).* Matched negation rho on $E$ needs $sqrt(pi n \/ 4) approx 2.4 times 10^8$ operations. The lab's 56-bit ladder measured $2.6 times 10^8$ steps. Index calculus on a same-field genus-3 $C$ is $tilde(O)(p) approx 7.2 times 10^16$, about $3 times 10^8$ times more. This is the explicit construction at the requested size, not an improvement.

#text(7.5pt)[Evidence: `runs_s56.json` (`data_s56.sha256`), `run_s56.log`, `binary_s56.sha256`, `RESULTS_S56.md`. A rerun from the rebuilt binary is byte-identical apart from wall time (1.97 s).]
