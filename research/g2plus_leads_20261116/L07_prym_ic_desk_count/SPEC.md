# L07 — implementation specification

For an executor that makes **no research choices**. Python 3 standard library
only (`-I`). One thread. Each script << 10 CPU-min, << 2 GB. All outputs are
exact integer/rational counts; no wall time. Everything produced is
`[pcc:unverified]`.

Directory: `research/g2plus_leads_20261116/L07_prym_ic_desk_count`
Create `src/` and `raw/`. Three files to write: `src/genus_table.py`,
`src/split_law.py`, `src/test_l07.py`. Then assemble `DERIVATION.md`.

The one empirical quantity is the **complete-splitting probability** of a
random reduced divisor of degree `d` over the degree-1 places of a curve over
`F_p`. Under the standard random-monic-polynomial model this is **exact**:

```
splitfrac(p, d) = C(p, d) / p^d        (d distinct rational roots)
```

because a monic degree-`d` polynomial over `F_p` splits into `d` distinct
degree-1 factors iff it is a product of `d` distinct `(x - r)`, and there are
`C(p,d)` such polynomials out of `p^d` monic degree-`d` polynomials. For
`p >> d` this is `~ 1/d!`. `d` is the sieving genus: `d = g` for hyperelliptic
genus-`g` IC, `d = 2g+1` for Prym IC on `D`. No curve arithmetic is needed.

## File 1 — `src/genus_table.py`

Pure arithmetic, no randomness. Use `fractions.Fraction`.

For `g` in `1..6` inclusive compute:

