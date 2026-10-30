setfield(13,1);
cl=classes_odd(13); print(#cl);
{for(i=1,6, c=cl[i][2]; r=lisos(c[2],c[3],13,2); r3=lisos(c[2],c[3],13,3); r5=lisos(c[2],c[3],13,5);r7=lisos(c[2],c[3],13,7);
  print(c," nE=",ellcard(ellinit([c[2],c[3]],13))," ",r," ",r3," ",#r5[1]," bad5=",r5[2]," ",#r7[1]," bad7=",r7[2]));}
b=bfs(cl[3][2][2],cl[3][2][3],13,[2,3],3); print(b);
