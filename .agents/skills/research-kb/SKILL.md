---
name: research-kb
description: Query the shared research-kb corpus before any research task, lead proposal, novelty or negative claim, factor-base/algorithm/coordinate choice, or subagent handoff. The corpus indexes crypto, crypto-autoresearcher, cryptanalysis, crypto-lean, cryptologic-archives and the paper collection (about 22k sources, 151k passages, 1.7k Lean declarations). Covers kb_status, search_knowledge, search_theorems, get_source, get_context, find_related, recent_sources, the result fields that matter, the CLI fallback, and the receipt to put in a note, session log or PR.
---

# Query the research knowledge base

Read `AGENTS.md` “Knowledge base first” first. The KB is a **finding aid**, not
an oracle: it points at the repository file, paper or Lean declaration you
still have to read. Its scores are relevance, not truth. A miss is not proof
that nothing exists.

Use this skill whenever the task is any of:

- a literature, prior-art or “has this been tried?” question;
- a new lead, mechanism, factor base, summation polynomial, solver, coordinate,
  isogeny or scaling assumption;
- a “first”, “best”, “not known”, “no effect”, “negative result” or novelty
  claim;
- a cross-repository question (crypto ↔ crypto-autoresearcher ↔ cryptanalysis ↔
  crypto-lean);
- a subagent handoff, so the child does not repeat or contradict the search.

## 1. Five-minute protocol

1. **Status once.** `kb_status`. Note `revision` and `last_refresh`. If
   `ready` is false, use the fallback in §6 and say so.
2. **Four queries, not one.** `search_knowledge` with:
   - the exact mechanism (“point decomposition Curve25519”);
   - the negative form (“negative result summation polynomial degree”);
   - the application (“factor base isogeny tower”);
   - the adjacent field (“resultant Semaev elimination”).
   Add `repo=` when you know the owning repository and `source_type=` to
   separate `paper`, `internal-note` and `formal-source`.
3. **Lean claims.** `search_theorems` for the declaration or statement. Quote
   the `name`, `environment` digest and `verification`; `source-only` is not a
   checked proof.
4. **Read the best 1–3 hits.** `get_source` for metadata and every origin;
   `get_context` for a bounded excerpt around a hit; `find_related` on one hit
   to find the sibling lead or the paper behind it.
5. **Catch the recent.** `recent_sources` before a survey or a “what changed?”
   question. It is ordered by ingestion time, not publication date.

A useful default is `top_k=6–10`; the server returns excerpts, not just titles.
If the first query is noisy, add exact phrases, identifiers (`KN-`, `KR-`,
`LEAD-`, `PCC-`) or a repository filter; do not conclude absence.

## 2. Query recipes

| goal | call |
|:--|:--|
| prior art for a mechanism | `search_knowledge(query="point decomposition Curve25519", top_k=10)` |
| negative result / closed lead | `search_knowledge(query="negative result Semaev degree of regularity", top_k=10)` |
| open problem | `search_knowledge(query="open problem prime-field summation polynomial", source_type="internal-note")` |
| cross-program leads | `search_knowledge(query="factor base isogeny tower", repo="crypto-autoresearcher")` |
| exact theorem | `search_theorems(query="CryptoResearch.Semaev.S3_root_iff", top_k=5)` |
| what just landed | `recent_sources(limit=20, repo="crypto")` |
| neighbours of a hit | `find_related(source_id="source:…", top_k=5)` |
| read the file | `get_source(source_id="source:…")` |
| read around a passage | `get_context(chunk_id="sha256:…", before=1, after=1)` |

Use `include_superseded=true` only when the history matters; otherwise the
default hides superseded notes.

## 3. Read the hit, not the score

For each hit, check:

- `origins[].repo`, `origins[].path`, `git_commit` — the file to open;
- `claim_status` — published, draft, withdrawn, superseded;
- `authority` — primary source, internal analysis, formal source;
- `evidence_level` — none is common; it is not a measurement;
- `superseded`, `superseded_by` — do not build on a superseded note;
- `verification` (theorems) — `source-only` is not a proof;
- `topics` — the other terms to query next.

Then open the primary path and read the relevant range. A KB excerpt may omit
constants, later revisions or caveats; the repository file or paper wins.

## 4. Record a receipt

In the task notes, the session-log entry or the PR body, add:

```
Sources checked (KB):
- source:<id> — <repo>:<path> @ <commit> — <what it changed in the plan>
- query "<q>" — no result; searched <repos>, <source types>
```

If the KB was unavailable, write `KB unavailable: <tool/error>; used <lead
atlas / rg / paper>.` This is the difference between “we looked” and “we
think we looked”.

A subagent prompt must carry the queries already run and the source IDs already
accepted; the child must not re-litigate them silently.

## 5. What the KB is good for here

- The cross-repository lead atlas (`knowledge/leads/ecdlp/`) and its status,
  falsifiers and foreclosures.
- Technique notes (`KN-TECH-*`), open problems (`KN-OPEN-*`) and decisions
  (`DEC-*`).
- Summaries of the paper collection, with the original PDF path and hash.
- Lean declarations and their environment digests.
- Recent ingestion, to catch a result that landed since the last session.

It does **not** replace `ecbench`, the committed session records, the
validation gate or a primary-source read.

## 6. Fallback when MCP is not loaded

The same catalog is available from the repository wrapper. It resolves the
sibling checkout and sets `RESEARCH_KB_HOME`/`RESEARCH_KB_CONFIG`, which a bare
CLI run does not:

```sh
scripts/research-kb status
scripts/research-kb search "point decomposition Curve25519" --top-k 8
scripts/research-kb search "LEAD-EXO-gag-holant-semaev-chain" --repo crypto-autoresearcher
scripts/research-kb theorems "CryptoResearch.Semaev.S3_root_iff"
scripts/research-kb brief "negative results for Semaev decomposition" | head -40
```

Override `RESEARCH_KB_BIN`, `RESEARCH_KB_HOME` or `RESEARCH_KB_CONFIG` if your
workspace layout differs. If neither MCP nor the wrapper is available, use the
lead atlas and repository paths, and mark the KB review unavailable in the
handoff.

## 7. Freshness and modes

- `kb_status.hybrid_current=false` means lexical FTS is current and the
  optional semantic index is not built. Use exact terms, identifiers and
  phrases; do not judge the corpus by a vague one-word semantic query.
- `network_fetch_on_search=false`: searching never fetches a new paper.
- Refresh is the operator's job: `research-kb watch --once` (or `--watch
  --interval 60 --pull`). If `last_refresh` is older than the work you are
  relying on, say so and read the repository file directly.

## 8. Anti-patterns

- Treating a high score as evidence, or an internal summary as the paper.
- One vague query, then “no prior work”.
- Searching only `crypto` when the lead atlas, technique notes and decisions
  live in `crypto-autoresearcher`.
- Quoting a Lean declaration without its `verification` state.
- Repeating a search a subagent already did, or handing a child no source IDs.
- Using the KB as a substitute for `ecbench`, the replay certificate or the
  validator report.

## 9. Checklist before you hand off

- [ ] `kb_status` consulted; revision/refresh recorded.
- [ ] 2–4 `search_knowledge` queries, including the negative form.
- [ ] `search_theorems` for any Lean-backed claim.
- [ ] Best 1–3 hits opened with `get_source`/`get_context`; primary path read.
- [ ] Receipt with `source_id`, `repo:path`, commit and effect.
- [ ] If unavailable: fallback used and the gap stated.
