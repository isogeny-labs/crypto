# Genus-3 transfer gates, leads #3 and #5: results (2026-11-20)

Standing: every number here is `[pcc:unverified]`: one session, a shared host, no
independent replay, no pcc claim filed. Protocol: [`PROTOCOL.md`](PROTOCOL.md), frozen
before data. `frozen.sha256` holds PROTOCOL.md, census.py as run and theta_arm.py, and
census.py was hashed before the census. Data hashes: `data.sha256`. Literature:
[`LITERATURE.md`](LITERATURE.md). Total compute: about 15 minutes single-threaded
(gates 1m47s, census 3m51s, theta arms about 9 minutes).

## Verdicts

| Lead | Gate | Outcome |
|---|---|---|
| #3 | Is the codomain of an affine-type (2,2,2)-isogeny hyperelliptic, a plane quartic, or curve-dependent? | **Literature: open.** The reachable summaries cover pair type only. **Numerically (over C): generically a plane-quartic Jacobian.** 0 of 48 affine-type codomains, and 0 of all 135 kernels on one curve, have a vanishing even theta-null. A positive control finds vanishing ones where expected. Rationality: exactly 2 affine-type Lagrangians are F_p-stable when `#J` is odd (odd model), with kernel over `F_{p^7}`. Confirmed on 1,200/1,200 prime-order curves. |
| #5 | `l = 3` kill rule: fewer than 5% of prime-order curves have a rational `l = 3` Lagrangian with `k <= 4` | **Frozen rule output: INCONCLUSIVE.** Arm A: `R_low` = 18/600 = 3.0% [1.9, 4.7], `R_high` = 24/600 = 4.0% [2.7, **5.9**]. The upper bound crosses 5%. Both point estimates are below 5%. **The rate depends on `p mod 3`: it is provably 0 for `p = 2 mod 3` and about 6-8% for `p = 1 mod 3`.** |

## Lead #5: census

### Instrument

- **G1:** 180/180 curves pass (integral `e2, e3`, Weil bounds), and every census curve
  passes too (0 failures in 3,678 irreducible draws).
- **G2:** `N_4`, counted directly over `F_{p^4}`, equals the value predicted from the
  charpoly on 10/10 curves at p = 13 and 10/10 at p = 17. This is an independent check
  of `N_1, N_2, N_3` and of the Newton step (`gates.json`).
- **P3 control (l = 2):** on 600/600 arm-A and 600/600 arm-B prime-order curves,
  `P mod 2 = (T^3+T+1)(T^3+T^2+1)`, with exactly 2 stable Lagrangians, each with `k = 7`.

### Census

Arm A: 600 prime-order curves, 100 at each `p` in {29, 31, 37, 41, 43, 47}, drawn from
25,456 random monic septics (3,678 irreducible).

Arm B: the 600 PRIME curves of the L06 census (p = 257 to 4099), reclassified from their
recorded charpolys. Arm B is a cross-check only and carries no verdict.

`l = 3`, threshold `k <= 4`:

| arm | YES (decided) | AMBIG | NO (decided) | `R_low` [Wilson 95] | `R_high` [Wilson 95] |
|---|--:|--:|--:|---|---|
| A (verdict) | 18 | 6 | 576 | 0.030 [0.019, 0.047] | 0.040 [0.027, 0.059] |
| B (check) | 17 | 0 | 583 | 0.028 [0.018, 0.045] | 0.028 [0.018, 0.045] |

By `p mod 3`. This breakdown was not a preregistered statistic; the mechanism behind it
(P1, P2) was:

| stratum | arm A YES / AMBIG / N | arm B YES / AMBIG / N | rate (YES) |
|---|---|---|---|
| `p = 2 mod 3` (29, 41, 47; 257, 521, 1031) | 0 / 0 / 300 | 0 / 0 / 360 | 0 (theorem, P1) |
| `p = 1 mod 3` (31, 37, 43; 2053, 4099) | 18 / 6 / 300 | 17 / 0 / 240 | A 0.060 [0.038, 0.093] (0.080 with AMBIG); B 0.071 [0.045, 0.111]; pooled 35/540 = 0.065 [0.047, 0.089] |

Both predictions written before data held on every curve:

- **P1:** no prime-order curve with `p = 2 mod 3` has a stable `l = 3` Lagrangian with
  `k <= 4`. 0 of 660 do.
