# Generalized GHS / Weil descent on the IPsec Oakley EC groups 3 and 4

Date 2026-10-23. Status: structural screen, exact magic-number spectrum,
isogeny-neighbourhood census (both twists), exponent-level cost model.
**No attack, no speedup, no vulnerability claim.** Nothing here is a
leaderboard or benchmark row, so no `VALIDATION.json`.

Curves, checked against RFC 2409 sections 6.3 and 6.4 on 2026-10-23
(a = 0 for both):

| group | field | b | RFC group order |
|---|---|---|---|
| 3 | `u^155 + u^62 + 1` | 0x07338f | 0x0800000000000000000057db5698537193aef944 |
| 4 | `u^185 + u^69 + 1` | 0x1ee9 | 0x01ffffffffffffffffffffffdbf2f889b73e484175f94ebc |

## Reproduce
```
cargo run --release --bin ghs_screen -- --degree 155 --modulus 0x800000000000000000000004000000000000001 --a 0 --b 0x07338f
cargo run --release --bin ghs_screen -- --degree 185 --modulus 0x20000000000000000000000000000200000000000000001 --a 0 --b 0x1ee9
cargo run --release --example ghs_magic_spectrum
cargo run --release --example ghs_oakley_isogeny_census -- 300
cargo run --release --example ghs_relation_constants
```
Outputs in `data/`. `data/census-single-representative-l3-SUPERSEDED.txt` is
the first run (l = 3, one twist representative per orbit); it is kept, not
used.

## Corrections to the first version of this report
1. The first cost model rows for m = 2, 3, 4 described magic numbers that
   cannot occur for n = 5 (finding 2); they and the "0.8 bits above rho"
   statement are withdrawn.
2. The census recorded one twist per j-orbit while saying it covered both
   (found in review of 85bcdcf0b); it now evaluates both and reports orbit
   and representative counts separately.
3. "Explicit characteristic-2 Velu isogeny not implemented" was wrong:
   `src/cryptanalysis/binary_velu.rs` has it. It was checked against Phi_3;
   a new test now also checks the codomain against the exact Phi_5 and Phi_7
   (a rational order-l kernel gives j' a root of Phi_l(X, j), and the group
   order is unchanged).

## Findings
1. **Classical GHS.** Best magic number m = 5 (n = 5; l = 31 / 37), genus
   2^(m-1) = 16 (type II). Other towers are far worse. MMT condition (8)
   holds. Neither curve is in the genus <= 4 range.
2. **The magic-number spectrum (new, exact).** m(b) = deg Ord_b + c (MMT
   Thm 5), where Ord_b divides x^n - 1 over **F_2** and c = 1 iff (x+1) does
   not divide Ord_b. By the normal basis theorem every divisor other than 1
   occurs. For odd n the irreducible factors are the cyclotomic cosets of 2
   mod n. `ghs_magic_spectrum` computes the spectrum and confirms it by
   enumerating every b != 0 in four small fields (N = 9, 10, 14, 15):
   observed = predicted in all four.
   - n = 5: spectrum {1, 5}. **Every curve not defined over the subfield has
     m = 5**; no isogeny can lower it. m = 1 is the subfield case, which the
     Oakley orders exclude by a size argument (not checked by computation here).
   - n = 37: {1, 37}. n = 31: seven values, smallest above 1 is 6. n = 155:
     62 values. n = 185: 12 values.
   - So extended GHS by isogeny cannot reduce genus below 16 on the n = 5
     tower. The question moves to Hess's generalized covers (different
     Artin-Schreier functions), which this repository does not implement, and
     to the cost of HCDLP at genus 16.
3. **Isogeny census (both twists).** Phi_2 mod 2 = (X^2 - Y)(X - Y^2), so
   2-isogenies are Frobenius/Verschiebung and identify conjugates. Moving by
   l in {3, 5, 7} over Frobenius orbits, 300 orbits per curve, 600 twist
   representatives:
   - Group 3: n = 5: all 600 have m = 5 (as finding 2 requires). n = 31:
     m in {26, 31}. n = 155: m in {126, 145, 146, 150, 151, 155}. Every
     observed value lies in the tower's spectrum.
   - Group 4: l = 5, 7 leave the 3-isogeny-isolated orbit (the first run's
     "no rational 3-isogeny" holds for l = 3 only). n = 5: all 600 have
     m = 5; n = 37: all 600 have m = 37; n = 185: m = 181 (48) or 185 (552).
     All lie in the tower spectra.
   - Walk is j-invariant level; no isogeny or kernel is built by the census.
