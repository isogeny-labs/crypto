# Protocol addendum: the "isogeny" base (frozen before its data)

2026-11-11. Follow-up to [`PROTOCOL.md`](PROTOCOL.md) / [`README.md`](README.md), which left the
"isogeny" base untested because its point set comes from Rust code. This addendum tests it with
the real code: a scratch program calls `crypto_lib::cryptanalysis::ic_framework::prime_tower::isogeny_tower(p, t, seed, ..)`
exactly as `src/bin/curve25519_family.rs` does (same p, seed 0) and prints V = x(R' + <Q>).
The base on the miniature is every point whose Weierstrass abscissa is in V
(`PrimeIsogenyBase`), both signs; its descent class is delta = chi(x_W − A/3) as before.

## Setting

- Miniature as in PROTOCOL.md: p = 1048573, A = 926 (rebuilt there and checked). Here the shift
  is A/3 mod p, as in `descent_class.py`.
- Cells of the oracle study: isogeny t ∈ {4, 5, 6}, seed 0 (the original measurement).
- Supplementary, unregistered: t = 9 (to match the supplementary run), and seeds 1 to 20 at t = 5, 6
  (the seed picks the auxiliary curve), to see whether the class mix depends on the seed.

## Question and decision rule

For each (t, seed): among the V-abscissae that lie on the miniature, the number with delta = +1 and
delta = −1.

- **Premise holds** (a fixed-class filter exists) only if every on-curve base abscissa has the same
  delta in all three registered cells.
- **Premise fails** if both classes are at least 10% in the registered cells (t = 4, 5, 6, seed 0).
  Cells with fewer than 8 on-curve abscissae are reported but cannot count for or against.

## Out of scope

No F4 cost, no hit counts (the base is small), no claim about other bases. If the tower code
cannot be built here, the check is not done and the README says so.
