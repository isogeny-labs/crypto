# Fix brief

Four review files list findings against the treatise: /tmp/claude-0/-home-user/bcb81261-6b2f-5531-9c6f-150f3e51a9ce/scratchpad/review/{front,algos,fields,frontier}.findings.md. Each finding names `<section>.tex:<line>`, what is wrong, evidence, and a replacement.

You own ONLY the section files listed in your task, under /tmp/claude-0/-home-user/bcb81261-6b2f-5531-9c6f-150f3e51a9ce/scratchpad/wt3/papers/ecdlp-index-calculus-treatise/sections/. Edit only those files (other agents are editing the others at the same time). Do not run git commands that change state (no checkout, commit, stash, merge); do not run make. Disk is nearly full.

For every finding, in ALL four review files, whose location is one of your files:
1. Re-verify it. Reviewers can be wrong. Re-read the cited primary source with `git -C /home/user/crypto show origin/main:<path>` (or /home/user/crypto-autoresearcher). Apply only if the source supports the finding.
2. Apply the smallest correct edit. Match the treatise's LaTeX style (\art{} paths, booktabs, existing macros; cite only bib keys already in refs.bib). For "duplication" findings: keep the full account in the location the finding names, cut the other to a short summary plus `\S\ref{...}` or `Chapter~\ref{...}`, and keep every label that something else references (grep all sections for `\ref{<label>}` before removing any label).
3. Cross-file consistency: if a fix changes a quantity that other files also state, note it in your log so the owner of that file can align it. Do not edit their files.
4. After editing, check that your files keep balanced braces and environments, and that every \ref you add points to a label that exists (grep the sections).

Log every finding you handled to `/tmp/claude-0/-home-user/bcb81261-6b2f-5531-9c6f-150f3e51a9ce/scratchpad/review/<group>.applied.tsv` with the columns `review-file F<n> | file:line | applied|skipped|partial | reason/evidence`.

Reply with ≤150 words: counts applied, skipped and partial; any finding you skipped because the source did not support it; any cross-file follow-up.
