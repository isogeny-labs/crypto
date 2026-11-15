# Tower formulas: does any genus make index calculus cheaper than rho on E?

**Label: formula-level, not measured.** No Jacobian arithmetic is done. Every
number below is evaluated from a published cost formula by
`src/bin/tower_formula.rs` and is an unverified assertion `[pcc:unverified]`.
Units are log2 of group-operation equivalents; no speedup is claimed.

Command: `target/release/tower_formula --json tower_formula.json --svg tower_formula.svg > tables.md`.
Outputs: `tower_formula.json` (full g = 1..64 grid per p), `tower_formula.svg`
(log2 cost vs g, p = 2^128 and 2^256, rho as a horizontal line), `tables.md`.

## Question and setup

E/F_p has a subgroup of prime order r about p. A tower of covers C_g -> E
carries the log into J(C_g)(F_p). The DLP subgroup stays of size about p
(about log2 p bits), while the Jacobian has size about p^g (about g log2 p
bits) `[pcc:unverified]`. The log lives in a quotient subgroup of the larger
group, and the formulas below do not shrink with the subgroup: they are priced
on the whole Jacobian (relations over the whole group, then reduction mod r).

Reference: Pollard rho on E with negation map, sqrt(pi r / 4), r about p, i.e.
log2 cost = (log2 p)/2 + 0.5 log2(pi/4) = (log2 p)/2 - 0.174. This matches
`CostModel::log2_rho_elliptic` up to its per-operation weight.

## Formulas and sources

Let n = log2 p, q = p (the genus-g curve is over F_p).

1. **Gaudry basic** (Gaudry, EUROCRYPT 2000): relation collection about g!
   trials per needed relation times a factor base of about p, giving g! p, plus
   sparse linear algebra about p^2 (factor base about p). Evaluated as
   log2( g! p + p^2 ). Dropped: log p factors, the constant on the linear
   algebra (a factor g appears in some write-ups).
2. **Reduced factor base, double large prime** (Theriault, ASIACRYPT 2003, exponent
   2 - 2/(2g+1); Gaudry-Thome-Theriault-Diem, Math. Comp. 76 (2007), exponent
   2 - 2/g): O~(p^(2-2/g)).
   - 2a "bare": exactly (2 - 2/g) n, dropping every g-dependent constant, the
     g! relation term and log factors. This is the optimistic reading and a
     lower bound for the family at fixed g.
   - 2b "CostModel form": g! p + g^2 p^(2-2/g), the form of
     `CostModel::log2_hyperelliptic_ic`
     (`src/cryptanalysis/jacobian_transfer/cost.rs`) without its per-operation
     weight; the g^2 is that model's `hyperelliptic_ic_genus_polynomial_exponent = 2`.
   - Same family and correction as `research/cover_route_prime_binary_20261008/README.md`
     (Correction 2026-10-11: the g! p term must be kept).
3. **Large genus, subexponential** (Enge-Gaudry, Acta Arith. 102 (2002); Enge,
   Math. Comp. 71 (2002), provable; Diem, Math. Comp. 80 (2011)):
   L_N[1/2, c] = exp(c sqrt(ln N ln ln N)), N = p^g, so ln N = g n ln 2, o(1)
   dropped. Evaluated as c sqrt(ln N ln ln N) / ln 2 in log2.
   - Condition: the theorems need g >= theta ln p for a fixed theta > 0
     (grid uses theta = 1, i.e. g >= ln p; theta = 0.5 and 0.25 shown for
     sensitivity). Outside it the expression is not an upper bound for any known
     algorithm. See `research/enge_primefield_cover_20261024/REPORT.md`: with
     d = Theta(log p) the guarantee's exponent exceeds rho's.
   - Constant: c = sqrt(2), taken from the repository's frozen
     `CostModel::enge_gaudry_c`; recalled from Enge-Gaudry, not re-derived or
     re-read here `[pcc:unverified]`. c = 1 is shown as sensitivity.
