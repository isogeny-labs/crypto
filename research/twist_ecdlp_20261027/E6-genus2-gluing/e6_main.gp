default(parisizemax,2000000000);
\\ e6_main.gp -- Stages A-D for one prime p.  Env: E6_P, E6_N (curves per 2-division type),
\\ E6_BRUTE (curves per type that also get brute-force F_p / F_{p^2} point counts), E6_SEED.
read("e6lib.gp");
P=eval(getenv("E6_P"));NPER=eval(getenv("E6_N"));NBR=eval(getenv("E6_BRUTE"));SEED=eval(getenv("E6_SEED"));
if(!isprime(P),error("P not prime"));
setrand(SEED+P);
print("RUN p=",P," N=",NPER," brute=",NBR," seed=",SEED+P);
Wf=ffgen(P^6,'W);
types=[[3],[1,2],[1,1,1]];
expEq=[3,2,6];   \\ expected number of Galois-equivariant isomorphisms E[2]->E'[2] per type
\\ ---- one curve ----
process(A,B,j,ti,idx,dobrute)={
  my(f=x^3+A*x+B,d=nonres(P),Ap,Bp,E,Et,Etw,nE,nEt,t,pat,patt,rk,rkt,pat2,pat2t,idf,rts,sig,eq,ne,ng,cnt,
     V,sol,g0,g1,g2,kap,mu,tau,rhs,ok,aT,aC,hT,hC,PT,PC,Ptar,Pctl,nJ,n1,n2,c,hm,sf,N,Nb,ell,cont);
  Ap=lift(Mod(A*d^2,P));Bp=lift(Mod(B*d^3,P));
  E=ellinit([A,B],P);Et=ellinit([Ap,Bp],P);Etw=elltwist(E,x^2-d);
  nE=ellcard(E);nEt=ellcard(Et);t=P+1-nE;
  pat=ftypeF(f,P);patt=ftypeF(x^3+Ap*x+Bp,P);
  pat2=ftypeF(elldivpol(E,2),P);pat2t=ftypeF(elldivpol(Et,2),P);
  rk=#select(z->z%2==0,ellgroup(E));rkt=#select(z->z%2==0,ellgroup(Et));
  idf=(Mod(1,P)*(d^3*subst(f,x,x/d))==Mod(1,P)*(x^3+Ap*x+Bp));
  ell=lpf(nE);
  print("A type=",ti," idx=",idx," p=",P," A=",A," B=",B," j=",j," d=",d," nE=",nE," nEt=",nEt," t=",t,
        " sum_ok=",(nE+nEt==2*P+2)," twistcard_ok=",(ellcard(Etw)==nEt)," pat=",pat," patt=",patt,
        " divpat_eq=",(pat2==pat2t&&pat2==pat)," rank2=",[rk,rkt]," rank2_eq=",(rk==rkt)," rootscale_ok=",idf,
        " jeq=",(E.j==Et.j)," gcd_odd=",oddpart(gcd(nE,nEt)));
  \\ stage D inputs (per curve)
  print("D type=",ti," idx=",idx," p=",P," nE=",nE," ell=",ell," bits_ell=",log(ell)/log(2.)," rho_steps=",sqrt(Pi*ell/2),
        " ctie=",sqrt(Pi*ell/2)/P," ctie_full=",sqrt(Pi*nE/2)/P," nE_isprime=",isprime(nE)," cofactor=",nE/ell," nJ_pred=",nE*nEt," nJ_over_p2=",1.*nE*nEt/P^2);
  \\ roots, Frobenius permutation
  rts=polrootsmod(f,[Wf.mod,P]);
  sig=vector(3,i,0);
  for(i=1,3,for(k=1,3,if(rts[i]^P==rts[k],sig[i]=k)));
  eq=select(pi->(vector(3,i,pi[sig[i]])==vector(3,i,sig[pi[i]])),allperms);
  ne=#eq;ng=0;cnt=0;
  foreach(allperms,pi,
    V=matrix(3,3,i,k,rts[i]^(k-1));
    sol=matsolve(V,vector(3,i,rts[pi[i]])~);
    g0=tofp(sol[1]);g1=tofp(sol[2]);g2=tofp(sol[3]);
    if(g0>=0&&g1>=0&&g2>=0,ng++));
  print("B0 type=",ti," idx=",idx," p=",P," frob_perm=",sig," n_equiv=",ne," n_equiv_expected=",expEq[ti]," n_interp_in_Fp=",ng," equiv_eq_interp=",(ne==ng));
  foreach(eq,pi,
    V=matrix(3,3,i,k,rts[i]^(k-1));
    sol=matsolve(V,vector(3,i,rts[pi[i]])~);
    g0=tofp(sol[1]);g1=tofp(sol[2]);g2=tofp(sol[3]);
    if(pi==[1,2,3],
      print("B1 type=",ti," idx=",idx," p=",P," pi=",pi," status=identity_g2=",g2," (expect 0 => no genus-2 curve)");next);
    if(g2==0,print("B1 type=",ti," idx=",idx," p=",P," pi=",pi," status=g2zero");next);
    g0=Mod(g0,P);g1=Mod(g1,P);g2=Mod(g2,P);
    kap=1/g2;mu=-g1/g2;tau=mu^2+A-kap*g0;
    ok=1;for(i=1,3,rhs=rts[i]^2-mu*rts[i]+mu^2+A; if(rhs!=kap*rts[pi[i]]+tau,ok=0));
    if(subst(Mod(1,P)*f,x,-mu)==0,
      print("B1 type=",ti," idx=",idx," p=",P," pi=",pi," status=singular_f(-mu)=0 ident_ok=",ok);next);
    aT=1;while(kronecker(lift(-kap*aT),P)!=-1,aT++);
    aC=1;while(kronecker(lift(-kap*aC),P)!=1,aC++);
    foreach([["target",aT],["control",aC]],arm,
      c=arm[2];
      hm=Mod(1,P)*subst(f,x,Mod(c,P)*x^2-mu);
      sf=(poldegree(gcd(hm,deriv(hm)))==0);
      if(!sf,print("B1 type=",ti," idx=",idx," p=",P," pi=",pi," arm=",arm[1]," status=sextic_not_squarefree");next);
      PT=hyperellcharpoly(hm);
      Ptar=x^4+(2*P-t^2)*x^2+P^2;Pctl=(x^2-t*x+P)^2;
      nJ=subst(PT,x,1);
      \\ quotient elliptic curves read off from the coefficients of C itself
      n1=affcount(subst(Mod(1,P)*f,x,Mod(c,P)*x-mu),P)+1;
      n2=affcount(polcoeff(hm,0)*x^3+polcoeff(hm,2)*x^2+polcoeff(hm,4)*x+polcoeff(hm,6),P)+1;
      N=[0,0];
      print("C type=",ti," idx=",idx," p=",P," pi=",pi," arm=",arm[1]," a=",c," mu=",lift(mu)," kappa=",lift(kap)," ident_ok=",ok,
            " charpoly=",PT," match_target=",(PT==Mod(1,P)*Ptar||PT==Ptar)," match_control=",(PT==Pctl)," nJ=",nJ," nE*nEt=",nE*nEt,
            " nJ_eq_prod=",(nJ==nE*nEt)," nJ_eq_nE2=",(nJ==nE^2)," n1=",n1," n2=",n2," n1_eq_nE=",(n1==nE)," n2_eq_nEt=",(n2==nEt),
            " n2_eq_nE=",(n2==nE)," nE_div_nJ=",(nJ%nE==0));
      if(dobrute,
        Nb=brutecount(hm,P);
        print("C2 type=",ti," idx=",idx," p=",P," pi=",pi," arm=",arm[1]," N1=",Nb[1]," N2=",Nb[2]," N1_eq_p+1=",(Nb[1]==P+1)," nJ_brute=",jacbrute(Nb[1],Nb[2],P)," brute_eq_pari=",(jacbrute(Nb[1],Nb[2],P)==nJ)));
    );
  );
};
runall()={my(ti=0,seen,s);
foreach(types,typ,
  ti++;seen=Set();
  for(idx=1,NPER,
    s=sampleE(P,typ,seen);seen=setunion(seen,Set([s[3]]));
    process(s[1],s[2],s[3],ti,idx,(idx<=NBR))));};
runall();
print("DONE p=",P);
quit;
