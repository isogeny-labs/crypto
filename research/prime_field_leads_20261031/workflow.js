export const meta = {
  name: 'prime-field-leads-20261031',
  description: 'Run the eight pre-registered prime-field ECDLP leads (L1, L2a, L2b, L3, C1-C5) as model-tiered agents, then synthesise into the leads note',
  phases: [
    { title: 'Leads', detail: 'one agent per lead; Sonnet for theorem/experiment leads, Haiku for write-ups' },
    { title: 'Synthesis', detail: 'fold outcomes into section 5 of the leads note and commit on the branch' },
  ],
}

const W = '/Volumes/SSD990-2/crypto-worktrees/prime-field-leads-20261031'
const EV = `${W}/research/prime_field_leads_20261031`
const NOTE = `${W}/research/notes/ecdlp-general/RESEARCH_PRIME_FIELD_LEADS_20261031.md`
const ATLAS = '/Volumes/SSD990-2/crypto-autoresearcher/knowledge/leads/ecdlp'

const COMMON = `
You are one agent of a pre-registered research workflow in the crypto repository worktree ${W}
(branch research/prime-field-leads-20261031, PR isogeny-labs/crypto#3103). Work ONLY from that
worktree; never cd into /Volumes/SSD990-2/crypto (the main checkout belongs to another branch).

Read first: ${NOTE} (your lead's section) and ${EV}/PLAN.md (frozen thresholds). Respect them;
do not move a threshold after seeing data.

Hard rules (repository AGENTS.md, in force):
- Counted metrics only: field multiplications, solver decisions, degrees, ranks, matrix sizes.
  No wall-time claim of any kind; the host load average is near 90 on 14 cores.
- No Python for result generation (Rust harnesses, msolve, sage-run are the tools); Python may
  only plot values already recorded in a data file beside the graph.
- Every output directory ends with REPORT.md, at least one SVG graph with its data file beside
  it, and a PDF built per the "Rendering" section of ${W}/.claude/skills/research-visuals/SKILL.md
  (read that section; if no renderer works on this host, say so in REPORT.md and leave the SVG).
  A write-up with fewer than two comparable values leaves a diagram of what was derived.
- Report every attempted cell, including timeouts and failures, with the recorded reason.
- No sentence claims a speedup, win or exponent change unless a counted measurement in your
  directory admits it. Observations only; interpretation is one short "Reading" paragraph.
- Disk: the root volume (/) is full. Set TMPDIR=${W}/.tmp (create it) and
  CARGO_TARGET_DIR=/Volumes/SSD990-2/tmp-cargo for every cargo command. Never write under /tmp,
  /private/tmp or the home directory. Build with
  PATH=$HOME/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin:$PATH (Homebrew cargo 1.93 cannot
  build this crate). Prefer existing examples/binaries over new code; if you must add Rust, put it
  under examples/ or src/cryptanalysis/ and keep it small and tested.
- Time box: no single cell beyond the limit named in your lead; the whole lead within about
  90 minutes of work. Stop and report partial results rather than overrun.
- Write only inside your output directory (plus the one extra file your lead names). Do NOT run
  git add/commit/push; the synthesis step commits. Never touch .gitmodules, .mcp.json, .claude/.
- Lead atlas: ${ATLAS} holds the lab's lead cards (YAML). Where your lead overlaps a card
  (named below), read the card and ADVANCE its next_action or cite its obstruction; do not
  re-derive what it already closes. Propose atlas edits in a file atlas_update.md in your
  directory; do not edit the atlas repository.

Return (as your final text, no prose around it) a JSON object:
{"lead": "<id>", "status": "done|partial|blocked", "outcome": "<one sentence, observation only>",
 "decision": "reopens|closes|null_confirmed|writeup|blocked", "files": ["<paths>"],
 "atlas": "<one line: which cards you advance or cite>", "notes": "<limits, failures, what did not run>"}
`

