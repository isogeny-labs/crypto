default(parisizemax,1000000000);
setrand(12345);
k=17; g=ffgen(2^k,'t); a=g^0;
tr0(x)=my(s=x,t=x);for(i=1,k-1,t=t^2;s+=t);s;
b=random(g); while(b==0,b=random(g));
E=ellinit([1,a,0,0,b]); N=ellcard(E); print("k=",k," #E=",N);
isE(x)=#ellordinate(E,x)>0;
{ok=1;for(i=1,3000, x=random(g); if(x==0,next); t0=tr0(x+a+b/x^2); if(isE(x)!=(t0==0), ok=0));}
print("lift criterion agrees: ",ok);
S3(x1,x2,x3)=(x1*x2+x1*x3+x2*x3)^2+x1*x2*x3+b;
pt(x)=[x,ellordinate(E,x)[1]];
{bad=0;n=0;for(i=1,2000, x1=random(g);x2=random(g); if(x1==0||x2==0||!isE(x1)||!isE(x2)||x1==x2,next);
  Q=elladd(E,pt(x1),pt(x2)); n++; if(S3(x1,x2,Q[1])!=0,bad++));}
print("S3 checks ",n," bad ",bad);
sb=b^(2^(k-1)); T0=[0,sb]; print("T0 on curve: ",ellisoncurve(E,T0));
{bad=0;for(i=1,500, x1=random(g); if(x1==0||!isE(x1),next); Q=elladd(E,pt(x1),T0); if(Q[1]!=sb/x1,bad++));}
print("x(P+T0) bad ",bad);
{xR=0;while(xR==0||!isE(xR),xR=random(g));}
{nE=0;nT=0;badE=0;badT=0;
for(i=1,2000, x1=random(g); if(x1==0||x1==xR,next); A=(x1+xR)^2;B=x1*xR;C=B^2+b; c=C*A/B^2; ps=(tr0(c)==0);
  if(isE(x1),nE++;if(!ps,badE++),nT++;if(ps,badT++)));}
print("E-type x1: ",nE," fail-trace(bad) ",badE,"; twist x1: ",nT," pass-trace(bad) ",badT);
\\ mixed pair (x1 E, x2 twist): stage-1 trace test
{nm=0;badm=0;ns=0;bads=0;
for(i=1,2000, x1=random(g);x2=random(g); if(x1==0||x2==0||x1==x2,next); A=(x1+x2)^2;B=x1*x2;C=B^2+b; c=C*A/B^2; ps=(tr0(c)==0);
  if(isE(x1)!=isE(x2), nm++; if(ps,badm++), ns++; if(!ps,bads++)));}
print("mixed pairs ",nm," pass(bad) ",badm,"; same-type pairs ",ns," fail(bad) ",bads);