4. **Cost model (exponent level, NOT a measurement).** Only m = 5 exists, so
   there is no walk cost:

   | curve | genus | Gaudry g!q^2 (constant 1) | Diem/Nagao q^(2-2/g), exponent only | rho |
   |---|---|---|---|---|
   | Group 3 | 16 | 2^106.3 (+29.8 bits vs rho) | 2^58.1 (-18.4 bits, constant unknown) | 2^76.5 |
   | Group 4 | 16 | 2^118.3 (+26.8 bits) | 2^69.4 (-22.1 bits, constant unknown) | 2^91.5 |

   The two columns disagree in sign, so the exponent analysis does not decide
   it: the hidden constant of a genus-16 Diem-type relation search and its
   sparse linear algebra is the open cost question (Menezes-Teske, section
   10). Menezes-Teske (ePrint 2004/235) record GF(q^5) as weak under plain
   GHS, so these curves sit in the regime the literature already flags.
   Units differ between columns and rho (field vs group operations); the
   section 5a band at 0, 0.1 and 1 GAE is not applied, so **no ordering of
   arms is claimed**.

## Measured relation-phase constants (Gaudry route, genus 16)
`ghs_relation_constants` measures two inner loops in the same generic
`F2mElement` arithmetic (unoptimized on both sides); output in
`data/relation-constants.txt`.
- **Smoothness probability.** A random monic degree-g polynomial over
  F_(2^8) splits into distinct linear factors with probability within 0.4% to
  10% of 1/g! for g = 2..6 (ratio measured/1/g!: 0.996, 0.988, 0.980, 0.973,
  0.904). The ratio falls with g because q = 256 is small next to g; at
  q = 2^31 the 1/g! value is the expected one but was not measured there
  (2^-44 is out of reach). The `x^q = x (mod u)` splitting test agrees with
  root finding on 5000 random degree-4 polynomials (0 disagreements).
- **Cost per trial.** Degree-16 splitting test over F_(2^31): about 0.63 ms.
  Affine EC addition proxy (1 inversion, 2 mul, 1 square) over F_(2^155):
  about 7.4 microseconds.
- **Result.** At genus 16 the relation phase alone needs about q/(1/16!) =
  2^75.3 trials, 2^94.5 ns in total, against rho at 2^76.5 steps = 2^89.3 ns:
  **5.2 bits slower**, before the factor-base build, divisor arithmetic and
  linear algebra, so this is a lower bound for the plain Gaudry route.
  Optimized code would shift both sides by unmeasured, different factors, so
  the sign is a measurement of these two loops only, not of the best
  implementations. It says nothing about the Diem/Nagao route, which needs far
  fewer smoothness trials; that route is still unmeasured.

## Distance remaining
Gaudry-type HCDLP: 27 to 30 bits above rho. Diem-type exponent: 18 to 22
bits below rho before constants. Neither is a measured end-to-end cost.

## Verification run
`cargo test --release --lib` on this branch (with `phi_exact` and the l = 5..13
lookup): 4488 passed, 0 failed, 119 ignored (the repo's own `#[ignore]` markers),
419 s. The new `phi_exact_matches_tables_and_kronecker` test is among the passes.

## Not done
- Hess generalized covers (direction 1 proper): needs function-field
  arithmetic. Magma and Sage are not installed; `pip install
  passagemath-standard` was tried and failed with "No space left on device"
  (about 3.3 GiB free; Sage needs several GiB more). The Menezes-Teske text
  could not be fetched (HTTP 403 on the PDF).
- Velu point-map transport at l >= 5 (the codomain is tested at l = 5, 7;
  the point map is tested at l = 3 only); the Diem/Nagao relation search,
  factor-base build and linear algebra (only the Gaudry smoothness loop is
  measured above).
- Conductor: the release download returned HTTP 403 through the sandbox
  proxy; `go install` through the module proxy worked, but `conductor up`
  needs Postgres or a Docker daemon, neither of which runs here, so
  `conductor check` was not run.
- No replay certificate, validator report or section 5a matched rho, so
  this is not eligible for the leaderboard or `benchmarks/`.