const RESULT = {
  type: 'object',
  properties: {
    lead: { type: 'string' },
    status: { type: 'string', enum: ['done', 'partial', 'blocked'] },
    outcome: { type: 'string' },
    decision: { type: 'string', enum: ['reopens', 'closes', 'null_confirmed', 'writeup', 'blocked'] },
    files: { type: 'array', items: { type: 'string' } },
    atlas: { type: 'string' },
    notes: { type: 'string' },
  },
  required: ['lead', 'status', 'outcome', 'decision', 'files', 'atlas', 'notes'],
}

const LEADS = [
  {
    id: 'L1', model: 'sonnet', label: 'L1 lower bound', prompt: `
Lead L1: conditional lower bound for algebraic index calculus over F_p. Output directory ${EV}/L1_lower_bound/
plus the note ${W}/research/notes/ecdlp-general/RESEARCH_ALGEBRAIC_IC_LOWER_BOUND.md (new file).

Write the theorem draft. Model: factor base cut out by a polynomial L in one coordinate (allow PKM-style
compositions of low-degree maps); decomposition step = any polynomial-system solver charged at least the
Macaulay rank at its solving degree; linear algebra charged at least the number of columns. Claim: total
cost >= p^{1/2 - o(1)} for every arity m. Lemmas to prove or mark as gaps with the exact missing statement:
(a) Bezout: a one-coordinate base of size |F| needs deg L >= |F|/2, and for the PKM composition the
    tower has >= log_d |F| levels of degree d, so the Macaulay matrix at any degree reaching a solution has
    rank >= |F|^{c} with c made explicit; (b) the MITM floor p^{1/2+1/m} as the comparison and the
    threshold e < m/2 - 1 (derive it cleanly); (c) linear algebra >= |F| = p^{1/m} columns, so m <= 2 is
    excluded by the relation count alone; (d) Jao-Miller-Venkatesan: hardness is uniform over the isogeny
    class, so any CM-panel or volcano-level variant is covered by the generic statement; (e) yield lemma:
    for a group-unstructured base the decomposition probability is |F|^m/(m! N)(1+o(1)) -- state it as the
    conjecture C4 is testing and mark it as the one unproved input.
Sources in the repository you must read and cite by path: ${W}/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md
(sections 1.1-1.3, 4.1), the PKM paper summary in ${NOTE} section 1, the Yokoyama-Yasuda subresultant bound
as referenced in /Volumes/SSD990-2/research/ECDLP_MECHANISM_CATALOG.md (M008, M005), and the atlas card
KR-IC-401ebf ("Naive Semaev index calculus over prime fields cannot beat generic methods"; find it with
grep -rl KR-IC-401ebf ${ATLAS}) so that you generalise rather than repeat it. Also read Shoup's framing in
${W}/research/notes/ecdlp-general/RESEARCH_REPRESENTATION_STRUCTURE.md TL;DR.
Deliverables: the note with every lemma labelled PROVED / GAP; REPORT.md in the output directory summarising
which lemmas closed; a diagram (SVG, e.g. graphviz dot kept beside it) of the proof dependency graph; PDF.
No experiment. decision = "writeup".`,
  },
  {
    id: 'L2a', model: 'sonnet', label: 'L2a slope e at m=3', prompt: `
Lead L2a: slope e of counted decomposition work at m = 3. Output directory ${EV}/L2a_slope_e_m3/.
Design (frozen in the note): interval base, arity 3, s = 6..12 bits per window (p from about 2^18 to
2^36), fresh primes per s, 3 curves per prime, 32 targets per curve, satisfiable and unsatisfiable cells
both counted. Arms: whichever repository prime-field decomposition harness builds and runs
(candidates: src/cryptanalysis/semaev_sat.rs, prime_field_smt.rs, semaev_decomp.rs, prime_small_x.rs,
examples/semaev_sat_bench.rs, examples/semaev_unsat.rs, examples/semaev_decomp_bench.rs; read their
headers and pick the one that exposes counted work for an interval-constrained S_4 decomposition), against
the enumeration / MITM reference (enumerate two windows, solve for the third; count field multiplications).
Metric: counted field multiplications plus solver decisions per attempt; fit log2(work) against s with a
bootstrap interval over curves and targets; report satisfiable and UNSAT cells separately.
Decision (frozen): CI entirely below e = 0.5 -> "reopens"; CI containing e = 2 -> "closes"; otherwise
"null_confirmed" if e >= 1 is inside the CI. Stop rule: no cell beyond 20 minutes; if the ladder cannot
reach s = 10, report the cells that ran and status "partial". Graph: log2(work) vs s per arm with the fit.
Atlas: read ${ATLAS}/ic-prime/LEAD-ICP-pkm-tower-algebraic-oracle.yaml and cite it; do not duplicate it.`,
  },
  {
    id: 'L2b', model: 'sonnet', label: 'L2b d_reg trajectory (M008)', prompt: `
Lead L2b: catalogue M008, d_reg trajectory of the constrained prime-field Semaev system. Output directory
${EV}/L2b_dreg_trajectory/. Design (frozen): S_4(x1,x2,x3,x(R)) plus membership polynomials
prod_{v in V}(x_i - v) for an explicit interval base V with |V| = round(p^{1/3}), p from 2^10 to 2^20
(|V| from 10 to 100), m = 3, msolve (/opt/homebrew/bin/msolve) as the solver; generate systems in Rust
starting from examples/dreg_ladder.rs, examples/ic_next10_03_pkm_degree.rs, examples/pkm_tower_pilot.rs or
src/cryptanalysis/semaev_leading_form.rs / sparse_macaulay.rs (read them first; reuse, do not rewrite).
Control: a random system with identical variable count, equation count and degree profile including
artificial membership equations of the same degree. Metric: maximal step degree reached (d_reg proxy from
msolve verbose output), first fall degree if exposed, Macaulay matrix size at the final step, against
log2 p; 4 random targets per p, 2 curves per p. Decision (frozen): a trajectory flat as |V| grows while the
random control grows -> "reopens"; both growing alike -> "null_confirmed". Stop rule: no cell beyond 30
minutes or 16 GiB (use ulimit -v); report every attempted cell. Graph: degree vs log2 p, both arms.
Read /Volumes/SSD990-2/research/ECDLP_MECHANISM_CATALOG.md lines 104-116 (M008) for the original card and
cite ${W}/research/notes/index-calculus/ for any earlier d_reg ladder you reuse.`,
  },
  {
    id: 'L3C5', model: 'haiku', label: 'L3 precomputation + C5 special primes', prompt: `
Two write-up leads. (1) Lead L3, output directory ${EV}/L3_precomputation/ plus ONE new section appended
at the end of ${W}/research/notes/ecdlp-general/RESEARCH_ECDLP_STATE_OF_THE_ART.md titled
"Precomputation and multi-target attacks on fixed curves (2026-10-31)". Content: Bernstein-Lange 2012
"Computing small discrete logarithms faster" (online about l^{1/3} after a per-curve table of about l^{2/3}),
Kuhn-Struik 2001 (k logarithms for about sqrt(k l) total), Corrigan-Gibbs-Kogan 2018 generic preprocessing
lower bound (S T^2 >= ~l). Table of the implied exponents for P-256 (l ~ 2^256), Curve25519 (l ~ 2^252) and
ECCp-131 (l ~ 2^131): table size, online cost, and multi-target cost for k = 2^10 targets, all as powers of
two, plus rho for comparison. State plainly that this is the only known sub-rho ONLINE cost on a standardised
curve and that it is not a break. First read the atlas card for KR-RHO-ea34b8 (grep -rl KR-RHO-ea34b8
${ATLAS}; it already records this known result) and KR-RHO-46c2c6 (multi-target); cite both and make the
crypto note consistent with them. Graph: online cost exponent vs table-size exponent for the three curves
(SVG with its CSV beside it). (2) Lead C5, output directory ${EV}/C5_special_primes/: a one-page closure
note on why 2^255-19 and the Solinas primes of the NIST curves touch field arithmetic only: doubling does
not preserve "x has small high bits", multiplication by 2 on x is not a curve map, and no base defined by
digit patterns is closed under any group operation; include a small diagram (SVG) of the three failed
closure properties. Cite ${W}/research/notes/cm-isogeny/RESEARCH_NIST_CYCLOTOMIC_POINT_HEIGHTS.md and
${ATLAS}/ic-prime/LEAD-ICP-xline-symmetry-special-j.yaml (the symmetry side is that card's domain; say so).
Both are write-ups: decision = "writeup". Return one JSON object with lead "L3+C5".`,
  },
  {
    id: 'C1', model: 'haiku', label: 'C1 lift torsion', prompt: `
Lead C1: the torsion copy of E(F_p) in every lift. Output directory ${EV}/C1_lift_torsion/.
FIRST read the atlas cards ${ATLAS}/transfer/LEAD-TRF-padic-canonical-section.yaml and (grep for it)
LEAD-EXO-padic-jet-canonical-lift and LEAD-TRF-cm-canonical-lift-heights. They already close this lead with
the theorem "the only homomorphic section is the prime-to-p torsion section, which the formal logarithm
kills". Your job is NOT to re-derive that. It is to (a) write the splitting argument in two paragraphs for
the L1 note: for any lift E~/Z_p of ordinary E/F_p and N = #E(F_p) prime to p, 0 -> E~_1(Z_p) -> E~(Z_p)
-> E(F_p) -> 0 splits because E~_1(Z_p) is isomorphic to Z_p (torsion-free, p > 2) and N is a unit in
Z_p, so E~(Z_p)_tors is a canonical homomorphic copy of E(F_p) on EVERY lift; the Serre-Tate canonical lift
adds only CM and algebraicity; every p-adic analytic homomorphism to (Q_p,+) (formal log, Coleman integral,
p-adic height) vanishes on torsion; the complex route needs the N-torsion field of the CM lift, of degree
about N; Smart's attack is the one case (N = p) where the sequence does not split. Check each step; if the
claim "any lift" needs p > 2 or good reduction, say so. (b) Cross-check against the cm_canonical_lift
falsification in ${W}/research/notes/cm-isogeny/RESEARCH_P256.md (grep canonical) and note agreement or
disagreement. (c) Write atlas_update.md proposing that LEAD-TRF-padic-canonical-section's obstruction
statement gain the phrase "holds for every lift, not only the canonical one" if it does not already say so.
Deliverables: REPORT.md with the two paragraphs, a diagram (SVG) of the exact sequence and which maps kill
torsion, PDF. decision = "writeup" (or "closes" if you find the atlas statement needed the strengthening).`,
  },
  {
    id: 'C2', model: 'sonnet', label: 'C2 volcano crater vs floor', prompt: `
Lead C2: one volcano experiment to retire the CM panel. Output directory ${EV}/C2_volcano/.
FIRST read ${ATLAS}/ic-prime/LEAD-ICP-isogeny-transported-relation-yield.yaml: it is open_blocked, its
best measurement is crater/floor relation-yield ratios 0.975 [0.603,1.451] and 1.127 [0.922,1.342] (no
separation), and its next_action is about finishing yield arms. Your experiment is the SOLVE-COST side the
card does not have: same isogeny class, crater curve vs floor curve, same prime-order subgroup, matched
interval bases, constrained S_4 decomposition at m = 3, counted work and solving degree. Advance that card's
next_action with this; do not file a duplicate.
Design: pick one toy prime p in 2^16..2^20 and an ordinary curve whose volcano has depth >= 2 with a shared
prime-order subgroup of order l on crater and floor (use src/cryptanalysis/isogeny_class_search.rs or the
jv_isogeny_walk / p256_isogeny_walk modules to find and walk; or sage-run if a Sage helper exists under
${W}/sage/). Interval base of size round(l^{1/3}) on each curve (same size). 32 targets per curve, same
target scalars on both curves (map targets through the isogeny so the instances correspond). Metric: counted
field multiplications of the decomposition routine (reuse the L2a/L2b harness choice: read
src/cryptanalysis/semaev_decomp.rs, semaev_sat.rs, prime_small_x.rs; pick one and state which) and, if msolve
is used, the maximal step degree. Decision (frozen): a systematic gap above 2x in counted work or one full
solving degree between crater and floor at equal l -> "reopens"; identical within the curve-to-curve spread
-> "null_confirmed" and name the catalogue mechanisms this closes (M010, M017, M027, M039, M083, M089 in
/Volumes/SSD990-2/research/ECDLP_MECHANISM_CATALOG.md; quote their one-line titles). Stop rule: 90 minutes
total; if no suitable class is found in 30 minutes, report that as the result with status "partial".
Graph: per-target counted work, crater vs floor, with medians.`,
  },
  {
    id: 'C3', model: 'haiku', label: 'C3 heavy-tail ceiling', prompt: `
Lead C3: the self-reducibility ceiling on heavy-tail mechanisms. Output directory ${EV}/C3_heavy_tail_ceiling/.
Argument: the ECDLP is random self-reducible within the group (R -> R + rP is one addition), so an
easy-target fraction f buys at most 1/f on the decomposition phase, and a sieve already prices it. Read
/Volumes/SSD990-2/research/ECDLP_MECHANISM_CATALOG.md and extract every mechanism that conditions on the
target R or on a "heavy tail" (at least M012, M014, M023, M044, M054, M067, M077; grep -n -i "heavy" and
"rare" to find the rest). For each: one-line title, what fraction f of easy targets it would need to change
the relation-phase exponent at m = 3 and m = 4 given the arithmetic in section 2 of ${NOTE} (an attempt
costing p^{e/m} beats rho iff e < m/2 - 1; a 1/f gain on the attempt cost lowers e by m log_p(1/f)... derive
the exact condition and show it), whether the mechanism's own text states a plausible f, and a recommended
disposition (close / keep as sieve-pricing only / unaffected because it does not condition on R). Produce
the table in REPORT.md and as CSV, a graph (SVG from the CSV: required log2(1/f) vs field size in bits for
m = 3, 4), PDF, and atlas_update.md listing which atlas cards (grep ${ATLAS} for "heavy", "tail", "rare",
"easy target") the ceiling touches. decision = "writeup".`,
  },
  {
    id: 'C4', model: 'sonnet', label: 'C4 pseudo-generic interval predicate', prompt: `
Lead C4: the interval predicate is pseudo-generic (conjecture plus toy measurement). Output directory
${EV}/C4_pseudo_generic/. FIRST read ${ATLAS}/exotic/LEAD-EXO-dl-character-sum-pseudorandom.yaml
(H-PSEUDO, exhausted_at_scale: character-sum ratio flat at 1.28-1.34 over 2^20..2^26). Your object is
different and say so: not the additive character of the discrete log over the base, but the additive
ENERGY of the base itself in the group, E(F) = #{(P1,P2,P3,P4) in F^4 : P1 + P2 = P3 + P4}, compared with a
uniformly random subset of E(F_p) of the same size. State the conjecture precisely in REPORT.md (ratio
E(F_B)/E(random) = 1 + o(1) uniformly in E, for F_B = {P : x(P) < B}), cite the nearest literature you can
name with confidence (Shparlinski and coauthors on points of elliptic curves in boxes; if unsure of a title,
describe rather than fabricate), and explain why it would be the missing yield lemma of L1 and would close
every sieve-style shortcut on the interval base.
Toy measurement in Rust (small example under examples/, reuse src/cryptanalysis/prime_small_x.rs or
p192_interval_bsgs.rs curve arithmetic if convenient): p from 2^12 to 2^18 (7 sizes), 8 random prime-order
or near-prime-order curves per p, |F_B| = round(sqrt(N)) and round(N^{1/3}) where N = #E(F_p); compute
E(F_B) exactly by hashing pair sums (|F|^2 sums; at |F| ~ 2^9 that is 2^18 sums per set, cheap), and the
same for 8 random subsets of the same size; report the ratio with the spread over curves and the binomial
expectation. Counted work only. Decision (frozen): a ratio departing from 1 by more than the spread AND
growing with p -> "reopens"; ratio 1 within spread -> "null_confirmed". Graph: ratio vs log2 p for both
sizes with error bars. Stop rule: 60 minutes of compute; report what ran.`,
  },
]

