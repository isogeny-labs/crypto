# Catalogue data

`catalogue.tsv` lists every `research/` study directory, `benchmarks/` row,
`problems/` record, research note and `docs/ic/` programme document on `main`
at 5cb553810e (2026-10-24), one row each, plus `CLUSTER:<name>` summary rows
for campaigns. `../tools/catalogue.py` turns it into the appendix
(`../sections/catalogue.tex`) and, with `--check <list>`, reports any listed
path that has no row.

Columns: `path`, `date`, `family` (koblitz / binary / prime / extension-cover /
rho-harness / cross-cutting / out-of-scope), `topic`, `class`, `headline`
(the study's own headline number, copied from its README or results file),
`label` (measured / derived / model), `boundary`, `comparator`, `status`
(current / superseded / corrected / withdrawn / open ...), `correcting_path`,
`cluster`.

How the rows were made: nine readers each took a slice of the paths, read
each study's README and results files, checked for errata against the
validation sweep, the ledger's audit section, `problems/` and the
leaderboard, and wrote one row per path. The rows were then checked
mechanically: of 4,420 numbers in 1,041 headlines, 76% appear literally in
the study's own files. A sample of the remaining 529 was traced by hand to
rounding, to the study's PDF, or to the validation sweep; the rest are not
individually verified. A random sample of rows was read against its
sources for meaning. The
rows are summaries, not re-measurements; the paper's text cites the
primary files directly.
