\\ Bounded structural diagnostic for idea 24; no timing or relation-collection claim.
\\ PARI/GP 2.17.3, p=1009. The S3 formula is the short-Weierstrass one.
p=1009;
x='x; y='y; w='w; t='t;
s3(u,v,z,a,b)=(u-v)^2*z^2-2*((u+v)*(u*v+a)+2*b)*z+(u*v-a)^2-4*b*(u+v);
support(a0,b0,z0)={
  my(a=Mod(a0,p),b=Mod(b0,p),z=Mod(z0,p),S,n=0,c=0,v);
  S=polresultant(s3(x,y,t,a,b),s3(t,w,z,a,b),t);
  for(i=0,4,for(j=0,4,for(k=0,4,
    v=polcoef(polcoef(polcoef(S,i,x),j,y),k,w);
    if(v!=0,n++);
    if((i==0||i==4)&&(j==0||j==4)&&(k==0||k==4)&&v!=0,c++);
  )));
  print("support a=",a0," b=",b0," z=",z0," degrees=",poldegree(S,x),",",poldegree(S,y),",",poldegree(S,w)," nonzero=",n,"/125 corners=",c,"/8");
};
for(q=1,3,support(2,3,if(q==1,7,if(q==2,17,29))));
support(2,3,0);
support(0,3,0);
support(2,0,0);
support(1,14,0);
support(1,14,7);
\\ Independent check of the seeded native example's three returned z values.
support(1,14,750);
support(1,14,620);
support(1,14,320);
pointinfo(a0,b0)={
  my(E,P);
  E=ellinit([Mod(a0,p),Mod(b0,p)]);
  P=[Mod(0,p),sqrt(Mod(b0,p))];
  print("x0point a=",a0," b=",b0," card=",ellcard(E)," card_prime=",isprime(ellcard(E))," order=",ellorder(E,P));
};
pointinfo(2,3);
pointinfo(0,3);
pointinfo(2,0);
pointinfo(1,14);
