\\ verify S3 for y^2 = x^3 + a x^2 + b x + c :  S3(x1,x2,x3)=0 iff x3 = x(P1 +- P2)
S3f(u,v,w,a,b,c) = (u-v)^2*w^2 - 2*((u+v)*(u*v+b) + 2*a*u*v + 2*c)*w + (u*v-b)^2 - 4*c*(u+v+a);
{
  my(t=ffgen(11^4,'t), bad=0);
  for(rep=1,200, my(a=random(t),b=random(t),c=random(t), E=ellinit([0,a,0,b,c],t)); if(E==[], next);
    my(P=random(E), Q=random(E)); if(P==[0]||Q==[0]||P[1]==Q[1], next);
    my(R1=elladd(E,P,Q), R2=elladd(E,P,ellneg(E,Q)));
    if(R1!=[0] && S3f(P[1],Q[1],R1[1],a,b,c)!=0, bad++); if(R2!=[0] && S3f(P[1],Q[1],R2[1],a,b,c)!=0, bad++));
  print("S3 failures: ", bad);
}