phase('Leads')
log(`Launching ${LEADS.length} lead agents (Sonnet: L1, L2a, L2b, C2, C4; Haiku: L3+C5, C1, C3)`)
const results = await parallel(LEADS.map(l => () =>
  agent(COMMON + '\n' + l.prompt, { label: l.label, phase: 'Leads', model: l.model, schema: RESULT })
    .then(r => r || { lead: l.id, status: 'blocked', outcome: 'agent returned nothing', decision: 'blocked', files: [], atlas: '', notes: 'null result' })
))
const summary = results.filter(Boolean)
log(`Leads finished: ${summary.map(r => `${r.lead}=${r.status}/${r.decision}`).join(', ')}`)

phase('Synthesis')
const synth = await agent(COMMON + `
You are the synthesis step. The lead agents returned these JSON results:
${JSON.stringify(summary, null, 2)}

Tasks, in order:
1. For each lead, open its directory under ${EV} and verify the claimed files exist and that REPORT.md has
   a graph and a PDF (or an explicit statement why the PDF could not be rendered). Verify that no sentence in
   any REPORT.md claims a speedup, win or exponent change beyond what a counted measurement in that
   directory admits; if one does, rewrite that sentence as an observation and record the edit.
2. Write section 5 "Outcomes" of ${NOTE} (replace the "Pending" line): one short paragraph per lead with
   its status, its decision against the frozen threshold in ${EV}/PLAN.md, the counted values that decide
   it (with units and sample sizes), and the file path. Add a final paragraph "What changed": which
   catalogue mechanisms and atlas cards the results close or advance, and state plainly that no exponent
   changed unless a lead's decision was "reopens" (then say exactly which threshold was crossed and that
   it needs an independent replication before any ledger row).
3. Write ${EV}/README.md: index of the lead directories, one line each, the decision column, and the
   collected atlas_update.md proposals concatenated under "Proposed atlas updates" (these are proposals
   for the crypto-autoresearcher repository, not applied here).
4. Append an entry to ${W}/docs/session-logs/2026-10-31.md (append with cat >>, never rewrite the file;
   it is about 2,700 lines) with the four required headings: What was done, Decisions made, Open threads,
   Errors. Record: PR #3103, the branch, Conductor 404, that auto-merge was not enabled (permission
   classifier refused), the root-disk-full incident, and each lead's status.
5. Stage ONLY these paths: research/prime_field_leads_20261031/, research/notes/ecdlp-general/
   RESEARCH_PRIME_FIELD_LEADS_20261031.md, research/notes/ecdlp-general/RESEARCH_ALGEBRAIC_IC_LOWER_BOUND.md
   (if present), research/notes/ecdlp-general/RESEARCH_ECDLP_STATE_OF_THE_ART.md, docs/session-logs/
   2026-10-31.md, and any new examples/*.rs or src/cryptanalysis/*.rs files the leads report in "files"
   (check them with cargo check --example <name> using CARGO_TARGET_DIR=/Volumes/SSD990-2/tmp-cargo before
   staging; a file that does not compile is left unstaged and reported). Never stage .gitmodules, .mcp.json,
   .claude/, .resume/ or anything else. Commit with a message starting "research: prime-field leads
   20261031 results (" + the decision summary + ")", a body listing each lead's decision, and the trailer
   line "Co-Authored-By: Claude Mythos 5.1 <noreply@anthropic.com>". Do NOT push.
Return JSON: {"committed": "<sha or none>", "section5": "<the Outcomes text you wrote>",
 "unstaged": ["<files left out and why>"], "edits": ["<claim sentences you rewrote>"]}
`, { label: 'synthesis', phase: 'Synthesis', model: 'sonnet', schema: {
  type: 'object',
  properties: {
    committed: { type: 'string' },
    section5: { type: 'string' },
    unstaged: { type: 'array', items: { type: 'string' } },
    edits: { type: 'array', items: { type: 'string' } },
  },
  required: ['committed', 'section5', 'unstaged', 'edits'],
} })

return { leads: summary, synthesis: synth }
