default(parisizemax,1000000000);
read("gp/common.gp");
\\ Independent GP verification: labels via ellordinate, relations via elladd, PC-A classification.
cur=Map();
{foreach(readstr("results/curves.txt"),ln, my(f=strsplit(ln," ")); mapput(cur,[f[1],eval(f[2]),eval(f[3])],apply(eval,f[4..#f])));}
FLD=Map();
getfield(t,k,idx)={my(p=mapget(cur,[t,k,idx]));
  if(t=="B", my(key=[k,p[1]],g=ffgen(polfromint(p[1]),'t)); my(bb=sum(i=0,k-1,bittest(p[2],i)*g^i)); return([g,bb,k,p[1],p[2]]),
     return([p[1],p[2],p[3]]))};
int2ff(v,g,k)=sum(i=0,k-1,bittest(v,i)*g^i);
\\ build curve objects
mkE(t,k,idx,twist)={my(d=getfield(t,k,idx),g,bb,E);
  if(t=="B", g=d[1];bb=d[2]; E=ellinit([1,if(twist,0,1),0,0,bb]); return([E,g,k]),
     my(p=d[1],A=d[2],B=d[3]); if(twist, E=ellinit([Mod(A,p),Mod(-B,p)]), E=ellinit([Mod(A,p),Mod(B,p)])); return([E,p,k]))};
pointof(t,ob,x,twist)={my(E=ob[1],xx);
  if(t=="B", xx=int2ff(x,ob[2],ob[3]), xx=Mod(if(twist,-x,x),ob[2]));
  my(ys=ellordinate(E,xx)); if(#ys==0,return(0)); [xx,ys[1]]};
\\ sample files
verify_file(fn)={my(lines=readstr(fn),n=0,chk=0,lmis=0,rfail=0,nrel=0,ntw=0);
  foreach(lines,ln, if(#ln==0,next); my(f=strsplit(ln," "),t=f[1],k=eval(f[2]),idx=eval(f[3]),m=eval(f[4]),xr=eval(f[5]),
      cs=eval(Str("[",f[6],"]")),lb=eval(Str("[",f[7],"]")));
    my(istw=(#f==7)); \\ PC-B lines have no flagged column
    my(obE=mkE(t,k,idx,0),obT=mkE(t,k,idx,1)); n++;
    \\ label check against explicit lift on E
    for(i=1,m, my(pe=pointof(t,obE,cs[i],0)); if((pe!=0)!=(lb[i]==1), lmis++));
    if(!istw,
      my(rP=pointof(t,obE,xr,0)); if(rP==0, lmis++; next);
      if(vecmin(lb)==1,
        my(pts=vector(m,i,pointof(t,obE,cs[i],0))); my(ok=0); my(E=obE[1]);
        forvec(s=vector(m-1,i,[0,1]),
          my(S=pts[1]); for(i=2,m, my(Pi=if(s[i-1],ellneg(E,pts[i]),pts[i])); S=elladd(E,S,Pi));
          if(S==rP||S==ellneg(E,rP), ok=1));
        nrel++; if(!ok,rfail++), ntw++),
      \\ twist-target line: everything lifts on E'
      my(rP=pointof(t,obT,xr,1)); if(rP==0, lmis++; next);
      my(pts=vector(m,i,pointof(t,obT,cs[i],1))); my(allok=1); for(i=1,m,if(pts[i]==0,allok=0));
      if(!allok, rfail++; next);
      my(ok=0); my(E=obT[1]);
      forvec(s=vector(m-1,i,[0,1]),
        my(S=pts[1]); for(i=2,m, my(Pi=if(s[i-1],ellneg(E,pts[i]),pts[i])); S=elladd(E,S,Pi));
        if(S==rP||S==ellneg(E,rP), ok=1));
      nrel++; if(!ok,rfail++)));
  [n,nrel,ntw,lmis,rfail]};
fd=fileopen("results/gp_verify.txt","w");
{foreach(externstr("ls results/samples/*.txt"),fn,
  my(r=verify_file(fn)); filewrite(fd,Str("SAMPLE ",fn," n=",r[1]," relchecked=",r[2]," twistcoord_labelonly=",r[3]," label_mismatch=",r[4]," rel_fail=",r[5])));}
\\ PC-A
{foreach(externstr("ls results/pca/*.txt"),fn,
  my(nm=strsplit(fn,"/")); my(base=nm[#nm]); my(t=if(Vecsmall(base)[1]==66,"B","P"),k=eval(strsplit(strsplit(base,"_k")[2],".")[1]));
  my(ob=mkE(t,k,0,0),lines=readstr(fn),n=0,mis=0);
  foreach(lines,ln, if(#ln==0,next); my(f=strsplit(ln," "),x=eval(f[1]),lb=eval(f[2]));
    my(pe=pointof(t,ob,x,0)); n++; if((pe!=0)!=(lb==1),mis++));
  filewrite(fd,Str("PCA ",fn," n=",n," mismatch=",mis)));
fileclose(fd);
quit;
