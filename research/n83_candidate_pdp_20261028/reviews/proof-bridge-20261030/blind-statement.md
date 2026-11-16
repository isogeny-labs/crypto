# Independent mathematical derivation task

Work only from this definition and primary literature. Do not read producer proof texts, code, results, sibling reports or parent history. Report actual sources read and any inability to conclude. This is a technical review, not a scientific promotion.

Let k be a prime field of characteristic p=4835703278761981908090881. Let m range from3 through9. There are m selector blocks z[i,j], j=0..26, and unrestricted intermediate variables U[0]..U[m-3]. The selector equations are z[i,j]^2-w[i,j]*z[i,j]=0 with nonzero weights. For original weights w[i,j]=2^j; for the alternative replace the final weight in every block by2^40. Set X[i]=sum_j z[i,j]. Define

S(x,y,z)=(x-y)^2*z^2-2*((x+y)*(x*y+2)+6)*z+(x*y-2)^2-12*(x+y).

The chain equations are S(X[0],X[1],U[0]); for j=1..m-3, S(U[j-1],X[j+1],U[j]); and S(U[m-3],X[m-1],t), with t a fixed field element. For m3 the middle list is empty. No U-field equation is present. These equations constrain candidate x domains, not a complete liftable-point lookup ideal.

Use ordinary total degree. For affine generators f_i of degrees d_i, a weighted homogeneous syzygy on their highest forms has coefficients q_i of degrees d-d_i. Trivial Koszul syzygies are the pairwise generator relations. A finite-field FFD convention may further quotient by p-th powers; state any effect at the degrees considered.

V_d is the smallest k-vector subspace containing input generators of degree at most d and closed under multiplying an included polynomial h by any polynomial a when deg(a*h)<=d. Distinguish this iterated closure from a one-shot Macaulay rowspace.

Independently determine: (1) any uniform lower obstruction and explicit upper witness for the onset of non-Koszul highest-form syzygies; (2) whether these facts yield a strict filtered fall for the affine system, under exactly what exceptional-target conditions; (3) what remains unproved about LFD upper bounds and degree of regularity. Derive quantities rather than assuming them. You may use a symbolic general odd selector width to explain the mechanism. Do not execute a new parameter sweep or change project files beyond your own report directory.
