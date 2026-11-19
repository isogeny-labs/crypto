\\ Independent endpoint, subgroup and homomorphism checks; dual input creation.
must(v,msg)=if(!v,error(msg));
pointline(P)=if(#P==1,"O",Str(lift(P[1])," ",lift(P[2])));
{
 iferr(
 my(data=read("main-audit.gpvec"),cv=data[1],main=data[2],probe=data[3],pr=cv[1],degree=cv[4],d=cv[6],subgroup=cv[11],E,C,T,D,H,R);
 must(main[5]==degree&&probe[5]==degree&&main[6]==pr&&probe[6]==pr,"parameter identity");
 must(main[4]==(degree-1)/2&&probe[4]==(degree-1)/2,"complete half-kernel coverage");
 must(main[1]==[cv[9],cv[10]],"retained exact quotient");
 must(#main[2]==33&&#main[3]==33&&main[2]==cv[14],"main exact input coverage");
 E=ellinit([0,0,0,cv[7],cv[8]],pr);C=ellinit(concat([0,0,0],main[1]),pr);
 must(ellcard(E)==subgroup&&ellcard(C)==subgroup&&isprime(subgroup),"endpoint counts");
 H=main[3][1];must(#H==2&&ellorder(C,H)==subgroup,"transported generator order");
 for(jj=1,33,must(ellisoncurve(E,main[2][jj])&&ellisoncurve(C,main[3][jj]),"membership");
   must(ellmul(C,H,if(jj==1,1,cv[13][jj-1]))==main[3][jj],"matched scalar image"));
 must(lift(Mod(d^2*probe[1][1],pr))==main[1][1]&&lift(Mod(d^3*probe[1][2],pr))==main[1][2],"auxiliary quotient scaling");
 must(#probe[2]==1&&probe[2][1]==cv[15]&&#probe[3]==1,"auxiliary probe identity");
 T=ellinit([0,0,0,cv[2],cv[3]],pr);D=ellinit(concat([0,0,0],probe[1]),pr);
 must(ellorder(T,cv[15])==degree&&ellweilpairing(T,cv[5],cv[15],degree)!=1,"independent kernel probe");
 R=probe[3][1];must(#R==2&&ellisoncurve(D,R)&&ellorder(D,R)==degree,"dual kernel order");
 write("case0-dual.txt",pr," ",probe[1][1]," ",probe[1][2]," ",degree," ",R[1]," ",R[2]," 33");
 for(jj=1,33,write("case0-dual.txt",pointline(main[3][jj])));
 print("PASS33 original-subgroup point images;32 matched scalars; exact retained quotient; independently verified dual kernel");
 ,failure,print(failure);quit(2));
}
quit(0);
