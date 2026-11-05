#!/bin/sh
# Heuristic XL cost of solving the Weil descent of G(x1,x2,x3,xR) = 0 over V^3 directly.
# Unknowns: N = 3d bits (x_i in span{1..z^(d-1)}); equations: n (the F2 coordinates of G = 0);
# degree 6 in the unknown bits (u_i^3 products, u_i = sqrt(x_i), from sqrt(e4) * e3). Bits satisfy
# b^2 = b, so monomials are multilinear. XL needs n * sum_{i<=D-6} C(N,i) >= sum_{i<=D} C(N,i) - 1.
# This is the standard random-system heuristic, not a measurement. d = (n+1)/3 (the m=2 optimum).
awk 'function C(n,k,  i,r){r=1; for(i=1;i<=k;i++) r=r*(n-k+i)/i; return r}
function S(n,k,  i,s){s=0; for(i=0;i<=k;i++) s+=C(n,i); return s}
BEGIN{
 print "n,d,N,linear_oracle_lmax,xl_D,log2_monomials,log2_matrix_cells,log2_m2_floor,log2_rho";
 split("13 19 23 31 37 41 53 83 131",A," "); split("4 6 8 10 12 14 18 28 44",B," ");
 for(i=1;i<=9;i++){ n=A[i]; d=B[i]; N=3*d; lmax=int((n+24)/31);
   D=7; while(D<=N && n*S(N,D-6) < S(N,D)-1) D++;
   mon=S(N,D); printf "%d,%d,%d,%d,%d,%.1f,%.1f,%d,%.1f\n", n,d,N,lmax,D,log(mon)/log(2),log(mon*n*S(N,D-6))/log(2),n-d+1,n/2 }}'
