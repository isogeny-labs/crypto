# Item 11 protocol: knowledge-base pass for the 2026-11-12 lead survey

Status of this file: written after a first exploratory batch of searches (queries/q1.json, q2.json) had been
read, not before. No later amendment. The rules below are what was applied to every proposal.

## Instrument
- Snapshot: isogeny-labs/research-kb release `snapshot-2026-11-14`, asset `catalog.sqlite.zst`, sha256
  verified against `manifest.json` (assets[].sha256) before decompression.
- Query path: SQLite FTS5 (`passage_fts`, bm25 order) over `passages`, joined to `locations` for repo:path;
  `declaration_fts` for Lean declarations. This is a lexical search, not the MCP server's hybrid ranking
  (`manifest.catalog.hybrid_current = false`). A miss is therefore weak evidence of absence.
- Corpus scope: repos `crypto` (814 files: notes, docs, papers, problems, NOT the dated `research/<slug>/` run
  directories), `crypto-autoresearcher` (21,718), `cryptanalysis` (164), `crypto-lean` (18), `crypto-slides` (82),
  `research-pdfs` (144). `ml-cryptanalysis` is not indexed.

## Procedure per proposal (P1 P2 P3 P5 J1-J4 K1-K3 S1-S3 M1, and the growth theorem N8)
1. Two to four FTS queries, at least one in negative or adversarial form (refuted, closed, unbounded, no
   analogue, dominated).
2. Read the best hits (passage text via `tools/ctx.py`, whole atlas card via `tools/atl.py`).
3. Read the atlas cards (plaintext in the corpus) of every id named under "Nearest" in the lane files.
4. Record: prior work (source_id, repo:path), classification, ranking effect.

## Classification vocabulary
- new: no hit in scope that states the idea or a measurement of it.
- partly anticipated: mechanism or question stated, measurement not done (or done on a neighbouring object).
- already proposed / already tried: a hypothesis, idea or run in scope asks the same question.
- refuted / closed: an in-scope record closes it with a measured or proved obstruction.

## Prediction and falsifier (stated before the classification was fixed)
Prediction: at least half of the 16 proposals have a partly-anticipated or stronger hit, because the lane
agents worked from ids only. Falsifier: fewer than 4 of 16. (Outcome in KB_PASS.md section 1.)

## Out of scope
No curve or engine computation. Every statement is [pcc:unverified]; no leaderboard or benchmark row.

## Amendment 2026-11-15 (arithmetic correction only)
The task lists 15 proposals (P1 P2 P3 P5, J1-J4, K1-K3, S1-S3, M1), not 16. Read "16" above as 15; the falsifier
"fewer than 4" and the prediction "at least half" (8 of 15) are unchanged in meaning.
