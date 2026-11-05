# Signed collector exceptional transition controls

Software validation, 2 November 2026. Seven deliberately invalid transition traces were accepted by the saved pre-change replay and rejected by the corrected replay. All five constructed positive controls passed before and after the correction.

The correction requires a chosen-duplicate restart immediately after a repeated choice, rejects unsolicited or repeated restarts, requires a walk restart to precede a candidate while below the cap, rejects repeated jump indices within a continuation, and permits choice only after two candidate dispositions or the cap.

Positive controls cover identity recovery, zero identity, opposite-sign duplicate at the cap, chosen-duplicate restart, and walk-duplicate restart. They contain 11 candidate equations and two restart equations. Both recovered scalars were checked on the source curve. These controls were constructed from the saved p=739 instance with target 2G; they are not native scheduler traces or observations of natural branch frequency.

The same regression test also rechecked six saved native streams, totaling 731 candidate equations. No collector was rerun. The pre-change check intentionally failed with all seven invalid traces accepted; the post-change check passed all three tests. Both source-binding checks passed. Raw outputs, exact source hashes, old replay source and the patch are retained.

The graph and group-label checks do not reconstruct the PRNG, selection priority or complete walk-set membership. The label checker uses native affine Cantor arithmetic shared with the producer fallback. Independent source-fidelity review remains pending. These checks provide no new timing or cost ratios.

Campaign planning remains 25,200 active-worker seconds with deadline 2026-11-03 06:00 UTC. This software validation starts no scientific worker; artifact binding, runtime qualification and prospective independent review remain prerequisites for scientific execution.