- **P2:** at `p = 1 mod 3`, the only non-NO patterns are `Pbar = (T+1)^2 (T^2+1)^2`,
  which is decided YES with `k = 4` (35 curves), and `Pbar = (T+1)^6`, which is AMBIG
  (6 curves). All 6 have `27 | #J(F_{p^2})`, so the integral necessary condition does
  not exclude them.

P1 is a theorem for every prime-order `J` (indeed every `J` with `3 ∤ #J`) at
`p = 2 mod 3`:

- The roots of order dividing 4 in `F_9` are `±1` and `±i`.
- `(T+1)^* = T - 1`, and `T - 1` cannot divide `Pbar` because `P(1) = #J ≢ 0 mod 3`.
- `T^2 + 1` alone has even degree, so it cannot make up the degree-3 factor `h`.
- A non-semisimple `pi|L` with `k = 3` would need eigenvalue 1, which is excluded the
  same way.

**Reading.** For a fixed target prime, the `l = 3`, `k <= 4` route:

- does not exist at all when `p = 2 mod 3`;
- exists on about 6-8% of prime-order curves when `p = 1 mod 3`. Those curves have a
  rational `l = 3` kernel whose points are defined over `F_{p^4}` (`Pbar = (T+1)^2 (T^2+1)^2`).

The frozen rule pools primes of both classes, so its value mostly reflects the share of
`p = 1 mod 3` primes in the sample. Under the frozen rule the gate is INCONCLUSIVE at
N = 600. It does not pass. The point estimate (3-4% pooled) is below the 5% line.

The cheapest way to decide the gate as frozen:

1. Resolve the 6 AMBIG curves. `pi = -u` with `u` unipotent. A Lagrangian with `pi = -1`
   on it exists iff `ker(pi + 1) = J^tw(F_p)[3]` contains a Lagrangian. That needs the
   3-torsion of the twist and its Weil pairing, not the charpoly.
2. Or enlarge the sample under a new freeze with a sequential-test correction.

Whether the route is worth anything also depends on the codomain type of the
`(3,3,3)`-quotient, which this study does not address. Tian, arXiv 2007.03172, treats
`(l,l,l)`-isogenies to non-hyperelliptic Jacobians.

Other `l` (arm A; YES / AMBIG of 600; arm B in `census_summary.json`):

| l | stable Lagrangian exists (YES / AMBIG) | `k <= 4` | `k <= 6` | `k <= 8` | `k <= 13` | `k_min` distribution, decided YES |
|---|---|---|---|---|---|---|
| 2 | 600 / 0 | 0 | 0 | 600 | 600 | {7: 600} |
| 3 | 195 / 25 | 18 / 6 | 18 / 25 | 45 / 25 | 159 / 25 | {4: 18, 8: 27, 13: 114, 26: 36} |
| 5 | 187 / 21 | 0 / 11 | 8 / 11 | 24 / 11 | 40 / 15 | {6: 8, 8: 16, 12: 16, 24: 18, 31: 54, 62: 52, 124: 23} |
| 7 | 192 / 17 | 3 / 1 | 4 / 9 | 6 / 9 | 20 / 9 | 14 values from 4 to 342; 65 at 171 and 17 at 342 |

About a third of prime-order curves carry a rational `l`-Lagrangian for each odd
`l <= 7`. This is consistent in size with L06's coprime-split criterion (167/600 at
`l = 3`). For most of them `k` is large: 13 or 26 at `l = 3`, 31 to 124 at `l = 5`, up
to 342 at `l = 7`.

### What the charpoly decides, and where it does not (method limits)

- **Squarefree `Pbar`** (384/600 at `l = 3`): `pi` is semisimple, the module and the
  symplectic structure are determined, and the stable Lagrangians are exactly the sums
  of one half of each dual pair `{f, f^*}`. Their number is `2^(#pairs)`, or 0 if a
  self-dual factor is present. `k` = lcm of root orders. Decided exactly.
- **Repeated factors:** the charpoly sees only the semisimplification.
  - Decided cases: a self-dual factor of multiplicity 1 means no Lagrangian. Pair
    blocks of multiplicity 1 and self-dual blocks of multiplicity 2 have the same
    stable-Lagrangian orders under every module structure. If no `h` with
    `h h^* = Pbar` gives a possible `k` (with `l^3 | #J(F_{p^k})`), the answer is no.
  - Everything else is AMBIG: for example `(T+1)^6`, pair blocks of multiplicity
    `>= 2`, and odd multiplicity `>= 3`. Deciding these needs the actual action of `pi`
    on `J[l]` (torsion points over `F_{p^k}`, and the Weil pairing for isotropy).
  - Non-semisimplicity can multiply `k` by `l`. The AMBIG bounds use the smallest
    possible `k`.
