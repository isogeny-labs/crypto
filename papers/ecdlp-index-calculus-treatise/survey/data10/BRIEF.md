# Treatise data pass (expansion 10): brief for each data agent

The user asked for "more data, and no AI slop" in the treatise
`papers/ecdlp-index-calculus-treatise/` (268 pages, 110 tables, only 15 figures).
Your job: for the chapters you own, put the committed measured data into the
text as tables and figures, and strip slop from those chapters.

## Where things are
- Worktree (sparse, branch claude/treatise-data-10-20261106):
  `/tmp/claude-0/-home-user/bcb81261-6b2f-5531-9c6f-150f3e51a9ce/scratchpad/wt3`
  Treatise: `papers/ecdlp-index-calculus-treatise/` (`treatise.tex`, `sections/*.tex`,
  `figures/`, `refs.bib`). Do NOT widen the sparse checkout.
- Repository data is NOT checked out. Read it from git without checking out:
  `git -C <wt3> ls-tree -r --name-only origin/main -- research/<dir>` and
  `git -C <wt3> show origin/main:<path>`. Disk is tight (about 3 GB free):
  never `git checkout` research data, never copy large files.
- Earlier survey digests (what exists and where): `papers/ecdlp-index-calculus-treatise/survey/*.md`
  and `survey/verify/*.verify.tsv`; review findings in `survey/review/`.

## What to add
1. Find the raw data behind each quantitative claim in your chapters: TSV/CSV/JSON
   under `research/`, `benchmarks/`, `docs/ic/leaderboard.json`, VALIDATION.json,
   RESULT/REPORT tables. Prefer per-instance or per-cell rows over summaries.
2. Add data tables where the text gives only a range or a sentence: per-cell rows
   with n/p/k, counts, medians, bootstrap intervals, the ratio to matched rho and
   its interval, and the row's standing (validated / unvalidated (pre-gate) /
   producer-reported). Put long tables in `longtable` or split them.
3. Add figures where a series exists (scaling with n or p, ratio to rho with
   intervals, distributions). For each figure:
   - extract the plotted values into `figures/data-<slug>.json` with a `sources`
     list (path at origin/main plus the git blob sha from `git rev-parse origin/main:<path>`);
   - plot with a script `figures/make_fig_<slug>.py` (matplotlib 3.11 is installed;
     read only the data JSON; write `fig-<slug>.pdf` and `.svg`; no titles inside
     the plot, labelled axes with units, log axes where values span decades,
     error bars for intervals, a dashed line at ratio 1 for comparisons with rho,
     colour-blind-safe colours, readable at \linewidth);
   - include it with `\begin{figure}[t]\centering\includegraphics[width=\linewidth]{figures/fig-<slug>.pdf}\caption{...}\label{fig:<slug>}\end{figure}`
     and cite the artefact in the caption with `\art{...}`.
   Use a unique slug prefix for your part (given in your task) so files never collide.
4. Every number you add must be copied from its source, not recomputed by hand. If
   you compute a derived value (a ratio, a median), do it in the figure script or a
   small script and say so in the caption. Never invent or round beyond the source's
   precision. If a source conflicts with the current text, fix the text and log it.
5. Keep standing honest: a producer-reported number is labelled as such; a pre-gate
   row says "unvalidated (pre-gate)"; nothing is called a speed-up, improvement or
   win unless the source's VALIDATION.json says so.

## No AI slop (applies to every sentence you write or touch in your files)
- No filler or hype: delete "notably", "crucially", "importantly", "significantly"
  (unless statistical), "remarkably", "it is worth noting", "it should be noted",
  "in summary", "in conclusion", "overall", "comprehensive", "robust", "landscape",
  "delve", "leverage", "underscore", "highlight(s)", "pivotal", "nuanced",
  "seamless", "a testament to", "plays a key role", "sheds light", "paves the way".
- No "not just X but Y", no "it's not X, it's Y", no rhetorical questions, no
  triplets for rhythm, no closing sentence that restates the paragraph.
- No hedge stacks ("may potentially suggest"). State the number and its interval.
- One claim per sentence. Prefer a table row to a sentence listing numbers.
- Em dashes only in number ranges (--); use commas, colons or full stops instead.
- Do not narrate the document ("In this section we will..."); start with the content.
- Keep mathematical prose terse and exact.

## Ownership and building
- Edit ONLY the section files listed in your task and figure/data files with your
  slug prefix. If another file needs a change, write it to your log instead.
- Do not run `make` in the worktree (other agents share it). To check your work,
  copy to your own build dir and build there:
  `B=<scratchpad>/data10/build-<slug>; rm -rf $B; mkdir -p $B/papers; cp <wt3>/papers/paper.mk $B/papers/; cp -r <wt3>/papers/ecdlp-index-calculus-treatise $B/papers/; cd $B/papers/ecdlp-index-calculus-treatise && make && make check`
  Your files must add no undefined references and no overfull boxes. Delete the
  build dir when finished.
- Do not commit or push. Do not run experiments.

## Report
Write `<scratchpad>/data10/<slug>.log.tsv` with one row per change:
`file  kind(table|figure|text|slop)  what  source_path  verified(yes/no)`.
Return a short summary: tables added, figures added, rows of data added,
corrections to existing numbers (old -> new, source), slop edits count,
anything you could not source.
