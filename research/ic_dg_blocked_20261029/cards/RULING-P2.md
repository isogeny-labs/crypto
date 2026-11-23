# Coordinator ruling on pre-launch check P2 — XP-20261029-dg-blocked

- **Date:** 2026-10-29, `coordinator` role, before any launch of the six jobs.
- **Frozen protocol:** `backlog/XP-20261029-dg-blocked/` at cd04cfd2d1. That directory is not edited.
- **Check report:** `research/ic_dg_blocked_20261029/cards/prelaunch-checks.md` at b985ed20da.
- **Ruling:** (a). The purpose of the P2 refusal clause is met for these six specs. P2 is passed under the added condition A1 below. Launch may proceed once the launch cards cite this file.

## What failed, and why it does not block

**The premise of the clause was wrong.** P2's third clause asked the blocked oracle to refuse p ≥ 2³² or more than 2¹⁶ signed points. That was my assumption, not a property of the code. At `be2969ce`, `PairTable::build_negation_folded_prime_blocked` (`ic_boundary.rs` 5753–5777) takes one of two layouts:
- it takes the `PackedPairTable` layout only when `g.p <= u32::MAX` and `fb.points.len() <= 1 << 16`;
- otherwise it builds the same table in the small-maps layout (`build_negation_folded_prime_blocked_maps`), under the same `kind` string.

**What the clause was for.** The clause existed so that the `dgb` arms really measure the packed, cache-blocked table and not a silent substitute. The branch is a deterministic function of two quantities that the frozen specs fix, so it can be settled from the inputs:
- p: `field_bits` is 24, 28 and 32 on the six curves (`ecbench plan --doc`), so p < 2³² on all six;
- the factor-base size: N_inc ≤ 1024 abscissae, so at most 2,048 signed points (the pilot's 32-bit record shows `signed_points` 2048 and `abscissae` 1024), which is far below 2¹⁶.

By the code the director read, all six curves take the packed branch.

**Why the fallback would not have broken the claim anyway.** The other layout is not the incumbent's builder. It would change what is measured ("packed table") but not the single-variable or identical-charged-work design. Even so, a result reached through it would carry the wrong label. That is why A1 closes the gap at analysis time.

## Added condition A1 (binding; fail-closed; tightens and loosens nothing)

A1 is added to wall-session §6.1 as a tenth condition. It also applies to the counted session's `dgb` records. It is checked by the analysis card and rechecked independently by the validator.

**A1 — packed layout.** For every `dgb` and `dgb-edge` record, warm-ups included, both of the following hold:
1. **Field size.** The curve's p, read from the record's ICV1 curve string, is ≤ 4,294,967,295.
2. **Signed points.** The factor base's signed-point count is ≤ 65,536.
   - It is read from the record's `signed_points` field.
   - If records lack that field, it is read as twice the record's `abscissae` field.
   - If records carry neither field, it is read from `ecbench plan --doc` for the frozen spec, and the analysis states which source it used.

If A1 fails for any record of a size, that size is undecidable. A failure is reported. It is never waived after data.

**Wall data.** A1 uses no wall field. Peak RSS and set-up time are not used to infer the layout.

## Scope consequence

Any result of this study holds for the **packed layout only**: p < 2³² and at most 2¹⁶ signed points. It does not transfer to the small-maps layout of the same oracle string. Fields with p ≥ 2³², or larger factor bases, are untested by this study. The README states this beside the §6.2 scope.

## Record

The committing session must allocate a `coordinator_decision` id for this ruling (`tools/allocate_id.py --next coordinator_decision --date 20261029`, confirmed with `--check`). It must archive the record with `subject: research/ic_dg_blocked_20261029/cards/RULING-P2.md`, `decision: approve` (P2 passed under A1), and `evidence_refs`: `cards/prelaunch-checks.md` at b985ed20da, `ic_boundary.rs` 5753–5777 at `be2969ce`. The launch and analysis cards record this file's sha256.