4. **Weighted columns**: add log2(Jacobian op / elliptic op) from `CostModel`
   (measured Cantor op counts to g = 32, extrapolated g^2.347 above, which the
   model itself says understates the cost). This always adds bits, so it only
   widens the gaps.

Reuse: `CostModel`, `log2_factorial`, `log2_add` are imported from the
repository. Formulas 1 and 3 and the bare 2a are re-implemented explicitly in the
binary because `CostModel` has no basic-Gaudry term and prices Enge-Gaudry
with the Jacobian order; formula 2b equals `CostModel` before weighting.

## Table (g = 1..64 grid; gap = method minus rho, in bits; positive means worse than rho)

#### p = 2^64  (rho on E = 31.83 log2 group ops)

| method | best g | log2 cost | rho | gap (bits) | weighted gap | note |
|---|---|---|---|---|---|---|
| Gaudry basic g!p+p^2 | 2 | 128.0 | 31.8 | +96.2 | +100.2 | g in 2..64 |
| p^(2-2/g) bare | 2 | 64.0 | 31.8 | +32.2 | +36.2 | constants dropped, lower bound on the family |
| g!p+g^2 p^(2-2/g) | 2 | 66.6 | 31.8 | +34.8 | +38.8 | CostModel form |
| L[1/2,sqrt2], g >= ln p | 45 | 251.3 | 31.8 | +219.5 | +233.9 | in-regime, inside g<=64 |
| L[1/2,sqrt2], unconstrained | 1 | 26.5 | 31.8 | -5.4 | -5.4 | INVALID outside g >= ln p |
| best valid overall | 2 | 64.0 | 31.8 | +32.2 | +36.2 | min of the rows above except unconstrained |

- theta=1: regime g0=45; L at g0 = 251.3 (c=1: 177.7); min over g0..32768 = 251.3; gap to rho = +219.5 bits (+145.9 with c=1)
- theta=0.5: regime g0=23; L at g0 = 171.5 (c=1: 121.3); min over g0..32768 = 171.5; gap to rho = +139.7 bits (+89.5 with c=1)
- theta=0.25: regime g0=12; L at g0 = 117.9 (c=1: 83.4); min over g0..32768 = 117.9; gap to rho = +86.1 bits (+51.6 with c=1)

#### p = 2^128  (rho on E = 63.83 log2 group ops)

| method | best g | log2 cost | rho | gap (bits) | weighted gap | note |
|---|---|---|---|---|---|---|
| Gaudry basic g!p+p^2 | 2 | 256.0 | 63.8 | +192.2 | +196.2 | g in 2..64 |
| p^(2-2/g) bare | 2 | 128.0 | 63.8 | +64.2 | +68.2 | constants dropped, lower bound on the family |
| g!p+g^2 p^(2-2/g) | 2 | 130.6 | 63.8 | +66.8 | +70.8 | CostModel form |
| L[1/2,sqrt2], g >= ln p | none | n/a | 63.8 | n/a | n/a | in-regime, inside g<=64 |
| L[1/2,sqrt2], unconstrained | 1 | 40.7 | 63.8 | -23.1 | -23.1 | INVALID outside g >= ln p |
| best valid overall | 2 | 128.0 | 63.8 | +64.2 | +68.2 | min of the rows above except unconstrained |

- theta=1: regime g0=89; L at g0 = 543.1 (c=1: 384.0); min over g0..32768 = 543.1; gap to rho = +479.3 bits (+320.2 with c=1)
- theta=0.5: regime g0=45; L at g0 = 371.2 (c=1: 262.5); min over g0..32768 = 371.2; gap to rho = +307.4 bits (+198.7 with c=1)
- theta=0.25: regime g0=23; L at g0 = 254.4 (c=1: 179.9); min over g0..32768 = 254.4; gap to rho = +190.6 bits (+116.1 with c=1)

