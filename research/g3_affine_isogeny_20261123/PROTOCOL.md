# Explicit plane-quartic codomain of an affine-type (2,2,2)-isogeny: validation protocol

Frozen 2026-11-23 before any step-3 validation data at the protocol rows (hashes in
`frozen.sha256`). Standing of every number produced under it: `[pcc:unverified]` (one
session, shared host, no independent replay, no pcc claim filed). Follows
`research/g3_affine_twist_20261121` (the twist test, its `RESULTS.md` and `ERRATA.md`).

## Question

`H: y^2 = f(x)` over `F_p`, `f` monic irreducible septic, `#J(F_p)` prime, `L` one of the
two Frobenius-stable affine-type Lagrangians of `J[2]`, `B = J/L`. Does the construction in
`fpquartic.py` / `build.py` give a plane quartic `X` over `F_p` with `Jac(X)` isogenous
to `J` in the way the twist test predicts: `Jac(X) ~ J` (same Frobenius polynomial) when
the twist study labels `B` "Jacobian over F_p", and `Jac(X) ~ J^tw` (Frobenius polynomial
`P_J(-T)`) when it labels `B` "twist"?

## Instrument (built before this freeze; development data listed below)

1. `derive2.py`: squares and same-top-row products of the 36 even theta constants of
   `B` as polynomials in `rho_ij = (e_j - e_i)^(1/4)` (duplication + Thomae, extending the
   twist study's `derive.py`, same seed-1 configuration and labelling; formula
   `formulas/prod_001000.json`, checked against direct theta series to 4e-12).
2. `tables.py` (complex numerics): Jacobi's derivative formula for all 2,016 azygetic odd
   triples (`formulas/jacobi.json`), 315 Riemann quartic theta relations
   (`formulas/riemann_rel.json`), and the Riemann quartic from odd-theta gradients with all
   28 gradient lines as bitangents. Re-checked on 5 period matrices (4 seeds, 4 kernels):
   `data/tables_check.json`.
3. `fpquartic.py` + `build.py`: the finite-field construction (steps listed in the
   `fpquartic.py` docstring): theta squares in `F_{p^56}`, individual thetas (row signs
   by the Riemann relations), Frobenius label action, a Frobenius-stable Aronhold set,
   its Riemann matrix by Jacobi's formula, Riemann's quartic, Galois descent (Hilbert 90),
   then point counts over `F_p, F_{p^2}, F_{p^3}` and the Frobenius polynomial.
   The construction uses only the branch points `u_i` (the even model's leading
   coefficient `f(t)` and the twist label are never read).
4. `run_validation.py`: the run below.

Development data (before this freeze): `p = 23` (non-protocol prime, 6 fresh curves, 12
kernels: 12/12 matched the label: 9 J, 3 twist, `dev/devrun.py`) and the protocol file's
first `p = 17` curve (row indices 0 and 1: one degenerate, one twist matched twist). That
curve is excluded from the population below.

## Population

`../g3_affine_twist_20261121/data/main.jsonl`, primes 17, 19, 29, 31, 37, 41, 43, 47,
curves 1 to 20 in file order at each prime (curve 0 skipped at every prime), both stable
affine kernels: 320 kernels. Model parameter `t` = the twist study's first `t`; the
label is the twist study's: `s = chi(-1) chi(f(t)) chi(S_L)`, Jacobian if `s = 1`, twist if
`s = -1`, degenerate if `S_L = 0`.

## Match criterion

`P_X` = Frobenius polynomial of `Jac(X)` from the exact counts `N1, N2, N3` of the
projective plane quartic `X` (Newton identities, as the twist study's census).
Match "J" if `P_X == P_J` exactly, "twist" if `P_X == P_J(-T)` exactly, else "neither".
A kernel is informative if `P_J != P_J(-T)`.

## Predictions and success rule

- **Primary.** Among the non-degenerate informative kernels, (i) at least 95% give
  status `ok` (a quartic over `F_p` was produced), and (ii) **every** produced quartic
  matches its label: label Jacobian -> "J", label twist -> "twist". One mismatch refutes
  the claim "construction + label agree" (it would be reported, not explained away).
- **Negative control.** The twist-labelled kernels are the negative control: their
  quartics must match `P_J(-T)` and not `P_J`. (The construction never sees the label.)
  The control is informative only if at least 20 twist-labelled kernels produce quartics.
- **Secondary.** Degenerate kernels (`S_L = 0`) give status
  `vanishing_even_theta (hyperelliptic codomain)`.
- **Gate G1 (replay).** Curves 1 to 3 at every prime (both kernels): a second
  construction with fresh fourth roots, `zeta8`, Hilbert-90 seed, another relabelling
  permutation `sig` and another model parameter `t` gives the same `N1, N2, N3`.
- **Gate G2.** `P_X` integral (Newton identities) for every produced quartic.

## Budget

Single thread, pure Python + python-flint, expected under 30 minutes.
