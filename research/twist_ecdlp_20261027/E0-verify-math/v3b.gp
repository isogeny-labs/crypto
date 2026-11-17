default(parisizemax,1000000000);
p=7;
fq(z)=(z^p==z);
rat(S)=(#S==1)||(S[1]^p==S[1]&&S[2]^p==S[2]);
{
 w=ffgen(p^2,'w); d=3; \\ 3 nonsquare mod 7
if(kronecker(d,p)!=-1,error("d")); del=sqrt(d*w^0);
E=ellinit([0,0,0,0,1]*w^0); Et=ellinit([0,0,0,0,d^3]*w^0);
A=List([[0]]); for(x=0,p-1,ys=ellordinate(E,x*w^0); for(j=1,#ys, if(fq(ys[j]),listput(A,[x*w^0,ys[j]]))));
Bp=List([[0]]); for(x=0,p-1,ys=ellordinate(Et,x*w^0); for(j=1,#ys, if(fq(ys[j]),listput(Bp,[x*w^0,ys[j]]))));
print("#E=",#A," #E'=",#Bp);
Bp=Vec(Bp); A=Vec(A);
B=vector(#Bp,j,if(#Bp[j]==1,[0],[Bp[j][1]/d,Bp[j][2]/(d*del)]));
nbad=0; for(j=1,#B,if(!ellisoncurve(E,B[j]),nbad++)); print("phi off-curve: ",nbad);
cntS=0; Bs=Set(); cntR=Map();
for(j=1,#B,for(i=1,#A, S=elladd(E,A[i],B[j]); if(rat(S), cntS++; Bs=setunion(Bs,Set([B[j]])); k=S; c=0; if(mapisdefined(cntR,[S],&c),mapput(cntR,[S],c+1),mapput(cntR,[S],1)))));
print("rational sums: ",cntS," (expected nE*g2=48) distinct B: ",#Bs);
print("B list: ",Bs);
print("all B 2-torsion: ",vecsum(apply(P->elladd(E,P,P)==[0],Vec(Bs)))==#Bs);
M=Mat(cntR); print("decomp counts per R: min ",vecmin(M[,2])," max ",vecmax(M[,2])," nR ",#M[,1]);
}
