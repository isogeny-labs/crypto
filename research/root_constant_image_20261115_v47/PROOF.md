# Constant-image elimination in root-conditioned norm fibers

At fixed nonzero h,k=0, normalized F3(x) equals x[x²+(z+ch)x+dh]+(x+z)(xu+v). For a nonzero rational source point (x,y) with x!=z, vanishing F3(x)+hx*y gives xu+v=x²+h*x*(cx+d+y)/(x+z). This is a linear field equation in u,v.

Split the exact affine uv fiber into independent u-changing directions and the kernel of its u projection. The latter have constant v images. Eliminate that image once. Each source point then requires solving only the projected u-changing directions. Reconstruct every kernel coordinate by exact back substitution. In the actual fiber dimensions are73=35+38, and quotient dimension83-38=45: each point system is35x45. Both rational lifts are retained. If no rational lift exists, the original norm root cell is empty because G(x)=hx is nonzero. Zero/h=0/pole cases require their separate charts and are not rejected by this formula.

Projected columns depend linearly on x. In a low18bit domain, precompute18x35 projected multiplication increments and update the35 columns by XOR in Gray order. This identity is checked here; a native full-scan implementation and paid speed measurement are next. No low-rank update of the whole old matrix is assumed.
