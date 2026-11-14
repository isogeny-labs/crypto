"""Write REPORT.typ from leads.json and the figures; compile with the typst wheel if available."""
import json, pathlib, collections, re, sys
HERE = pathlib.Path(__file__).resolve().parent
D = json.loads((HERE / "leads.json").read_text()); L = D["leads"]
INS = [r for r in L if r["verdict"] != "out-of-scope"]
c = collections.Counter(r["verdict"] for r in L); s = collections.Counter(r["evidence_strength"] for r in INS)
src = collections.Counter(r["source_kind"] for r in L)
def t(x): return str(x).replace("/", "/\u200b").replace("\\", "\\\\").replace("#", "\\#").replace("@", "\\@").replace("$", "\\$").replace("*", "\\*").replace("_", "\\_").replace("<", "\\<").replace(">", "\\>").replace("[", "\\[").replace("]", "\\]")
flag = [r for r in INS if r["flagship"]]
flips = [r for r in INS if r["verdict"] == "refuted" and re.search(r"open|proposed|next_step|unexamined|blocked", str(r["recorded_disposition"]), re.I)]
checks = [r for r in L if (r.get("check_run_today") or {}).get("ran")]
rt = HERE / "checks" / "redteam.json"; nchg = None
if rt.exists():
    R = json.loads(rt.read_text()); nchg = sum(1 for x in R if x.get("change")); nrev = len(R)
