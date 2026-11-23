# Atlas delta for the bigger-ring exits (2026-11-11)

This session cannot push to `isogeny-labs/crypto-autoresearcher` (GitHub access here is scoped to
`crypto` and `cryptanalysis`), so the atlas edit is delivered as `ATLAS_DELTA.patch` (a `git diff`
against the sparse clone of `crypto-autoresearcher` `main` at `3ba0739cb9`, paths under
`knowledge/leads/ecdlp/`). Apply it there with `git apply`, then run `python3 tools/lead_atlas.py
--check` in the full clone (the sparse clone used here lacks `ledger/`, so its `--check` baseline
already reports 1,567 reference errors and cannot validate anything) and `--stamp` per the
`lead-atlas` skill.

## What was re-checked in the atlas (read from the YAML, not assumed)

| lead | `disposition` | `last_reviewed` | obstruction kind | what it closes |
|:--|:--|:--|:--|:--|
| `LEAD-ICP-extension-field-embedding` | `open_next_step` | 2026-11-01 | cost model (Gaudry `p^{2−2/k}`) | the `F_{p^k}` exit |
| `LEAD-EXO-padic-jet-canonical-lift` | `open_next_step` | 2026-11-01 | proof (prime-to-`p` jets lift uniquely; jet oracles GGM-simulable) | the jet exit **as a logarithm channel** |
| `LEAD-TRF-padic-canonical-section` | `closed_obstructed` | 2026-10-29 | theorem (the formal logarithm kills the only homomorphic section) | the torsion section **as a logarithm channel** |

So the note's earlier sentence "the jet exit is the only one of the three still open in the atlas"
was imprecise: by `disposition`, two of the three (`extension-field-embedding` and
`padic-jet-canonical-lift`) read `open_next_step`, each with a recorded obstruction, and one reads
`closed_obstructed`. None of the three entries asks the equation-budget question (extra
cross-summand equations for the decomposition system); that is what the delta adds.

## What the patch changes

- `LEAD-EXO-padic-jet-canonical-lift`: two evidence entries (`jet_exit/README.md`,
  `examples/jet_digit_degree.rs`) and one `forecloses` line: torsion-section `p`-adic digits as extra
  cross-summand equations for PKM tower systems, generic degree on Kummer bases at toy scale.
- `LEAD-TRF-padic-canonical-section`: one evidence entry (the section as an equation source).
- `LEAD-ICP-pkm-tower-algebraic-oracle`: two evidence entries (the multiples presentation, the
  null-support slope, the slope threshold and the budget; the problem record).
- `last_reviewed` set to 2026-11-11 on the three files. Dispositions are left unchanged: changing one
  is the atlas reviewer's call under the `lead-atlas` skill, and the measurements here are toy-scale
  development checks, `[pcc:unverified]`.
