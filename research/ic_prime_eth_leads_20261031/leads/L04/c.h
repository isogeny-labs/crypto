struct V{ u32 v[128]; int n; bool det; };
// returns 0 undecided, 1 conflict, 2 solved. mask = fixed bit positions (i*s+j), asg = their values
static int decide(const Inst&I,int cap,bool filt,u32 mask,u32 asg){
  int m=I.m,s=I.s; V vs[6]; u32 W=(1u<<s)-1;
  for(int i=0;i<m;i++){
    u32 fm=(mask>>(i*s))&W, fv=(asg>>(i*s))&fm; int n=0;
    for(u32 v=0;v<=W;v++) if((v&fm)==fv && (!filt||I.valid[v])) vs[i].v[n++]=v;
    vs[i].n=n; vs[i].det=n<=cap; if(n==0) return 1;
  }
  for(int i=m;i<2*m-2;i++){ vs[i].n=0; vs[i].det=false; }
  int nc=m-1; int con[4][3];
  con[0][0]=0;con[0][1]=1;con[0][2]=m;
  for(int k=1;k<=m-3;k++){con[k][0]=m+k-1;con[k][1]=k+1;con[k][2]=m+k;}
  con[m-2][0]=2*m-3; con[m-2][1]=m-1; con[m-2][2]=-1;
  bool ch=true; u32 cst=(u32)I.xR;
  while(ch){ ch=false;
    for(int c=0;c<nc;c++){
      int* sl=con[c]; int und[3],nu=0;
      for(int j=0;j<3;j++) if(sl[j]>=0 && !vs[sl[j]].det) und[nu++]=j;
      if(nu>=2) continue;
      if(nu==0){
        const u32* d[3]; int dn[3];
        for(int j=0;j<3;j++){ if(sl[j]<0){d[j]=&cst;dn[j]=1;} else {d[j]=vs[sl[j]].v;dn[j]=vs[sl[j]].n;} }
        char keep[3][16]; memset(keep,0,sizeof keep);
        for(int a=0;a<dn[0];a++)for(int b=0;b<dn[1];b++)for(int e=0;e<dn[2];e++)
          if(s3zero(I,d[0][a],d[1][b],d[2][e])){keep[0][a]=keep[1][b]=keep[2][e]=1;}
        for(int j=0;j<3;j++){ if(sl[j]<0) { if(!keep[j][0]) return 1; continue; }
          V&w=vs[sl[j]]; int nn=0; for(int a=0;a<w.n;a++) if(keep[j][a]) w.v[nn++]=w.v[a];
          if(nn==0) return 1; if(nn<w.n){w.n=nn;ch=true;} }
      } else {
        int u=und[0],o1=(u+1)%3,o2=(u+2)%3; const u32* d[3]; int dn[3];
        for(int j=0;j<3;j++){ if(sl[j]<0){d[j]=&cst;dn[j]=1;} else if(j!=u){d[j]=vs[sl[j]].v;dn[j]=vs[sl[j]].n;} else {d[j]=0;dn[j]=0;} }
        u32 U[300]; int nU=0; bool any=false; u64 out[2];
        for(int a=0;a<dn[o1]&&!any;a++)for(int b=0;b<dn[o2];b++){
          int r=roots(I,d[o1][a],d[o2][b],out); if(r<0){any=true;break;}
          for(int t=0;t<r;t++){ bool f=false; for(int q=0;q<nU;q++) if(U[q]==(u32)out[t]){f=true;break;} if(!f) U[nU++]=(u32)out[t]; }
        }
        if(any) continue;
        V&w=vs[sl[u]];
        if(sl[u]<m){
          char inU[128]; memset(inU,0,sizeof inU); for(int q=0;q<nU;q++) if(U[q]<=W) inU[U[q]]=1;
          int nn=0; for(int a=0;a<w.n;a++) if(inU[w.v[a]]) w.v[nn++]=w.v[a];
          if(nn==0) return 1; if(nn<w.n){w.n=nn;ch=true;} w.det=w.n<=cap;
        } else {
          if(nU==0) return 1;
          if(nU<=cap){ for(int q=0;q<nU;q++) w.v[q]=U[q]; w.n=nU; w.det=true; ch=true; }
        }
      }
    }
  }
  for(int i=0;i<2*m-2;i++) if(!vs[i].det) return 0;
  return 2;
}
