# Prepared inversion-free trace-zero binary norm chart

Suppose the finite target(z,y) lies on Y^2+XY=X^3+A X^2+B andTr(A+z)=0. Choose c with c^2+c=A+z and d=y+zc. For the k=0quintic norm chart set

r=z+ch+u, w=s+zr=z^2+yh+v.

This is a bijective affine change between(r,s)and(u,v)for every h, including h=0. Direct coefficient algebra gives the complete ascending quintic

H=[zv^2, v^2, zu^2+hv+dh^2, u^2+hu, h^2+h+z, 1].

In particular r0=z+ch solves E2=0for every h; w0=z^2+yh solves E4=E5=0. No per-h inversion or half-trace is needed for those particulars. The two cchoices differ by1and correspond to the retained r-kernel shift h, not a discarded branch. For prefix factor bases, E4membership can restrict v to the fixed square-root image of W4, independent of h. E3and support constraints still require solving.

The prototype computes one half-trace context for odd binary extension degree. Trace-one inputs return unsupported, not empty; even extensions require a separately solved AS context. Target curve validation and canonical inputs are caller preconditions. Odd-characteristic curves use their separate compiler.

Status: syntax-checked, unexecuted and unregistered. Next freeze independent tiny complete(h,u,v)norm oracles, zero branches, both cchoices and trace-one refusal. For actualGF83, the map in3*83bits has Boolean degree at most2; evaluating the zero vector,249units andall30876pairs suffices to certify its full ANF against independent literal norm multiplication, provided the degree bound is checked. Charge context creation, coordinate conversion and output verification when comparing with the strongest existing coefficient compiler. This chart is not an outer norm search, ghost-branch closure or original root-solving speedup. The algebra uses established AS/norm identities; no novelty claim.
