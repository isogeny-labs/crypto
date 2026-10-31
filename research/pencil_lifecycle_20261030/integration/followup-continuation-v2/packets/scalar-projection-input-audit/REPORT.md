# Frozen scalar-projection input audit

2026-10-31; T-1125. Producer metadata verification and source audit; independent review pending.

## Recorded result

All nine frozen plan hashes matched the previous reconciliation index. The eight enrollment plans contain 1,260 distinct workload IDs spanning 21 curve metadata identities, each represented by 60 targets. The separate gate has one workload and is excluded from those cohort totals. All 1,261 records pass order divisibility, supplied cofactor equality, duplicate order/r field agreement, Hasse interval containment, and r > 4p arithmetic. All have gcd(cofactor,r)=1. Nineteen cohort curve identities have cofactor16 and two have cofactor32. No prime checks, point arithmetic, target generation, transfer replay, group-order computation or scientific execution was performed. These are supplied-metadata checks, not certificates that the values describe the actual Jacobian.

## Source-chain refinement

The previous report deliberately inspected solve_one locally. The broader call chain adds a concrete guard: src/cryptanalysis/ecbench/workload.rs:268-305 constructs StudyInstance then calls i.validate(). StudyInstance::validate in three_leads.rs:1308-1370 checks primality of r, r*h=order, h not divisible by r, nonzero valid generator and [r]generator=0. For prime r the h guard implies gcd(h,r)=1. This source reading supports the ecbench construction path; direct callers of solve_one must not silently inherit a check they did not call. No new guard was added.

The source also checks the candidate elliptic order lies in the Hasse interval. When r^2 > 16q (here q=p^2), the interval has width4p smaller than r. Given a valid nonzero point of prime order r, the true elliptic order is divisible by r; the interval contains at most one multiple of r. This explains the certificate strategy in interval_order. Metadata audit confirms the interval and width inequalities, but does not replay the primality or point witness. Crucially, this certifies an elliptic order conditional on its checks; transferring it to the Jacobian still needs the bound isogeny/Weil-restriction argument.

## Existing transfer evidence located

The test quadratic_fields_and_transfer (three_leads.rs:3065-3090) enumerates points for p in {19,23,31,43,59,67,83,103}, checks norm(tau(P))=[2]P and checks100 sampled additivity pairs per field. This is a source description, not a fresh test result. It is not the21 cohort fields. A universally certified norm-tau identity would imply tau(P) is nonzero for nonzero P of odd prime order, since tau(P)=0 would imply [2]P=0. It would not by itself establish the destination group cardinality or every arithmetic implementation branch. These are separate next proof bindings.

## Reproduction and custody

Run audit.py from this checkout. It hashes every input before reading records and refuses mismatch, missing files, changed enrollment count or duplicate cohort workload IDs. Input paths and hashes are retained in input-receipts.json and the prior reconciliation index; no fallback or regenerated plan is allowed. rows.jsonl retains one metadata-only record per workload; curves.json groups by canonical serialization of the curve fields listed in the script (generator excluded). No planted scalar or target coordinates are exported. PLAN.md was written before executing the audit. This is a deterministic frozen-record audit, with no timing/statistical comparison. Source hashes are in source-bindings.json. The PDF and SVG depict only the supplied cofactors.

## Literature and limits

The previous scalar-projection-source-audit/REPORT.md gives the finite-group retraction and quotient criterion with Mathlib references. Sage's official elliptic finite-field documentation, has_order and set_order sections, explicitly distinguishes Hasse-range sanity checks from stronger order evidence: https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_finite_field.html . It informed the wording here; Sage was not run. The ecdb tool was unavailable, so this audit used the already hash-bound input metadata rather than asserting a new database curve fact.

The full1,260-target scientific continuation, runtime qualification and independent review remain separate. No budget was renewed and no cost ratio was populated. Next bind the actual Scholten isogeny and norm-tau source proof, then discharge the destination-order and nonzero-transfer premises.

Publication: rows.jsonl exceeds the64KiB data policy and lives in the content-addressed artifact store. The pointer is .artifacts.json; remote-readback.json records a fresh download and SHA256 verification. Hydrate with tools/artifacts/artifacts.py pull before inspecting the raw rows.
