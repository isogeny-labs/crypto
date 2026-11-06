# Research KB usage audit

The KB rule (AGENTS.md, “Knowledge base first”) asks every research session to
record a **Sources checked (KB)** receipt: a `source_id` and `repo:path`, or an
explicit `KB unavailable: <why>`. This audit turns that rule into a number.

It reads only `docs/session-logs/*.md`; it never calls the KB, edits a log or
changes a result.

## Run it

From a repository in the sibling layout:

```sh
python3 scripts/kb_usage_report.py --since 2026-11-05
python3 scripts/kb_usage_report.py --since 2026-11-05 --json \
  --out docs/knowledge-base-usage-baseline.json
```

`--root` may be repeated to name repositories explicitly; without it the
script audits this checkout plus the sibling `crypto`, `crypto-autoresearcher`,
`cryptanalysis` and `crypto-lean` when present. `--fail-under 0.5` makes a batch
fail below that coverage; the default is report-only.

`--ref` reads the logs from a git ref instead of the working tree, so a
measurement of committed history ignores uncommitted sessions:

```sh
python3 scripts/kb_usage_report.py --ref origin/main --since 2026-11-05
```

### Scheduled report

`.github/workflows/kb-usage-report.yml` runs the audit weekly and on demand in
`crypto`. It writes the table to the step summary, uploads the JSON as an
artifact and uses `--ref HEAD`. It is advisory: no pull request is blocked and
`--fail-under` is not wired in.

The script and its tests are shared byte-identically by the three repositories
(`docs/shared-code.lock.json`), so each repository runs the same workflow
against its own logs. Its first run (2026-11-06) reported `crypto`
alone at 135 entries, 11 receipts and 8.1%, which matches the local
`--ref` measurement at that commit.

## What counts

- **Entry**: a dated `##` session-log entry, or a `### <time> UTC` entry (the
  `crypto-autoresearcher` style). Subheadings are part of the entry above.
- **Receipt**: the entry contains `Sources checked (KB)`, a `Sources checked:`
  line that names `KB`, or `KB unavailable`.
- **Source IDs**: `source:…`, `paper:…` and `lean:…` identifiers in the entry.
- **Coverage**: receipts / entries in the selected date range.
- **Duplicates**: an exact repeated heading (the whole heading line, including
  the date and usually the time) counts once and is reported as `duplicates`.
  Merge conflicts have copied whole dated blocks into one file; a repeat is the
  same entry, not a second session.

An unavailable receipt counts because the session recorded the attempt and the
gap; pretending the KB was consulted is worse than saying it was down.

## Baseline at policy landing

`docs/knowledge-base-usage-baseline.json`, entries dated 2026-11-05 or later:

| repository | entries | receipts | coverage | unique source IDs |
|:--|--:|--:|--:|--:|
| `crypto` | 112 | 2 | 1.8% | 6 |
| `crypto-autoresearcher` | 4 | 1 | 25.0% | 3 |
| `cryptanalysis` | 2 | 1 | 50.0% | 3 |
| **total** | **118** | **4** | **3.4%** | **6** |

The first number is intentionally low: the policy landed on 2026-11-05 and the
logs before it have no receipt. The four receipts are the policy entries
themselves; the sibling entries live on their policy branches until those PRs
merge. Use it as the before mark, not a verdict.

## Current measurement

Committed `main`, entries dated 2026-11-05 or later, measured after the policy
PRs merged:

| repository | entries | duplicates | receipts | coverage | unique source IDs |
|:--|--:|--:|--:|--:|--:|
| `crypto` | 109 | 31 | 15 | 13.8% | 7 |
| `crypto-autoresearcher` | 10 | 0 | 4 | 40.0% | 5 |
| `cryptanalysis` | 4 | 0 | 3 | 75.0% | 5 |
| **total** | **123** | **31** | **22** | **17.9%** | **7** |

The delta from the 0.9% pre-policy mark (1 receipt in 114 entries) is +17.0
points. Later sessions now record receipts on their own, not just the policy
entries, so the number is beginning to measure habit rather than the template.

## Reminder window, 2026-11-06

The cross-runtime reminder merged at 2026-11-06T04:29Z. Unique entries by date:

| date | entries | receipts | coverage |
|:--|--:|--:|--:|
| 2026-11-05 | 110 | 18 | 16.4% |
| 2026-11-06 | 9 | 2 | 22.2% |

Both 2026-11-06 receipts belong to this programme's own audit entries. Of the
three non-programme sessions with a timestamp after the merge, none recorded a
receipt. One day and three sessions is not a trend; the number to watch is
whether later sessions record receipts without the audit prompting them.

## Diagnosis: usage gap, not a recording gap (2026-11-06)

Committed `main`, entries dated 2026-11-05 or later:

| measure | value |
|:--|--:|
| entries as written | 140 |
| exact duplicate headings | 31 |
| unique entries | 109 |
| receipts | 15 |
| unique entries without a receipt | 94 |
| of those, entries that mention the KB at all | 2 |
| of those, entries with a query or `source:` signal | 0 |

The receiptless entries are overwhelmingly merge, custody, CI and verification
work. Only two mention the KB in prose, and neither records a query or a
source. So the crypto gap is not a recording gap: sessions are largely not
querying the catalog in the first place.

One client-side asymmetry the logs support: the Claude Code hook printed the
reminder, while `scripts/relay.py session-start`, which Codex also runs,
printed nothing unless a runtime handoff was pending. The reminder now comes
from the relay for every runtime, and the Claude-only echo is removed.

The 31 duplicates are merge artifacts: whole dated blocks copied into the same
file. The audit now counts an exact repeated heading once and reports the
skipped count, so the number above is the deduplicated one.

## What the audit does not see

- Pull-request bodies and chat answers (the PR-body field is a later step).
- Work in repositories not checked out beside this one.
- Whether the session read the source it cited; the receipt is a pointer, and
  the evidence rules still apply.
- Sessions that used the KB but forgot the receipt. A low number can mean
  “used but unrecorded”; that is still a process gap.

## Next measurements

- Read the weekly workflow artifact and keep the delta; the goal is a rising
  trend with no drop in evidence quality, not a quota.
- Add a `Sources checked` field to the PR template once receipts are habitual.
- If coverage stalls, inspect the logs for the failure mode (unknown tool,
  unavailable server, no trigger) before adding more policy.
- Raise retrieval quality only against a measurement: lexical misses first,
  then the hybrid index.
