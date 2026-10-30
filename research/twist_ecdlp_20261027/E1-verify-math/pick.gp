q=103; setrand(777);
for(c=0,3, ok=0; while(!ok, a=random(q);b=random(q); if((4*a^3+27*b^2)%q==0,next); nt=#polrootsmod('x^3+a*'x+b,q); if((c==0&&nt==0)||(c==2&&nt==1)||(c==3&&nt==3),ok=1; print(c," ",a," ",b," nT=",nt)); if(c==1,ok=1)));
