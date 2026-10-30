default(parisizemax,2000000000);
\\ my own structural predictor, V = all of F_q, same targets as v1.gp
q=eval(getenv("VQ"));a=eval(getenv("VA"));b=eval(getenv("VB"));Wc=eval(getenv("VW"));NT=eval(getenv("VT"));
Eq=ellinit([a,b],q);cardE=ellcard(Eq);
f(x)=(x^3+a*x+b)%q;
rts=lift(polrootsmod('x^3+a*'x+b,q));
tau(e,x)=(e+(3*e^2+a)*lift(1/Mod(x-e,q)))%q;
setrand(Wc+7);
{tot=0;totd=0;
for(tt=1,NT,
  while(1,P1=random(Eq);Q1=random(Eq);R=elladd(Eq,ellmul(Eq,P1,1+random(cardE)),ellmul(Eq,Q1,1+random(cardE)));if(R==[0],next);if(ellmul(Eq,R,2)==[0],next);break);
  xR=lift(R[1]);
  S=Set();
  \\ degenerate {x,x,xR}, x in W
  for(x=0,q-1,if(kronecker(f(x),q)==-1,S=setunion(S,Set([vecsort([x,x,xR])]))));
  for(r=1,#rts,e=rts[r];x3=tau(e,xR);
    for(x=0,q-1,if(kronecker(f(x),q)==-1,x2=tau(e,x);S=setunion(S,Set([vecsort([x,x2,x3])]))));
  );
  nd=0;dg=0;
  for(z=1,#S,tr=S[z];if(tr[1]==tr[2]||tr[2]==tr[3]||tr[1]==xR||tr[2]==xR||tr[3]==xR,dg++,nd++));
  tot+=nd;totd+=dg);}
print("PRED_ND ",tot," PRED_DEG ",totd);
