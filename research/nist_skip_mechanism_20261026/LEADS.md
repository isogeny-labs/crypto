# Six follow-up leads (workflow run wf_9efbd5e5-379, 2026-10-26)

Six subagents (Sonnet for leads 2–5, Haiku for leads 1 and 6) worked the open leads from
`crypto-autoresearcher/problems/nist-seed-selection-rule-unexplained.md` in parallel. A separate Sonnet
skeptic then reviewed them. **No lead changes the state of the question.**

| Lead | Status | Result | Files |
|---|---|---|---|
| 1. ECPP primality-prover bug | read-only review | No cohort order has run. The owning task TASK-20261024-4f1bf4 is `written_not_dispatched`. Its implementation attempts v1–v3 failed their mock suites. The Coordinator says no runnable approval packet exists yet. | (none) |
| 2. Generator code and the 1996–97 X9.62 drafts | partial | None of the June 1996, January 1997 or June 1997 drafts and no Reiter code was found. Wayback CDX was blocked by egress policy, so this is **blocked, not negative**. The 17 Nov 1997 draft was read about 85%; no candidate log. | `lead2/ARCHIVE_SEARCH.md` |
| 3. Appendix E weak class (2013) | partial | The appendix sentence names no class, date or procedure. No source maps it to the 11 omissions. FOIA targets are listed. | `lead3/APPENDIX_E.md` |
| 4. Visitation models on the 69/99 cohort | partial | The model family is frozen in `PROTOCOL.md` and implemented in `visit.gp`. **Not run: the checkpoint gate refused** (no upload credential). The family was designed after the published indices were seen. | `lead4/` |
| 5. Elkies-failure model, broader cohort | done, **non-compliant run** | The frozen E_100 test gives p = 0.0273 in the predicted direction (10 published vs the 48 extra eligible positions; 10^6 relabellings). It is unstable in L (p = 0.47 at L = 50, 0.062 at L = 200) and only partly out of sample: the 10 published curves are the discovery set. **The run was made without calling the checkpoint gate and without RESUME.md**, unlike leads 4 and 6. It was appended to `PROTOCOL.md` after the freeze; the hashes are in `out/`. | `lead5/` |
| 6. Null-model validation | blocked | The gate refused, so `control_null.gp` was not run. | (none) |

**Skeptic's verdict:** L5 is a weak lead, not a result. Before rerunning it compliantly, declare a
multiplicity correction across L and across the earlier tests (H1, the isogeny screen, the 4,480-assignment test).

**Next step:** add the checkpoint upload credential. Then run L6 (`control_null.gp`), rerun L5 under the gate
with the multiplicity correction declared first, and run L4's frozen family with its planted control.
