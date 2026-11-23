# One first norm after giant-block contraction

Derived 2026-11-09, T-2014. Use the unchanged split, squarefree baby algebra A=K[T]/H_G and dual numbers K[s]/s^2. For each complete giant block i, Round68 supplies a numerator jet U_i=M0_i+s*M1_i in A[s]/s^2 and a nonzero scalar denominator d_i=H_G(x(A_i))^2*H_Q(x(B_i))^2. Its paired factor product is Norm(U_i)/d_i. All scientific artifacts remain unverified producer evidence pending independent review. [pcc:unverified]

Product before norm. Compute Z=product_i U_i in A[s]/s^2, reducing modulo H_G after every multiplication. Then

product_i Norm(U_i)=Norm(Z).

Proof: at the distinct baby roots t_j the quotient algebra is a product of copies of K. Both sides equal product_(i,j)(M0_i(t_j)+s*M1_i(t_j)). Products commute in the dual-number ring, so interchanging i,j is valid even when constant coefficients vanish. This proof uses no inverse of M0_i or of an intermediate product and covers arbitrary nilpotent jets. It is the split-algebra form of multiplicativity of the determinant norm. [pcc:unverified]

Streaming update. Represent a jet by(A0,A1), each a polynomial of degree<b. Merge with(B0,B1) as

(A0,A1)*(B0,B1)=(A0*B0, A0*B1+A1*B0) modulo H_G.

The first block initializes the accumulator directly. Each further merge uses exactly3 polynomial products and2 reductions modulo H_G. Cache formation is unchanged and costs at most19b+13 K multiplication calls per block. The extra accumulator contains at most2b coefficients, excluding multiplication temporaries and retained reference/qualifier data. No bivariate resultant surface is constructed. [pcc:unverified]

Scalar factors. Maintain the product d=product_i d_i as a nonzero scalar. Separately multiply the center jets and any clipped-tail factors into a scalar jet C. The complete query is

C*Norm(Z)/d,

with division by d applied to both first-norm coefficients. Thus there are2 output divisions per query with complete baby blocks, versus2 per complete block. An empty complete-block set has Z=1,d=1, no norm call and no division. Tail and center factors are not inserted as a replacement baby polynomial; their original multiplicity/index coverage remains explicit. [pcc:unverified]

Singular cases. If one baby root makes Z0 vanish, the simple-common-root branch returns Z1(t)*Res(H_G/(T-t),Z0). If two distinct baby roots vanish, the first norm is zero. If two factors vanish at the same baby root, Z0 has that root but Z1(t)=0, so the simple-root branch also returns zero; counting only distinct gcd roots without evaluating Z1 would be wrong. Center-only zeros are handled in C and need not make the algebra numerator singular. These cases motivate separate empty,unit,simple,same-root-twice,distinct-root,zero-jet and zero-perturbation controls against dense determinants, explicit root products and per-factor norms. [pcc:unverified]

Costs. For n complete blocks, the algebra contraction replaces n first-norm calls by one first norm plus3(n-1) polynomial products and2(n-1) reductions. Denominator evaluation still occurs for each block, with n-1 additional scalar products to accumulate d. The polynomial product work and its reductions must be counted; it is not a free batching step. The native implementation uses classical polynomial arithmetic. Constructor/cache costs, point generation, reference validation and decoding are unchanged and separately recorded. [pcc:unverified]

Frozen predictions. On the prescribed P192 panel there are24 final norm calls rather than72, with6 unit and18 simple outcomes because6 planted zeros lie at scalar centers. On the complete small panel there are2694 final calls rather than11382, with924 algebra units (nonzero norm constant, including any zero-perturbation backend case) and1770 simple zero outcomes. The backend branch codes are retained separately. These are finite-panel predictions from the earlier zero locations, not population estimates. [pcc:unverified]

Sources checked. Round68 coefficient-cache-proof.md gives U_i,d_i and fixed cache costs; Round64 resultant-proof.md specifies singular-safe first-norm branches. Stacks0BIE, https://stacks.math.columbia.edu/tag/0BIE, read2026-11-09, defines determinant norms and multiplicativity. The split-root proof above applies directly over dual numbers. KB tools/wrapper unavailable; bounded searches of prior study reports/proofs were also made. No novelty assertion for multiplicativity itself. [pcc:unverified]
