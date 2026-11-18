# Deviations (each logged before the data it concerns was read)

- **D1 (2026-11-18 19:20 UTC).** The p = 32771 search (`search.sh 32771 1800`) was stopped by the
  2 h background-task limit. Residues 0, 1 and 2 had completed (their `cell` rows are present).
  Residue 3 (seeds ≡ 3 mod 4) had not.
  - **What was found before the stop.** Prime-order seeds 882, 1461, 355 and 1623.
  - **Re-scan.** `search_chunks.sh` re-scans residue 3 over 1..1800 in chunks of 28 seeds, split
    across 4 processes. Every chunk writes its own `cell` row, so a later stop leaves a known
    covered range.
  - **Effect on the rule.** None: the search is deterministic, and the frozen rule ("5 smallest
    admitted seeds in 1..N, extend in blocks of 400") is unchanged.
  - **Reading the files.** `curves.py` reads every `search_p32771_*.jsonl*` file; duplicate
    instance rows are the same seed.
- **D2 (2026-11-18 20:50 UTC).** Extension blocks under the frozen rule:
  - 1801–2200 found none.
  - 2201–2600 found 2374, 2508, 2579 and 2600.
  - 2601–3000 was queued in the same command and then stopped, because it cannot change the 5
    smallest. Its partial files were deleted unread.

  The chosen p = 32771 seeds are 355, 882, 1461, 1623 and 2374.
