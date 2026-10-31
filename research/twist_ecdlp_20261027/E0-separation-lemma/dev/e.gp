setfield(7,1); cv=mkcurves(1,[0,1,1]); print(type(cv[1])," ",type(cv[2])," ",type(cv[3])," ",type(cv[4]));
Bp=ptsFq(cv[2]); print(Bp); P=cv[3](Bp[2]); print(type(P)," ",P);
ph=cv[3]; print(type(ph(Bp[2])));
Bphi=vector(#Bp,j,ph(Bp[j])); print(type(Bphi[1]), type(Bphi[2]));
print(ellisoncurve(cv[1],Bphi[2]));
