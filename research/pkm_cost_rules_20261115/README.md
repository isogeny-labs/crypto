# PKM cost rules: s < m-3 versus c < (m-1)/2 (2026-11-15)

Standing: every number here is [pcc:unverified]. Desk derivation and numeric checks only; no solver runs, no commit.
PRESET.txt (sha256 996597a5...d7c3) was frozen 2026-11-15T23:39Z before any script ran.

## Derivation (first principles)
Per attempt (one target, one whole solve returning all solutions): yield mu = 2^N/(m! r); relations needed ~B; attempts A >= max(1, B/mu);
per-attempt charge B^s; LA charge B (linear) or m B^2 (quadratic). With B = r^beta, uniform depth:
  E(beta) = max(l*beta, max(0, 1-(m-1)beta) + s*beta).
- Without the max(0, .) floor and with l = 2: E* = 2/(m+1-s) and beta* = 1/(m+1-s). E* < 1/2 iff s < m-3: the closure rule
  (prime_ic_leads_20261110 sec 2.4). For s > 2 its optimum has attempt exponent 1-(m-1)/(m+1-s) < 0, i.e. fewer than one solve.
- With the floor: E* = l/(m-1+l-s) for s <= l, else s/(m-1). Beat-rho bar s < min(m-1-l, (m-1)/2) = (m-1)/2 for m >= 5 under either
  LA charge: the 10-31 rule, and the 11-13 continuation's gamma < 7/2 at m = 8. The two rules are branches of one optimisation; they
  agree for m <= 5 and the closure rule is wrong (too generous) for m >= 6.
- Mixed tower depths (N bits, B = 2^{max t_i}) do not evade the floor (K2).
- One output per call: bar s < m/2 - 1 (= 3 at m = 8).

## Checks (kill rule in PRESET.txt)
K1 numeric vs closed form max dev 1.7e-4 (< 2e-3); floored beat-set never equals {s < m-3} for m >= 6. PASS.
K2 mixed-depth gain over uniform floored optimum 2.2e-16. PASS.
K3 2/(m+1-s) reproduces the closure's 0.645 (m6), 0.430 (m8), 0.518 (m7), 1.140 (m4). PASS.  H1 survives.

## Results: out/rules.out, out/results.json, out/m8_rules.{svg,png,pdf}
