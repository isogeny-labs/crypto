#set document(title: "Rho-reference audit")
#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)

#align(center)[
  #text(size: 15pt, weight: "bold")[Rho-reference audit of the lab's IC-at-or-below-rho rows]\
  #v(2pt)
  `research/rho_reference_audit_20261121` · 2026-11-21 · full table: `AUDIT.md` (revision 5)
]

*Why.* In `research/humbert_genus5_cover_20261021` the genus-5 IC result went from "below rho" to
"above rho" twice, each time only because the rho reference was made honest. First a 4-thread
batched negation rho (about 14× the scalar one), then a SIMD kernel (about 4.5× more). This
audit asks which other rows in the lab rest on a rho reference that a strong one would overturn.
Every number in it is copied from the cited files and is an unverified assertion [pcc:unverified].

*Method.* This is a read-only enumeration of 37 rows: leaderboard rows, `VALIDATION.json` files,
benchmark rows and research headlines. For each row the audit records the rho side (measured or
ideal; threads, batching, SIMD, negation and other automorphisms; same host or not), with file:line
evidence. It then estimates a plausible strong-rho factor $F$, computes the margin $M$ =
1/(IC/rho), and assigns a class. Rows that are unranked across the GAE band (AGENTS.md §5a) are
NOT-A-CLAIM. An independent verifier reviewed revisions 2–4. Revision 5 applies its last findings
and was not re-verified [pcc:unverified]. research-kb was unavailable, so no source ids were recorded.

#figure(image("figures/audit_classes.svg", width: 80%), caption: [Rows by class.])

*Findings* [pcc:unverified]:
- 5 rows are ROBUST. Four are weak or special families: the JV cover (rows 13, 15) and GHS-weak
  (rows 17, 28 at q ≥ 4051). The fifth is the genus-4 row counted in group operations against the
  ideal step floor (row 1).
- No row on a generic prime-order elliptic curve or a standard Koblitz curve is ROBUST. 11 rows have
  already been reversed by the repository's own later work. Two of those reversals, rows 10 and 11,
  are inferred rather than rerun.
- Every wall-time row (0, 19, 31, 32) has reversed, or is at risk, once rho was engineered like the IC.
- 3 rows are AT-RISK: 8, 16 and 32. Rows 4 and 5 hold only in the FPM band and are NOT-A-CLAIM under §5a.

*Rerun plans* (in `AUDIT.md`, step 5; not yet run):
- Plan A: row 32, multi-target IC against the best batched and unbatched rho at 2^24–2^28. It builds,
  and runs in about 30–45 min on one pinned CPU.
- Plan B: row 5, genus 3 against a negation rho with a fast group law. It needs a `--group-law` flag.
- Plan C: row 16 (and row 28 at q = 991, 2011), GHS-weak against a batched negation rho. It needs a
  `rho_neg_lanes` function.
