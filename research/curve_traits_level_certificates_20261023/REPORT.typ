#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= Stored certificates for the last depth-1 End(E) levels

_curve\_traits, branch `claude/curve-traits-level-certs`, 2026-10-23. Data: `certs.jsonl` (the certificates), `certificates_table.json`, `verify_timings.txt` (the build's own stage timings); graph: `plot.py`._

== What changed
Nineteen depth-1 volcano levels on $"GF"(p^k)$ curves stayed open because the depth-1 point test needs points of $E(op("GF")(q^m))$ with $m$ up to 239, beyond any build budget. The points are now searched once, offline, by `search.gp` (PARI/GP, fast field arithmetic), and stored in `docs/curves/level-certificates.json`. The build only checks them (`points.rs`, `verify_level_certificate`): $g$ irreducible over $"GF"(q)$ by Ben-Or's test, the twisting element a non-square through its norm, the group order from the trace, each point on the curve, and its order. A point of order $ell^v$ with $ell^v || N$ proves the $ell$-part cyclic, the floor. A certificate that fails stops the build.

Two binary curves were open for a different reason, and the isogeny walks now settle them:
- $"GF"(2^61)$, $ell = 31543$: class-group orders are now exact up to $|Delta| < 2^80$ (was $2^60$). A cycle of 17934 13-isogenies then fits $op("Cl")(cal(O)_(7^3))$ and not $op("Cl")(cal(O)_(7^3 dot 31543))$, so this is the crater.
- $"GF"(2^47)$, $ell = 31$: open walks are now lengthened to the exact class orders of the surviving orders rather than their class numbers. No 5-isogeny walk returns within 22936 steps, the order of the class in $op("Cl")(cal(O)_31)$, so this is the floor.
PARI's `qfbpow` gives the same orders, and the tests pin both curves.

#figure(image("verify_time.png", width: 85%), caption: [Seconds the build spends checking each certificate, against the degree of its field over GF(p). Stage timings from the build log; times under 0.5 s are not logged and are drawn at 0.5.])

== Results (counts from the two `traits.json` files, no sampling)
Against `main`: 21 levels change, all from open to decided (19 certificates, all floors; 2 binary by walks). No decided level changes. Open levels: 34 → 13. The 13 are the $ell = 2, 3$ prime-power levels that \#2428 decides, so with both merged no level is open. Build: 28.4 CPU-minutes, 10.6 minutes wall on the 4-core container (`main`: 22.0). Of the 6.4-minute difference, 2.4 CPU-minutes are certificate checks, at most 32 s for one (ℓ = 479, degree 1434 over GF(p)).

== Scope
Every verdict is checked in the build from the stored data alone; the search script is not trusted. The 14 curves with unfactored $Delta$ cofactors (112–152 digits) are untouched here.
