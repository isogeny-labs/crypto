# Exact scope

`PoleAnchor.two_pole`: for rational c,t,d with t-d and1-d nonzero,
the actual difference c/(t-d)-c/(1-d) equals
c*(1-t)/((t-d)*(1-d)).

`PoleAnchor.node_preserved`: for arbitrary rational polynomials w,c,h
and any rational t with c(t)=0, the actual polynomial w+c*h has value w(t).
Matrix-entry application is written; no structured matrix H is assumed to
have a fast factorization by either theorem.

Lean4.35.0; CryptoLean148f5dc389b341401e06d76abe7783c4ccf42f2e;
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2; transitive pins in
lake-manifest.json. Standard foundations only; no custom axiom, sorry or
native evaluator. Valuation and global algorithm claims remain unformalized.

Final clean build, transitive shared axiom audit and actual-repository gate
pass:2 declarations,0 failing,0 native-trusted. Official import-inclusive
fresh replay completed with exit0; fresh.log explicitly names PoleAnchor.
Initial successful source and its unused-tactic warning are preserved.
