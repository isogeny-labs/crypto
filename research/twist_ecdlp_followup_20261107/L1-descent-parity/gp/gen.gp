default(parisizemax,1000000000);
\\ stage 0: curve and target generation for one size s (section 11 step 0). env: S, OUTDIR
read(Str(getenv("GPDIR"),"/lib.gp"));
S=eval(getenv("S"));
NCURV=[0,0,0,0];
cfg(s)={ \\ [ncurves, ntargets per curve]
  if(s==20,[6,5],if(s==24,[6,4],if(s==28,[4,3],[3,2])))};
lpf(n)={my(f=factor(n)[,1]);f[#f]}
band(N,s)=(N>=0.8*2^s && N<=1.25*2^s);
nomC(s)=if(s==20,101,if(s==24,256,if(s==28,645,1625)));

\\ arms A,B (curves over F_p)
gen_AB(s)={
  my(c=cfg(s),nc=c[1],nt=c[2],pn,p,a,b,Ep,N1,N2,t,N,l1,l2,ls,out,seed,js,cnt,ME,MW,tries,fi,cs,recs=List(),k,gs);
  pn=nextprime(2^(s\2));
  while(1,
    p=pn; seed=7000000+10000*s+100*1+0;
    setrand(seed);b=1+random(p-1);a=random(p);
    out=List();js=List();tries=0;
    while(#out<nc && tries<p,
      tries++;a=(a+1)%p;
      if((4*a^3+27*b^2)%p==0,next);
      Ep=ellinit([a,b],p);N1=ellcard(Ep);t=p+1-N1;N2=p+1+t;N=N1*N2;
      if(N1%2==0||N2%2==0,next);
      if(gcd(N1,N2)!=1||!issquarefree(N)||N1==p||N2==p||t==0||!band(N,s),next);
      l1=lpf(N1);l2=lpf(N2);
      if(N1/l1>15||N2/l2>15,next);
      if(setsearch(Set(js),lift(Mod(a^3,p)/Mod(4*a^3+27*b^2,p))),next);
      listput(js,lift(Mod(a^3,p)/Mod(4*a^3+27*b^2,p)));
      ls=max(l1,l2);cs=if(ls==l1,1,-1);
      ME=0;MW=0;for(x=0,p-1,fi=kronecker((x^3+a*x+b)%p,p);if(fi==1,ME++);if(fi==-1,MW++));
      listput(out,[a,b,N,N1,N2,ls,N/ls,cs,ME,MW]));
    if(#out>=nc,break);
    pn=nextprime(pn+1));
  setrand(7000000+10000*s+100*1+1);
  out=Vec(out);
  for(ci=1,#out,for(ti=1,nt,
    k=random(out[ci][3]);gs=random(2^31);
    listput(recs,[1,s,#recs+1,p,2,fq_par(p,2),out[ci][1],0,0,out[ci][2],0,0,out[ci][3],out[ci][4],out[ci][5],out[ci][6],out[ci][7],k,gs,out[ci][8],out[ci][9],out[ci][10]])));
  Vec(recs);
}

\\ arms D (deg 2) and C (deg 3): generic curves over F_{p^deg}
gen_G(s,deg)={
  my(c=cfg(s),nc=c[1],nt=c[2],pn,p,fp,a,b,E,N,ls,h,js,out,tries,j,seed,ar,recs=List(),k,gs,V,x,arm=if(deg==2,4,3),vc);
  pn=nextprime(if(deg==2,2^(s\2),nomC(s)));
  while(1,
    p=pn;seed=7000000+10000*s+100*arm+0;
    setrand(seed);fp=fq_par(p,deg);fq_setup(p,deg,fp);
    out=List();js=List();tries=0;
    while(#out<nc && tries<4000,
      tries++;
      a=random(GT);b=random(GT);
      if(infp(a)||infp(b),next);
      if(4*a^3+27*b^2==0,next);
      E=ellinit([a,b]);j=E.j;
      if(infp(j),next);
      N=ellcard(E);
      if(!band(N,s)||N==p^deg,next);
      ls=lpf(N);h=N/ls;if(h>8,next);
      if(#ellgroup(E)!=1,next);
      if(ellissupersingular(E),next);
      if(setsearch(Set(js),j),next);
      listput(js,j);
      vc=0;for(x=0,p-1,if(issquare(x^3+a*x+b),vc++));
      listput(out,[a,b,N,ls,h,vc]));
    if(#out>=nc,break);
    pn=nextprime(pn+1));
  setrand(7000000+10000*s+100*arm+1);
  out=Vec(out);
  for(ci=1,#out,for(ti=1,nt,
    k=random(out[ci][3]);gs=random(2^31);
    my(ap=out[ci][1].pol,bp=out[ci][2].pol);
    listput(recs,[arm,s,#recs+1,p,deg,fp,polcoef(ap,0,'tt),polcoef(ap,1,'tt),polcoef(ap,2,'tt),polcoef(bp,0,'tt),polcoef(bp,1,'tt),polcoef(bp,2,'tt),out[ci][3],0,0,out[ci][4],out[ci][5],k,gs,0,out[ci][6],0])));
  Vec(recs);
}

wr(file,recs)={
  write(file,"arm,s,inst,p,kdeg,fpar,a0,a1,a2,b0,b1,b2,N,N1,N2,lstar,h,k,gseed,cstar,ME,MW");
  for(i=1,#recs,write(file,strjoin(vector(#recs[i],j,Str(recs[i][j])),",")));
}
f=Str(OUTDIR,"/curves_",S,".csv");
system(Str("rm -f ",f));
allr=concat([gen_AB(S),gen_G(S,2),gen_G(S,3)]);
wr(f,allr);
print("stage0 s=",S," instances=",#allr," arms=",Set(vector(#allr,i,allr[i][1])));
