# PKM map relation-yield sweep (2026-11-23)

Status: fixed before the first run. This is a component screen, not a complete ECDLP comparison.

## Question and inputs

On `p=1000003`, compare genuine PKM auxiliary-curve isogeny maps from the recorded CM and Method 1 artifacts. Each map is the composition of the artifact's rational Vélu steps, with terminal value `c`. Sweep affine input translations `x -> x-delta` as explicit degree-one precompositions. The resulting fiber is `V+delta`; these translations change the placement of a factor base, not its algebraic degree or subgroup structure. The two source artifacts are byte-for-byte copies of `research/pkm_smooth_tower_20261114/data/{cm,isogeny}_p1000003.json` on `origin/main` at task start. SHA-256: CM `2609a9d4391f63e12e2a2818f888e05f8e951d9ec85213b2d4e588e47d2f0ed2`; Method 1 `1a9fb81b7092529f5fff607fa23f772b3a76ea2bd10e4fe22af0b53f5389363e`.

Target curve: `E: y^2=x^3+2x+3` over the same field. Each candidate uses *all* abscissae in the map fiber that lift to `E`, with both signs. Its control is the first exactly matching number of lifting abscissae in `[0,p)`. Thus each comparison has equal factor-base size, and the candidate's membership map describes the entire candidate base before the unavoidable curve-lifting filter. Fixed target points are generated from seed 20261123, independently of all factor bases. Use the first 48 for screening and the next 96 as untouched confirmation. For every target, count one success if a verified two-point decomposition exists and separately count ordered decompositions. Include failures and all candidates in the JSONL record.

## Work and acceptance

Measure map evaluation, base construction, and all target-query wall time on one host; report CPU time too. Include source-map degrees, tower sizes, fiber checks, lifting counts, and field inversion counts. Source-artifact search/build time is unavailable and must be marked uncharged; these results cannot support a cold end-to-end speed claim. A candidate advances only if its screen success count exceeds its size-matched interval control and its confirmation has (1) greater success count, (2) a paired one-sided exact binomial tail below `0.01 / candidates_tested` on discordant targets, and (3) lower measured query CPU per successful target, including failures. This gate is intentionally difficult; random placement alone is not a useful structural gain.

Sweep deterministic deltas `0,1,2,4,8,16,32,64` for each source map, capped at 16 candidates. Stop after the cap or the confirmation gate. All failures are retained. Repeating with more candidates or changing targets requires a new protocol version. No cloud job or source search beyond the two recorded maps is authorized by this protocol.

Sources: Petit, Kosters, Messeng, [Algebraic approaches for the ECDLP over prime fields](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf), §§3.1–3.4; `docs/ic/SWEEPER.md`; `research/isogeny_vs_mu_factor_base_20261102/`; KB `source:81a446d8a0f88023cfe934a2fcb608bb` (prior `prime-algebraic` sweep).
