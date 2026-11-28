\\ Exact retained public CM case; native PARI arithmetic generates inputs.
pointline(P)=if(#P==1,"O",Str(lift(P[1])," ",lift(P[2])));
{
 iferr(
 my(pr=5629830687772387,aa=5627477827931875,bb=84482773359877,
    degree=33555419,K=[229789005348518,5112785640887010],d=2,
    actuala=5620419248410339,actualb=675862186879016,
    floora=2539833775737119,floorb=1737207970637906,
    subgroup=5629830654216971,G=[2,1401162638053187],T,E,C,group,probe=[0],candidate,
    scalars=List(),points=List(),trace,disc,pairing);
 T=ellinit([0,0,0,aa,bb],pr);E=ellinit([0,0,0,actuala,actualb],pr);
 C=ellinit([0,0,0,floora,floorb],pr);
 if(!isprime(pr)||!isprime(degree)||!isprime(subgroup),error("primality"));
 if(lift(Mod(d^2*aa,pr))!=actuala||lift(Mod(d^3*bb,pr))!=actualb,error("twist coefficients"));
 if(kronecker(d,pr)!=-1,error("expected nonsquare twist"));
 if(ellcard(E)!=subgroup||ellcard(C)!=subgroup,error("independent endpoint counts"));
 if(!ellisoncurve(E,G)||ellorder(E,G)!=subgroup||ellorder(T,K)!=degree,error("orders"));
 trace=pr+1-subgroup;disc=trace^2-4*pr;
 if(trace!=33555417||disc!=-19*degree^2||qfbclassno(-19)!=1,error("CM discriminant"));
 if(lift(E.j)!=pr-884736||E.j==C.j,error("CM j classes"));
 setrand(20261119);group=ellgroup(T,,1);
 for(jj=1,#group[3],
   candidate=ellmul(T,group[3][jj],group[2][jj]/degree);
   if(#candidate>1&&ellweilpairing(T,K,candidate,degree)!=1,probe=candidate;break));
 if(#probe==1||ellorder(T,probe)!=degree,error("independent auxiliary torsion"));
 pairing=lift(ellweilpairing(T,K,probe,degree));
 setrand(20261119);listput(points,G);
 for(jj=1,32,my(kk=random(subgroup-1)+1);listput(scalars,kk);listput(points,ellmul(E,G,kk)));
 write("case0-main.txt",pr," ",aa," ",bb," ",degree," ",K[1]," ",K[2]," ",#points);
 for(jj=1,#points,write("case0-main.txt",pointline(points[jj])));
 write("case0-probe.txt",pr," ",aa," ",bb," ",degree," ",K[1]," ",K[2]," 1");
 write("case0-probe.txt",pointline(probe));
 write("case0.gpvec",lift([pr,aa,bb,degree,K,d,actuala,actualb,floora,floorb,subgroup,G,Vec(scalars),Vec(points),probe,trace,disc,pairing]));
 print("retained case0: independent counts, prime orders, ordinary CM discriminant,32 targets and auxiliary torsion validated");
 ,failure,print(failure);quit(2));
}
quit(0);
