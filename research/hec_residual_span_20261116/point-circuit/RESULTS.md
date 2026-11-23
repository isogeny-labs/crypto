# Point-source specialization result

Once-only exact symbolic execution at 4ff60ed3fe3 exited0. Substitution K(Q)=(0,1,r,r²) reduces each of the16 retained forms to6–8 monomials (116 total). FORMS.tsv retains every coefficient, including all charts. This is an integer-polynomial specialization, not arithmetic speed or full-solve evidence. [pcc:unverified]

For the first chart define U=K3-r*K2+r²*K1, V=K4-r²*K2-2*r³*K1 and S=r5+c. The retained polynomials give

    B11=U²
    B12=U*V+2*S*K1².

Here B11 denotes array form0 and B12 form1, using mathematical one-based indices. Expanding U² gives its six terms. Expanding U*V gives K3*K4-r*K2*K4+r²*K1*K4-r²*K2*K3+r³*K2²-2*r³*K1*K3+r4*K1*K2-2*r5*K1²; adding2*(r5+c)*K1² cancels the r5 term and leaves exactly form1. For an actual point source, S=s². No division by U occurs in these identities, so the factorization itself includes U=0; the differential output can still require another chart.

This shares an intermediate across two entries rather than evaluating their monomials independently. Constants r²,r³,S can be precomputed per source, with setup charged. This is a classical formula factorization, with no novelty claim. It covers a point source, not every degree-two source; arbitrary source changes retain unknown logarithm columns.

The previous result's4.547 optimistic bound applies to its fixed implementation, not to a new circuit. However, these two identities alone do not establish a tenfold opportunity: B13/B14, all other charts, projective output reconstruction, split/root extraction, oriented recovery and relation yield remain. Next admission requires an exact polynomial identity check for a complete proposed circuit and a prospective cost ledger before a new arithmetic gate. Do not rerun the old producer or substitute sampled favorable inputs.
