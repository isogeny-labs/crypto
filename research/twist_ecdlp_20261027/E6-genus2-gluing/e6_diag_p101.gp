\\ post-hoc diagnostic (NOT part of the frozen protocol): how many distinct j exist per 2-division type at p, A,B != 0, disc != 0
read("e6lib.gp");
cnt(p)=
{my(S=[Set(),Set(),Set()],types=[[3],[1,2],[1,1,1]],f,j,k);
 for(A=1,p-1,for(B=1,p-1,if((4*A^3+27*B^2)%p==0,next);f=x^3+A*x+B;j=lift(Mod(1728*4*A^3,p)/Mod(4*A^3+27*B^2,p));
  for(k=1,3,if(ftypeF(f,p)==types[k],S[k]=setunion(S[k],Set([j]))))));
 print("DIAG p=",p," distinct_j_per_type=",vector(3,kk,length(S[kk])))
};
cnt(101);cnt(211);
quit;
