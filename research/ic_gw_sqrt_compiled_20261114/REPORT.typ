#set page(paper: "a4", margin: 1.8cm)
#set text(size: 10pt)
#set par(justify: true)

= Compiled fixed-exponent square root in the plane-quartic IC (GHS-weak family)

_research/ic_gw_sqrt_compiled_20261114, 2026-11-14. Protocol frozen at 9e6252ffa3 before data. Every result is an unverified assertion (pcc:unverified); no leaderboard row moves before the validation gate._

== Question
At c#sub[D] = 1.7 the per-pair square root of the relation search is an interpreted op list with hardware u64 division. A compiled form (segments x \u{2192} x#super[2#super[n]] \u{00D7} r, exact Barrett reduction) performs the same multiplications on the same values. Does the relation stage get faster?

== Gates
G1: 8,580 counted fields compared REF vs CHAIN, 0 mismatches (also REF pass 1 vs 2 and A/A). G2: 64 of 64 units verified.

== Result
#table(columns: 6, align: (left, right, right, right, right, right),
  [q], [seed 1], [seed 2], [seed 3], [geo-mean], [total CPU DL],
  [262111], [0.689], [0.785], [0.854], [0.773], [0.889],
  [524287], [0.931], [0.861], [0.940], [0.910], [1.002],
  [1048507], [0.876], [0.862], [0.655], [0.791], [0.956],
  [2097091], [0.879], [0.891], [0.821], [0.863], [0.969],
  [4194271], [0.926], [0.944], [0.863], [0.910], [0.993],
)
relations.cpu CHAIN / REF, mean of two passes. Maximum 0.944: below 1 in all 15 cells. A/A floor 0.160 (REF re-runs 0.989 and 1.160), so the frozen rule's separation test (max below 0.840) fails: *below 1 but not separated from noise*.

#figure(image("sqrt_compiled.png", width: 100%), caption: [Left: CPU ratio by size with the A/A band. Right: each cell, both passes.])

== Caveats
L0 shared 4-vCPU container; REF pass-to-pass relations.cpu varies 0.80-1.28, much noisier than the pilot's 0.1%. Counted work is identical, so the distance to the matched rho is unchanged; this is a time change only. Local run because the Iso Lab EKS budget is exceeded.

== Next
Rerun on an isolated host (or with many more A/A pairs and taskset) to resolve the 10-20% effect.
