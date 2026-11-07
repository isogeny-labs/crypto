# Explicit affine chart: domain and DVR proof packet

This packet records a formal statement about the *constructed affine chart*
and its localization at the origin. It makes no claim that the localization
has yet been identified with the smooth projective curve's infinity stalk.

For an arbitrary field `K` and coefficients `a3,a2,a1,a0`, let

`A = K[z,w]/(w²-z(1+a3 z²+a2 z³+a1 z⁴+a0 z⁵))`,

and let `m` be the kernel of origin evaluation `A → K`. The previous
`Common22ExplicitChart.lean` proves `m=(z,w)`, the factor in parentheses is a
unit in `A_m`, and the maximal ideal of `A_m` equals `(w)`. The new Lean
sources prove the following additional facts with no coefficient or
characteristic restrictions:

| Claim | Pinned Lean theorem | Result |
| --- | --- | --- |
| `W²-t*u` irreducible when `t` is prime and `t∤u` | `Common22Eisenstein.outer_quadratic_irreducible` | Checked |
| Nested chart polynomial irreducible over `K[z]` | `Common22Eisenstein.nested_chart_irreducible` | Checked |
| Original two-variable equation irreducible | `Common22Domain.chart_equation_irreducible` | Checked |
| `A` is a domain | `Common22Domain.chart_ring_domain` | Checked |
| `w` survives in `A` and `A_m` | `chart_wbar_ne_zero`, `stalkW_ne_zero` | Checked |
| `A_m` is a DVR | `Common22Domain.origin_stalk_dvr` | Checked |
| `w` is an irreducible local parameter of `A_m` | `Common22Domain.stalkW_irreducible` | Checked |

The irreducibility proof treats the equation as a monic quadratic in `w`
over the one-variable ring `K[z]`. Its constant coefficient has exactly one
factor of `z` because the second factor has constant coefficient `1`.
`Common22Domain.lean` transports the result through an explicit variable
swap and Mathlib's `finSuccEquiv`; irreducibility then gives a prime
principal ideal and a domain quotient. The degree-two equation cannot
divide the degree-one polynomial `w`, so `w` is nonzero in the quotient.
Localization preserves this fact. Mathlib's Noetherian-local-domain
criterion converts the already proved principal maximal ideal into the DVR
statement after ruling out a field.

The following are still **open** in Lean: construction of the smooth
projective completion and proof that its infinity stalk is isomorphic to
this `A_m`; transport of the normalized valuation and principal divisor
to that projective stalk; Mumford reduction and `Pic⁰` correspondence; and
the independent Rust law's semantic equivalence to those operations. A DVR
statement about the affine chart alone does not discharge those gates.
This proof packet adds no target solves, operation counts, or rho ratios.

`domain-dvr.csv` and `domain-dvr.json` are the standardized claim ledger.
`domain-dvr.svg` shows dependencies; `DOMAIN_DVR.pdf` is the printable
report. The sources use the pinned Lean 4.34.1 toolchain. Replay from this
directory with:

```sh
TMPDIR=/dev/shm lake env lean Common22Eisenstein.lean
TMPDIR=/dev/shm lake env lean Common22Domain.lean
```

Both source files have `#print axioms` audits. The final build receipt is
`domain-dvr-build.log`; successful output should list only `propext`,
`Classical.choice`, and `Quot.sound` for each final theorem.
