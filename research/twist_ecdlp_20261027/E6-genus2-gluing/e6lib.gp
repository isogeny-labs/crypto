\\ e6lib.gp -- shared functions for E6 (genus-2 gluing of E and its quadratic twist E').
\\ Pure PARI/GP.  No randomness here; callers call setrand().

nonres(p)={my(d=2);while(kronecker(d,p)!=-1,d++);d};
\\ sorted degree pattern of the factorisation of f over F_p
ftypeF(f,p)={my(F=factormod(f,p),n=matsize(F)[1]);vecsort(vector(n,i,poldegree(F[i,1])))};
\\ affine point count of y^2 = h(x) over F_p, h integer-coefficient polynomial
affcount(h,p)={my(hz=lift(h),s=0);for(u=0,p-1,s+=1+kronecker(subst(hz,x,u)%p,p));s};
\\ sample (A,B) giving y^2=x^3+Ax+B over F_p with the requested 2-division type, j not in {0,1728}
sampleE(p,typ,seen)={my(A,B,f,j);
  while(1,
    A=random(p);B=random(p);
    if(A==0||B==0,next);
    if((4*A^3+27*B^2)%p==0,next);
    f=x^3+A*x+B;
    if(ftypeF(f,p)!=typ,next);
    j=lift(Mod(1728*4*A^3,p)/Mod(4*A^3+27*B^2,p));
    if(setsearch(seen,j),next);
    return([A,B,j]))};
\\ element of F_p ? returns integer in [0,p-1] or -1
tofp(c)={my(l=lift(c));if(type(l)=="t_POL",if(poldegree(l)>0,return(-1));l=polcoeff(l,0));lift(l)};
allperms=[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]];
oddpart(n)={n>>valuation(n,2)};
lpf(n)=vecmax(factor(n)[,1]);

\\ count the points of the genus-2 curve y^2=h(x) (deg 6) over F_p and F_{p^2}, return [N1,N2]
brutecount(h,p)={my(hz=lift(h),c6=polcoeff(hz,6),N1,N2=0,W,hf,z,v);
  N1=affcount(hz,p)+if(kronecker(c6,p)==1,2,0);
  W=ffgen(p^2,'w);hf=hz*W^0;
  for(a=0,p-1,for(b=0,p-1,z=a+b*W;v=subst(hf,x,z);
     if(v==0,N2+=1,if(v^((p^2-1)/2)==1,N2+=2))));
  N2+=2;  \\ two points at infinity over F_{p^2} (F_p* is squares in F_{p^2})
  [N1,N2]};
\\ order of Jacobian from N1,N2
jacbrute(N1,N2,p)=(N1^2+N2)/2-p;