- **Small primes:** arm A uses `p <= 47`. Arm B (p up to 4099) gives the same structure
  and rates within the intervals. The rates depend on `p mod l`, the multiplier class.
- **Lagrangian means maximal isotropic for the Weil pairing on `J[l]`:** existence of
  a rational kernel, not of a computable isogeny, and nothing about the codomain.

## Lead #3: numerical arm T (theta-nulls over C)

Instrument gate T0 passed on all 3 random curves (seeds 1, 2, 3):

- `|tau - tau^T|` is at most 1.7e-29.
- Quadrature at 24 and 48 Gauss nodes per segment agrees to 3e-29.
- `J` has exactly one vanishing even theta-null, relative size 2.7e-17, 2.0e-17 and
  1.1e-16. The next smallest are 0.13, 0.085 and 0.14.

The vanishing characteristic `[111;101]` fixes `q_0`. The 3 a-cycle half-periods,
which are differences of two Weierstrass points, have `q_0 = 1`, as they must.

| curve | affine-type codomains | pair-type codomains | smallest relative even theta-null, affine / pair |
|---|---|---|---|
| seed 1 | 16 plane quartic | 49 plane quartic | 8.3e-3 / 8.1e-3 |
| seed 2 | 16 plane quartic | 49 plane quartic | 9.1e-3 / 1.4e-2 |
| seed 3 | 16 plane quartic | 49 plane quartic | 1.9e-3 / 4.8e-3 |

All 135 kernels on seed 1, checked as 8 partial inversions x 64 graphs = 512
codomains with repetition (added check D2): 512 plane quartic, 0 with any vanishing
even theta-null.

Positive control (added, D2): a curve with branch points stable under `x -> -x` and
`x -> 16/x`, so `J` is isogenous to a product of elliptic curves. Its own `J` is
hyperelliptic (1 vanishing). Over the 512 codomains, 21 have 6 vanishing theta-nulls
(`E x A_2`) and 3 have 9 (`E_1 x E_2 x E_3`). So the detector does see vanishing
theta-nulls on the codomain side.

**Decision (frozen rule):** "generically plane quartic". The affine-type
`(2,2,2)`-quotient of a generic genus-3 hyperelliptic Jacobian is the Jacobian of a
smooth plane quartic. This is evidence over C for the generic member of the family, not
a theorem. It says nothing about which `F_p`-curves lie on the special closed subset,
and nothing about Serre's twist obstruction over `F_q`. It does remove the "maybe it
is hyperelliptic" kill branch of lead #3. What remains open:

- an efficient construction of the affine-type isogeny, with kernel points over
  `F_{q^7}`;
- the twist question;
- the destination measurement (rank #1).

## Deviations

- **D1.** The arm-B loader in `census.py` had the wrong sign convention for L06's
  `s1, s3`. All 600 rows failed the built-in convention check, so no wrong data was
  produced. It was fixed to `chi = [p^3, -p^2 s1, p s2, -s3, s2, -s1, 1]` (L06
  SPEC.md line 196) after the arm-A run. This is the one-line diff; arm A code is
  unchanged. Hash of the census.py as run for arm A: `frozen.sha256`. Current hash:
  `data.sha256`.
- **D2.** Added after the frozen arm-T run, not in the protocol: `theta_control.py`
  and `theta_all135.py`, which reuse `theta_arm.py` unchanged. The first control (65
  kernels only) found no decomposable codomain on the symmetric curve, because the
  split kernels lie outside those 65. The all-135 sweep found them.
- **D3.** The census and the theta arm ran at the same time for about 4 minutes: two
  single-threaded processes on a shared 4-vCPU host. Nothing here is timed.

## Files

`PROTOCOL.md`, `frozen.sha256`, `census.py`, `gates.json`, `census_out.jsonl`,
`census_log.json`, `armb_out.jsonl`, `census_summary.json`, `theta_arm.py`,
`theta_out.json`, `theta_run.log`, `theta_control.py`, `theta_control_out.json`,
`theta_all135.py`, `theta_all135_{control,random}.log`, `LITERATURE.md`,
`RESULTS.md`, `data.sha256`.
