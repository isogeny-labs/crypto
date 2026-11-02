\\ GP source for the P1 construction subcommands (embedded by src/construct.rs, evaluated by libpari
\\ through dlopen).  One definition per top-level line (indented lines continue it); see Pari::load.
\\ Globals: p a b n TR E0 P0 Q0 k0 H Ep phi Eaux Gq Rq RR.   All outputs are strings of decimals.

\\ first rational point at x >= x0 (mod p): the Python `Ctx.point`
P1pt(E,x0)=my(x=x0%p,y);while(1,y=ellordinate(E,x);if(#y,return([x,lift(y[1])]));x=(x+1)%p);

P1init(pp,aa,bb,nn,tr)=p=pp;a=aa;b=bb;n=nn;TR=tr;E0=ellinit([a,b],p);
  if(ellcard(E0)!=n||p+1-n!=TR||!isprime(n),error("E0 check failed"));
  Str(lift(E0.j));

P1jform(aa,bb)=Str(lift(Mod(1728*4*aa^3*(4*aa^3+27*bb^2)^(-1),p)));

P1plant(xP,k)=P0=P1pt(E0,xP);k0=k;if(ellmul(E0,P0,n)!=[0],error("P order"));Q0=lift(ellmul(E0,P0,k));
  Str(P0[1]," ",P0[2]," ",Q0[1]," ",Q0[2]);

\\ isomorphic-model control: a*u^4, b*u^6; returns "a b j-equal oncurve(image of Q0)"
P1iso(u)=my(ai=a*u^4%p,bi=b*u^6%p,Ei=ellinit([ai,bi],p),qx=Q0[1]*u^2%p,qy=Q0[2]*u^3%p);
  if(lift(Ei.j)!=lift(E0.j)||ellcard(Ei)!=n,error("iso control"));
  Str(ai," ",bi," ",qx," ",qy," ",ellisoncurve(Ei,[qx,qy]));

\\ Elkies roots of Phi_l(j0, Y) mod p, sorted
P1roots(l)=my(Pj=subst(polmodular(l),'x,Mod(lift(E0.j),p)),r=polrootsmod(Pj,p));Str(vecsort(Vec(lift(r))));

\\ l-division polynomial, Frobenius on x-coordinates, eigenvalues of Frobenius on E[l]
P1prep(l)=Psi=elldivpol(E0,l);Xp=Mod('x,Psi)^p;Str(vecsort(Vec(lift(polrootsmod('x^2-TR*'x+p,l)))));

\\ split g = K1*K2 (two kernel polynomials sharing x_lam); `which` 0/1 picks the smaller/larger
P1split(g,l,which)=my(f0,x1,F1,xs,K,cs,K1,q,K2,pr);
  f0=subst(lift(factormod(g,p)[1,1]),'x,'y)*Mod(1,p);
  x1=Mod('y,f0);F1=x1^3+a*x1+b;
  xs=vector((l-1)/2);xs[1]=x1;xs[2]=(x1^4-2*a*x1^2-8*b*x1+a^2)/(4*F1);
  for(k=2,(l-1)/2-1,my(xk=xs[k],num=(xk*x1-a)^2-4*b*(xk+x1));xs[k+1]=num/(xk-x1)^2/xs[k-1]);
  K=1;for(k=1,(l-1)/2,K=K*('x-xs[k]));
  cs=vector(poldegree(K)+1,i,my(c=lift(Vec(K)[i]));if(c!=polcoef(c,0,'y),error("factor not in an eigenline"));polcoef(c,0,'y));
  K1=Polrev(Vecrev(cs));K1=K1/pollead(K1);
  q=divrem(g,K1);if(q[2]!=0,error("split remainder"));q=q[1];K2=q/pollead(q);
  pr=if(lex(lift(Vec(K1)),lift(Vec(K2)))<=0,[K1,K2],[K2,K1]);
  pr[which+1];

\\ kernel polynomial H of the lambda-eigenline; returns its degree; nl = number of eigenvalues
P1kern(l,lam,idx,nl)=my(k=min(lam,l-lam),fm=elldivpol(E0,k-1),f0=elldivpol(E0,k),fp1=elldivpol(E0,k+1),
    F='x^3+a*'x+b,num,den,xk,g);
  if(k%2==0,num=fm*fp1*4*F;den=f0^2,num=fm*fp1;den=4*F*f0^2);
  xk='x-Mod(num,Psi)/Mod(den,Psi);
  g=gcd(Psi,lift(Xp-xk));
  if(poldegree(g)==(l-1)/2,H=g/pollead(g),
    if(poldegree(g)!=l-1||nl!=2,error("kernel degree"));
    H=P1split(g,l,idx);
    if(poldegree(H)!=(l-1)/2,error("split degree")));
  poldegree(H);

\\ Velu isogeny of the kernel H: "j' a' b'"
P1velu()=my(Iso=ellisogeny(E0,H));Ep=ellinit(Iso[1],p);phi=Iso[2];
  if(Iso[1][1]!=0||Iso[1][2]!=0||Iso[1][3]!=0,error("a1a2a3"));
  Str(lift(Ep.j)," ",lift(Iso[1][4])," ",lift(Iso[1][5]));

\\ checks; xs = vector of 12 starting abscissae (pairs a,b for 6 hom tests)
P1verify(l,xs)=my(nume=phi[1],dene=phi[3]^2,dg=[poldegree(nume),poldegree(dene)],degok,hom=1,onc=1,cardok,fP,fQ,ordok,dlpok,jnot);
  degok=(dg==[l,l-1]&&phi[3]/pollead(phi[3])==H);
  for(i=1,6,my(A=P1pt(E0,xs[2*i-1]),B=P1pt(E0,xs[2*i]),S=elladd(E0,A,B),fa=ellisogenyapply(phi,A),fb=ellisogenyapply(phi,B),fs=ellisogenyapply(phi,S));
    onc=onc&&ellisoncurve(Ep,fa);hom=hom&&(elladd(Ep,fa,fb)==fs));
  cardok=(ellcard(Ep)==n);
  fP=ellisogenyapply(phi,P0);fQ=ellisogenyapply(phi,Q0);
  ordok=(fP!=[0]&&ellmul(Ep,fP,n)==[0]);
  dlpok=(ellmul(Ep,fP,k0)==fQ);
  jnot=(lift(Ep.j)!=lift(E0.j));
  Str(dg[1]," ",dg[2]," ",degok," ",hom," ",onc," ",cardok," ",ordok," ",dlpok," ",jnot," ",
      if(#fP==1,"O",Str(lift(fP[1])," ",lift(fP[2])))," ",if(#fQ==1,"O",Str(lift(fQ[1])," ",lift(fQ[2]))));

P1kcoef()=Str(lift(Vec(H)));

\\ ---- tower -------------------------------------------------------------------------------
\\ auxiliary curve candidate: 1 iff non-singular and the largest cyclic factor is divisible by 2^T
P1auxtry(A,B,T)=my(E,d);
  if((4*A^3+27*B^2)%p==0,return(0));
  E=ellinit([A,B],p);d=ellgroup(E,,1);
  if(#d[2]>0&&d[2][1]%(2^T)==0,Eaux=E;Dg=d;1,0);

P1auxinit(T)=my(order=ellcard(Eaux),d1=Dg[2][1],G=Dg[3][1]);
  Qaux=ellmul(Eaux,G,d1/2^T);
  if(ellmul(Eaux,Qaux,2^T)!=[0]||ellmul(Eaux,Qaux,2^(T-1))==[0],error("Q order"));
  Str(order," ",d1," ",if(#Dg[2]>1,Dg[2][2],0));

P1tryR(x)=my(y=ellordinate(Eaux,x));if(#y,Rq=[x,lift(y[1])];1,0);

\\ tower of height t built from the 2^T-order subgroup; xs = t starting abscissae for the check points
\\ output: t stages of (num0 num1 num2 den0 den1 a b), then c, then the 2^t sorted abscissae
P1tower(t,T,xs)=my(Ej=Eaux,Qj=ellmul(Eaux,Qaux,2^(T-t)),Rj=Rq,out=List(),Qt,V,cur,c);Qt=Qj;
  for(jj=0,t-1,
    my(K=ellmul(Ej,Qj,2^(t-jj-1)),Iso,f,num,den,lc,xx,yy,Pj,img);
    if(K==[0]||K[2]!=0,error("2-torsion kernel"));
    Iso=ellisogeny(Ej,K);
    f=Iso[2][1]/(Iso[2][3]^2);num=numerator(f);den=denominator(f);
    if([poldegree(num),poldegree(den)]!=[2,1],error("x-map degrees"));
    lc=pollead(den);num=num/lc;den=den/lc;
    xx=xs[jj+1];yy=ellordinate(Ej,xx);while(#yy==0,xx=xx+1;yy=ellordinate(Ej,xx));
    Pj=[xx,lift(yy[1])];img=ellisogenyapply(Iso[2],Pj);
    if(subst(num,'x,xx)/subst(den,'x,xx)!=img[1],error("x-map vs ellisogenyapply"));
    listput(out,lift(polcoef(num,0)));listput(out,lift(polcoef(num,1)));listput(out,lift(polcoef(num,2)));
    listput(out,lift(polcoef(den,0)));listput(out,lift(polcoef(den,1)));
    listput(out,lift(Ej[4]));listput(out,lift(Ej[5]));
    Qj=ellisogenyapply(Iso[2],Qj);Rj=ellisogenyapply(Iso[2],Rj);Ej=ellinit(Iso[1],p));
  if(#Rj==1,error("c at infinity"));c=lift(Rj[1]);
  V=vector(2^t);cur=Rq;
  for(k=1,2^t,if(#cur==1,error("coset hits O"));V[k]=lift(cur[1]);cur=elladd(Eaux,cur,Qt));
  V=vecsort(V);
  for(k=2,2^t,if(V[k]==V[k-1],error("coset collapses")));
  \\ every v runs down the chain to c: re-derive the stages from `out`
  for(k=1,2^t,my(u=Mod(V[k],p));
    for(jj=0,t-1,my(o=7*jj,nu=out[o+1]+out[o+2]*u+out[o+3]*u^2,de=out[o+4]+out[o+5]*u);
      if(de==0,error("den zero"));u=nu/de);
    if(lift(u)!=c,error("tower does not reach c")));
  Str(Vec(out)," ",c," ",V);

\\ ---- targets -----------------------------------------------------------------------------
P1tgt(ks)=RR=vector(#ks,i,if(ks[i]==k0,Q0,lift(ellmul(E0,P0,ks[i]))));
  Str(vector(#ks,i,Str(RR[i][1]," ",RR[i][2])));

\\ rebuild phi from the stored kernel polynomial (high -> low coefficients) and check it
P1arm(co,jx,ax,bx)=my(h=Pol(vector(#co,i,Mod(co[i],p))),Iso,fP,res=List());
  if(h!=Pol(co)*Mod(1,p),error("Pol"));
  Iso=ellisogeny(E0,h);Ep=ellinit(Iso[1],p);
  if(lift(Ep.j)!=jx||lift(Iso[1][4])!=ax||lift(Iso[1][5])!=bx,error("stored j/a/b"));
  phi=Iso[2];fP=ellisogenyapply(phi,P0);
  listput(res,Str(lift(fP[1])," ",lift(fP[2])));
  for(i=1,#RR,my(img=ellisogenyapply(phi,RR[i]));if(!ellisoncurve(Ep,img),error("image off curve"));listput(res,Str(lift(img[1]))));
  Str(Vec(res));
