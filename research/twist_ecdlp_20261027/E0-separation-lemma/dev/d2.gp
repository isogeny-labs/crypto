{foreach([2,3],m, setfield(2,m); cl=classes_ord(); print("ord m=",m," classes=",#cl," gamma=",enc(D_)," s=",enc(DEL_));
  for(i=1,#cl, r=t1class(cl[i][1],cl[i][2],cl[i][3]); emit(r)));}
{foreach([2,3],m, setfield(2,m); cl=classes_ss(1); print("ss m=",m," curves=",#cl);
  for(i=1,min(#cl,6), r=t1class(cl[i][1],cl[i][2],cl[i][3]); emit(r)));}
