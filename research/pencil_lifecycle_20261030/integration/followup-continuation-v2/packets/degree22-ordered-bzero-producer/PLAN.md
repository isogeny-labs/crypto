# Ordered B-zero route preservation: producer plan

Compose the newly checked UInt64 ordered guard with the existing BZero point/doubling correspondence. Source basis: exact retained projective_law_gate.rs lines727-753 and911-935, PROOF_AUDIT, degree22-guards-review/NEXT-WORD-PLAN. No new formula or novelty claim.

Statement: prime5<=p<2^31, canonical a,b and curve coefficients, nonzero a.z/a.w, normalized validity of a on the monic quintic and IsCoprime f f.derivative. Exact crossPreludeWord p a b=some sameUDouble and literal B=0 imply s!=0 by the guard theorem; retain extracted-point canonicality/nonzero ordinate and selected doubled-output canonicality/nonzero weights/normalized validity. Do not require b validity or its nonzero weights: only its source dispatch is consumed. No full add22 return theorem, class-sum theorem or compiler/counter correspondence is claimed. All new producer dependencies retain pending independent-review status.

Fixed controls: complete original p5 nonunit valid/coprime fixture from the previous field packet, explicitly check outer sameUDouble, apply composed theorem without an s!=0 argument, exact doubled output(2,1,3,2,3,3). Zero-ordinate route must select inverse; nonzero-B same-double fixture must lie outside this lemma; p2 excluded. Keep failed checks. Separate axiom readback and exact local input bindings. No scientific workload or cost measurement.

Before controls: also use differently scaled b=(4,0,1,0,1,1) over F5 against the original a=(1,0,4,0,2,3); its q/s crossed values coincide. The source payload remains based on a.
