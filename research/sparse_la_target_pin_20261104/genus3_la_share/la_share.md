# Genus-3 index calculus: how much of the whole solve is linear algebra

Source: committed records only, `research/ic_improve4_20261015/01-genus3-relations/results/run/` (per-run JSONL
`b<bits>_c<curve>_{r2,c4,w}.jsonl`, plus `analysis.md`, `analysis_c2.md`, README). No experiment was run; arithmetic
by jq. Machine-readable twin: `la_share.json`.

## Units and how each field is read

- Unit (from README/PROTOCOL.md and `fn cost_units` in `examples/ic_improve4_01_genus3_rel.rs`):
  `cost = c[0] + bitlen(p-1)*c[1] + c_ell*c[2]`, i.e. M_q + ceil(log2 q) I_q + c_ell * M_ell, counted operations
  (not wall time). Primary c_ell = 1; c_ell = 9 is the protocol's sensitivity.
- `cost` (record field `results[].cost`) = whole solve. `rel_cost` = relation phase. `la_cost` = linear algebra.
  Checked: `rel_cost + la_cost == cost` on every selected record (relative error 0). The SGE step (`sge_cost`, 13 at 2^22) is
  inside `la_cost`, and negligible.
- `cost_cl9`, `la_cost_cl9` = the same with M_ell weighted 9. LA is almost all M_ell (`la_counters[2]`), so c_ell
  moves LA and barely moves the relation phase.
- Per q, each field is the geometric mean over the common curves (what `analysis.md` does; it reproduces
  `analysis.md`: incumbent whole 4.956e6 at 2^12, 9.967e10 at 2^22, LA share 0.22 / 0.30). Curves: 0-2 for all q, plus
  3-4 for q = 2^18..2^21 (r2 and c4 arms); `w` arm curves 0-2 only. `n_curves` is in the JSON.
- LA share = LA/whole (derived by `la_cost`/`cost`). Amdahl bound = whole/(whole - LA) (derived by
  `cost`/`rel_cost`-style arithmetic: relation phase held fixed, LA set to 0).
- Arm names: records carry a cx suffix (`r2:lpsgebw2@8`); stripped here. **`r2:lpsgebw2+C1C2` is not an arm in these
  records.** Nearest is `w:lpsgebw2+C1C2` (32 lockstep walks, curves 0-2, charged for the start-up of 31 extra walks); shown
  as the stand-in and labelled so. `c4:lpsgebw2o100c@16+C1C2` is present.

## (1) Per arm, per q (c_ell = 1, the primary unit)

whole, relation, LA are geometric means of `cost`, `rel_cost`, `la_cost`. Last two columns are the c_ell = 9
sensitivity (from `cost_cl9`, `la_cost_cl9`).

### c4:lpsgebw2o100c@16+C1C2

| log2 q | n curves | whole (cost) | relation (rel_cost) | LA (la_cost) | LA share | Amdahl bound | LA/relation | LA share, c_ell=9 | Amdahl, c_ell=9 |
|---|---|---|---|---|---|---|---|---|---|
| 12 | 3 | 5231675 | 4804968 | 426120 | 0.081 | 1.09 | 0.09 | 0.444 | 1.8 |
| 13 | 3 | 11414125 | 9729967 | 1679162 | 0.147 | 1.17 | 0.17 | 0.608 | 2.55 |
| 14 | 3 | 23162290 | 16983932 | 6172609 | 0.267 | 1.36 | 0.36 | 0.766 | 4.27 |
| 15 | 3 | 62461630 | 38649949 | 23807992 | 0.381 | 1.62 | 0.62 | 0.847 | 6.54 |
| 16 | 3 | 196336992 | 119540659 | 76791816 | 0.391 | 1.64 | 0.64 | 0.853 | 6.78 |
| 17 | 3 | 502213575 | 261051075 | 241157909 | 0.48 | 1.92 | 0.92 | 0.893 | 9.31 |
| 18 | 5 | 1362704927 | 709404474 | 653259368 | 0.479 | 1.92 | 0.92 | 0.892 | 9.29 |
| 19 | 5 | 3686854618 | 2021031404 | 1665801917 | 0.452 | 1.82 | 0.82 | 0.881 | 8.42 |
| 20 | 5 | 9745381603 | 5781358060 | 3964001953 | 0.407 | 1.69 | 0.69 | 0.861 | 7.17 |
| 21 | 5 | 25724682275 | 16815023178 | 8909605132 | 0.346 | 1.53 | 0.53 | 0.827 | 5.77 |
| 22 | 3 | 68722295459 | 49772772123 | 18949438673 | 0.276 | 1.38 | 0.38 | 0.774 | 4.43 |

### r2:lpsgebw2

