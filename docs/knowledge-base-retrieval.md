# Research KB retrieval measurement

Adoption is not the only number that matters: a session that queries the KB and
gets the wrong record learns to skip it. This page records a fixed, repeatable
measurement of whether the shared catalog returns the record a researcher
expects.

## The probe

`scripts/kb_retrieval_probe.py` reads `docs/kb-retrieval-queries.json`: nine
questions phrased the way a session asks them, each with the `repo:path` a good
answer should contain. It runs each through `scripts/research-kb search`,
records the rank of the expected source (1-based, or none in the top k) and
reports top-1, top-k and MRR.

```sh
python3 scripts/kb_retrieval_probe.py
python3 scripts/kb_retrieval_probe.py --top-k 8 --json --out /tmp/probe.json
```

It only reads the KB. A score is relevance, not truth, and a miss is not
evidence that a record does not exist.

## Baseline, 2026-11-05

Lexical backend (`sqlite-fts5`, `hybrid_current=false`), top 5:

| query | rank |
|:--|--:|
| Height-compressing global lifts elliptic surfaces | 1 |
| Prime-to-p jets canonical Teichmuller lifts p-adic formal-group digits | 1 |
| Graphical Algebraic Geometry holant Semaev decomposition chain | 1 |
| Quadratic-phase Gowers U3 correlation small-x predicate discrete log | 1 |
| Miller functions S-unit descent pairing-free Miller cocycles | 1 |
| Growth of Groebner solving degree for prime-field summation-polynomial systems | 1 |
| Universal algebraic-factor-base no-go prime-field index calculus | 1 |
| ECDLP lead atlas already tried | 1 |
| curve identity across repositories ecdb comparisons | 4 |

Top-1 8/9 (88.9%), top-5 9/9 (100%), MRR 0.917.

## What this does and does not justify

- The distinctive technical questions that precede a lead, a factor base or a
  novelty claim resolve correctly. The one rank-4 query still returns the
  expected record in the top five, next to the same file in the sibling repo.
- It does not justify a hybrid index yet. Nothing here shows a lexical recall
  failure that an embedding model would fix; building one adds infrastructure
  and a second source of ranking drift.
- It also does not establish general recall. The set is small and chosen from
  distinctive record titles, so it measures reachability of named records, not
  coverage of every useful passage.
- The honest next trigger for an index change is a measured drop here, or a
  session that names a real query the lexical backend misses. Record the query
  and the expected record first, then change the index, then rerun the probe.

## Rules for changing the set

- Add a query only when a real session needed it, and record the expected
  `repo:path` from the source itself.
- Never remove a failing query after a change; a lower number is information.
- Keep the set fixed between index changes so the delta means something.
