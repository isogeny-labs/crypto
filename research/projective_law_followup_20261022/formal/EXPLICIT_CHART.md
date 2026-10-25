# Explicit infinity-chart local ring: review packet

The pinned Lean development `Common22ExplicitChart.lean` constructs, for every
field `K` and coefficients `a3,a2,a1,a0`, the affine quotient

`A = K[z,w]/(w² - z(1+a3 z²+a2 z³+a1 z⁴+a0 z⁵))`

and the localization `A_m` at the origin evaluation ideal `m`. The formal
proof establishes that origin evaluation descends to `A`, that `m` is maximal
and equals `(z,w)`, and that `A_m` is Noetherian and local. In `A_m`, the
factor `u=1+a3 z²+a2 z³+a1 z⁴+a0 z⁵` is a unit, `w²=z*u`, and the maximal
ideal equals `(w)`. These results discharge the *previously supplied*
coordinate-ideal and unit hypotheses of `Common22LocalParameter.lean` for
this particular constructed local ring.

The displayed algebra does not by itself supply the remaining geometry.
`A` has not been proved integral in Lean; one prospective route is
Eisenstein in `w` at the prime `z` over `K[z]`, followed by transport back
to the multivariable quotient. A domain/DVR instance for `A_m` has not
been established. Nor has this localization been identified with the stalk
of the smooth projective completion at infinity. Thus the normalized
valuation, principal divisor, Mumford reduction, and `Pic⁰` transport
remain open. This packet asserts no universal projective-law correctness
and changes no measured target counts or rho cost ratios.

`explicit-chart.csv` records each checked and open claim in a stable table;
`explicit-chart.svg` and `EXPLICIT_CHART.pdf` provide a graph and printable
report. The source is pinned to Lean 4.34.1 by `lean-toolchain`; the source
has `#print axioms` audits for every final theorem. The audit output lists
only `propext`, `Classical.choice`, and `Quot.sound`.

Replay from this directory:

```sh
TMPDIR=/dev/shm lake env lean Common22ExplicitChart.lean
```

The final local replay of the command above exited 0. Its output contains all
13 theorem axiom audits, each listing only `propext`, `Classical.choice`, and
`Quot.sound`, with no Lean errors. The host delayed the `lake` wrapper after
the Lean child finished, but it subsequently returned a successful exit.
