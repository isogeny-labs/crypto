#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#set par(justify: true)

= The matched Koblitz rho reference with v5's jump selections charged

EXP-KOBLITZ-REF-002 · 2026-11-20 · protocol `PROTOCOL.md` (frozen `e141a2ca27`; Amendment 1 frozen `8786f1309f`
before the rerun) · instrument `ic_boundary::TunedWalker` (`jump_selections`, `jump_select_word_ops`),
`ecbench::band::WALK_SELECT_WORK_COUNTERS` · runner `run.sh` · analysis `analyze.py` → `results.json` · figure
`make_figures.py`. L0 (shared container, `--cpus none`); counted GAE are deterministic; wall time is not used.
All values are measurements on these five toy curves; nothing here concerns cryptographic sizes [pcc:unverified].

== Result

The owner adopted `min(strong, v5)` as the matched Koblitz rho reference below about $2^(38)$, with v5's jump-index
selection charged first. With it charged at $w$ GAE per selection in the §5a band, *tallied v5 is still the cheaper
reference on every curve and weight measured here* ($log_2 r$ 15.4 to 27.8), so the matched reference is v5 in all
15 cells. Charging the selections raises v5's priced cost at $w > 0$: at $w = 1$ v5 ÷ strong rises from 0.12–0.50
(audit, untallied) to 0.18–0.97, and at n37 ($log_2 r$ 27.8) the interval [0.882, 1.067] includes 1, so the two
references are tied there at $w = 1$. The distance of the generator-orbit IC arm to the matched reference at $w = 0.1$
moves from 1.69 to 1.56 at n29 (the closest row) and is 2.9–4.5 elsewhere. This changes the distance remaining, not
any IC cost: it is not an improvement of the index calculus.

#figure(image("figures/fig1-matched-reference.svg", width: 100%),
  caption: [Left: v5 ÷ strong per curve with its 95% interval at three weights; dashed lines are the audit's untallied
  ratios. Right: the generator-orbit IC arm ÷ the matched reference (v5 in every cell).])

== Verdicts (frozen predictions)

- *M1 soundness:* held. All five sessions verified with `verify --replay-all` (0 problems, `AUDIT_SUMMARY.json`);
  every v5 A/A interval contains 1 at all weights.
- *M2 only the tally changed:* held. v5's mean S at $w = 0$ equals the audit's on every curve (relative 1e-9).
- *M2b (Amendment 1):* held. v5's S(w) minus the audit's equals $w dot "mean"("jump_selections") \/ sqrt(r)$ to a
  relative 1e-6 in all ten cells ($w = 0.1, 1$).
- *M3 v5 cheaper below $2^(28)$:* held. v5 ÷ strong $< 1$ with the interval excluding 1 at all three weights on n17,
  n23, n29, n31. On n37 (reported without threshold) it is 0.581 and 0.661 at $w = 0, 0.1$ and tied at $w = 1$.
- *M4 distance:* expectation held: n29 `gen-c1` ÷ matched at $w = 0.1$ is 1.557 [1.403, 1.728], inside [1.4, 1.9].

#table(columns: 7, stroke: 0.4pt, inset: 3pt,
  table.header([curve], [log2 r], [v5 ÷ strong, w = 0], [w = 0.1], [w = 1], [audit w = 1], [audit derived w = 1]),
  [n29], [15.4], [0.079 [0.075, 0.083]], [0.091 [0.086, 0.096]], [0.179 [0.166, 0.192]], [0.119], [0.149],
  [n17], [16.0], [0.093 [0.089, 0.097]], [0.108 [0.103, 0.114]], [0.211 [0.198, 0.225]], [0.137], [0.174],
  [n31], [20.5], [0.168 [0.157, 0.180]], [0.199 [0.186, 0.214]], [0.383 [0.352, 0.416]], [0.221], [0.302],
  [n23], [22.0], [0.261 [0.240, 0.284]], [0.307 [0.282, 0.335]], [0.545 [0.495, 0.601]], [0.310], [0.428],
  [n37], [27.8], [0.581 [0.534, 0.632]], [0.661 [0.606, 0.721]], [0.970 [0.882, 1.067]], [0.504], [0.737],
)

The audit's derived estimate charged one selection per walk step. v5 makes two per walk operation (1.99–2.00 measured):
each step selects its jump, and the look-ahead re-tests the next point, which the code always evaluates (a
non-short-circuit `&`). That is why the measured effect is about twice the derived one. The tally counts what the code
does; a walker that reused the re-test's index for the next step would make one, and v5 would be cheaper at $w > 0$.

#table(columns: 6, stroke: 0.4pt, inset: 3pt,
  table.header([curve], [IC arm], [÷ matched, w = 0], [w = 0.1], [w = 1], [audit ÷ v5, w = 0.1]),
  [n29], [gen-c1], [1.114 [0.977, 1.270]], [1.557 [1.403, 1.728]], [2.982 [2.740, 3.244]], [1.691],
  [n29], [k1-l10], [9023.579 [8421.432, 9668.781]], [8412.907 [7829.957, 9039.259]], [6453.440 [5946.770, 7003.278]], [9134.097],
  [n17], [gen-c1], [4.370 [3.169, 6.027]], [4.544 [3.337, 6.186]], [5.063 [3.841, 6.674]], [4.985],
  [n31], [gen-c2], [2.762 [2.333, 3.270]], [2.901 [2.474, 3.401]], [3.263 [2.838, 3.751]], [3.284],
  [n23], [gen-c3], [3.436 [2.995, 3.944]], [3.441 [3.014, 3.929]], [3.453 [3.058, 3.899]], [3.933],
  [n23], [gen-c4], [3.198 [2.802, 3.649]], [3.268 [2.881, 3.708]], [3.440 [3.067, 3.858]], [3.735],
  [n23], [k1-l9], [185.906 [171.702, 201.285]], [176.890 [162.981, 191.987]], [154.778 [141.617, 169.161]], [202.147],
  [n37], [gen-c8], [4.610 [4.034, 5.268]], [4.441 [3.898, 5.059]], [4.050 [3.582, 4.578]], [5.194],
)

== Amendment 1 and the first run

The first run (kept in `runs/pre-amendment-1/`) priced the selections only in the bitop/v1 unit; the band, which every
comparison reads, did not see them, and every ratio equalled the audit's. The instrument did not do what the protocol
said. Amendment 1 adds the per-selection count behind the marker `counted:jump_selections`, which the band prices at
$w$ like the table and sibling work; records without the marker are unchanged. Predictions were not changed; the
analyst had seen the first run's (audit-equal) numbers. Band unit tests that read raw records kept in the S3 store
fail here with missing-file errors (14, none an assertion); the pricing test passes.

== Limits

- Raw sessions, audits and logs are not in Git (the store upload is unavailable to CI and to this container); their
  SHA-256 are in `RAW_SHA256SUMS`. They exist only in the producing container.
- Five curves, $log_2 r <= 27.8$; the owner's boundary is about $2^(38)$. Above $log_2 r approx 28$ the $w = 1$
  advantage is gone; n41 and n53 were not rerun.
- Adopting the reference in `docs/ic/*`, the leaderboard and `boundary_targets.json` is a follow-up, as the protocol says.
