\\ e2_micro.gp -- S0 cost calibration: 20000 pair steps, lookups and group additions, plus wall-time projections for the grid.  Env: E2_BITS
default(parisizemax,2000000000);
default(realprecision,38);
read("e2_lib.gp");
bits=e2_envnum("E2_BITS",26);
tt=e2_curves(bits,1); p=tt[1]; cv=tt[2][1]; a=cv[1]; b=cv[2]; l=cv[3];
print("HEADER,gp_version=",version(),",bits=",bits,",p=",p,",a=",a,",b=",b,",l=",l);
Ec=ellinit([a,b],p);
mu=1.5; N=round(mu*p^(1/3));
fb=e2_scan(a,b,p,N); VE=fb[1];
\\ pair steps: 200 abscissae -> 200*201/2 = 20100 steps (incl. 200 doublings)
t0=getabstime(); h=e2_half(VE[1..200],a,b,p,0,200); tpair=getabstime()-t0; npair=h[2];
print("S0,pair_steps=",npair,",ms=",tpair,",us_per_step=",1000.*tpair/npair);
hE=e2_half(VE,a,b,p,0,N);
t0=getabstime(); SE=vecsort(hE[1]); tsort=getabstime()-t0; n=#SE;
print("S0,sort_n=",n,",ms=",tsort,",us_per_n_log2n=",1000.*tsort/(n*e2_clog2(n)));
x0=p\2; while(kronecker((x0^3+a*x0+b)%p,p)!=1, x0++);
P=[Mod(x0,p),ellordinate(Ec,x0)[1]];
R=ellmul(Ec,P,12345);
t0=getabstime(); for(i=1,20000, R=elladd(Ec,R,P)); tadd=getabstime()-t0;
print("S0,group_additions=20000,ms=",tadd,",us_per_add=",1000.*tadd/20000);
setrand(1);
xs=vector(20000,i,random(p));
t0=getabstime(); PR=0; for(i=1,20000, e2_find(~SE,xs[i])); tlk=getabstime()-t0;
print("S0,lookups=20000,ms=",tlk,",probes=",PR,",us_per_lookup=",1000.*tlk/20000);
\\ projections (seconds of CPU) = pairs + sorts + 3 collections-equivalent ; ARMS=all adds PC1 + NC1 + J0
tp=tpair/npair; ts=tsort/(n*e2_clog2(n)); ta=tadd/20000; tl=tlk/20000;
proj(bs,m,all)={ my(pp=precprime(2^bs-1), NN=round(m*pp^(1/3)), WW=ceil(0.5*NN*log(NN)), y=min(1,NN^2/((pp-1)/2.)), T=WW/y, nk=NN^2, sec);
  sec = 2*(NN*(NN+1)/2)*tp                                  \\ both tables
      + (2*nk*e2_clog2(nk)+2*nk*e2_clog2(2*nk))*ts          \\ sorts: E, E', joint
      + 2*T*(ta+2*tl);                                       \\ collections E and E' (two lookups each)
  if(all, sec += T*(ta+tl) + T*(ta+tl) + NN^2*tp);          \\ PC1 + NC1 + J0
  sec/1000. }
foreach([22,26,30,32], bs, foreach([0.75,1.5,3], m, print("PROJ,bits=",bs,",mu=",m,",core_s=",proj(bs,m,0),",all_s=",proj(bs,m,1))));
quit;
