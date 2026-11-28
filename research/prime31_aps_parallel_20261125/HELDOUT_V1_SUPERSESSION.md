# Held-out V1 input registration amendment

The V1 coordinates in `heldout_public_targets.json` and caller-only answers
in `heldout_caller_answers.json` were generated and committed before any APS
or rho arm was run on them. They are retained for provenance and are **not**
the admitted primary panel. Their generator recorded the target seed, but
did not freeze a complete per-case, per-replicate solver-stream manifest or
the randomized scheduling stream required by
`docs/experiment-seeds.md` (policy `isogeny-labs/experiment-seeds/v1`).

The next panel will use a new V2 study ID, target seed, and public Q set.
It will register all target, APS source, rho walk and logical restart,
scheduling, and analysis seeds before any V2 arm is run. V1 will never be
silently substituted for V2 or pooled with its results. Status:
[pcc:unverified] input-custody amendment, no performance result.
