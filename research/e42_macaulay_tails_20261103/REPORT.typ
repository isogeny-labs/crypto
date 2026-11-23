#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
= E42 / E42b: V's stored Macaulay tails reserved to the fill (W, `chain-tails`)

*Run and results report, 2026-11-03.* Research note `research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`: §9.54 (E42 pre-registration, `e37d72e172`, after Stage 0 and before any W code), §9.55 (E42 measured: controls kill), §9.56 (E42b pre-registration, `95e9f95a05`, before any Stage A or end-to-end data) and §9.57 (E42b results). Code: `9b061b9710`, `38eac2b3ea` (Stage 0 instrument), `3858929530` (`src/cryptanalysis/macaulay_tails.rs`: W), `673433fef5` (`klein_v4` hook, committed alone), `efc700b36d` (record field `tail_extent_ops`), `6dfe52f95e` (harness), `6dddf03954` (E42b harness: `--salt`, verdict ids). Binaries from clean, blob-checked exports, pinned by sha256 (`experiments/49_macaulay_tails/binary_sha256.txt`, `binary_commit.txt`); after two container restarts the binaries were rebuilt from the same exports, and the two that were not byte-identical were checked by replay (see Limits). Records: `experiments/49_macaulay_tails/`. Host: the shared 4-vCPU container; isolab-eks unreachable, `conductor` not installed. *Operation counts only, no wall-time claim.* Python only plots recorded values; every number comes from `examples/e42_stage0.rs`, `e42_checks.rs`, `e42_stage_a.rs` and `e42_tables.rs`.

== Stage 0: where V's tail cells are (22 curves × 200 calls, counts only)
Per call (ranges of the 22 curve means): V stores 102,825–102,831 tail cells. Each row's build extent (to the column of $"TM"(f_i) dot "shift"$, the trailing monomial; 0 exceptions) covers 81,300–81,400 of them; growing a row whenever a pivot's last nonzero lies further right reaches 82,737–82,801 (equal to the oracle "rightmost ever nonzero" extent on every curve). So about 20,050 cells (19.5 %) at the right end are never nonzero. Only 36,775–36,893 tail cells are ever nonzero, but a sparse row's merges cost at least $9.8 dot 10^5$ index operations a call (8× V's whole priced build): *closed*. Fill arrives throughout the echelon (924–930 growth events, 1,401–1,481 cells), so exact growth with re-allocation copies about $2.0 dot 10^5$ cells: *closed*. Static reserves: to the end of the build end's weighted-degree block, 86,138–86,202 cells + 8,120–8,721 copied; to the end of the next block (W), 92,475–92,540 cells + 420–563 copied, 6.0–7.0 re-allocations. Every contiguous rule needs a charged fill check per row update (≈ 4,200 a call). Rows outside the normal forms' dependency closure hold ≈ 12,600 cells of fill extent but need the staircase in advance (a trace ceiling, not proposed). Lead atlas: no lead covers this build (nearest: `LEAD-ICP-prime-ic-constant-engineering`).

#figure(image("figures/stage0_tail.svg", width: 85%), caption: [Stage 0 (`stage0_v2.json`, GLS $p = 101$ seed 1, 200 calls, stream `seed ^ 0xE42D_0000`): V's stored tail cells per call by the weighted degree of the row's lead: ever nonzero, zero inside the fill extent, and right of the fill extent (never nonzero).])

#figure(image("figures/levers.svg", width: 92%), caption: [Stage 0, same curve: recorded components of each storage rule, stacked (stored cells, copied cells, fill checks and splits, re-allocations). W is the `k = 2` reserve.])

#figure(image("figures/fill_arrival.svg", width: 70%), caption: [Stage 0, 22 curves × 200 calls: growth cells by the column of the update that pushes a row's end right.])

== W (`chain-tails`)
V with each row allocated from its lead to $B_2(delta_i + "wdeg"("shift"))$: the end of the block after the one holding its build end. A charged fill check runs on every row update, and on overflow the row is re-allocated (charged, copies included) to the end of the block holding the pivot's end. Priced: $S_W(g) = "total_muls" + 97 g ("stored" + "offset ops" + "tail ops")$. Tail ops: one per support term (least-degree scan), two per built row, one per pivot (its end), one per row update (fill check), one per re-allocation. Offset ops are V's convention and equal V's on every call.

== E42: stopped by its own controls kill (§9.55)
On stream `0xE42C_0001` (320 calls) every correctness control passed: W = A counted, the counter check, P1 (caps at the build end, fill check off) detected, P2 (counter short by one per row) detected, G0 (caps at the build end, fill check on) = A counted with the counter check, all 320/320; the perturbed coefficient changed 190/190 non-empty solution sets. But G0 made 245.6 re-allocations a call against the registered [900, 960], a number mis-derived from Stage 0's exact-regrowth counter (W regrows to the block end). The committed harness folds that range into G2.231, so the kill fired and E42 stopped with W unranked. E42b (§9.56) re-registered the question before any ranking data existed. It keeps the same W, rows, pricing and rank rule, re-derives the G0 count (now descriptive), and runs the controls on a fresh stream.

