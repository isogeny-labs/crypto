#set document(title: "S3: exact inversion hygiene in the native hyperelliptic IC relation phase")
#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1")
#set text(size: 9.5pt)
#show heading.where(level: 1): set text(size: 14pt)
#show heading.where(level: 2): set text(size: 11pt)
#show table: set text(size: 8pt)

= S3 (track T2, lever L1): exact inversion hygiene, genus 2–4

*Study:* `research/hec_ic_speedup_20261113/` (PLAN.md §9, S3-RESULTS.md, LEADS-20261114.md). \
*Protocol* frozen at `10e4dde42f` (also the `old` build); *code* `451cb16235`; *scripts*
`91e5a13ef1` (branch `research/hec-t2-20261114`). \
*Record:* `benchmarks/records/20261115T0426Z-hec-s3-inversion-hygiene.json`, two runs of `run_s3.sh`:
`s3-20261114/` (first set) and `s3-20261114-rerun/` (the pre-registered rerun). \
*Host:* one arm64 macOS laptop, single thread, *shared*, pre-registered as such. Load average was
52–401 in the first set and 50–153 in the rerun (`host.txt`). \
*Class:* engineering on the relation phase. Same arithmetic results (gates G7–G9), fewer field
inversions; no wall-clock gain established. No `S` claim, no advance, not validated.

== What changed

- `Fp::inv`: extended Euclid on `u64` remainders with `i64` cofactors, instead of `i128`.
- `Poly::divrem`: no inversion, and no multiplication by 1, when the divisor is monic. That covers
  every `mulmod` in $x^p mod u$.
- Degree-2 oracle and EDF: $1 slash 2 = (p+1) slash 2$.

== Pre-registered verdicts

#table(columns: (auto, 1fr, auto), align: left,
  [*id*], [*expectation (PLAN.md §9)*], [*verdict*],
  [E8], [G7: new `inv` and `divrem` equal the pre-S3 routines. G8: walk and oracle hashes equal on all 13 stream rows. G9: 55 runs identical apart from timings.], [*holds*: 44 stream and 18 end-to-end processes, all equal],
  [E9], [`Fp::inv` per oracle call $<= 5$ at genus 3–4, 0 at genus 2; per walk step falls on every row], [*falsified*: genus 4 at $p$ = 61 and 1009 give 5.07 and 5.17; the rest holds],
  [E10], [per-trial old/new $>= 1.25 times$ on every genus-3/4 row], [*not determined*: A/A floor failed in both sets],
  [E11], [per-trial old/new $>= 1.10 times$ on every genus-2 row], [*not determined*: same],
)

#figure(image("inversions.svg", width: 100%),
  caption: [Field inversions per walk step and per oracle call, `count-inv` builds of each arm,
  100,000-trial stream. Deterministic. Record 20261115T0426Z-hec-s3-inversion-hygiene.])

#figure(image("ratios.svg", width: 92%),
  caption: [Paired per-trial ratio old/new (green) and A/A old/old′ (grey), 7 repetitions per set.
  The grey band is the pre-registered A/A validity window; 11/13 and 10/13 rows fall outside it.
  Same record.])

== Reading

- *Shown.* The same arithmetic with about half the inversions in the walk (6.5–9.0 → 3.0–4.5 per
  step), and a third or less in the genus-3/4 oracle (9.4–16.2 → 3.6–5.2 per call). An inversion
  was measured at 35–90 ns, against about 1.2 ns for a multiplication.
- *Not shown.* A wall-clock gain on this host. The A/A medians span 0.65–1.49, wider than the
  effect the bars asked about. The first set's medians are above 1 on all 13 rows; the rerun's are
  below 1 on 4 rows.
- *Kept anyway.* Lever L1 is exact and removes work. It stays in the code without a timing verdict.
  The stop rule is not triggered, because E10 is undetermined, not falsified.

== Deviations and limits

- The first rerun attempt was stopped after about a minute and restarted under `benchrec run`, so
  that the record lists the command. Its partial output was deleted unread.
- `commit_dirty` is true in the record: only the tracked `.resume/RESUME.md` was modified.
- A quiet-host (reserved-CPU) rerun under the same §9 protocol would settle E10 and E11. It has not
  been done.
- No external comparison (§15): no open hyperelliptic-IC implementation is known. Not validated.
- *Next:* LEADS-20261114.md ranks 22 leads from three hunts (code, repo history, literature). First
  is a line-sieve feasibility probe (Sarkar–Singh), which would replace the walk and the oracle in
  the relation phase.
