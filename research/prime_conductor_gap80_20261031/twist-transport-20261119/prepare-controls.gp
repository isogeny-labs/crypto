\\ Independent finite-field point generation; no timing analysis.
default(parisizemax,2000000000);
pointline(P)=if(#P==1,"O",Str(lift(P[1])," ",lift(P[2])));
makecase(v,d)={
 my(pr=v[1],aa=v[2],bb=v[3],degree=v[4],K=[v[5],v[6]],E,T,pts=List(),ys,file,root);
 E=ellinit([0,0,0,aa,bb],pr);
 if(ellorder(E,K)!=degree,error("auxiliary order"));
 T=ellinit([0,0,0,d^2*aa,d^3*bb],pr);listput(pts,[0]);
 if(kronecker(d,pr)==1,
   root=sqrt(Mod(d,pr));listput(pts,[Mod(d,pr)*K[1],d*root*K[2]]);
   listput(pts,ellmul(T,pts[2],2)));
 for(xx=0,pr-1,
   ys=ellordinate(T,xx);
   for(jj=1,#ys,if(#pts<32,listput(pts,[xx,ys[jj]]))));
 file=Str("inputs/p",pr,"-d",d,".txt");
 write(file,pr," ",aa," ",bb," ",degree," ",K[1]," ",K[2]," ",#pts);
 for(jj=1,#pts,if(!ellisoncurve(T,pts[jj]),error("generated membership"));write(file,pointline(pts[jj])));
 print("case p=",pr," d=",d," quadratic_character=",kronecker(d,pr)," points=",#pts);
};
{
 iferr(
  for(jj=1,6,makecase([71,5,5,19,4,35],[1,2,3,4,5,7][jj]));
  for(jj=1,3,makecase([11,1,7,5,1,3],[1,2,4][jj]));
  print("generated9 bounded controls");
 ,failure,print(failure);quit(2));
}
quit(0);
