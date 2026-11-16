# Exact frontier census and representation-overlap audit

Sole-model derivation, no new oracle execution or speed measurement. For q=2^d,d>=3, five distinct source bags have only affine rank3 or4. The previous centroid lemma characterizes rank3 by a root equal to the sum. For a fixed centroid b, the other four points form an affine2-plane avoiding b. There are (q-1)(q-2)/6 linear2-planes, q/4 cosets each, and exactly one coset per linear plane through b. Thus

    N3=q*(q-4)*(q-1)*(q-2)/24,
    N3/binom(q,5)=5/(q-3).

Independently, an ordered affine-rank4 tuple is obtained by choosing its last point(q choices) and four independent differences(q-1,q-2,q-4,q-8 choices). Dividing by120 gives

    N4=q*(q-1)*(q-2)*(q-4)*(q-8)/120.

Direct algebra gives N3+N4=binom(q,5). At d3 there are56rank3 and0rank4 bags; at d4 there are1680rank3 and2688rank4 bags. The latter agrees with the existing independently controlled Darboux codec's16centroids*28forms*6bags. At d18 the exact rank3 fraction is5/262141, about0.001907%. These counts concern all distinct bags in the binary subspace, NOT bags satisfying the target equation, rational-ordinate filtering or any planted benchmark. No hard-instance probability is inferred.

Consequently the centroid-quartic branch remains useful exact coverage, but by itself is not a persuasive plan for generic full all-roots speedup. The dominant rank4 branch must stay central.

An attempted new rank4 Schur chart is not a new representation: alternating rank4 moment matrices can be written [A B; B^T B^T A^-1 B] on a nonsingular4principal block. A has28values; B has up to56bits at d18. However the existing darboux_source_codec_v1 already supplies canonical RREF charts, all28forms in16/8/4affine cells, and all six source classes. Its9180variable-class partitions and exact weighted global count already expose the missing chart cost. Reusing it is preferable to rebuilding the same geometry.

The raw four-column frame has720symplectic aliases per moment matrix. Decoding retains six genuinely different unordered bags;720/6=120 is precisely source permutation redundancy. Therefore a claim that removing720frames yields720xsearch savings beyond an already sorted source search would be false. Fewer local primaries or one chart do not remove the global partitions or six branch checks. Existing moment_factor_v1, finite_place_sieve_v1/MOMENT_GLOBAL_PLAN.md and darboux_source_codec_v1/PROOF.md were read directly. This is a bounded internal overlap audit, not literature priority clearance.
