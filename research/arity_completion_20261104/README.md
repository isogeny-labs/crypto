# Arity completion (2026-11-04): the m ≥ 5 cells that can still change a conclusion

**Status.** Protocols frozen before data; cards compiled; **no cell has run.**
Nothing here is a measurement, an improvement or a distance to rho.

**Why these cells.** The arity review (`research/factor_base_arity_review_20261024`)
and the 2026-11-03 catalogue review found three cell families that no record
covers and whose outcome changes a conclusion: prime-field PKM towers at
m = 5; chained-S3 decomposition with a Frobenius-union base at m = 5 and 6 on
Koblitz n = 23..41; and the same at n = 83 with the 180 s cell cap lifted. The
m = 2..9 prime sweep (`research/ic_arity_sweep_20261027`) and the family-level
bounds in the factor-base encyclopedia §7 say a full factorial run over
families is not justified; the arity dimension is.

| study | question | protocol | status |
|---|---|---|---|
| `prime_tower_m5/` | closure degree D and cost of Kummer and isogeny towers at m = 5, N = 10, 15, 20 | `protocol.json` frozen at `1fd6ece55f`, sha `2b82eecd…` | cards compiled, not run |
| `koblitz_m5_m6/` | can chained-S3 SAT with `union:S` decide m = 5, 6 at n = 23..41 within a cap scaled to the m = 4 root table; n = 83 with the cap at 1800 s | see its `protocol.json` freeze block | drafting / cards pending |
| `cards/` | mechanical task cards (experiment-director compile mode) | binds the frozen files by sha256 | lint `CARD OK`; run after the build card pins the binary |

## Gates before any run (none passed in the authoring container)

The launch gates are listed in `problems/arity-completion-launch-gates.md`:
a valid AWS identity for account `104186811093` (isolab-eks, S3 checkpoint,
artifact store), or a `CHECKPOINT_UPLOAD_TOKEN`; recorded budget headroom;
the resume note pushed. The authoring container's STS call returns
`InvalidClientTokenId`, so the checkpoint hook blocks every launcher here.

## Decisions on the director's specification items (main session, 2026-11-04)

1. **Binary hash.** Cards 2–4 of each study bind the instrument binary only
   after the build card's receipt exists; the credentialed session re-dispatches
   the director with that sha256 and the cards take new ids (write-once).
2. **Outer timeouts.** Build card 3600 s; helper gates 60 s; each cell's
   outer `timeout_seconds` = the protocol's inner `timeout` + 300 s, so the
   inner `timeout` always fires first and the pilot's own cap record is kept.
3. **Capped cells.** A wall cap (exit 124) or a memory abort is
   `inconclusive_cap` under both protocols, not a failure: cell commands list
   `success_exit_codes: [0, 124, 134, 137]` so a capped cell does not abort
   the later cells of its card; the verdict gates still require the controls
   to pass before any candidate cell.
4. **`/usr/bin/time`.** The protocol commands use it; the executing image must
   provide it (`time` package). The authoring container lacks it, which is one
   more reason the smokes here are development checks only.
5. **Controls.** The mechanical gate checks D only; the full row identity
   against `research/ic_next10_20261014/03-pkm-degree/results.csv` is the
   director's assessment step, as the card report says.

## Development smokes (not evidence)

Release builds of `pkm_tower_pilot` and `n83_msweep_cell` were made in this
container after the freezes, and one toy cell per family was run only to
confirm the command lines execute (L0, no isolation, no `/usr/bin/time`).
Their outputs live under `*/smoke/` and are excluded from every analysis.

## dreg v4 (the solving-degree half)

`research/dreg_high_arity_20261101` remains at "refreeze pending, no scientific
run"; its steps a–h (new immutable source commit, four-binary rebuild, canonical
refreeze, draw custody, launch request, account and budget gates) are unchanged
by this study and need the same credentials. Its cells (5,11,2), (5,13,3),
(6,11,2) are the binary-field counterpart of `prime_tower_m5/`.
