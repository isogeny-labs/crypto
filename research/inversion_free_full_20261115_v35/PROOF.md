# Inversion-free all-k binary norm chart
For a valid finite target (z,y) on Y²+XY=X³+AX²+B, assume Tr(A+z)=0 and choose c²+c=A+z. Set d=y+zc. For arbitrary h,k,u,v define r=z+ch+u and w=s+zr=z²+yh+ck+v. This is affine bijective for every h,k, including both zeros. The inverse is u=r+z+ch, v=s+zr+z²+yh+ck.

With F=X³+rX²+sX+t, G=hX+k, t=z³+rz²+sz+(hz+k)y, the literal norm F²+XFG+(X³+AX²+B)G² vanishes at z. Its monic quotient by X+z is exactly ascending

[zv²+dk², v²+kv, zu²+hv+dh²+ku+k², u²+hu+k, h²+h+z, 1].

Derivation: starting with w0=z²+yh+v0, substitution gives E2=u²+hu+k, E3=zu²+hv0+dh²+ckh+ku+k², E4=v0²+kv0+(c²+c)k², E5=zv0²+[y+z(c²+c)]k². Setting v0=v+ck cancels ckh and the E4 k² term; E5 coefficient becomes y+zc=d. No inverse of h or k, squarefree hypothesis or division by them is used. Replacing c by c+1 and d by d+z corresponds to u->u+h,v->v+k, leaving original coordinates and H unchanged. No branch is discarded.

Each map has Boolean degree at most two, since fixed field multiplication and squaring are F2 bilinear/linear. F,G,t are affine in332inputbits after substitution. Comparing literal convolution and closed formula at zero,332units and54946pairs (55279points) proves equality of all quadratic ANF coefficients, hence all GF83^4 inputs. This is an exact compiler identity, not enumeration or completion of original339variable chain. Factor support, rational/twist/ghost branches and all source/U extraction remain separate.

The first validation exhausts h,k,u,v for GF4/GF8, A0/A1 and both AS choices; unique1024point samples per context cover GF16/GF32. It checks inverses of the coordinate map and AS-choice equivariance. The tiny field table is independently bit-convolved. Even extensions use explicitly supplied AS contexts; the inherited compile_context only supports odd degree. Valid target/canonical coordinates are caller preconditions; trace-one is unsupported, never empty.

Cost O(1) field operations per coefficient evaluation with one reusable AS context. This simplification makes E4=v²+kv a universal block independent of h and target constants; E2=u²+hu+k, E3 couples the two. Whether this reduces paid Schur/rank or search work is a falsifiable next experiment. Ordinary AS/norm identities are prior art, novelty unestablished. Sole-model review, no external admission.
