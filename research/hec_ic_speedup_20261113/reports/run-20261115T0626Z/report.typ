#set document(title: "S4: the full-base line sieve at genus 3 and 4, counted against the walk")
#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1")
#set text(size: 9.5pt)
#show heading.where(level: 1): set text(size: 14pt)
#show heading.where(level: 2): set text(size: 11pt)
#show table: set text(size: 8pt)

= S4 (track T2, lead 1): the full-base line sieve at genus 3 and 4

*Study:* `research/hec_ic_speedup_20261113/` (PLAN.md §10, S4-RESULTS.md, SIEVE-PROBE-20261114.md). \
*Protocol* frozen at `44e2e36c57`; *code* `af2f5c83a6`; *scripts* `f9517dfb1e` (branch
`research/hec-sieve-s4-20261115`). Binaries built from a `git archive` of `af2f5c83a6`. \
*Record:* `benchmarks/records/20261115T0626Z-hec-s4-line-sieve.json`, one run of `run_s4.sh`
(`s4-20261115/`). \
*Host:* one arm64 macOS laptop, single thread, shared; load average 92–174 (`host.txt`). The
primary statistics are counts and do not depend on load. \
*Class:* engineering on the relation phase. No `S` claim, no advance, not validated.

== What was measured

Both arms on the same instance and planted $k$, 5 fresh seeds per row: the native walk
(`Solver::Dense`) and the full-base line sieve of Sarkar–Singh (`Solver::Fast`). Feature
`count-ops` counts every 64×64 field product (including each term of the lazily reduced
`Poly::mul`), Montgomery reductions and inversions, per phase. Verification of sieve relations is
counted separately and excluded.

== Pre-registered verdicts

#table(columns: (auto, 1fr, auto), align: left,
  [*id*], [*expectation (PLAN.md §10)*], [*verdict*],
  [E12], [G10: every sieve relation holds in the Jacobian, every $k$ verifies. G11: count and timing builds agree apart from excluded fields.], [*holds*: 110/110 lines each],
  [E13], [P1 = walk/sieve products per relation $>= 5$ on every bar row (genus 3; genus 4 with $p >= 127$)], [*holds*: 5.8–16.9],
  [E14], [P2 = the same ratio for inversions $>= 10$ on every bar row], [*holds*: 14–1,168],
  [E15], [P3 = walk/sieve products over relation phase + solve $>= 2$ on bar rows with $p <= 1021$], [*holds*: 5.5–10.9],
  [E16], [at (3, 4001) the sieve's solve does more mulmods than the walk's], [*holds*: 8.48M vs 2.02M],
)

#figure(image("relcost.svg", width: 100%),
  caption: [Field products per relation in the relation phase, walk against sieve, median of 5
  seeds with min–max. `count-ops` build. Record 20261115T0626Z-hec-s4-line-sieve.])

#figure(image("ratios.svg", width: 100%),
  caption: [P1, P2 and P3 per row with their bars (dashed). Bars apply to genus 3 and genus 4 with
  $p >= 127$; P3's to $p <= 1021$. Same record.])

== Reading

- *Shown.* In one counted unit, the full-base sieve's relations cost 5.8–16.9× fewer field products
  than the walk's at genus 3 and 8.2–16.1× fewer at genus 4 ($p >= 127$), with the per-line Cantor
  addition included. Inversions fall far more. End to end in counted products the sieve wins
  5.5–10.9× at $m lt.eq 506$.
- *The limit.* At $m approx 2000$ the sieve's weight-$(g+1)$ rows fill in under the plain sparse
  solver: the solve's 8.48M mulmods outweigh the relation phase, and P3 = 0.79. A solver for sieve
  rows (follow-up B) or a reduced base is needed beyond $m approx 1000$.
- *Clock* (timing build, descriptive): total walk/sieve 2.6–4.0× at genus 3 up to $p = 1021$,
  3.0–5.4× at genus 4 for $p >= 127$, 0.77 at (3, 4001).

== Limits

- The counted unit leaves out additions, memory traffic and allocation, so P1 is not a clock ratio.
- The walk is the current native walk, without Tier-2 per-trial levers.
- Full base only; the reduced-base large-prime regime is LP-SIEVE-PROBE-20261115.md (not under
  protocol).
- External comparison (§15): not run; no open hyperelliptic-IC implementation is known.
