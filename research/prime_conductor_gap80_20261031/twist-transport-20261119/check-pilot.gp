{
 iferr(
 my(rows=read("pilot-audit.gpvec"),count=0,E);
 if(#rows!=8,error("expected8 original attempts"));
 for(jj=1,#rows,my(v=rows[jj]);E=ellinit([0,0,0,v[2],v[3]],v[1]);
   if(ellorder(E,v[5])!=v[4]||ellmul(E,v[5],v[7])!=v[6],error("original subgroup/target law"));
   if(v[8]>=0,if(v[8]>=v[4]||ellmul(E,v[5],v[8])!=v[6],error("recovered scalar"));count++));
 if(count!=4,error("expected4 recovered logarithms"));
 print("PASS8 original target/subgroup checks and4 independent recovered logarithms;4 IC exhaustions are not solves");
 ,failure,print(failure);quit(2));
}
quit(0);
