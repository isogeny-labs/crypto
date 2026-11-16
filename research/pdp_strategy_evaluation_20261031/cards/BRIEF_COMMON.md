# Shared brief: evaluate novel strategies for the point decomposition problem (PDP)

You are one of five parallel evaluators. This is a DESK EVALUATION: decide, with evidence,
which strategies are worth running. Do not implement them. Do not edit any repository.

## What "success" means here (owner's rule, quote)
A verified, measured improvement over the current index-calculus (IC) baseline is a success.
Beating Pollard rho is the long-term goal, reached iteratively. The gap to the matched rho is
the distance remaining, never a pass/fail gate. Relabelling and accounting changes are not
improvements. A negative or null result is valid but is not an improvement. Baseline:
`/home/user/crypto/docs/ic/LEADERBOARD.md` (every row sits above matched rho).

## What I have already established (verified this session, do not redo)
1. The ECDLP lead atlas (153 leads: 74 closed with a named obstruction, 53 open_next_step,
   17 open_blocked, 9 open_unexamined) is at
   `/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/atlas/ca/knowledge/leads/ecdlp/`
   (one YAML per lead under ic-binary, ic-engine, ic-prime, transfer, structure, exotic, generic;
   README.md there explains the schema and the dispositions). Query it with
   `cd /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/atlas/ca &&
   python3 tools/lead_atlas.py --match "<idea>" --limit 5` (read only).
2. A closure needs a named obstruction; do not re-open a closed lead unless its `revisit_when`
   holds. A null or timed-out search is not an obstruction.
3. My own earlier study (merged, PR #2423) is in
   `/home/user/crypto/research/pdp_literature_leads_20261022/` (LEADS.md header lists the
   "Standing constraints": generic-group bounds, closed-by-theorem items, measured facts;
   dossiers in leads/LEAD-NN.md, LEAD-15 = Diem-Thome plane quartics on the cubic-GHS cover).
   It is NOT in the atlas yet (grep finds no mention).
4. LEAD-19 result (merged): on the repo's random-binary ladder rows with cofactor h>1, replacing
   each factor-base point P by [h]P gives S(A')/S(B) = 1.29 (f2m15, h=2), 1.58 (f2m21, h=2),
   3.36 (f2m27, h=6); class "engineering"; every row still above matched rho. Report:
   `/home/user/crypto/research/lead19_projected_base/README.md`. KN-FIND-007 (yield
   conservation): expected relations per candidate do not depend on the factor base, so the
   levers are cost per candidate, number of targets K, the matrix, and target sharing.
5. Measured context from the atlas/leaderboard: root finding beats F4 by 10^2-10^5 at stage
   level but moves end-to-end S by at most 1.26x; the only live exponent regime is the
   genus-3 transfer after cubic GHS (q^1.30 vs rho q^1.5); the best algebraic estimate for
   ECC2K-130 m=4 is about +8.43 bits over rho (floor +7.62).

## Hard constraints on how you work
* DISK: this container has under 600 MB of writable space left. Do NOT run cargo, do NOT build
  the crate, do NOT clone or download anything large. Write only your own small output file.
  A single-file standalone Rust program compiled with plain `rustc -O` into
  `/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/eval/<your-id>/`
  is allowed for a toy arithmetic check (label it exploratory, never a claim). Python is allowed
  for analysing COMMITTED data (repository rule: Python only for analysis and rendering).
* LOCAL CHECKOUTS: `/home/user/crypto` is current to 2026-10-24 (includes the merged study).
  `/home/user/cryptanalysis` is VERY stale (commit 1aa48b5) and `/home/user/crypto-autoresearcher`
  is stale with no ledger of recent records. An atlas `evidence:` path that is not present locally
  is "not verifiable locally": say so. Never guess what such a file contains.
* LITERATURE FIRST: check primary sources with WebSearch/WebFetch for every mechanism you rely
  on (papers, eprint, official docs). Verify assumptions, parameters and conclusions rather than
  relying on memory or snippets. Record each source (title, authors, year, URL or eprint id) and
  how you used it. If a source is inaccessible or nothing relevant exists, say so and state the
  search scope. Never invent a citation. An unsuccessful search is not proof of no prior work.
* TRY TO REFUTE the lead first: find the strongest reason it cannot improve the baseline. A lead
  that survives an honest refutation attempt is worth more than one that was only described.
* Be precise about what is measured vs derived vs assumed. No improvement claims without a
  measurement. Classify any claimed effect as advance / engineering / relabelling / accounting.

## Output (one card per lead, at most ~350 words each, plain markdown)
```
### <LEAD-ID> — <one-line title>
Verdict: PROMOTE | RUN-AS-SPECIFIED | REVISE | CLOSE (with the named obstruction) | UNVERIFIABLE
Novelty: vs the atlas (and which neighbouring leads), vs my 19-lead study, vs the literature (cited)
Evidence check: for each atlas `evidence:` path: present locally? does it say what the YAML claims?
Prediction: the quantitative effect on S (or on the bottleneck it targets), with the arithmetic and a range
Decisive experiment: exact command or protocol sketch, a frozen kill criterion, compute/RAM/time/disk
Risks and contradictions found: ...
Confidence: low | medium | high, and why
Sources: ...
```
Save your cards to `.../scratchpad/eval/<your-id>.md` (create the directory path if needed) AND
return the same text. Finish with a 3-line summary: best candidate, worst, and what you could not
verify.
