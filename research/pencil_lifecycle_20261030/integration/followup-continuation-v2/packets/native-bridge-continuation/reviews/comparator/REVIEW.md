# Independent projective comparator review

Reviewer /root/monic_review, 2026-11-01. Source-first plan read native walk_impl_uncached.rs same before Comparator body. The native loop tests all sixteen modular cross-product equalities. The expected field argument was independently recorded: nonzero pivot in x gives y=(yi/xi)x, and nonzero y forces nonzero scale.

## Findings

Six main declarations are faithful to the field-vector claim. pivot_scale uses only a nonzero pivot in x and the all-minors predicate; it does not silently assume yi nonzero. same_iff_nonzero_scale obtains nonzero scale from nonzero y, then proves the reverse direction by multiplication commutativity. same_trans explicitly requires nonzero intermediate y as well as endpoints, preventing the zero-vector false transitivity. scaled_same permits zero scale, correctly proving only raw minors equality. zero_matches_everything explicitly records the raw comparator's degeneracy. No mathematical defect found.

Native fidelity is partial. Fin4 universal quantification enumerates the same sixteen ordered index pairs as source ranges0..4, but the packet defines a Prop over arbitrary Field coefficients; it does not prove a compiled Boolean-loop equivalence. The native mul is word modular arithmetic, requiring prime modulus/domain, multiplication no-overflow or widening, canonical product comparisons and residue equality reflection. Native raw x!=[0;4] implies decoded nonzero only for canonical coordinates; a raw word p is nonzero yet decodes to0. That input invariant must accompany any source projection use. This review does not promote raw byte nonzero to field-vector nonzero.

## Fresh checks and controls

Fresh Comparator, ComparatorControls and ComparatorReadback checks passed: six main declarations, eight producer controls, fourteen named readbacks. Six additional reviewer controls in IndependentControls2 passed: a nonunit pivot with multiple nonzero coordinates; explicit finite16 Boolean acceptance and rejection; nonproportional rejection; final-coordinate pivot; and finite16 zero-vector acceptance. Twenty passing named readbacks use only propext, Classical.choice and Quot.sound.

Retained workflow errors: the first replay script selected generic Controls/Readback from a dependency root rather than producer ComparatorControls/ComparatorReadback. That narrower unrelated replay is preserved as superseded-misnamed-receipts.json and excluded from the comparator result; corrected exact modules were freshly checked. The first independent controls used unavailable vector notation, causing elaboration errors and sorryAx readbacks. Those failed logs are retained and excluded; a separate corrected file uses explicit coordinate functions and passes. No producer source changed.

## Workflow scope

Separate reviewer, prospective local plan, source-first pivot derivation, exact named-module replay, independent finite Boolean controls and axiom checks completed. No sibling reports read. Imported helpers/cache remain a trust boundary; no full dependency rebuild or independent kernel checker. Plan not precommitted and producer input publication at dispatch not established. Multi-reviewer independence validation and coordinator publication remain separate. Model fidelity faithful_scoped; native Boolean/word/compiled bridge partial. No scientific execution or official transition.

Primary source references: native sixteen-cross-product comparator, Comparator/ComparatorControls/ComparatorReadback, and the standard field pivot cancellation argument recorded prospectively. No novelty claim. The next concrete native bridge is canonical residue equality reflection for each word product plus a proof that the actual input vectors decode nonzero.

Binding limitation: a second idempotent replay overwrote inputs.json with an empty map because no new files were copied. reconstructed-source-bindings.json records current source hashes, and exact target module bytes were compared with originals; this is post-check binding, not complete pre/post input coverage. Preserve this limitation in publication.

Provenance repair: final-check.py freshly rebuilt Comparator, ComparatorControls, ComparatorReadback and IndependentControls2, preserving exact local snapshot source/helper inputs in final-before/final-after. All native exits0 and 152 local inputs unchanged. This supplies fresh bounded pre/post evidence; the earlier gap remains recorded. Cached external Mathlib dependency unions remain outside this snapshot map.
