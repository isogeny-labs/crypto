# Singular-safe field-resultant first coefficients

Derived construction, 2026-11-09, T-1947. Let K be the working field, H monic square-free of degree b>0, and A=K[Z]/H. The requested output is Norm_A(E0+s*E1)=A0+s*B0 modulo s^2. This written proof specifies field-only branches and their domains; native qualification and costs are separate unverified producer evidence. [pcc:unverified]

Reduce E0,E1 modulo H. Over a splitting field the roots t_i are distinct and the norm is product_i(E0(t_i)+s*E1(t_i)). Every identity below follows from this product, hence descends to K. The implementation uses classical field-Euclidean resultants and extended gcd. It does not implement a subresultant polynomial-remainder sequence, half-gcd or fast polynomial-multiplication package. [pcc:unverified]

Zero perturbation. If E1=0, return(Res(H,E0),0). This includes the denominator norms of Round63, whose perturbation is zero. It is correct even when the constant norm vanishes. This branch need not construct or invert a multiplication matrix. [pcc:unverified]

Unit case. If gcd(H,E0)=1, compute q=E1/E0 in A by extended Euclid. Then

A0=Res(H,E0), B0=A0*Tr_A(q).

The trace can be computed without a matrix:

Tr_A(q)=coefficient_(Z^(b-1))((q*H') mod H).

To prove it, write v=(q*H') mod H and use Lagrange interpolation v(Z)=sum_i v(t_i)*H(Z)/((Z-t_i)*H'(t_i)). Each quotient H(Z)/(Z-t_i) is monic of degree b-1, so its leading coefficient gives sum_i v(t_i)/H'(t_i)=sum_i q(t_i). Square-freeness makes the denominators nonzero. This proof does not divide by b and remains valid when the characteristic divides b. [pcc:unverified]

One common root. If the monic gcd is D=Z-t, write H=D*Hbar. Then E0 vanishes at precisely that root, giving

A0=0, B0=E1(t)*Res(Hbar,E0).

This evaluates the one surviving cofactor. All operations are over K because a linear gcd supplies t in K. The b=1 edge is important: Hbar=1 has an empty norm equal to1, including when E0 reduces to zero. The code handles the constant-first-polynomial resultant before its zero-second-polynomial branch. [pcc:unverified]

Two or more common roots. If gcd(H,E0) has degree at least two, each summand in the first coefficient contains another vanishing E0 factor. Thus A0=B0=0. This is an exact first-order statement and makes no claim about higher spectral coefficients. It is included as a negative/control branch even though a valid unique-zero graph input has at most one vanishing baby factor. [pcc:unverified]

Field resultant recurrence. For f,g of degrees n,m, with nonzero remainder r=f mod g of degree d,

Res(f,g)=(-1)^(n*m)*lc(g)^(n-d)*Res(g,r).

The terminal cases are a nonzero constant g, contributing g^n, and a zero remainder with positive-degree g, contributing zero. A constant first polynomial has its own terminal case. Every polynomial division in this routine uses the inverse of a nonzero leading coefficient in K. It never divides by a nilpotent or a zero norm. Extended gcd similarly returns a monic common divisor and an inverse only in the unit case. [pcc:unverified]

Adapter and exact references. Round63 supplies signed baby H,beta,gamma and the already-formed E0,E1,D^2 for each complete paired block. This routine returns the numerator jet and denominator norm through that contract. The selected normalized norm is explicitly fed into the complete-block response after equality with the paired-stream reference. A dense unit-pivot jet determinant is also retained and separately counted on exactly the same polynomial inputs. The qualifier runs both implementations; its total cost and memory therefore include the references. [pcc:unverified]

Complexity at this implementation level. With classical polynomial multiplication and Euclidean arithmetic, this norm component uses O(b^2) field operations and O(b) polynomial workspace, instead of constructing b-by-b multiplication matrices and performing O(b^3) dense elimination. Input-polynomial formation, signed interpolants, denominators, giant blocks and point generation remain additional. With n giant blocks the stated component bound is O(n*b^2); it does not give sublinear total response evaluation or a cross-giant batch algorithm. [pcc:unverified]

Frozen qualification. The complete Round63 scientific panel is replayed unchanged:381 decoded cases and2694 coefficient queries. Its11382 paired blocks call numerator and denominator norms, giving22764 matched norm inputs. The synthetic split-H controls cover units, one root, multiple roots, E0=0 and fixed small polynomials on all three fields. In the complete panel,1770 baby numerators have a simple common root and420 additional complete blocks vanish at their separate center factor. The Round63 count2190 refers to complete blocks including that center; it is not a count of singular b-by-b baby multiplication matrices. [pcc:unverified]

Full-width P192 qualification. For each of the24 preserved27-factor cases, interpolate the four explicit signed pair jets on x(jG),j=1..4, for each of three blocks. The field-only norm agrees with the four-pair product in all72 instances, including18 simple-common-root and54 unit cases. The resulting24 coefficients match the old Round63 file. This deliberately constructs the norm inputs from factors and charges preparation separately; it is a full-width norm check, not a full P192 response or a fast public constructor. [pcc:unverified]

Sources checked. The Stacks Project section0BIE, https://stacks.math.columbia.edu/tag/0BIE , supplies trace/norm-as-multiplication-matrix background for field extensions. The split etale-algebra formulas used here are proved directly above by roots and Lagrange interpolation. Bernstein-De Feo-Leroux-Smith, https://arxiv.org/pdf/2003.10118 , supplies Euclidean-resultant and jet background and the distinction between evaluation and expanded output. Round63 supplies the signed norm inputs, domain guards and complete records. KB tools/wrapper remain unavailable. No novelty or absence claim is made for the standard norm, gcd, trace or resultant ingredients. [pcc:unverified]