| column | formula |
|---|---|
| `g` | the Prym dimension |
| `h` | `g + 1` (genus of base curve C') |
| `genus_D` | `2*g + 1` (Riemann–Hurwitz, étale double cover) |
| `dim_P` | `g` |
| `dominant` | string `"true"` if `g <= 5` else `"false"` |
| `prym_2lp_exp` | `Fraction(2) - Fraction(2, 2*g+1)` |
| `hyper_g_2lp_exp` | `Fraction(2) - Fraction(2, g)` |
| `rho_prym_exp` | `Fraction(g, 2)` |
| `prym_beats_hyper` | `"true"` if `prym_2lp_exp < hyper_g_2lp_exp` else `"false"` |

Print to stdout a TSV with this exact header line first:

```
g	h	genus_D	dim_P	dominant	prym_2lp_exp	hyper_g_2lp_exp	rho_prym_exp	prym_beats_hyper
```

Then one row per `g`. Render each `Fraction` as `num/den` (e.g. `16/9`) and,
in trailing columns after a tab, also its decimal to 6 places:
append three more columns `prym_2lp_dec`, `hyper_g_2lp_dec`, `rho_prym_dec`.
Final header:

```
g	h	genus_D	dim_P	dominant	prym_2lp_exp	hyper_g_2lp_exp	rho_prym_exp	prym_beats_hyper	prym_2lp_dec	hyper_g_2lp_dec	rho_prym_dec
```

Decimals via `float(fr)` formatted `"%.6f"`.

## File 2 — `src/split_law.py`

Two parts. Output is JSON Lines to stdout; each line is one JSON object with a
`"kind"` key. Use only `json`, `fractions.Fraction`, `math`, `random`.

### Part A — exact closed form (no sampling)

Constants:
```
PRIMES  = [101, 1009, 10007]
DEGREES = [1, 2, 3, 4, 5, 8, 9, 10, 11]
```
For each `p` in `PRIMES`, each `d` in `DEGREES`:
- `cpd = math.comb(p, d)` (0 if `d > p`, never here)
- `frac = Fraction(cpd, p**d)`
- `inv_dfact = Fraction(1, math.factorial(d))`
- emit:
```
{"kind":"exact","p":p,"d":d,"C_p_d":cpd,"p_pow_d":p**d,
 "splitfrac_num":frac.numerator,"splitfrac_den":frac.denominator,
 "splitfrac_dec":float(frac),"inv_d_factorial_dec":float(inv_dfact)}
```

Then emit the **penalty ratios** that the exponent derivation uses, for
`(g, d_hyper=g, d_prym=2g+1)` with `g in {4,5}`, at each `p`:
- `ratio = splitfrac(p, g) / splitfrac(p, 2*g+1)` as a `Fraction`
```
{"kind":"penalty","p":p,"g":g,"d_hyper":g,"d_prym":2*g+1,
 "ratio_num":ratio.numerator,"ratio_den":ratio.denominator,
 "ratio_dec":float(ratio)}
```

### Part B — Monte-Carlo instrument check (POS + AA), small d only

Rationale: expected events for `d>=4` are `< 1` in any feasible `T`, so MC is
run only where it is informative. This part only *validates* the closed form
and the sampler; it does not feed the decision.

Constants:
```
MC_P       = 101
MC_DEGREES = [1, 2, 3]
MC_TRIALS  = 200000
MC_SEEDS   = [20261116, 70261116]
```
For each `d` in `MC_DEGREES`, each `seed` in `MC_SEEDS`:
- `rng = random.Random(seed * 1000 + d)`  (derived seed, recorded)
- repeat `MC_TRIALS` times:
  - draw a monic degree-`d` polynomial: coefficients `c[0..d-1]` each
    `rng.randrange(MC_P)` (leading coeff fixed 1; these are the lower-degree
    coeffs). Polynomial `f(x) = x^d + c[d-1] x^{d-1} + ... + c[0]`.
  - count distinct roots in `F_p`: `roots = sum(1 for x in range(MC_P) if
    eval_poly(f, x, MC_P) == 0)`. `eval_poly` by Horner mod `p`.
  - `complete = (roots == d)`  (degree-`d` monic with `d` distinct roots splits
    completely and is automatically squarefree)
  - increment `n_complete` if `complete`.
- `frac_mc = Fraction(n_complete, MC_TRIALS)`
- emit:
```
{"kind":"mc","p":MC_P,"d":d,"seed":seed,"derived_seed":seed*1000+d,
 "trials":MC_TRIALS,"n_complete":n_complete,
 "frac_mc_dec":float(frac_mc),
 "exact_dec":float(Fraction(math.comb(MC_P,d), MC_P**d))}
```

Determinism: identical `(d, seed)` must reproduce `n_complete` exactly.

Complexity: `|MC_DEGREES| * |MC_SEEDS| * MC_TRIALS * MC_P` Horner steps
`= 3*2*200000*101 ≈ 1.2e8` modular ops — well under the limits.

## File 3 — `src/test_l07.py`

`unittest`. Hand-computed expected values:

1. `genus_table` for `g=4`: `h=5`, `genus_D=9`, `prym_2lp_exp==Fraction(16,9)`,
   `hyper_g_2lp_exp==Fraction(3,2)`, `rho_prym_exp==Fraction(2)`,
   `prym_beats_hyper=="false"` (since `16/9≈1.778 > 3/2`).
2. `genus_table` for `g=5`: `h=6`, `genus_D=11`,
   `prym_2lp_exp==Fraction(20,11)`, `hyper_g_2lp_exp==Fraction(8,5)`,
   `rho_prym_exp==Fraction(5,2)`, `prym_beats_hyper=="false"`.
3. `genus_table` for `g=6`: `dominant=="false"` (Prym map not dominant).
   For `g=5`: `dominant=="true"`.
4. `splitfrac(101, 1) == Fraction(1, 1)` (POS exact: every linear poly splits).
5. `splitfrac(101, 2) == Fraction(5050, 10201) == Fraction(50, 101)`
   (`C(101,2)=5050`, `101^2=10201`). Also `== Fraction(101-1, 2*101)`.
6. `splitfrac(p, d)` decreasing in `d` for fixed `p=1009`, `d=1..11`.
7. `eval_poly` Horner: `f(x)=x^2+1` at `x=10, p=101` equals `101`→`0`? No:
   `100+1=101≡0`, so `eval_poly([1],2,10,101)` representation — test the helper
   directly on `x^2 - 2` style cases with known roots mod a small prime, e.g.
   `x^2 - 1 (mod 7)` has roots `{1,6}` → 2 distinct roots.
8. MC agreement: for `d=1`, every trial is complete (`n_complete==trials`), so
   assert `frac_mc == 1.0` exactly for both seeds. For `d=2`, assert
   `abs(frac_mc - float(splitfrac(101,2))) < 4*sqrt(p2*(1-p2)/T)` with
   `p2=float(splitfrac(101,2))`, `T=MC_TRIALS` (AA + POS tolerance gate).
9. AA: for `d=2`, the two seed streams agree within
   `4*sqrt(p2*(1-p2)/T)` of each other.

Expose the computation as importable functions so tests call them without
spawning subprocesses: `genus_row(g) -> dict`, `splitfrac(p,d) -> Fraction`,
`eval_poly(coeffs, x, p) -> int` (coeffs low-to-high of the **non-leading**
terms, degree inferred by caller), and a `mc_count(p,d,seed,trials) -> int`.

## Output file formats (summary)

- `raw/genus_table.tsv` — TSV, header as above, 6 data rows.
- `raw/split_law.jsonl` — JSON Lines; `kind in {exact, penalty, mc}`.
- `raw/tests.txt` — unittest stderr; must end `OK`.

## DERIVATION.md assembly (executor, after gates pass)

Copy PROTOCOL §1 (geometry, Riemann–Hurwitz, Prym dimension, dominance),
§2 (exponent table), and §3 (cover-construction cost). Insert from the data:

- the `genus_table.tsv` row block for `g=4,5` (exponents exact and decimal);
- the `penalty` ratios `splitfrac(p,g)/splitfrac(p,2g+1)` for `g=4,5` at each
  `p`, stated as the per-trial smoothness penalty of sieving on `D` (genus
  `2g+1`) versus a hyperelliptic genus-`g` model;
- the POS/AA MC confirmation lines (`d=1` frac `=1.0`; `d=2` frac matches
  `50/101`), as evidence the splitting instrument is correct.

State the verdict from PROTOCOL §8 verbatim: **killed** at `g=4` and `g=5`,
because every identified Prym anti-invariant relation family sieves on `D` of
genus `2g+1` and so has 2LP exponent `2 - 2/(2g+1) > 2 - 2/g`, with cover
construction an additional non-polynomial one-time cost. Report the negative
plainly; do not move the threshold. Label every statement `[pcc:unverified]`.

If any correctness gate (PROTOCOL §7) fails, write that the numeric check is
void and that the analytic kill (§2) stands on the degree argument alone.
