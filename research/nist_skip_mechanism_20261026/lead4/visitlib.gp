\\ Pure functions of the visitation models (PROTOCOL.md). No data access.
\\ el: sorted eligible indices; Ym: bitmask over positions of el (bit j-1 = el[j] published).

\\ Family T scan. perm=0 strict, perm=1 permissive. stop=1: stop after k accepts (strict).
\\ returns [mask of accepted, hidden rejections visited, published visited]
simT(el,Ym,r,phi,a,stop,perm)={
  my(x=1+phi,g=r+1,k=hammingweight(Ym),acc=0,mask=0,hid=0,pv=0,e,j);
  for(j=1,#el,
    if(!perm&&stop&&acc>=k,break);
    e=el[j];
    if(e<x||(e-x)%g,next);
    if(perm,
      if(bittest(Ym,j-1),mask+=2^(j-1);pv++;x=e+1+a,hid++;x=e+g),
      mask+=2^(j-1);acc++;x=e+1+a));
  [mask,hid,pv]};

\\ Family Q: full scan with at most q accepted per class; keyv[j] in 0..8.
simQ(el,keyv,q)={
  my(cnt=vector(9),mask=0);
  for(j=1,#el,my(c=keyv[j]+1);if(cnt[c]<q,cnt[c]++;mask+=2^(j-1)));
  mask};

\\ model grids
RS=[0,1,2,3,4,5,6,7];
AS=[0,1,2,3,4,5,6,8,10,15,20,30,40,50,100];
mkT()={my(M=List());for(r=0,7,for(phi=0,r,for(i=1,#AS,listput(M,[r,phi,AS[i]]))));Vec(M)};
TM=mkT();            \\ 540 (r,phi,a) triples
QM=vector(15,i,[(i-1)\5+1,(i-1)%5+1]);   \\ [kappa, q]

\\ random k-subset of 1..m as bitmask (partial Fisher-Yates)
rsub(m,k)={my(v=vector(m,i,i),mask=0,t,j);
  for(i=1,k,j=i+random(m-i+1);t=v[i];v[i]=v[j];v[j]=t;mask+=2^(v[i]-1));mask};
