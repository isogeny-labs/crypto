default(parisizemax,4000000000);
twist_div_monic(U)={U/pollead(U)};
twist_div_valid(D,F)={if(poldegree(D[1])>2||pollead(D[1])!=1||poldegree(D[2])>=poldegree(D[1])||((D[2]^2-F)%D[1])!=0,error("invalid reduced divisor"))};
twist_div_reduce(U,V,F)={U=twist_div_monic(U);V=V%U;while(poldegree(U)>2,U=twist_div_monic((F-V^2)/U);V=(-V)%U);my(D=[U,V]);twist_div_valid(D,F);D};
twist_div_add(A,B,F)={my(z=bezout(A[1],B[1]),h=bezout(z[3],A[2]+B[2]),d=h[3],U=A[1]*B[1]/d^2,V=(h[1]*z[1]*A[1]*B[2]+h[1]*z[2]*B[1]*A[2]+h[2]*(A[2]*B[2]+F))/d);twist_div_reduce(U,V,F)};
twist_div_norm(D,E,NX,DX,NY,DY,base)={my(U=D[1],V=D[2],degree=poldegree(U),h=gcd(U,DX),S=[0]);while(poldegree(h)>0,U=twist_div_monic(U/h);h=gcd(U,DX));if(poldegree(U)>0,my(xx=lift(Mod(NX,U)/Mod(DX,U)),yy=lift(Mod(V*NY,U)/Mod(DY,U)),al=polcoef(xx,0),be=polcoef(xx,1),ga=polcoef(yy,0),de=polcoef(yy,1));if(poldegree(U)==1,S=[al,ga],if(be==0,if(de==0,S=elladd(E,[al,ga],[al,ga]),if(2*ga-de*polcoef(U,1)!=0,error("constant-x quotient trace"))),my(slope=de/be,xxsum=2*al-be*polcoef(U,1),x3=slope^2-xxsum);S=[x3,slope*(al-x3)-ga])));if(!ellisoncurve(E,S),error("norm image off target curve"));ellsub(E,S,ellmul(E,base,degree))};
