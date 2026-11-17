# B-zero support geometry producer plan

Source: retained projective_law_followup_20261022/PROOF_AUDIT.md lines391-447, in particular its written B=-2 Res(u,v) and Weierstrass-support argument; exact source helper lines727-753. Existing BZero.normalized_root, zero_y_square and MixedCancellation.support_factor provide reusable algebra.

For a general field and normalized u=X^2+qX+r, v=sX+t with s!=0 and (qs-t)t-rs^2=0, define xW=-t/s and xP=t/s-q. Prove explicit support factorization, v(xW)=0, f(xW)=0 and f(xP)=(2t-qs)^2 from u|(f-v^2). Under IsCoprime f f.derivative prove 2t-qs!=0 and xW!=xP. Then bind the weighted original-input branch using nonzero weights,2!=0 and exact sourceB=0, and match xP,yP to the literal extracted point's normalized coordinates. Do not claim a formal divisor-class theorem: the principal-divisor relation for x-xW remains its own obligation. No performance claim or scientific workload.

Controls fixed before proof checks: p5 u=X^2-X,v=X,f=X^5+X^2-X has W=(0,0),P=(1,1), using existing explicit Bezout certificate and weighted a=(1,0,4,0,2,3). Singular f=X^5,u=X^2,v=X has both roots0 and ordinate0; record that distinction. Nonzero residual B example q=0,r=1,s=1,t=0 violates support factorization at W0/P0. Separate axiom readback and local input custody; preserve failures. Independent review pending.

Before the next proof attempt: add the general simple-root cofactor lemma: f(xW)=0 and IsCoprime f f.derivative imply f=(X-xW)g with g(xW)!=0. Positive F5 cofactor is X^4+X-1 at xW0. This supplies polynomial local data only, not a divisor valuation theorem.

Attempt3 exposed an unavailable root-divisibility lemma identifier. Preserve that failure. Derive the branch linear divisibility from the already imported extracted_valid theorem and zero ordinate, and make the general cofactor lemma consume that divisibility. The branch needs no extra premise; no new library import or dependency fetch is needed.
