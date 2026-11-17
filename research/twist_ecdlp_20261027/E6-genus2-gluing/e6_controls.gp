default(parisizemax,2000000000);
\\ e6_controls.gp -- controls for one prime.  Env: E6_P, E6_NPOS, E6_NNEG, E6_NNULL, E6_SEED.
read("e6lib.gp");
P=eval(getenv("E6_P"));NPOS=eval(getenv("E6_NPOS"));NNEG=eval(getenv("E6_NNEG"));NNULL=eval(getenv("E6_NNULL"));SEED=eval(getenv("E6_SEED"));
if(!isprime(P),error("P not prime"));
setrand(SEED+P+7);
print("CTRL_RUN p=",P," npos=",NPOS," nneg=",NNEG," nnull=",NNULL," seed=",SEED+P+7);
\\ PC1: random split sextics y^2=a x^6+b x^4+c x^2+d ; charpoly must equal product of the quotients' charpolys
posctl()={my(a,b,c,d,F,Fm,h,hm,PT,n1,n2,t1,t2,k=0,hit=0,tries=0);
  while(k<NPOS,
    tries++;a=1+random(P-1);b=random(P);c=random(P);d=1+random(P-1);
    Fm=Mod(1,P)*(a*x^3+b*x^2+c*x+d);
    if(poldegree(gcd(Fm,deriv(Fm)))>0,next);
    hm=Mod(1,P)*(a*x^6+b*x^4+c*x^2+d);
    n1=affcount(a*x^3+b*x^2+c*x+d,P)+1;
    n2=affcount(d*x^3+c*x^2+b*x+a,P)+1;
    t1=P+1-n1;t2=P+1-n2;
    PT=hyperellcharpoly(hm);
    k++;
    if(PT==(x^2-t1*x+P)*(x^2-t2*x+P),hit++));
  print("POS p=",P," n=",NPOS," hits=",hit," tries=",tries)};
\\ NC1: Galois-module detector on pairs of curves with different 2-division type
negGal()={my(types=[[3],[1,2],[1,1,1]],i,j,s1,s2,bad=0,seen=Set(),n=0);
  for(r=1,NNEG,i=1+random(3);j=1+(i+random(2))%3;  \\ j != i
    s1=sampleE(P,types[i],Set());s2=sampleE(P,types[j],Set());
    n++;
    if(ftypeF(x^3+s1[1]*x+s1[2],P)==ftypeF(x^3+s2[1]*x+s2[2],P),bad++));
  print("NEGGAL p=",P," pairs=",n," flagged_isomorphic=",bad)};
\\ NC2: null population = uniform random squarefree sextics; hit = charpoly of the form x^4+(2p-t^2)x^2+p^2, t^2<=4p
nullctl()={my(c,h,hm,PT,k=0,hit=0,ev=0,t2,tries=0,hs=0);
  while(k<NNULL,
    tries++;h=sum(i=0,6,(if(i==6,1+random(P-1),random(P)))*x^i);
    hm=Mod(1,P)*h;
    if(poldegree(gcd(hm,deriv(hm)))>0,next);
    PT=hyperellcharpoly(hm);k++;
    if(polcoeff(PT,3)==0&&polcoeff(PT,1)==0,ev++;
      t2=2*P-polcoeff(PT,2);
      if(t2>=0&&t2<=4*P&&issquare(t2),hit++;if(t2==0,hs++))));
  print("NULL p=",P," n=",NNULL," even_charpoly=",ev," hits=",hit," hits_t0=",hs," tries=",tries)};
if(NPOS>0,posctl());
if(NNEG>0,negGal());
if(NNULL>0,nullctl());
print("CTRL_DONE p=",P);
quit;
