\\ Exact field, 512 candidate limit, t=8.
{
p=0x48e1d43f293469e33194c43186b3abc0b;
found=0;
for(b=1,512,
  if((4+27*b*b)%p==0,print("SINGULAR ",b);next());
  E=ellinit([1,b],p);n=ellcard(E);
  print("ORDER ",b," ",n);
  if(n%256!=0,next());
  for(x=0,255,
    ys=ellordinate(E,x);
    if(#ys==0,next());
    P=[x,ys[1]];Q=ellmul(E,P,n/256);
    if(Q==[0]||ellmul(E,Q,128)==[0]||ellmul(E,Q,256)!=[0],next());
    for(rx=256,511,
      rys=ellordinate(E,rx);
      if(#rys==0,next());
      R=[rx,rys[1]];
      if(ellmul(E,R,256)==[0],next());
      print("CERT ",b," ",n," ",P," ",Q," ",R);
      T=R;
      for(k=0,255,print("X ",k," ",T[1]);T=elladd(E,T,Q));
      found=1;break()
    );
    if(found,break())
  );
  if(found,break())
);
print("FOUND ",found);
}
