# Nagao unit elimination toward the rho-parity goal

17 November 2026. User correction: “Two row parity” means **rho parity**.
The objective remains cold, single-target recovery against the strongest matched
rho reference, including unsuccessful work and setup. This component does not
replace that acceptance condition.

## Evidence and prior work

KB unavailable: no research-KB MCP is loaded; the repository wrapper has no
executable. Intended queries: “Nagao genus 3 quadratic extension unit variable
elimination”; “Joux Vitse five variable substitution F4 negative results”.
Fallback: the exact tracked native `weil_forms`, six-quadric solver and Nagao
materializer; study amendments 1, 13 and 38; stage41 report and stage42/43
ownership correction. Stage41's descent subtotal is 3426280619436 of
5645040401736 observed primitive units. Complete totals remain null.

Primary source read: Joux–Vitse, *Cover and Decomposition Index Calculus on
Elliptic Curves Made Practical*, sections 3.1–3.3, author PDF
<https://www-fourier.ujf-grenoble.fr/~viva/research/articles/CoverDecomp.pdf>,
SHA256 `7ca88c54ebfcb6fac8d05d313bfe8c26d3cb133081c55c4ab6ea4c1b45729b38`.
It gives the Nagao quotient construction and distinguishes it from the
precomputed homogeneous sieve. This is ordinary exact substitution, with no
novelty claim or transferred performance estimate.

## Algebraic hypothesis

For monic degree7 H and degree3 u, write
`Q=((x+mu)^2 H-A^2)/u`, `A=-(x+mu)v+(lambda0+lambda1*x)u`.
Then `Q5=H6+2*mu-lambda1^2-u2`. Its imaginary part has a nonzero
constant coefficient on `Im(mu)`, so eliminate that coordinate without dividing
by a varying parameter. Substitute into the other five equations; recover the
sixth coordinate and replay every original equation. Degree can rise to four;
an actual comparison may reject this hypothesis.

## Development and acceptance

- Preserve an unmodified native library build and record its exact revision,
  compiler and host. Current main lacks some global path dependencies; a
  separately identified historical library is a compatibility boundary, not
  a current-main build assertion.
- Implement an opt-in checked reduction, with bounded polynomial expansion,
  exact coefficient arithmetic and work hooks. Keep existing solver defaults.
- Test the algebra independently by exhaustive small-field lifting and
  planted Nagao fixtures, including empty sets and unsupported pivots.
- Compare complete solution sets of the six-variable and reduced solvers on
  fixed public residuals, preserving caps and failures. Local executions here
  are development checks; no admitted wall comparison is planned.
- Native diagnostic fixtures: public `generate_spec(p,seed)` at (53,2),
  (101,409), (1009,389); residuals G and the sum of the first six factor-base
  divisors, base RNG seed13, solver RNG seed947, six/unit/six/unit order.
  Degree24, staircase64, 30-second per-solve cap; no cap is an empty result.
  Compare sorted complete coordinate sets, cofactor geometry and full/partial
  group replay. Both within-method repeated primitive tallies must agree.
- Charge preparation, substitution, lifting and all original-equation replay.
  Report deterministic component counts only with the same owner-thread scope.
  No stage ratio becomes a full-DLP or rho-parity claim.

No paid launch: remaining balance of the original USD5 ceiling is unknown.
The unchanged full experiment remains open; no alternative paid allocation
is authorized by this plan. A remote comparative run requires a frozen source,
known remaining budget and the maintained isolation/accounting gates.

## Requirements

| Requirement | State |
|---|---|
| Faster algebraic finder compatible with graph elimination | Implementation pending |
| Exact solutions and full relation checks | Native qualification pending |
| Matched complete scalar recovery versus strongest rho | Open; complete accounting and admitted runs required |

