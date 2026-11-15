#set page(margin: 2cm)
#set text(size: 10pt)
= Open-addressing hash and compiled square root in the plane-quartic IC
_Study ic\_gw\_fasthash\_20261114. All numbers \[pcc:unverified\], L0 local run (Iso Lab EKS budget exceeded, owner-chosen local route)._

*Hypothesis.* Open-addressing tables, no per-candidate allocation and prefetch in `find\_relations\_2lp` (HASH), and the same plus the
compiled Barrett square root (COMBO), lower `relations.cpu` at identical counted work (c#sub[D] = 1.7, k = 18..22, seeds 1-3, two passes, 94 units).

*Gate.* 79 unit comparisons, 237 records, 0 counted mismatches; `fastcheck` 0 mismatches over 200,000 inputs per size.

#image("fasthash.png", width: 100%)

*Decision (frozen rule).* HASH: *not supported* (A/A floor 20.7% from one slow REF unit; pass-mean ratio 0.594..0.923).
COMBO: *supported* (pass-mean ratio 0.430..0.561, every cell and pass below 1 minus the floor).

#table(columns: 3, align: left,
[q], [HASH relations.cpu / REF, seeds 1/2/3], [COMBO relations.cpu / REF, seeds 1/2/3],
[262111], [0.859 / 0.643 / 0.808], [0.521 / 0.445 / 0.517],
[524287], [0.702 / 0.676 / 0.705], [0.520 / 0.555 / 0.561],
[1048507], [0.629 / 0.923 / 0.635], [0.499 / 0.492 / 0.509],
[2097091], [0.603 / 0.601 / 0.627], [0.477 / 0.430 / 0.496],
[4194271], [0.594 / 0.623 / 0.671], [0.455 / 0.462 / 0.470],
)

*Secondary.* Total CPU / REF at q = 4194271: HASH 0.858, COMBO 0.796. Peak RSS ratio at q = 4194271: HASH 1.000, COMBO 1.000.
Counted work is identical, so the gap to the matched rho is unchanged; this is a time-only gain, not a counted improvement.