#### p = 2^192  (rho on E = 95.83 log2 group ops)

| method | best g | log2 cost | rho | gap (bits) | weighted gap | note |
|---|---|---|---|---|---|---|
| Gaudry basic g!p+p^2 | 2 | 384.0 | 95.8 | +288.2 | +292.2 | g in 2..64 |
| p^(2-2/g) bare | 2 | 192.0 | 95.8 | +96.2 | +100.2 | constants dropped, lower bound on the family |
| g!p+g^2 p^(2-2/g) | 2 | 194.6 | 95.8 | +98.8 | +102.8 | CostModel form |
| L[1/2,sqrt2], g >= ln p | none | n/a | 95.8 | n/a | n/a | in-regime, inside g<=64 |
| L[1/2,sqrt2], unconstrained | 1 | 52.1 | 95.8 | -43.8 | -43.8 | INVALID outside g >= ln p |
| best valid overall | 2 | 192.0 | 95.8 | +96.2 | +100.2 | min of the rows above except unconstrained |

- theta=1: regime g0=134; L at g0 = 852.5 (c=1: 602.8); min over g0..32768 = 852.5; gap to rho = +756.6 bits (+506.9 with c=1)
- theta=0.5: regime g0=67; L at g0 = 581.0 (c=1: 410.9); min over g0..32768 = 581.0; gap to rho = +485.2 bits (+315.0 with c=1)
- theta=0.25: regime g0=34; L at g0 = 398.2 (c=1: 281.6); min over g0..32768 = 398.2; gap to rho = +302.4 bits (+185.7 with c=1)

#### p = 2^256  (rho on E = 127.83 log2 group ops)

| method | best g | log2 cost | rho | gap (bits) | weighted gap | note |
|---|---|---|---|---|---|---|
| Gaudry basic g!p+p^2 | 2 | 512.0 | 127.8 | +384.2 | +388.2 | g in 2..64 |
| p^(2-2/g) bare | 2 | 256.0 | 127.8 | +128.2 | +132.2 | constants dropped, lower bound on the family |
| g!p+g^2 p^(2-2/g) | 2 | 258.6 | 127.8 | +130.8 | +134.8 | CostModel form |
| L[1/2,sqrt2], g >= ln p | none | n/a | 127.8 | n/a | n/a | in-regime, inside g<=64 |
| L[1/2,sqrt2], unconstrained | 1 | 61.8 | 127.8 | -66.0 | -66.0 | INVALID outside g >= ln p |
| best valid overall | 2 | 256.0 | 127.8 | +128.2 | +132.2 | min of the rows above except unconstrained |

- theta=1: regime g0=178; L at g0 = 1167.1 (c=1: 825.3); min over g0..32768 = 1167.1; gap to rho = +1039.3 bits (+697.5 with c=1)
- theta=0.5: regime g0=89; L at g0 = 797.2 (c=1: 563.7); min over g0..32768 = 797.2; gap to rho = +669.4 bits (+435.9 with c=1)
- theta=0.25: regime g0=45; L at g0 = 546.5 (c=1: 386.4); min over g0..32768 = 546.5; gap to rho = +418.7 bits (+258.6 with c=1)

#### p = 2^521  (rho on E = 260.33 log2 group ops)

| method | best g | log2 cost | rho | gap (bits) | weighted gap | note |
|---|---|---|---|---|---|---|
| Gaudry basic g!p+p^2 | 2 | 1042.0 | 260.3 | +781.7 | +785.7 | g in 2..64 |
| p^(2-2/g) bare | 2 | 521.0 | 260.3 | +260.7 | +264.7 | constants dropped, lower bound on the family |
| g!p+g^2 p^(2-2/g) | 2 | 523.6 | 260.3 | +263.3 | +267.3 | CostModel form |
| L[1/2,sqrt2], g >= ln p | none | n/a | 260.3 | n/a | n/a | in-regime, inside g<=64 |
| L[1/2,sqrt2], unconstrained | 1 | 94.1 | 260.3 | -166.2 | -166.2 | INVALID outside g >= ln p |
| best valid overall | 2 | 521.0 | 260.3 | +260.7 | +264.7 | min of the rows above except unconstrained |

