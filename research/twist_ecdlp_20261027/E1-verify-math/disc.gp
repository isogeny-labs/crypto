S3(x1,x2,z)=(x1-x2)^2*z^2-2*((x1+x2)*(x1*x2+a)+2*b)*z+(x1*x2-a)^2-4*b*(x1+x2);
P=S3(x1,x2,z);
A=polcoef(P,2,z);B=polcoef(P,1,z);C=polcoef(P,0,z);
d=B^2-4*A*C - 16*(x1^3+a*x1+b)*(x2^3+a*x2+b);
print("disc identity residual: ",d);
print("symmetric x1<->z: ",S3(z,x2,x1)-S3(x1,x2,z));
print("symmetric x2<->z: ",S3(x1,z,x2)-S3(x1,x2,z));
qq=1009;aa=3;bb=7;
E=ellinit([aa,bb],qq);
bad=0;cnt=0;
S3n(x1,x2,x3)=((x1-x2)^2*x3^2-2*((x1+x2)*(x1*x2+aa)+2*bb)*x3+(x1*x2-aa)^2-4*bb*(x1+x2))%qq;
{for(x1=0,60,my(f1=(x1^3+aa*x1+bb)%qq);if(kronecker(f1,qq)!=1,next);
  for(x2=0,60,my(f2=(x2^3+aa*x2+bb)%qq);if(kronecker(f2,qq)!=1,next);
    my(P1=[x1,lift(sqrt(Mod(f1,qq)))],P2=[x2,lift(sqrt(Mod(f2,qq)))],S,D);
    S=elladd(E,P1,P2);D=elladd(E,P1,ellneg(E,P2));
    for(x3=0,qq-1,
      my(z=(S3n(x1,x2,x3)==0),g=0);
      if(#S>1&&lift(S[1])==x3,g=1); if(#D>1&&lift(D[1])==x3,g=1);
      \\ x3 any x in F_q (class E or W): E-points P3 only count when x3 is class E; for class W the zero has no E partner
      if(kronecker((x3^3+aa*x3+bb)%qq,qq)==1,cnt++; if(z!=g,bad++)))));}
print("S3 zero vs group-law (E-class x3): mismatches ",bad," of ",cnt);
