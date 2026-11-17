# R4b: verification of #2302 ("no registry verdict changed")

Date 2026-10-26. Checked at origin/main `baa1951357`; #2302 is `68e803b045`
(branch commit `2c2e5e5d83`). Label: **MEASURED** (regeneration and diff, run
locally; CI could not run, GitHub Actions jobs do not start).

## Result: the claim is confirmed, with a stronger reason than #2302 gave

1. **Byte-identical regeneration at #2302's own registry.** Registry at
   `2c2e5e5d83` (sha256 `584d070f...a1cf`, 2,572 curves), built with
   `jacobian_transfers build --budget 200000`, is byte-identical to the
   committed `docs/curves/jacobian-transfers.json` at that commit.
2. **Old vs new cost model, same registry.** Comparing the file before #2302
   (C4 = 1.227e12) with after (C4 = 8.69e10): 0 verdict changes, 0 best-route
   changes, 0 route-count changes, 0 changed `log2_total`/`ratio_log2`. The
   summary block is equal. The only differences are `notes` and the
   `decomposition_solve_basis` label (`degree_cubed_floor` ->
   `degree_cubed_estimate`) on 1,324 extension-IC routes.
3. **Why nothing moved:** no registry curve has an extension-IC route with
   k = 4. Routes by k: 2:2, 3:5, 5:1, 6:1050, 10:137, 11:1, 14:128. Only the
   5 k = 3 routes use a measured price. C4 is not consumed by any row, in
   either the 2,572-curve or the current registry. So the 13.9x change in C4
   could not alter a verdict. It would matter only if a k = 4 curve is added.

## Current main is stale for a different reason (not #2302)

The registry has grown since (sha256 `e7e8cd2c...`, 20,592 curves).
`jacobian_transfers check` on main therefore fails ("stale"). Regenerating
at current main against the committed file: all 2,572 committed curves are
present, 0 verdict changes, 1 row differs (`icv1-f2m73-tm184271214331-9d25cc67`,
subgroup status `largest_prime_of_order` -> `registry_representation`, same
order and bits), 18,020 new curves, all `boundary`. Cost model is equal.
The committed JSON/MD were not regenerated here (out of scope; 18k-curve
rewrite). `JACOBIAN_TRANSFERS.md` summary table is identical to
`jacobian_transfers table` output of the committed file.

## Tests and workflow steps run locally

- `cargo test --release --lib jacobian_transfer`: 28 passed, 0 failed.
- `jacobian_transfers show "Oakley Group 3"`, `show ECC2K-130`,
  `rank --top 10`, `rank --class formula_advantage`, `chart --svg --tikz`: all ran.
- Table-vs-doc diff: identical.
- Not meaningful on current main: `check` (stale registry, above) and the
  "reject an altered file" step.

## The D^3 "floor" comment and test (cost.rs ~600-630)

Already consistent. The doc comment on `extension_decomposition_degree_exponent`
calls D^3 an estimate, not a lower bound, and cites #1792's ~2^26 against
2^36. The test `extension_ic_charges_measured_solves_and_k_factorial_trials`
asserts a measured price below D^3 is charged as measured
(`d4.log2_extension_solve_mults(4) < 36.0`). No code change needed.

## Environment: origin/main did not compile when this was run

Merge damage in three files stopped `cargo build`: unclosed delimiter in
`src/cryptanalysis/binary_semaev_chain_sat.rs` (missing `}` after
`verify_external_assignment`, from `4f1fa86ac1`; also a duplicated
`#[derive]` on `FrobeniusOrbitChoiceWires`), and unclosed delimiters in
`src/cryptanalysis/ecbench/methods.rs` and `ecbench/workload.rs` (from
`671a5fb6c7`, large blocks dropped). To measure, these were patched or
taken from the main-side merge parents in the working tree only; none of that
is in this commit. Results above do not depend on those files.
