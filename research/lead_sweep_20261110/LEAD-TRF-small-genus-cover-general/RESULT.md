# RESULT: LEAD-TRF-small-genus-cover-general (desk_only)

## Outcome
- Status: desk_done. Kind: desk_only (phase1_index.json; run_cmd empty). run.py not run.
- Wall: a few seconds of grep/reading. Exit code: n/a.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): g_min(6)=9, g_min(10)=49, g_min(14)=41 reproduced exactly via Riemann-Hurwitz; falsified if any smaller or if derivation needs an unstated assumption; inconclusive if derivation cannot be reconstructed from recorded text.
- Measured: the only recorded text is the three numbers ("genus floors 9/49/41 at k=6/10/14", atlas_survey.md l.60/272) and tally entry (closed_obstructed, last_reviewed 2026-10-29, atlas_tally.json l.1089). A grep of research/ for the numbers finds only the sweep protocols and the atlas survey. The cover-degree formula, the Frobenius-orbit ramification rule and the family (degree 2^m or ell Artin-Schreier/Kummer) are not stated, so the Riemann-Hurwitz computation cannot be reconstructed from the recorded text.
- Verdict: inconclusive. Controls: null (k=3 Joux-Vitse) not run, positive (published char-2 GHS genus) not run, A/A n/a. Controls_ok = false (not exercised).
- Also note: the atlas lists the lead as open_next_step in one place and closed_obstructed in the tally (disposition conflict).

## Reading
- Nothing here tests the lead's claim; this was a text audit. No stochastic toy exists for it.
- Not searched: cryptanalysis repo and the research-kb for the original derivation (out of scope of the allowed 20 minutes and tools). Likely the 9/49/41 values come from a separate lead, TRF-odd-prime-ghs-cover-floor, which is flagged for a different run.
- Not surprising: the floors are numbers without derivation, so they remain assertions.

## Bearing on the sub-sqrt(p) question
none: no derivation was reconstructed, and even a confirmed floor would bound one cover family over F_{p^k} only, never a prime-field curve.

## Standing
[pcc:unverified], single local desk pass, no independent replay.