| log2 q | n curves | whole (cost) | relation (rel_cost) | LA (la_cost) | LA share | Amdahl bound | LA/relation | LA share, c_ell=9 | Amdahl, c_ell=9 |
|---|---|---|---|---|---|---|---|---|---|
| 12 | 3 | 4955643 | 3858221 | 1096849 | 0.221 | 1.28 | 0.28 | 0.719 | 3.56 |
| 13 | 3 | 13694547 | 9416211 | 4277930 | 0.312 | 1.45 | 0.45 | 0.804 | 5.09 |
| 14 | 3 | 37438189 | 24744903 | 12691093 | 0.339 | 1.51 | 0.51 | 0.822 | 5.61 |
| 15 | 3 | 101220171 | 63160476 | 38050692 | 0.376 | 1.6 | 0.6 | 0.844 | 6.42 |
| 16 | 3 | 279731828 | 165202204 | 114528254 | 0.409 | 1.69 | 0.69 | 0.862 | 7.24 |
| 17 | 3 | 775352060 | 457897254 | 317425975 | 0.409 | 1.69 | 0.69 | 0.862 | 7.24 |
| 18 | 5 | 2045787112 | 1202562654 | 843220637 | 0.412 | 1.7 | 0.7 | 0.863 | 7.31 |
| 19 | 5 | 5475758140 | 3324655760 | 2151085503 | 0.393 | 1.65 | 0.65 | 0.853 | 6.82 |
| 20 | 5 | 14406246350 | 9101855245 | 5304356332 | 0.368 | 1.58 | 0.58 | 0.84 | 6.25 |
| 21 | 5 | 37567844300 | 24802954391 | 12764856815 | 0.34 | 1.51 | 0.51 | 0.822 | 5.63 |
| 22 | 3 | 99668914531 | 69706862682 | 29961913165 | 0.301 | 1.43 | 0.43 | 0.795 | 4.87 |

### r2:lpsgebw2+C1

| log2 q | n curves | whole (cost) | relation (rel_cost) | LA (la_cost) | LA share | Amdahl bound | LA/relation | LA share, c_ell=9 | Amdahl, c_ell=9 |
|---|---|---|---|---|---|---|---|---|---|
| 12 | 3 | 3966019 | 2868631 | 1096849 | 0.277 | 1.38 | 0.38 | 0.775 | 4.44 |
| 13 | 3 | 12480388 | 8202111 | 4277930 | 0.343 | 1.52 | 0.52 | 0.824 | 5.69 |
| 14 | 3 | 30208946 | 17516065 | 12691093 | 0.42 | 1.72 | 0.72 | 0.867 | 7.52 |
| 15 | 3 | 82711045 | 44652398 | 38050692 | 0.46 | 1.85 | 0.85 | 0.885 | 8.67 |
| 16 | 3 | 255980491 | 141450912 | 114528254 | 0.447 | 1.81 | 0.81 | 0.879 | 8.29 |
| 17 | 3 | 629442453 | 311991670 | 317425975 | 0.504 | 2.02 | 1.02 | 0.902 | 10.16 |
| 18 | 5 | 1664125904 | 820902027 | 843220637 | 0.507 | 2.03 | 1.03 | 0.902 | 10.24 |
| 19 | 5 | 4383923488 | 2232823814 | 2151085503 | 0.491 | 1.96 | 0.96 | 0.897 | 9.67 |
| 20 | 5 | 11362252567 | 6057866442 | 5304356332 | 0.467 | 1.88 | 0.88 | 0.887 | 8.88 |
| 21 | 5 | 29255324306 | 16490439534 | 12764856815 | 0.436 | 1.77 | 0.77 | 0.875 | 7.97 |
| 22 | 3 | 75325018321 | 45362985892 | 29961913165 | 0.398 | 1.66 | 0.66 | 0.856 | 6.94 |

### w:lpsgebw2+C1C2

| log2 q | n curves | whole (cost) | relation (rel_cost) | LA (la_cost) | LA share | Amdahl bound | LA/relation | LA share, c_ell=9 | Amdahl, c_ell=9 |
|---|---|---|---|---|---|---|---|---|---|
| 12 | 3 | 4778937 | 3672312 | 1102289 | 0.231 | 1.3 | 0.3 | 0.729 | 3.69 |
| 13 | 3 | 13032536 | 8857959 | 4173868 | 0.32 | 1.47 | 0.47 | 0.809 | 5.24 |
| 14 | 3 | 30255530 | 17478554 | 12766791 | 0.422 | 1.73 | 0.73 | 0.868 | 7.57 |
| 15 | 3 | 80886080 | 42441344 | 38439416 | 0.475 | 1.91 | 0.91 | 0.891 | 9.15 |
| 16 | 3 | 248700421 | 134699257 | 113998365 | 0.458 | 1.85 | 0.85 | 0.884 | 8.62 |
| 17 | 3 | 604104634 | 289289264 | 314806119 | 0.521 | 2.09 | 1.09 | 0.907 | 10.79 |
| 18 | 3 | 1605345322 | 758521946 | 846822462 | 0.528 | 2.12 | 1.12 | 0.91 | 11.05 |
| 19 | 3 | 4220699789 | 2067217294 | 2153482219 | 0.51 | 2.04 | 1.04 | 0.904 | 10.38 |
| 20 | 3 | 10876507285 | 5549020164 | 5327461062 | 0.49 | 1.96 | 0.96 | 0.896 | 9.64 |
| 21 | 3 | 27834401968 | 15068243623 | 12766057623 | 0.459 | 1.85 | 0.85 | 0.884 | 8.62 |
| 22 | 3 | 71271063766 | 41368857611 | 29902201073 | 0.42 | 1.72 | 0.72 | 0.867 | 7.51 |

