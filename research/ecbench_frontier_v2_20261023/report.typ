// Results report: the docs/bounds frontier under procedure 2.
// Every number in the tables is read from the JSON files beside this source
// (extract.sh), which are copied field for field from the sealed bound
// records and the two frontier documents.
#set page(paper: "a4", margin: 1.6cm, numbering: "1")
#set text(size: 9.5pt)
#show table: set text(size: 8pt)
#set heading(numbering: "1.")
#let recs = json("records.json")
#let old = json("frontier-v1.json")
#let new = json("frontier-v2.json")
#let byid = (:)
#for r in recs { byid.insert(r.bound_id, r) }
#let name(id) = if id == none { "none named" } else if id in byid { raw(byid.at(id).file) + " (" + byid.at(id).method + ")" } else { raw(id) }
#let f3(x) = if x == none { "–" } else { str(calc.round(x, digits: 3)) }
#let ci(c) = if c == none { "–" } else { "[" + f3(c.at(0)) + ", " + f3(c.at(1)) + "]" }

= The `docs/bounds` frontier under procedure 2 (2026-10-23)

*What this is.* A scoreboard change, not a measurement: no session was run
for it. The frontier page `docs/bounds/FRONTIER.md` moves from
#raw(old.frontier_id) (`ecbench.frontier/v1`, main `3a1ea541d`) to
#raw(new.frontier_id) (`ecbench.frontier/v2`) because the procedures that
fit bounds and rank them changed (`docs/bounds/README.md` §3a, §5): the
review in `DEEP_DIVE.md` §1.3 found that the v1 page published more than its
data supported. All values come from committed sessions through
`docs/bounds/refit.sh`; every published v1 record is unchanged and still
re-derives (`ecbench bound check`), and each seed fit has a fit-v2 twin
`<name>-v2.json`, which is what the v2 frontier ranks.

*What changed in the procedure.*
- Two bounds are compared only on the curves they share, per curve with
  that curve's interval; bounds sharing none are not comparable.
- Only fit-v2 bounds on four or more sizes of a whole session (no `--curve`,
  no `--tier`) are ranked; the rest, and every v1 record, are listed, not
  ranked.
- A method dominates only if every comparable bound of it does (ranked on
  its worst bound).
- Where either side has unpriced work, a dominance must hold at 0, 0.1 and 1
  GAE per unpriced operation (AGENTS.md §5a); the break-even price is shown.
- A leader is named only when its intervals separate from every other
  method's on shared sizes; otherwise a tie set is listed.
- Fit v2: the exponent interval comes from resamples with at least three
  distinct sizes and exists only from four sizes; intervals that exclude
  their estimate, duplicate draws, unaudited or incomplete sessions and a
  non-default resample count are refused; tail diagnostics are recorded.

= Per domain: old and new ranking

F frontier, D dominated, NR listed and not ranked (v1 records, fewer than four
sizes, or a curve or tier subset).

#table(
  columns: (auto, auto, 1.3fr, auto, 1.7fr),
  inset: 4pt,
  [*domain*], [*v1 F / D*], [*v1 ops leader*],
  [*v2 F / D / NR*], [*v2 ops leader or tie set*],
  ..for d in new.domains {
    let o = old.domains.find(x => x.family == d.family and x.tier == d.tier)
    (
      d.family + " " + d.tier,
      str(o.frontier.len()) + " / " + str(o.dominated.len()),
      name(o.ops_leader),
      str(d.frontier.len()) + " / " + str(d.dominated.len()) + " / " + str(d.unranked.len()),
      if d.ops_leader != none { name(d.ops_leader) } else {
        "none; tie: " + d.ops_tie.map(t => name(t)).join(", ")
      },
    )
  }
)

The v1 leaders were named on point estimates. In prime toy, `bsgs.negation`
(1.100 [0.978, 1.225]) and `rho.negation` (1.453 [1.184, 1.772]) overlap, so
v2 names no leader. In Koblitz toy, the v1 leader was a three-size,
tier-subset bound (`calibration-koblitz-toy-rho-frob`, degrees 17, 19, 23)
compared with bounds fitted on other curves; it is no longer ranked, and on
the four shared degrees the order between `rho.signed_frobenius` and
`bsgs.negation` changes with the degree. In Koblitz medium the strong rho
stays the ops leader, now because its intervals separate on all six shared
degrees; the memory leader moves from a three-degree bound to the pair claw at
the cryptanalysis\#175 shape.

#figure(image("ops-status.svg", height: 88%),
  caption: [Ops ratio to the floor with 95 % two-stage bootstrap intervals for
  every fit-v2 bound, per domain; colour and marker give the v2 status, the
  label the status of the bound's published v1 record on the v1 page. Values
  from `records.json`; statuses from `frontier-v1.json` and `frontier-v2.json`.])

= Dominance claims that fail inside the pricing band

Each row is a pair the charged cost would rank but the band does not: the
dominator wins at w = 0 and loses, or ties, by w = 1 GAE per unpriced
operation. The break-even price is where the shared-size mean ops figures cross.

#table(
  columns: (auto, auto, auto),
  inset: 4pt,
  [*would be dominated*], [*by*], [*break-even (GAE / unpriced op)*],
  ..for d in new.domains {
    for p in d.price_checks.filter(p => p.holds_across_band != true) {
      (name(p.dominated), name(p.dominator), f3(p.break_even_price))
    }
  }
)

= Exponent intervals and tails

#figure(image("alpha-intervals.svg", width: 92%),
  caption: [Fitted α with 95 % interval, published fit-v1 record (grey) and its
  fit-v2 twin (blue; a diamond means no interval: fewer than four sizes). The
  v1 ends at -1.0, 0.0, 2.5 and similar were degenerate one-size resamples.])

#table(
  columns: (auto, auto, auto, auto),
  inset: 4pt,
  [*bound (fit v2) with a large tail*], [*runs*], [*max / median*], [*top-run share*],
  ..for r in recs.filter(r => r.fit_version == 2 and r.tail != none and r.tail.large) {
    (raw(r.file), str(r.tail.runs), f3(r.tail.max_over_median), f3(r.tail.top_run_share))
  }
)

= Scope and limits

Toy and medium tiers only, planted targets, `ecbench.gae`. No bound here is a
scaling result or a statement about cryptographic sizes. The v2 page is more
conservative than v1: fewer bounds are ranked, ties stand, and an ordering
that depends on how unpriced work is priced is left unresolved. The two
epoch-1 challenge verdicts are procedure 1 and unchanged
(`research/ecbench_bounds_challenges_20261006/`); their sessions' arms are
ranked as ordinary fit-v2 bounds (`session-*-v2`). Canonical graphs checked:
the frontier page and JSON (regenerated); `research/ecbench_frontier_20261013/`
is a dated synthesis citing v1 ids that still exist, left unchanged; no other
consumer of `docs/bounds/frontier.json` exists in `docs/`.
