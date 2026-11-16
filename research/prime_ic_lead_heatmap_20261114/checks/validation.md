# Lead verdict validation report

Verdict directory: `/tmp/claude-0/-home-user/c2970330-965d-56e2-8425-0faf587c0f40/scratchpad/verdicts`

## 1. Files

| File | Records | Parse status |
|---|---|---|
| W1 | 12 | ok |
| W2 | 18 | ok |
| W3 | 28 | ok |
| W4 | 35 | ok |
| W5 | 27 | ok |
| W6 | 28 | ok |
| W7 | 40 | ok |
| W8 | 48 | ok |
| W9 | 68 | ok |

Total records: **304** across 9 files.

## 2. Schema

### Records missing required fields (0)
None.

### Bad enum or type values (0)
None.

### Labels longer than 38 chars (0)
None.

### Duplicate ids across files (0)
None.

### Duplicate ids within a single file (0)
None.

### pcc field not equal to "unverified" (0)
None.

### Non-object entries in arrays (0)
None.

## 3. Citation check

| Outcome | Citations |
|---|---|
| resolved | 1298 |
| vaulted (expected unreadable) | 0 |
| missing (no file found) | 0 |
| non-path note (not checked) | 292 |

### Per worker
| File | Resolved | Vaulted | Missing | Non-path |
|---|---|---|---|---|
| W1 | 59 | 0 | 0 | 12 |
| W2 | 76 | 0 | 0 | 18 |
| W3 | 122 | 0 | 0 | 28 |
| W4 | 127 | 0 | 0 | 35 |
| W5 | 210 | 0 | 0 | 27 |
| W6 | 135 | 0 | 0 | 28 |
| W7 | 172 | 0 | 0 | 0 |
| W8 | 192 | 0 | 0 | 144 |
| W9 | 205 | 0 | 0 | 0 |

### Missing citations, grouped by worker
None.

### Non-path notes (free text, not checked)
- x48 `crypto:docs/session-logs/2026-11-10.md..2026-11-14.md (grep for the id`
- x48 `cryptanalysis: (grep for the idea id: no hit)`
- x48 `research-kb and ecdb unavailable in this container; grep used`
- x35 `research-kb MCP and ecdb not available in this container (grep only)`
- x28 `not available: research-kb MCP, ecdb (grep used)`
- x28 `research-kb MCP and ecdb not available in this container; grep over /h`
- x27 `note: research-kb MCP and ecdb unavailable in this container; grep onl`
- x18 `research-kb MCP and ecdb NOT available in this container; grep used in`
- x12 `research-kb MCP and ecdb not available in this container; grep only`

## 4. Consistency flags

### evidence measured but why_this_verdict cites only 'toy' (review) (11)
- `LEAD-ICP-residual-partial-relations` `W1`
- `LEAD-ICP-gfp5-symmetrized-gaudry` `W2`
- `LEAD-ICP-isogeny-transported-relation-yield` `W2`
- `LEAD-ICP-retained-setup-online` `W2`
- `LEAD-ICE-rootfind-pairtable-oracle` `W3`
- `LEAD-ICE-charged-toy-ic-exponent-vs-rho` `W3`
- `LEAD-ICE-semaev-yield-model-coset-typing` `W3`
- `LEAD-STR-class-representative-ic-selection` `W3`
- `LEAD-STR-special-j-ic-semaev` `W3`
- `LEAD-STR-curve-generation-provenance-seeds` `W3`
- `LEAD-TRF-anomalous-padic-lift` `W4`

### evidence proof but why_this_verdict cites only 'toy' (review) (8)
- `LEAD-ICE-generic-oracle-arity-cost-floor` `W3`
- `LEAD-STR-isogeny-walk-weak-neighbour` `W3`
- `LEAD-STR-secp256k1-cm-class-audit` `W3`
- `LEAD-STR-jmv-random-self-reduction` `W3`
- `LEAD-TRF-padic-canonical-section` `W4`
- `LEAD-EXO-curve-model-representation` `W4`
- `IDEA-20261109-1130d0` `W5`
- `IDEA-20261109-c4cd2e` `W6`

Info: 9 flagship record(s) with verdict open or blocked (expected, not flagged).

## 5. Summary

### Verdict x column
| verdict \ column | FB | ORACLE | LIFT | TRANSFER | BARRIER | ENG | total |
|---|---|---|---|---|---|---|---|
| refuted | 21 | 30 | 23 | 35 | 49 | 8 | 166 |
| engineering | 4 | 1 | 0 | 1 | 1 | 4 | 11 |
| open | 3 | 6 | 9 | 2 | 3 | 0 | 23 |
| blocked | 4 | 10 | 0 | 1 | 2 | 0 | 17 |
| unexamined | 2 | 5 | 4 | 0 | 9 | 0 | 20 |
| out-of-scope | 2 | 3 | 0 | 6 | 7 | 49 | 67 |
| total | 36 | 55 | 36 | 45 | 71 | 61 | 304 |

### Verdict x evidence_strength
| verdict \ evidence | proof | measured | toy | desk | none | total |
|---|---|---|---|---|---|---|
| refuted | 27 | 8 | 75 | 56 | 0 | 166 |
| engineering | 0 | 5 | 4 | 2 | 0 | 11 |
| open | 0 | 3 | 12 | 5 | 3 | 23 |
| blocked | 0 | 3 | 8 | 1 | 5 | 17 |
| unexamined | 0 | 0 | 2 | 0 | 18 | 20 |
| out-of-scope | 0 | 3 | 6 | 55 | 3 | 67 |
| total | 27 | 22 | 107 | 119 | 29 | 304 |

### source_kind counts
| source_kind | count |
|---|---|
| atlas | 93 |
| idea-pr | 143 |
| crypto-study | 68 |

### Flagship list (9)
- `LEAD-ICP-pkm-tower-algebraic-oracle` (blocked, ORACLE, W1)
- `LEAD-TRF-global-lift-small-height` (open, LIFT, W4)
- `LEAD-EXO-global-lift-height` (open, LIFT, W4)
- `IDEA-20261109-2debfa` (open, ORACLE, W6)
- `CL-A8` (open, LIFT, W9)
- `CH-L1` (open, ORACLE, W9)
- `CH-L4` (open, LIFT, W9)
- `ENC-0.1` (open, FB, W9)
- `ENC-0.2` (open, FB, W9)

## Headline

- Records: 304; schema issues: 0; missing citations: 0; vaulted citations: 0; consistency flags: 19; file parse failures: 0.

