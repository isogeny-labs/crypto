# PROTOCOL: genus-3 GHS covers of binary curves over F_{2^{3l}} (n = 3)

Frozen 2026-10-24, before any data was taken (hash of this file recorded in
README.md). Scope: toy fields F_{2^N}, N = 3l, l = 2..7. This is a structural
census. It makes no claim about any deployed curve and no cost or speed claim.
Every number in this study is labelled DERIVED (from the algebra below) or
MEASURED (computed by the tool in `tool/census.rs`).

Curve family: E_{a,b}: y^2 + xy = x^3 + a x^2 + b over K = F_{2^N}, b != 0.
Tower F_2 < k = F_{2^l} < K, n = [K:k] = 3, sigma(x) = x^{2^l} (order 3 on K).
Code under test: `ec_trapdoor::{magic_number, ghs_type_i, ghs_genus_with_type,
mmt_condition_8, absolute_trace, relative_trace}` (library, unmodified).

## 1. Derivation (Q1): module structure of K over F_2[sigma]

sigma^3 = 1 on K, so K is a module over R = F_2[x]/(x^3 - 1). Over F_2,
x^3 - 1 = (x + 1)(x^2 + x + 1) with coprime irreducible factors, so
R = F_2 x F_4 and K = K1 + K2 (direct sum of F_2[sigma]-modules) with

* K1 = ker(sigma - 1) = k, dim_F2 = l. sigma acts as 1 (x + 1 kills it).
* K2 = ker(sigma^2 + sigma + 1) = ker Tr_{K/k}, dim_F2 = 2l (Tr_{K/k} =
  1 + sigma + sigma^2 is onto k, so its kernel has dimension 3l - l). It is an
  F_4-vector space (sigma acts as a primitive cube root of unity), so every
  nonzero v in K2 has annihilator exactly x^2 + x + 1.

The idempotent projection onto K1 is Tr_{K/k} (3 = 1 in characteristic 2).
Write sqrt(b) = u + v, u = Tr_{K/k}(sqrt b) = sqrt(Tr_{K/k} b) in k, v in K2.
The sigma-annihilator of sqrt(b) (MMT's Ord_b) is therefore

    Ord_b = (x + 1)^{[u != 0]} * (x^2 + x + 1)^{[v != 0]}.

Squaring is a bijection of K, so counts over sqrt(b) equal counts over b. MMT
Theorem 5(i) (as implemented): m = deg Ord_b + 1 if (x-1) does not divide
Ord_b ("type I", genus 2^{m-1} - 1), else m = deg Ord_b ("type II", genus
2^{m-1}).

| class | condition on b | Ord_b | plain dim | type | m | genus | count (DERIVED) |
|---|---|---|---|---|---|---|---|
| A | b in k* | x+1 | 1 | II | 1 | 1 | 2^l - 1 |
| B | Tr_{K/k}(b) = 0, b != 0 | x^2+x+1 | 2 | I | 3 | 3 | 2^{2l} - 1 |
| C | b not in k, Tr_{K/k}(b) != 0 | x^3+1 | 3 | II | 3 | 4 | (2^l - 1)(2^{2l} - 1) |

Total 2^{3l} - 1. Consequences: m in {1, 3} only (m = 2 and m >= 4 never
occur for n = 3); genus 3 holds exactly when Tr_{K/k}(b) = 0, b != 0; the genus-3
fraction is (2^{2l}-1)/(2^{3l}-1), about 2^{-l}. Check against the toy census
(l = 4): 15, 255, 3825; 255/4095 = 6.23%. The fraction halves with each
increment of l (l=7: 0.78%).

Prediction P1: the library's (m, type, genus) tally over all b in K* equals the
table for every l = 2..7 exactly (zero deviations); class B equals the set
{Tr_{K/k}(b) = 0}; class A equals the set {b in k}.

## 2. Q2: MMT condition (8)

`mmt_condition_8` = (n odd) or (Tr_{K/F2}(a) = 0) or (Tr_{K/k}(b) != 0).
n = 3 is odd, so it is true for every (a, b).
Prediction P2: 100.000% of genus-3 b satisfy it, for a = 0 and for a with
Tr_{K/F2}(a) = 1 (a = smallest integer-encoded element with absolute trace 1).
This is derived, not informative about the cover; the descent obstruction
exists only for even n. Detector positive control (P2c): on towers with
n = 2 (l = 3, 4, 5; N = 6, 8, 10) and Tr(a) = 1, the fraction of b in K* with
condition (8) true equals the fraction with Tr_{K/k}(b) != 0, which is
(2^{2l} - 2^l)/(2^{2l} - 1) exactly (DERIVED); the tool must reproduce it, and
for a = 0 it must be 100%.

## 3. Q3: large prime subgroup

Point count (MEASURED, exhaustive over x): put y = xz; for x != 0,
z^2 + z = x + a + b/x^2 has 2 solutions iff Tr(x + a + b x^{-2}) = 0, else 0.
#E(K) = 2 + 2 * #{x in K*: Tr(x) + Tr(a) + Tr(b x^{-2}) = 0}. (The 2 counts
the point at infinity and (0, sqrt b).) Tables: a primitive polynomial is found
by search, exp/log tables, trace bit table. Checks: Hasse bound on every count;
#E even (point (0, sqrt b)); #E = 0 mod 4 iff Tr(a) = 0; #E(a) + #E(a') =
2*2^N + 2 for Tr(a)=0, Tr(a')=1 (twist); count invariant under b -> b^2.

**Deviation from the brief, stated up front.** "r does not divide
#E(F_{2^l})" needs E to be defined over k. A class-B curve has b not in k (its
b is in k only if b = 0), so E has no model over k and #E(k) does not exist; the
trace-kills-subgroup mechanism for subfield curves is not applicable. The
literal criterion is evaluated only where defined (arm D below). For the
class-B curves the evaluated quantities are:

