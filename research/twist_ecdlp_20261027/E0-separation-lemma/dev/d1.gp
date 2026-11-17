default(parisizemax,2000000000);
{foreach([3,5,7],p, setfield(p,1); cl=classes_odd(p); print("p=",p," classes=",#cl);
  for(i=1,#cl, r=t1class(cl[i][1],cl[i][2],cl[i][3]); emit(r)));}