== E42b controls and identity
Stream `0xE42C_0002`, 320 calls: W = A counted 320/320, counter 320/320, P1 detected 320/320, P2 detected 320/320, G0 = A counted and counter 320/320 (246.3 re-allocations a call, G2.246's [200, 300]), perturbed coefficient 217/217. Under the E42 binary all 9 existing configurations are identical to their references: E40's seven, plus V on GLS $p = 101$ s2 and E24 $p = 101$ s1 against E40's stored records.

== Stage A (10 curves × 1,000 calls, stream `seed ^ 0xE42A_0001`)
On all 10,000 calls: every solver returned the existing solver's output; W and V matched A counted in fallback flag, every phase count and the pivot digest; W's offset operations equalled V's; W's counter equalled its final row lengths plus discarded buffers; W's lead scans were ≤ V's (equal on average, ≈ 5,877). Per curve: W clean cells 92,948–92,983 (V 102,826–102,827), 6.07–6.53 re-allocations a call, tail ops 5,470–5,503, slack over the fill extent 10,172–10,227 cells, fallbacks identical in A, V and W. $S_V slash S_W$ = 1.000000 / 1.0209–1.0232 / 1.0305–1.0338 at 0 / 0.1 / 1 GAE (σ1: 1.0203–1.0225 / 1.0293–1.0324). The dense-update cells (unpriced, every arm; Stage 0 replay) are V 744,936–756,368 and W 604,853–614,069 a call.

== End to end (32 rows, E37's stream `E37E0001`, Karatsuba)
W's records equal E40's V records on all 32 rows (and E37's A records) outside the tag, timing, lead scans and the storage fields: the same `total_muls`, relations, logs and walks. Every log is recovered and every walk is correct. Negative control: V′ on `E42E0002` is not ranked against V (6 ahead, 13 behind, 1 unresolved). W′ against V′, paired, gives 1.0000 / 1.0217–1.0229 / 1.0316–1.0337, and W′ equals V′ outside the allowed fields.

#table(columns: 5, align: (left, left, left, left, left),
  [quantity], [0 GAE], [0.1 GAE], [1 GAE], [order, 32 rows],
  [$S_V slash S_W$], [1.0000], [1.0219–1.0230], [1.0319–1.0338], [W equal at 0, ahead at 0.1 and 1],
  [$S_A slash S_W$], [1.0000], [1.2793–1.2908], [1.4064–1.4273], [W equal at 0, ahead at 0.1 and 1],
  [$S_(E 25) slash S_W$], [2.5054–2.5766], [1.7839–1.8447], [1.4555–1.5049], [W ahead at every weight],
  [σ1 $S_V slash S_W$], [1.0000], [1.0213–1.0223], [1.0306–1.0323], [same order],
  [σ4 $S_V slash S_W$], [1.0000], [1.0206–1.0215], [1.0291–1.0306], [same order],
  [$S_W slash rho$], [$6.00 dot 10^3$–$1.98 dot 10^5$], [$1.55 dot 10^4$–$5.29 dot 10^5$], [$5.97 dot 10^4$–$2.29 dot 10^6$], [rho ahead on 32/32],
)

#figure(image("figures/band.svg", width: 100%), caption: [E42b end to end (`experiments/49_macaulay_tails/summary.json`, 32 rows, stream `E37E0001`): $S_V slash S_W$ and $S_A slash S_W$ per row at 0, 0.1 and 1 GAE (GAE = 97 $FF_p$ multiplications).])

#figure(image("figures/rho.svg", width: 100%), caption: [Distance remaining to the matched rho (walk (iii)): $S_W slash rho$ per row at 0, 0.1 and 1 GAE; rho is ahead where the value exceeds 1, which is every row at every weight.])

== Verdict
Every E42b prediction holds (G2.244–G2.257). *By §9.56's rule W is ranked ahead of V: equal at 0 GAE (no improvement there), ahead by 1.022–1.023× at 0.1 GAE and 1.032–1.034× at 1 GAE on all 32 rows.* W is ahead of A at the positive weights and ahead of E25 at every weight. Class: engineering, a data-structure constant on the build. The gain is small: of the ≈ 20,050 never-nonzero right-end cells, W keeps ≈ 10,200 as reserve slack and pays ≈ 5,500 charged checks and lookups, leaving a net ≈ 4,350 charged operations a call (3.5 % of V's). The best row (GLS $p = 397$ s1 arm B) has $S_W slash rho$ = $6.00 dot 10^3$ / $1.55 dot 10^4$ / $5.97 dot 10^4$.

== Limits and deviations
- *Weights.* The gain exists only at positive band weights. Its size is set by the per-cell price: a `u16` cell and a fill check are both priced at 9.7 or 97 $FF_p$ multiplications. This is a sensitivity range, not a calibration.
- *Scope.* E25's and E24's curves, $p <= 521$, Macaulay degree 11. The reserve rule ($k = 2$) was chosen on Stage 0's stream.
- *Unpriced savings.* W also saves dense-update cell traffic (≈ $1.4 dot 10^5$ a call), unpriced for every arm and not claimed.
- *E42 kill.* E42 was killed by a mis-derived control-count prediction (§9.55); E42b re-registered the question before any ranking data existed.
- *Restarts.* Two container restarts wiped the build directory. Binaries were rebuilt from the same clean exports. Two binaries were not byte-identical, and each was checked by replay before use: `e42_checks_b`, and after the second restart `quartic_pipeline_gls`. The second produced only the last 11 end-to-end records. Stage A was rerun whole. All of this is disclosed in `binary_commit.txt` and §9.57.
- *Not done.* Independent validation, the leaderboard and `benchmarks/`, which are left to the main session.
