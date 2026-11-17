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

## Companion repositories

`companion.tsv` has the same columns with a leading `repo` column; paths are
relative to that repository at these commits (default branches, 24 October
2026):

| repo | commit | visibility |
|---|---|---|
| crypto-autoresearcher | ab0609ebee | public |
| cryptanalysis | c5aa7b39bf | private |
| ecc-challenges | 86f8a57a5e | public |
| ml-cryptanalysis | 58494969fd (branch csd-milestone-1) | private |
| cryptanalysis-rl | 24d90557b2 | private |
| ecbench | 0657f29b3d | public |
| isogeny-labs | d613806737 | public |
| isolab | eb3f285cc1 | private |
| ecdlp-prime-field-smallness | 4f20be9b36 | private |
| crypto-lean | 148f5dc389 | public |

Coverage units: in crypto-autoresearcher every `experiments/*` directory,
every `ledger/evidence` and `ledger/corrections` record, every knowledge
finding, technique, open problem, frontier and lead, every `research/` and
`analysis/` entry, and one counted row each for the process and literature
record directories; in cryptanalysis every entry of `experiments/`,
`research/`, `ecc2k130/`, `suite/`, `challenges/`, `reports/`, `benchmarks/`,
`docs/` and every other top-level entry; in the smaller repositories every
top-level entry. `../tools/companion.py --check <repo> <units>` reports units
with no row (0 for every repository at these commits). Rows for
crypto-autoresearcher specifications, evidence and corrections were extracted
from the records by script; the rest were written by readers. Of 2,531
numbers in reader-written headlines, 87.6% appear literally at the pinned
commit; the numbers used in the paper's text were read from source.
Out of scope: the local 48 GB Autolab byte mirror (not on GitHub), conductor,
cairn, poltergeist and progress-metrics (infrastructure), ghs-trapdoor
(two files), and the WDSat and cado-nfs forks (third-party engines).