## (2) Fitted exponents, and whether LA overtakes the relation phase

Exponent = slope of log cost against log q, geometric means; "reported" = from `analysis.md` / `analysis_c2.md` fields
`cost`, `rel_cost`, `la_cost` (c_ell = 1). "own" = derived by least squares on the geomeans above (agrees with the
reported values to 0.001-0.003; see JSON `own_ols_exponent_check_derived`).

| arm | range | whole | relation | LA |
|---|---|---|---|---|
| r2:lpsgebw2 (incumbent) | 16..22 reported | 1.408 | 1.450 | 1.335 |
| | 12..22 reported | 1.430 | 1.419 | 1.458 |
| r2:lpsgebw2+C1 | 16..22 reported | 1.374 | 1.407 | 1.335 |
| | 12..22 reported | 1.415 | 1.389 | 1.458 |
| w:lpsgebw2+C1C2 (stand-in) | 16..22 | whole 1.368 (README text); own 1.368 | 1.395 reported | 1.337 reported |
| | 12..22 | own 1.392 | 1.354 reported | 1.460 reported |
| c4:lpsgebw2o100c@16+C1C2 | 16..22 reported | 1.413 | 1.473 | 1.314 |
| | 12..22 reported | 1.399 | 1.357 | 1.544 |

- Over 16..22 every arm has relation exponent above LA exponent (by 0.115 for the incumbent, 0.16 for C4). The
  LA share therefore falls with q: incumbent 0.412 at 2^18 (its maximum) to 0.301 at 2^22. Over 12..22 the LA exponent
  is higher, but that is the small-q rise (share 0.22 at 2^12 to 0.41 at 2^16).
- Does LA overtake relations (LA share > 0.5, c_ell = 1)? Incumbent: no, at any q in range (maximum 0.412 at 2^18).
  C4: no (maximum 0.480 at 2^17). C1: yes only at 2^17-2^18 (0.50, 0.507), back to 0.398 at 2^22.
  w:lpsgebw2+C1C2: at 2^17-2^19 (maximum 0.528 at 2^18), 0.420 at 2^22.
  The crossing is a bump, not a trend; no arm has LA above relations at 2^22.
- With c_ell = 9 (sensitivity), LA is the larger phase at every q for every arm: incumbent share 0.72 at 2^12, 0.86 at
  2^16-2^18, 0.795 at 2^22. The LA/relation split is therefore dominated by how an M_ell is priced against M_q.
  The README's primary unit is c_ell = 1; the c_ell = 9 weighting is not tied by the records to a measured ratio.

## (3) Conclusion (upper bounds from the data, not predictions)

1. Incumbent `r2:lpsgebw2`, c_ell = 1: LA is 0.301 of the whole at 2^22 (derived la_cost/cost), peak 0.412 at 2^18; making LA
   free would cut the whole by at most 1.43x at 2^22 (Amdahl, derived), at most 1.70x at 2^18, and 1.28x at 2^12.
2. After C1 (+C2) the relation phase shrinks and the bound grows: `r2+C1` 1.66x at 2^22 (peak 2.03x at 2^18);
   `w+C1C2` 1.72x at 2^22 (peak 2.12x); C4 1.38x at 2^22 (peak 1.92x), because C4 already spends less on LA
   (LA 0.632 of the incumbent's at 2^22).
3. Under c_ell = 9 the same bounds are 4.9x (incumbent, 2^22), 6.9x (C1), 7.5x (C1C2 stand-in), 4.4x (C4); this is the
   ceiling if M_ell is truly 9 M_q, and nothing in the records settles that.
4. Genus 3 is a regime where LA can move end-to-end cost: it is 30-41% (c_ell = 1) to 79-86% (c_ell = 9) of the
   whole solve, and 2^22 sits on the falling side of the share curve (LA exponent 1.335 vs relation 1.450 over 16..22),
   so the gain from a faster LA shrinks as q grows.
5. Limits of these bounds: they hold the relation phase and kappa fixed (a cheaper LA would move the optimal kappa and
   might do better); they assume LA cost 0, which no solver reaches. The incumbent block Wiedemann (2,1) already
   uses 1.50 mat-vecs per column and mat-vecs are 0.79-0.80 of its LA products at 2^18..2^22 (`analysis.md`
   columns `matvecs / cols`, `mat-vec share of LA products`), so a target-pin stop can only save part of the LA
   share above; the records contain nothing that measures that part.

## Not found / not done

- No `r2:lpsgebw2+C1C2` arm; the `w:` stand-in has different walk structure, curves 0-2 only.
- No matched-rho gap here (README: not measured in this item).
- No record field for the fraction of LA a target-pin stop would save; no wall-time share used (wall times are
  descriptive, shared L0 host).
- q = 2^23 not run (README: memory limit), so the range ends at 2^22.
- Multi-curve geometric means; per-curve spread of LA share was not tabulated (the JSON keeps per-q geomeans only).
