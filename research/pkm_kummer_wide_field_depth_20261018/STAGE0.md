# Stage 0 (PROTOCOL.md §3): identity and memory sizing — recorded 2026-10-18

| cell | prime | wall | trace peak | compared with the study's row |
|:--|:--|--:|--:|:--|
| `m = 4, N = 12`, target 0 | `p_40` (Fp52 IFMA) | 20 s | 315 MB | identical: `D` 7, `max_cols` 9452, `steps` 29, `muladds` 51,598,623,198, `inconsistent` true, `x_r`, `planted_ok`; `pkm_tower_check verify` agrees |
| `m = 4, N = 12`, target 0 | P-224 (FpWide<4+spare>) | 300 s | 1,138 MB | identical on the same fields; in-row `--verify` ok, verdict agrees |

**Identity holds at both primes.** Memory multiples against the recorded
`p₁` peak of this cell (171 MB, PKM note): `R_p40 = 315/171 = 1.84`,
`R_P224 = 1138/171 = 6.65` (*derived*). Stage-2 derived peaks
(`peak(p₁) × R_p`): `m = 4, N = 16`: 14.9 GB at `p_40`, 53.9 GB at P-224;
`m = 3, N = 18`: 23.4 GB at `p_40`, 84.5 GB at P-224. `R_p` comes from a small
cell where fixed overhead weighs more, so these are estimates.

**Launcher fix (disclosed, before stage 1).** `run.sh`'s sampled RSS in
`.meta` read 0 for both stage-0 processes: it sampled only children of the
launched process, which `exec`s the engine. Fixed to sample the process and
its children; the engine's own trace (`peak … MB`) is the reference figure
throughout and was unaffected.