doc = f"""#set page(paper: "a4", margin: 1.6cm)
#set text(font: "DejaVu Sans", size: 9pt)
#set heading(numbering: "1.")
#align(center)[#text(size: 15pt, weight: "bold")[Prime-field index-calculus leads: the corpus chased in parallel, drawn as a heat map]]
#align(center)[2026-11-14 · isogeny-labs/crypto `research/prime_ic_lead_heatmap_20261114/` · every statement `[pcc:unverified]`]

= Scope and standing
A search round over records (AGENTS.md "Research searches must leave visual reports"). It re-reads every prime-field $E(bb(F)_p)$ index-calculus lead the lab has written down, in three layers: the 30 `ic-prime` atlas leads plus 63 prime-relevant atlas leads of other families (read through the decrypted 2026-11-07 survey and the 2026-11-10 sweep, since the atlas itself is vault-encrypted and `KEY` is unavailable here); the 183 idea-campaign records of crypto-autoresearcher PRs \\#3183–\\#3260 (plaintext PR heads, extracted today); and 68 post-atlas proposals recorded in this repository (closure ledger, five-lab chase, P/S proposals, equation-budget note, sub-exponential probes, Iwasawa and moonshot records, encyclopedia cells 0.1/0.2/A12). {len(L)} records; {len(INS)} in scope; {c['out-of-scope']} out of scope (binary, Koblitz, $bb(F)_(p^k)$ covers, Curve25519 formula audits, quantum).

No pcc claim, no `bench.record/v1`, no isolab run, no leaderboard or benchmark row, no atlas edit. research-kb and ecdb were unavailable (gap recorded); novelty rests on grep.

= Method
Nine Sonnet workers (one assignment each), a Haiku validator (schema, enums, duplicate ids, 1,298 cited paths resolved, 0 missing) and an Opus red team over every open, blocked and flagship record plus 12 sampled desk-only refutations{f" ({nchg} proposed changes out of {nrev} reviewed)" if nchg is not None else ""}. Each lead gets one verdict (refuted, engineering, open, blocked, unexamined, out-of-scope), one evidence strength (proof, measured, toy, desk, none), one mechanism column, the newest sources, and the cheapest next test. Workers ran the record's own cheapest check when it was stdlib Python under three CPU-minutes: {len(checks)} such checks today, all in the scratchpad, none replayed.

#pagebreak()
= The heat map
#figure(image("figures/lead_cells_p1.svg", width: 100%, height: 86%, fit: "contain"), caption: [Verdict per lead by mechanism, page 1 of 3 (factor bases; oracles). Colour = verdict (teal open, slate blocked, grey unexamined, amber engineering, red refuted); black border = live exponent-level question; dots = evidence strength (4 proof, 3 measured, 2 toy, 1 desk); glyph = source (triangle atlas, disc idea record, square crypto study). Data `leads.json`, code `figures/plot.py`; the single-sheet version is `figures/lead_cells.svg`.])
#pagebreak()
#figure(image("figures/lead_cells_p2.svg", width: 100%, height: 92%, fit: "contain"), caption: [Page 2 of 3: lifting and p-adic; transfers, covers, structure.])
#pagebreak()
#figure(image("figures/lead_cells_p3.svg", width: 100%, height: 92%, fit: "contain"), caption: [Page 3 of 3: barriers, spectra, controls; engineering and accounting.])
#figure(image("figures/verdict_matrix.svg", width: 100%), caption: [Counts behind the cells: verdict by mechanism column and by evidence strength (n = {len(INS)} in-scope leads).])

= Headline
In scope: open {c['open']}, blocked {c['blocked']}, unexamined {c['unexamined']}, engineering {c['engineering']}, refuted {c['refuted']}. Evidence: proof {s['proof']}, measured {s['measured']}, toy {s['toy']}, desk {s['desk']}, none {s['none']}. Sources: atlas {src['atlas']}, idea records {src['idea-pr']}, crypto studies {src['crypto-study']}.

- No lead is a sub-$sqrt(p)$ prime-field ECDLP method and none moved an exponent today. The idea records self-declare exponent delta 0 (one is $N^(3\\/2)$, worse than rho).
- The live exponent-level questions are two, as the 2026-11-12 closure ledger said; the workers had flagged nine cells and the Opus red team cut the flags to the two distinct questions. (1) The PKM/Kummer tower fall-degree growth at m ≥ 8 (`CH-L1`; cells 0.1 and 0.2 restate it). m ≤ 7 is closed; the D(8,3) cell is no longer decisive, because the Lean-checked cost floor of `research/prime_norm_rank_20261112/cost-floor-report/` removes the m = 8 gain even at D(8,3) = 9 (dense-floor exponent at least 4/7), so it is marked refuted (proof); the surviving test is the sparse relation-finder panel A with the whole-call floor charged (kill if the per-call exponent's lower bound is ≥ 3.5). Today's PR \\#4151 cell, $d_F >= 6$ at t = 12, is one more point on the C-step law. (2) Lifting with growing rank (`CH-L4`): open only because unmeasured; Theorem X bounds per-trial success by $|M|\\/q$ (paper only); the blocker is an executable small-height lift selector. `IDEA-20261109-2debfa` lost its flag: its best reachable exponent is 2/(3 − e₂) ≥ 2/3 even if its open lemma holds.
- The distance to rho did not move: Table B of the IC leaderboard still reads 5.47× to 13.5× matched rho and widens with size.

= Flagships
#table(columns: (auto, auto, auto, 1fr, 1fr), inset: 4pt, stroke: 0.4pt,
 [*id*], [*verdict*], [*evidence*], [*why*], [*cheapest next test*],
""" + "\n".join(f" [`{t(r['id'])}`], [{t(r['verdict'])}], [{t(r['evidence_strength'])}], [{t(r['why_this_verdict'][:300])}], [{t(r['cheapest_next_test'][:220])}]," for r in flag) + f""")

= What changed relative to the records
Recorded as open or proposed and read as refuted today: {len(flips)} ({sum(1 for r in flips if r['source_kind']=='atlas')} atlas, {sum(1 for r in flips if r['source_kind']=='idea-pr')} idea records, {sum(1 for r in flips if r['source_kind']=='crypto-study')} crypto studies); every one is listed with its reason in `TABLES.md`, and the atlas ones in `ATLAS_DELTA.md`. Several 2026-11-10 sweep verdicts were traced to artefacts (small n, defective positive controls, noisy rho references, mis-set thresholds) and not followed. One merged idea record (`IDEA-20261109-1130d0`, anomalous sibling) is empty: $alpha equiv 0 mod (pi - 1)$ forces $N | q$, so $q$ is never prime (6,000 candidates over 20 curves, all divisible by $N$). Seven idea records carry mis-set falsifiers or predictions (`179c5d`, `410419`, `e78107`, `00e2fd`, `876571`, `fde21e`, `5114b3`); none changes a verdict, all need correction before a run.

= Next steps
Desk: correct the mis-set falsifiers, record the empty construction, apply the atlas delta once `KEY` is available. Local CPU: an executable small-height lift selector and the L4-check F1 height comparison at 8–12 bits (lifting flagship); the Vélu-tower exact LFD at k = 4..10 (cell 0.2, exactness only). Large host, owner's call: not D(8,3); the sparse PKM relation-finder panel A at m = 8 with the whole-call floor charged, through isolab-eks with a task card, once the L1 closure source is recovered or replaced.

= Files
`leads.json` (304 records with sources), `TABLES.md`, `ATLAS_DELTA.md`, `checks/validation.md`, `checks/redteam.md`, `figures/`, `BRIEF.md`, `assign/`, `corpus/idea_index.json`. Sources checked: see README "Sources checked".
"""
(HERE / "REPORT.typ").write_text(doc)
try:
    import typst
    typst.compile(str(HERE / "REPORT.typ"), output=str(HERE / "REPORT.pdf"), root=str(HERE))
    print("REPORT.pdf built")
except Exception as e:
    print("typst unavailable or failed:", e); sys.exit(0)