- theta=1: regime g0=362; L at g0 = 2532.0 (c=1: 1790.4); min over g0..32768 = 2532.0; gap to rho = +2271.7 bits (+1530.1 with c=1)
- theta=0.5: regime g0=181; L at g0 = 1736.9 (c=1: 1228.2); min over g0..32768 = 1736.9; gap to rho = +1476.6 bits (+967.9 with c=1)
- theta=0.25: regime g0=91; L at g0 = 1192.8 (c=1: 843.4); min over g0..32768 = 1192.8; gap to rho = +932.5 bits (+583.1 with c=1)

"weighted gap" adds the Jacobian-versus-elliptic operation weight. The
"unconstrained" row is the L formula evaluated at g = 1, where it is not valid.

## Where the subexponential regime starts

Regime start g0 = ceil(theta ln p) for the 5 sizes (theta = 1 / 0.5 / 0.25):
2^64: 45 / 23 / 12; 2^128: 89 / 45 / 23; 2^192: 134 / 67 / 34;
2^256: 178 / 89 / 45; 2^521: 362 / 181 / 91 `[pcc:unverified]`.
Inside g <= 64 only p = 2^64 (theta = 1) reaches it, plus the smaller-theta
variants at the smaller sizes. At g0 the formula is already above rho in every
case (gaps from +51.6 bits for p = 2^64, theta = 0.25, c = 1 up to +2271.7
bits for p = 2^521, theta = 1), and a scan of g0..32768 finds the minimum at g0
because the expression increases with g `[pcc:unverified]`. So the
subexponential regime never beats rho in the computed range.

Caveat that matters: the L expression evaluated at g = 1 (outside any
hypothesis) is below rho for every p (for example 61.8 vs 127.8 at 2^256). That
reflects the dropped o(1) and an invalid regime, not an algorithm: no
subexponential algorithm is known for g = 1 or small fixed g. Counting it
would be a mistake `[pcc:unverified]`.

## Conclusion

- For every p, the best valid genus is g = 2, and its cost is at best the
  bare exponent p^1, which is (log2 p)/2 + 0.17 bits above rho; with the
  g! p and g^2 terms it is 2.6 bits more `[pcc:unverified]`.
- Gaudry basic is 192.2 bits above rho at 2^128 and 384.2 bits above at 2^256
  `[pcc:unverified]`.
- Past g = 2 the exponent 2 - 2/g only rises toward 2, and g! p grows with it,
  so the cost rises with genus. This agrees in direction with the toy
  measurements in `../tower/TOWER.md` and `../RESULTS.md` `[pcc:unverified]`.
- The gap to rho at the best genus is +32.2, +64.2, +96.2, +128.2 and +260.7 bits
  for p = 2^64, 2^128, 2^192, 2^256 and 2^521 (bare formula, the most
  favourable one), i.e. half the key size `[pcc:unverified]`.
- No genus 1..64, and no genus in the valid large-genus regime up to 32768,
  makes the published index-calculus cost lower than rho on E at these sizes
  `[pcc:unverified]`.

## Caveats

- Formula-level only; constants are dropped as listed, o(1) terms are dropped.
- Asymptotic bounds are not exact costs for finite p; the bare 2a row is
  deliberately favourable to index calculus.
- Constant c = sqrt(2) and the theta range come from memory and the repo model,
  not a fresh read of the papers.
- Cover existence is not modelled: a genus-g cover of a given E with the
  subgroup transferring may not exist (see `../ATLAS.md`), which only makes the
  picture worse for the tower.
- Not a benchmark run: no `bench.record/v1` record applies.
