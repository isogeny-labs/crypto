# Corrections to Delphi program 69640fc0955f

Operator note, 2026-11-05. This program was the first live run of
`delphi program`: OpenRouter free models only, two rounds, with
`atlas-match`, `card-lint` and `writeback` as its only tools and no compute.
It tested the loop's mechanics. It produced no measurement.

**Round 2's outcome "refuted" is void.** Its worker passed lead ids to
`atlas-match`, which then ran `lead_atlas.py --match`, a text search. An id
finds nothing that way, and the tool also could not parse the atlas output
(`--match --json` prints its text table after the JSON). The program
reviewer read the empty answer as "the lead does not exist" and refuted the
claim. `LEAD-ICP-full-width-arity-grid` exists, with disposition
`open_blocked` (`knowledge/leads/ecdlp/ic-prime/` in crypto-autoresearcher).
The claim about it was neither tested nor refuted [pcc:unverified].

**Round 1's outcome "inconclusive" rests on the same defect.** Its reviewer
said `LEAD-ICE-charged-toy-ic-exponent-vs-rho` returned an empty array for the
same reason [pcc:unverified].

Fixed in `tools/delphi_tools` (crypto, branch
`claude/delphi-atlas-writeback-20261105`). `atlas-match` prints a record when
given a `LEAD-`/`KR-` id. A missing id now says "no record with this id"
instead of returning an empty search. Only the leading JSON value of the
atlas output is read. Nothing in this directory changes a lead's disposition.
