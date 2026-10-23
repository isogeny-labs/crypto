# Generalized GHS / Weil descent on the IPsec Oakley EC groups 3 and 4

Date 2026-10-23. Status: structural screen, isogeny-neighbourhood census and
an exponent-level cost model. **No attack, no speedup, no vulnerability claim.**
Nothing here is a leaderboard or benchmark row, so no `VALIDATION.json`.

Curves (RFC 2409, bit i = z^i, a = 0): Group 3 `F_2[z]/(z^155+z^62+1)`,
b = 0x07338f; Group 4 `F_2[z]/(z^185+z^69+1)`, b = 0x1ee9. The hex encodings
are my reading of the RFC and should be checked against it before citing.

## Reproduce
```
cargo run --release --bin ghs_screen -- --degree 155 --modulus 0x800000000000000000000004000000000000001 --a 0 --b 0x07338f
cargo run --release --bin ghs_screen -- --degree 185 --modulus 0x20000000000000000000000000000200000000000000001 --a 0 --b 0x1ee9
cargo run --release --example ghs_oakley_isogeny_census -- 2000
```
Outputs: `data/oakley3-screen.json`, `data/oakley4-screen.json`, `data/census-output.txt`.

## Findings
1. **Classical GHS (direction 1 baseline).** Both curves have best magic
   number m = 5 (n = 5; l = 31 / 37), genus 2^(m-1) = 16. Other towers are
   far worse (genus 2^25 and up). MMT condition (8) holds; neither curve is in
   the genus <= 4 range.
2. **Isogenies (direction 2).** Phi_2 mod 2 = (X^2 - Y)(X - Y^2), so 2-isogenies
   are Frobenius/Verschiebung and only identify conjugates; my first census
   counted them as 155 distinct curves and was wrong. Counting Frobenius orbits
   and moving with l = 3 only (the only odd l tabled):
   - Group 3: a chain of 2000 distinct orbits (BFS depth 1000, so two
     directions of one crater). Every one has m = 5 for n = 5. A hit at m = 4
     is expected with probability about q^-1 = 2^-31 per orbit (heuristic), so
     2000 orbits is no evidence either way.
   - Group 4: **no rational 3-isogeny**; the orbit is isolated. Reaching other
     orbits needs l = 5, 7, ... (Phi_l not tabled in `binary_isogeny.rs`).
   - The walk is j-invariant level only; no explicit isogeny or kernel is built.
3. **Practicality (direction 3).** Exponent-level model (`cost_model` in the
   example; walk step cost 0.7 ms measured, converted at an UNMEASURED 100 ns
   per rho step; hit probability q^(m-n)):

   | curve | m | genus | walk | Gaudry g!q^2 | rho |
   |---|---|---|---|---|---|
   | Oakley 3 | 5 | 16 | 2^13 | 2^106.3 | 2^76.5 |
   | Oakley 3 | 4 | 8 | 2^44 | 2^77.3 | 2^76.5 |
   | Oakley 3 | 3 | 4 | 2^75 | 2^66.6 | 2^76.5 |
   | Oakley 4 | 5 | 16 | 2^13 | 2^118.3 | 2^91.5 |
   | Oakley 4 | 4 | 8 | 2^50 | 2^89.3 | 2^91.5 |
   | Oakley 4 | 3 | 4 | 2^87 | 2^78.6 | 2^91.5 |

   Oakley 3 at m = 4 sits 0.8 bits above rho at constant 1, so the hidden
   constants decide it, and that is the precise open item (Menezes-Teske
   section 10). Oakley 4 at m = 4 is 2.2 bits below rho in the same model, but
   only if an m = 4 orbit were reachable, and it has no 3-isogeny. Nagao-type
   q^(2-2/g) is shown as exponent only; its constant is unknown, so it is not
   a bound. Units differ between walk, HCDLP and rho (field operations vs
   group operations); the section 5a band at 0, 0.1 and 1 GAE is not applied,
   so **no ordering of arms is claimed**.

## Distance remaining (success-criterion framing)
Unmodified GHS is far from rho on both curves (Gaudry model 2^106 vs 2^76.5
and 2^118 vs 2^91.5). The one configuration within error of rho is an
undiscovered m = 4 isogenous curve for Group 3.

## Not done (and why)
- Hess generalized covers (direction 1 proper): needs function-field
  arithmetic (Magma/Sage; neither is installed here). Nothing in this change
  searches for smaller genus than classical GHS.
- Explicit char-2 Velu isogeny, Phi_l for l >= 5, genus-4..8 relation
  collection and linear algebra with real constants.
- No replay certificate, validator report or matched rho in the section 5a
  band, so this is not eligible for the leaderboard or `benchmarks/`.
- Conductor is not installed in this session; no `conductor check` was run.
- PDF: pdflatex is absent; the graph is `ghs-oakley.svg` (hand-laid from the
  table above).
