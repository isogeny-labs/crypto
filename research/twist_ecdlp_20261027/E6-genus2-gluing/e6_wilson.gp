\\ Wilson 95% interval for k/n pairs given in E6_PAIRS="k/n,k/n,..."  (post-run helper; no randomness)
rate(k,n)={my(z=1.959964,ph=k/n,den=1+z^2/n,ctr=(ph+z^2/(2*n))/den,hw=z*sqrt(ph*(1-ph)/n+z^2/(4*n^2))/den);[ph*1.,max(0.,ctr-hw)*1.,min(1.,ctr+hw)*1.]};
default(realprecision,8);
foreach(strsplit(getenv("E6_PAIRS"),","),s,my(v=strsplit(s,"/"),k=eval(v[1]),n=eval(v[2]));if(n>0,print("WILSON k=",k," n=",n," ",rate(k,n))));
quit;
