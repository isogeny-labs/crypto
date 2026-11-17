#set page(paper: "a4", flipped: true, margin: 1.4cm)
#set text(size: 9.3pt)
#show raw: set text(size: 8.3pt)
= Last fall degree against the number of summands m — run report (E8)

*Study* `research/fall_degree_bounds_20261008` (aburan28/crypto). Amendment 9 (cells and reading rule) was committed before any E8 draw. *Engine* `examples/hky_par.rs`, the E7 binary (sha256 `24cf10d6…`, gated in Amendment 7). Host: Claude Code cloud container, 4 vCPU, 15 GB. E8 holds `(n, ℓ)` fixed and raises m, since HKYY's growth requirement is in m: `N = (m − 2)n + mℓ`.

== Recorded values
#figure(image("fall_degree.svg", width: 100%), caption: [Panels 1–3 as in the E7 report. Right: E8, d\_F per draw against m for the (8, 2) ladder (blue; the m = 6 draws are E7's) and the (6, 2) ladder (orange); lines are the lower medians the rule uses.])

#table(columns: 6, inset: 3.5pt,
  [*ladder*], [*m*], [*N*], [*d\_F per draw*], [*median*], [*degree-4 time, s*],
  [(8, 2)], [3, 4, 5, 6, 7], [14, 24, 34, 44, 54], [m=3 4 4 4 3 · m=4 4 4 3 4 · m=5 4 4 4 4 · m=6 4 4 4 4 · m=7 4 4 3 4], [4 4 4 4 4], [m=7: 228–339],
  [(6, 2)], [3 … 8], [12 … 52], [m=3 4 3 3 4 · m=4 4 3 3 3 · m=5 4 4 4 3 · m=6 4 4 3 4 · m=7 4 4 4 3 · m=8 3 3 3 3], [3 3 4 4 4 3], [m=7: 81–100],
)

*Reading (Amendment 9), as registered.* The (8, 2) ladder is *flat in m*. The (6, 2) ladder is *mixed*: no median reaches 5, but the medians are not non-decreasing, because m = 8 falls to 3. That drop is a small-field effect (F#sub[2^6], a 4-element factor-base span, refuted at degree 3 in about 2 s on every draw). The rule is not adjusted for it. No draw has d\_F ≥ 5, up to m = 8.

== Not run, and why
- *N = 58–60* (m = 6 (10, 3), m = 5 (15, 3)): needs a host with ≥ 64 GB. `AWS_SECRET_ACCESS_KEY` and a region are absent from the environment. `e7/aws_large_tier.sh` is prepared (no SSH, gate first, S3 upload, self-terminating) and unrun.
- *N ≈ 100*: about 4.1M degree-4 columns, a ≈ 1 TB dense basis. `e8/N100_SCOPING.md` records what a sparse engine would have to do. Fill-in is the unknown. Nothing is measured or extrapolated.

== Checks
Lean `lean/FallDegree/Verdict.lean`, kernel `decide`, 246 rows: all earlier verdicts hold, plus the E8 ladder readings. Changing three m = 7 draws to 5 fails the build; changing one does not, because the rule is a median. cairn result objective `sha256:23aabfa4…` accepted by the kernel; `cairn audit`: chain intact.

== Scope
Toy sizes only: m ≤ 8 with n ≤ 8, so m ≈ n#super[1/3] (HKYY's regime) is not reached. This is not a bound. No exponent, speedup or security claim.
