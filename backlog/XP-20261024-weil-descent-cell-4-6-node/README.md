# Weil-descent dense closure cells (4,6), m = 3 l = 8, m = 4 l = 7 with FGHR 2-torsion, WDSat and crossbred, on one 128 GB node (launch card)

Status: **draft, activation-gated**. The run is a degree curiosity unless the m = 4 repricing puts the D = 5 row within reach (see "Activation gate"). Blocked on two unbuilt instruments (see "Open inputs").

Question: at the Weil-descent cells (4,6), (m = 3, l = 8) and (m = 4, l = 7), does the refutation degree of the dense MutantXL closure stay at 4 or rise to 5, and what is the FGHR 2-torsion arm's degree on the same instances, with WDSat and crossbred timings beside it?

## Sources (every number cited)

- [research/dreg_closure_driver_20261016/RESULTS.md](../../research/dreg_closure_driver_20261016/RESULTS.md): the dense closure engine at degree 5, N = 40 needs 760k columns, about 70 GB (the "about 70 GB" figure). Degree 4 is built in full on every draw. At N = 40 and 42, m = 3 with l = 6 refutes at 4 and l = 7 does not (the degree rises at l = 7). Degree rises once m·l ≥ 21. Degree-5 past l = 7 is unmeasured: the sparse engine is too slow, and dense degree 5 at N = 40 needs about 70 GB. Peak RSS at degree 4 is 4 to 8 GB per cell.
- [backlog/XP-20261018-m4-solving-degree-equivariant/](../XP-20261018-m4-solving-degree-equivariant/README.md) (card.json): the m = 4 design. Its blocked list records that the FGHR m = 4 system is not built (`symmetrised_terms` returns None for m >= 4; M1) and that the C3 Faugere-Svartz blocks are not integrated (M2). Its decision value cites the ECC2K-130 degree-5 row at +10.32 bits.
- [research/ic_leads_review_20261018/README.md](../../research/ic_leads_review_20261018/README.md) section 3.1: the node proposal. Its three cells decide between "degree 5 holds" (+10.32 bits at ECC2K-130, column-touch units at w = 1) and "degree follows l" (+112 bits). It says the FGHR 2-torsion arm was specified on 10-10 and never run, and that the Faugere-Svartz block arm is also unrun. It also asks for WDSat and crossbred on the same PDP instances.

## Activation gate (explicit)

Run only after the m = 4 repricing (research/m4_reprice_20261024/, workstream A) shows that the D = 5 row is within reach. Otherwise the cell is a degree curiosity and no 128 GB node is rented for it. Under REVIEW F1 the gate is closed at the current bracket unless the owner overrides it (see below).

- **Reading the gate.** The gate reads workstream A's rho-step (B) bracket for D = 5 at ECC2K-130, from research/m4_reprice_20261024/table.md (m = 4, D = 5, l = 34, N = 398; that table is on workstream A's branch, not yet on origin/main): +37.27 bits at w = 2.00, +46.33 at w = 2.25 and +49.23 at w = 2.33, so +37.3 to +49.2 over w = 2 to 2.33. In column-touch units (A) the same rows are +45.71 at w = 2.00, +54.77 at w = 2.25 and +57.67 at w = 2.33 (+45.7 to +57.7 over w = 2 to 2.33). The w = 1 figures are +6.95 (B) and +10.32 (A). The leads figure +10.32 is column-touch at w = 1 and is not the gate.
- **Activation gate: CLOSED under REVIEW F1.** REVIEW.md F1: if the best D = 5 row is above about +30 bits in rho units, downgrade the (4,6) cell to a degree curiosity and do not rent memory for it alone. The B bracket (+37.3 to +49.2 bits over w = 2 to 2.33) lies above +30, so the gate is closed unless the owner overrides it. An override must be recorded in PROTOCOL.md with a new threshold or an explicit override note; this card does not set one.

## Resources

One 128 GB node. Closure at degree 5 about 70 GB (RESULTS.md). The FGHR and crossbred runs are not sized in the sources read. The node is shared with no other job. Estimates only: RAM 128 GB, CPUs unknown (see card.json).

## Frozen inputs (pinned vs open)

| input | status |
|:--|:--|
| dense closure driver | pinned: research/dreg_closure_driver_20261016/run.sh and examples dreg_ladder and dreg_closure; the run.sh degree-4 call is `dreg_closure --to 4 --max-cols 155000`, the degree-5 call (`--to 5`, about 760k columns) is derived from RESULTS.md and not yet run |
| cell (4,6) to (n, l) mapping | **open**: the label is used in leads section 3.1 and the mapping is not in the sources read (the m = 4 closure at (15, 6) is in research/dreg_closure_m4l6_20261018, not read) |
| draws and seeds | open: PREREGISTRATION.md is in the dreg driver directory and was not read |
| FGHR 2-torsion arm | **open**: specified 10-10 (leads 3.1), not located; FGHR m = 4 system not built (m4 card M1) |
| Faugere-Svartz block arm | **open**: not integrated (m4 card M2) |
| WDSat | **open**: PLAN.md workstream B records WDSat as absent from the repository |
| crossbred | **open**: no implementation located in the sources read |

## Decision rule

- Degree 5 holds at (4,6), at m = 3, l = 8 and at m = 4, l = 7: the degree does not follow l at these cells, and the "degree 5 holds" price row (leads section 3.1, +10.32 bits at w = 1 in column-touch units) is re-read from the repricing bracket, not from this figure.
- Degree follows l (rises at l = 8 or 7 beyond the measured rise): the row is withdrawn and the +112-bit reading stands.
- The FGHR arm and WDSat/crossbred timings are read only as published-solver timings on the same instances; they do not replace the closure degree.

## Open inputs

1. The activation threshold in rho-units (owner decision; see the gate).
2. The FGHR 2-torsion arm, Faugere-Svartz block arm, WDSat, and crossbred implementations on the same PDP instances.
3. The (4,6) cell mapping and draw seeds.
4. The node (128 GB, CPU count) and its owner.
