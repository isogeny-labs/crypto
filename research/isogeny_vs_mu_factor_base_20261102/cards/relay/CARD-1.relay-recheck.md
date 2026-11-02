# Relay re-check: CARD-isogeny-vs-mu-factor-base-20261102-1 (main session, 2026-11-02)

The director's assessment (`../CARD-isogeny-vs-mu-factor-base-20261102-1.assessment.yaml`)
found `relay_verbatim: false`. It traced this to 12 `output ...` lines whose
paths were shortened to `.../`. The shortening was done by the main session
when it pasted the executor's text into the director's assess prompt. The
executor's own hand-back carried the full paths.

The re-check runs on the executor's text exactly as it arrived
(`CARD-1.executor-report.txt`):

- `check-relay` still reports `relay_verbatim: false`, but the only lines it
  lists are the 10 `output` lines that are not already in sorted order.
- Cause: the `brief` that `tools/task_card.py run` prints lists outputs in
  card order. The receipt is written with `sort_keys=True` (line 536), so the
  `brief` that `check-relay` recomputes from the receipt lists them sorted,
  and `_missing` requires the lines in order.
- The same text with only the `output` lines sorted
  (`CARD-1.executor-report.sorted.txt`) passes:
  `relay_verbatim: true` and `missing_lines: []`.

Conclusion: the executor's relay was verbatim; every digest equals the
receipt. The failure comes from a line-order mismatch inside the tool, and
`check-relay` will flag every multi-output card listed out of sorted order.
`tools/task_card.py` is canonical in aburan28/crypto-autoresearcher and is not
edited here; the fix belongs upstream. The execution itself was assessed as
matching the card.
