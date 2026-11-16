args=[[12,0,3,3,"random",0.65,100],[12,0,3,3,"interval",0.65,100],[12,1,2,3,"random",0.65,100]];
runall()={my(A,r);
for(i=1,#args,A=args[i];
 r=mycount(A[1],A[2],A[3],A[4],A[5],A[6],A[7]);
 for(t=1,#r,print("MY\t",A[1],"_",A[2],"_",A[3],"_m",A[4],"_",A[5],"_",A[6],"\t",r[t][1],"\t",r[t][2],"\t",r[t][3],"\t",r[t][4],"\t",r[t][5])))};
runall();
