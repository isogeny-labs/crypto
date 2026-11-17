default(parisizemax,2000000000);
print("gp version: ", version());
setrand(12345); a1=random(10^9); setrand(12345); a2=random(10^9); print("setrand reproducible: ", a1==a2);
q=1009; a=Mod(3,q); b=Mod(7,q);
f(x)=x^3+a*x+b;
chk()={my(ok=1,cnt=0,x1,x2,A,B,C);
 for(i=0,60, for(j=0,60, x1=Mod(i,q);x2=Mod(j,q); if(x1==x2,next);
  A=(x1-x2)^2; B=-2*((x1+x2)*(x1*x2+a)+2*b); C=(x1*x2-a)^2-4*b*(x1+x2);
  if(B^2-4*A*C != 16*f(x1)*f(x2), ok=0); cnt++));
 print("disc identity B^2-4AC == 16 f(x1) f(x2) on ",cnt," pairs: ",ok);}
chk();
E=ellinit([3,7],q); print("ellcard q=1009: ", ellcard(E));
