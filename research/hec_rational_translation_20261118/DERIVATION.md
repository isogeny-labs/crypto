# Generic formulas and admission budget [pcc:unverified]

Let F=x^5+f3*x^3+f2*x^2+f1*x+f0, u=x²+s*x+t, v=v1*x+v0, Q=(q,y). All arithmetic is in odd characteristic. Put U=u(q), alpha=(y-v(q))/U. With U nonzero the CRT interpolant w=v+alpha*u satisfies w(q)=y; the monic quadratic u_next=(F-w²)/(u*(x-q)) gives

A=alpha², B=2alpha*v1, C=f3-t+s²-s*q+q²,
s_next=q-s-A,
t_next=C-B-A*(s+q),
v_next1=-v1-alpha*(s-s_next),
v_next0=-v0-alpha*(t-t_next).

The coefficient identities follow by comparing x4 and x3 in F-w² and reducing -w modulo u_next. Precompute q² and reuse s*q: this DAG uses7M+2S+1I, with doubling as addition. main.rs deliberately uses readable redundant products and is not the optimized-cost implementation. Fixed parameter preparation, checks and exceptions remain chargeable.

For u_prev=u(P-Q)=x²+r*x+z, obtain A=q-s-r=alpha_minus², B=C-z-A*(s+q)=2alpha_minus*v1, where alpha_minus=(-y-v(q))/U. Set

R=coefficient_x(F mod u)=s^4-3s²t+t²+f3(s²-t)-f2*s+f1,
L=-A*R/B-A*U-(2q+s)*B/4.

Indeed v0=-y-alpha_minus*U-q*v1, and reducing v² modulo u yields R=-2y*v1-U*B-(2q+s)*B²/(4A). Thus L=alpha_minus*y. For nonzero U,A,B,y,

A_plus=A+4L/U+4y²/U²,
B_plus=B+2L*B/(A*U).

Insert these into the same s_next,t_next formulas. Signed recovery uses alpha_minus=L/y, v1=B*alpha_minus/(2A), v0=-y-alpha_minus*U-q*v1. This requires no signed current coefficients in its inputs. The recurrence loses its generic formula at the explicit zero/degree conditions; it is not an exception-free Kummer implementation.

Admission: the literal recurrence evaluates multiple inverses and products to reconstruct quantities that direct Mumford arithmetic already stores. It has no established complete-cost advantage over7M+2S+1I; batching inverses, homogenization and fixed-constant preparation need separate actual DAGs. Even qualifying all identities does not admit a10x full solver experiment. Existing collector-free bounds require eliminating costs beyond collection. Do not count P-Q as a fresh second output: it is supplied state. No genus3/4 formula is derived here.
