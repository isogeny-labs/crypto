# Review brief: treatise expansion (264 pages)

You are an independent reviewer. Be adversarial: assume every producer, including the drafters and the survey digests, may have made errors or overclaimed. Do NOT edit any file in the repository. Write findings only.

Sources:
- Treatise sections: /tmp/claude-0/-home-user/bcb81261-6b2f-5531-9c6f-150f3e51a9ce/scratchpad/wt3/papers/ecdlp-index-calculus-treatise/sections/<name>.tex (read-only; this is the top of the PR stack, branch claude/treatise-expansion-8-20261106).
- Verification logs the drafters wrote: .../papers/ecdlp-index-calculus-treatise/survey/verify/*.verify.tsv (claim | value | source).
- Primary sources: read with `git -C /home/user/crypto show origin/main:<path>` (and /home/user/crypto-autoresearcher, `git show origin/main:<path>`). Do not check out, build, download or run anything; disk is nearly full.

Check, in your assigned sections:
1. Accuracy: pick at least 25 numbers spread across your sections (favour headline numbers, numbers in abstracts/summaries/tables, and anything compared with rho) and re-read each from its primary source. Report every mismatch.
2. Standing: any result called validated/proved/verified/improvement without a validator report, pcc proof, or Lean kernel check behind it; any measured result not labelled with its standing where the source has none; any "beats/faster/below rho" wording outside the pricing-band rule (rank only where the order holds at w = 0, 0.1, 1).
3. Consistency: contradictions between your sections and the rest of the treatise (grep the other sections for the same quantity, study dir or claim). Especially: older chapters (abstract, intro, model, prog_*, summer, distance, audit, conclusion, prog_openproblems) that still state something a newer chapter corrects.
4. Duplication: the same result told twice at length in different chapters (name both locations; recommend which keeps the full account).
5. Clarity: sentences that are wrong, garbled, or unreadable; unexplained jargon at first use; tables whose columns are not defined.
6. Gaps: important results in your sections' area that are still missing (name the source dir).

Output: write `/tmp/claude-0/-home-user/bcb81261-6b2f-5531-9c6f-150f3e51a9ce/scratchpad/review/<yourgroup>.findings.md` with one finding per entry:
`## F<n> [severity: blocker|major|minor] <section>.tex:<line>` then: what is wrong (quote the text), evidence (source path:line + quoted snippet), the exact replacement text or action.
Severity: blocker = factually wrong number/claim or overclaim about rho/validation; major = contradiction, missing standing label, stale text, large duplication; minor = clarity.
Reply with ≤150 words: counts by severity and the 5 most serious findings in one line each.
