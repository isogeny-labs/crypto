rate(k,n)={my(z=1.959964,ph=k/n,den=1+z^2/n,ctr=(ph+z^2/(2*n))/den,hw=z*sqrt(ph*(1-ph)/n+z^2/(4*n^2))/den);[ph*1.,max(0.,ctr-hw)*1.,min(1.,ctr+hw)*1.]};
default(realprecision,8);
foreach([[221,221],[580,580],[56,56],[112,112],[1300,1300],[1300,0],[221,0]],v,print("WILSON k=",v[1]," n=",v[2]," ",rate(v[1],v[2])));
quit;