* S1 (MEASURED): r_max = largest prime factor of #E(K); "shaped" iff
  r_max > sqrt(#E(K)).
* S2 (MEASURED): cofactor h = #E/r_max <= 4 (NIST-style) — the stricter
  reading, only possible if #E = 0 mod 4 or r_max = #E/2.
* GHS context (DERIVED, not computed): for n = 3, g = 3 the cover C/k has
  Jac(C) isogenous to Res_{K/k}E, so #Jac(C)(k) = #E(K); a prime r | #E(K)
  therefore also lives in a genus-3 Jacobian over k. Whether the descended
  cover exists as a smooth model was not constructed here.

Arms (for each l = 4..7, plus l = 2, 3 as extras; a in {0, a1 with Tr = 1}):
* B: all class-B b (b in K*, Tr_{K/k}(b)=0). Reported over all b (weighted)
  and over Frobenius-orbit representatives (min of log b under log -> 2 log, one
  per orbit; the effective independent sample), with Wilson 95% intervals on
  orbit representatives.
* G (negative control, generic curves): 20000 b uniform on K* (seed fixed in
  source), same a values. Tests whether class B differs from a generic curve.
* I (null integers): 20000 integers uniform in the Hasse interval
  [2^N+1-2*floor(sqrt(2^N)), 2^N+1+2*floor(sqrt(2^N))] with the residue
  of #E mod 4 of the matching arm (0 for a = 0, 2 for Tr(a) = 1). What a
  structureless count would give.
* D (literal criterion, contrast): b in k*, a in k with Tr_{K/F2}(a) matching;
  #E(k) counted over k; "shaped-literal" iff some prime r | #E(K), r does not
  divide #E(k), r > sqrt(#E(K)).

Prediction P3 (heuristic, Dickman-type: for n with a fixed power 2^v,
P(r_max > sqrt(n)) = ln(1/u), u = log sqrt(n) / log(n/2^v), about 0.55-0.65 for
l = 4..7): arm B's S1 fraction is statistically indistinguishable from arm G
and arm I (|difference| < 2 pooled standard errors at each l); no structure of
the cover forces or forbids a large prime factor. A difference larger than
that rejects P3. Expected surviving class-B curves per l = fraction * (2^{2l}-1).
"Does the 6% survive": the answer is the count above, a fraction ~2^{-l} of
all curves times ~0.6.

## 4. Controls

* Positive control 1 (Q1): every b in k has m = 1 (class A tally = 2^l - 1, all
  m = 1, genus 1); the l = 4 tally reproduces 15 / 255 / 3825.
* Positive control 2: condition-8 detector on n = 2 (section 2, P2c).
* Cross-check: >= 50 random rows (l = 3..7, all classes, both a) are also run
  through the built `target/release/ghs_screen` binary (`--degree N --modulus
  hex --a --b`); magic number, genus, type and condition (8) of its n = 3 row
  must equal the tool's library calls, 0 mismatches.
* Negative control: arms G and I above.

## 5. Method and limits

Implementation: `tool/census.rs`, standalone rustc binary linked against the
built `crypto_lib` rlib (no edits to src/, examples/, Cargo.toml). Rust only;
the only Python is the Typst PDF renderer wheel (document layout, no results).
Single thread. The primitive polynomial per N is the numerically smallest
(lowest-bits-first search) for which z has multiplicative order 2^N - 1.
Counts do not depend on the choice of modulus (field isomorphism), but the
bit patterns of specific b do; the cross-check uses the same modulus.
Limits: toy fields; #E up to 2^21 only; fractions near 0.6 at these sizes are
not extrapolated to cryptographic N; no DLP cost computed.
