#!/bin/sh
# Hybrid XL for the direct S4 descent: guess g of the N = 3d unknown bits (cost 2^g), then XL on the rest.
# When fewer than 7 bits remain, the residual is enumerated (2^N' evaluations): total cost 2^N, i.e. pure enumeration of V^3.
# n equations of degree 6 over F2 in N' = N - g bits, multilinear monomials; XL degree D minimal with
# n * sum_{i<=D-6} C(N',i) >= sum_{i<=D} C(N',i) - 1. Cost of one solve ~ M^2 cells (optimistic
# sparse linear algebra, M = monomials), total cost 2^g * M^2. Compared with the 2+1 split: 2^(d+1) solves
# of a (3d-1)-unknown linear system, each ~ (3d)^2 cells. Heuristic (random-system XL), not a measurement.
awk 'function C(n,k,  i,r){r=1; for(i=1;i<=k;i++) r=r*(n-k+i)/i; return r}
function S(n,k,  i,s){s=0; for(i=0;i<=k;i++) s+=C(n,i); return s}
function l2(x){return log(x)/log(2)}
BEGIN{
 print "n,d,N,best_g,xl_D_at_best,log2_cost_hybrid_best,log2_cost_g0,log2_cost_2plus1";
 split("13 19 23 31 37 41 53 83 131",A," "); split("4 6 8 10 12 14 18 28 44",B," ");
 for(i=1;i<=9;i++){ n=A[i]; d=B[i]; N=3*d; best=1e9; bg=-1; bD=0; c0=0;
   for(g=0; g<=N; g++){ Np=N-g; if(Np<7){ cost=g+Np; D=0 } else { D=7; while(D<Np && n*S(Np,D-6) < S(Np,D)-1) D++; M=S(Np,D); cost=g+2*l2(M) }
     if(g==0) c0=cost; if(cost<best){best=cost; bg=g; bD=D} }
   two=(d+1)+2*l2(3*d);
   printf "%d,%d,%d,%d,%d,%.1f,%.1f,%.1f\n", n,d,N,bg,bD,best,c0,two }}'
