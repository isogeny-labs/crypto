# Next implementation: guarded binary block reconstruction

Unexecuted design from the completed rank table. All256states have rank>=1:96rank3,144rank2,16rank1. Therefore two free delayed slots suffice globally;they must be constrained canonically when a state needs fewer. Do not discard the16rank1states or claim a uniform63-to60selector map.

1. Reuse the frozen invertible basis theta=U*y+K*z,with60quotient and3delayed coordinates. Derive the8state-index bits as linear forms of y in the exact early-image basis. The affine base offset is the same fixed one used for the rank table;do not shift table states independently.

2. For each frozen state,reduce R while carrying row-operation masks on the symbolic three-bit right-hand side b. Store a3x3linear operator S mapping b to the zero-free-coordinate particular solution,up to2kernel vectors,and up to2left-null compatibility masks. The existing C137solver did not expose pivot-row operation masks;implement a fresh version returning them and exhaust all512matrices/all8rhs against brute force. Sneed not solve an incompatible rhs;left-null constraints must reject it.

3. Emit the state-dependent Sentries,kernel entries,compatibility entries and active-free flags using complete256entry truth tables or exact small ANFs,sharing monomial gates. Their inputs are only the8linear early-state bits,not all63parameters. Include every entry and verify the emitted functions against all256states before using them.

4. Build the normalized coefficient family at theta=U*y with all delayed coordinates0. Compute b=(c18,c19,c20)there,sharing its clipped prefix. Let z=S(state)*b+K_free(state)*t,where t has two slots. Assert every left-null compatibility equation on b. Assert t_j=0whenever state needs fewer than j+1free delayed coordinates. Then each compatible yandallowed t maps bijectively to a correction-zero theta;unused free slots cannot create duplicate models.

5. Fuse reconstruction into coefficient bits before nonlinear generation: F_theta=F_(U*y)+sum_j D_(K_j)*z_j. The delayed directions vanish below17,so early prefix work can be shared. Retain the final raw product,all unconsumed correction rows,root words,and the full63selector/76norm-qreadback. Only the three proved consumed corrections may be replaced by0after compatibility/canonical-slot assertions.

Promotion:full small solution sets including inconsistent and singular branches,repeated roots,all256table functions,and source/qinverse readback must pass. Then profile the complete graph/CNF and all cofactor/guard/table/recovery costs against the170149-clause fused one-row baseline. Only a new frozen accepted variant justifies another changed native diagnostic. The conditional bound7*2^58and its factor32/7relative to2^63are parameter counts,not speedups or original root counts.
