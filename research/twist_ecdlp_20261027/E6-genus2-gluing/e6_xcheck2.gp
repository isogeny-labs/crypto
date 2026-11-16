default(parisizemax,2000000000);
cntj(p)={my(S=[Set(),Set(),Set()],typ,j,f);
  for(A=1,p-1,for(B=1,p-1,if((4*A^3+27*B^2)%p==0,next);
    f=Mod(1,p)*(x^3+A*x+B);
    my(r=#polrootsmod(f,p)); \\ number of distinct roots in F_p: 0 ->[3], 1->[1,2], 3->[1,1,1]
    my(k=if(r==0,1,if(r==1,2,3)));
    j=lift(Mod(1728*4*A^3,p)/Mod(4*A^3+27*B^2,p));
    S[k]=setunion(S[k],Set([j]))));
  vector(3,i,#S[i])};
print("p=101 ",cntj(101));
print("p=211 ",cntj(211));
\\ Legendre-lambda formula check for [1,1,1]: j=256(l^2-l+1)^3/(l^2(l-1)^2), count distinct j over l!=0,1, excluding j=0,1728
lam(p)={my(S=Set());for(l=2,p-1,my(j=lift(256*Mod(l^2-l+1,p)^3/(Mod(l,p)^2*Mod(l-1,p)^2)));if(j!=0&&j!=1728%p,S=setunion(S,Set([j]))));#S};
print("lambda p=101 ",lam(101)," p=211 ",lam(211));
\\ supersingular j count
ss(p)={my(c=0);for(A=1,p-1,for(B=1,p-1,if((4*A^3+27*B^2)%p==0,next);c+=0));c};
quit;
