s=File.read('kernel.cpp')
def change(s,a,b)
  raise "source mismatch" unless s.scan(a).size==1
  s.sub(a){b}
end
s=change(s,'int fibmode=0;extern "C" void sequence(int on){fibmode=on;}',
  'int fibmode=0,completion=0;bool verify_rows=false;long long probes,slopes,rows_done,fallbacks;extern "C" void sequence(int on){completion=on;}')
s=change(s,'int C,int L,bool Small=false> int run','int C,int L,bool Small=false,int Mode=0> int run')
s=change(s,'int limit=Small?n:(Small?n:(n+2)/3);',<<'CPP')
int base_limit=Small?n:(n+2)/3;
int limit=Mode==1?(3*base_limit+3)/4:Mode==2?(base_limit+3)/4:base_limit;
if(verify_rows)probes+=limit;
CPP
s=change(s,'(pos==(Fixed?1:2))','(pos==(Mode==1?1:Mode==2?0:Fixed?1:2))')
s=change(s,'CompactRow row{};if constexpr(Fixed){if(count[t]==2){',<<'CPP')
CompactRow row{};
if constexpr(Mode!=0){
  if(verify_rows)++slopes;
  int ia=std::abs(ids[t][0])-1;
  int a=poles[ia].x, sum=m(t*t+q[1]), b,c;
  if constexpr(Mode==1){
    b=poles[std::abs(ids[t][1])-1].x;c=m(sum-a-b);
  }else{
    int pairs=m(3LL+q[1]*sum-2LL*t*(q[3]+t*q[1])-q[0]);
    int s=m(sum-a),prod=m(pairs-a*s),disc=m(s*s-4*prod);
    int r=root[disc];
    if(r<0)continue;
    b=m((s+r)*inv[2]);c=m(s-b);
  }
  int xs[]={a,b,c};bool good=true;
  for(int k=0;k<3;k++){
    int x=xs[k];if((k>=1 && x==a) || (k==2 && x==b))continue;
    int u=m(x*x+q[1]*x+q[0]);
    int y=m(q[2]+q[3]*x+1LL*t*u);
    if(!u){good=false;break;}
    if(!y){if(!Fixed){good=false;break;}continue;}
    int j=index[x];if(j<0 || j>=base_limit){good=false;break;}
    int mult=(a==x)+(b==x)+(c==x);
    row.add(j,(poles[j].y==y?-1:1)*mult);
  }
  if(!good)continue;
  if(verify_rows){
    // Verify u times the reconstructed cubic equals f minus (v+t*u)^2.
    int product[]={m(-1LL*a*b*c),m(a*b+a*c+b*c),m(-a-b-c),1};
    int line[]={m(q[2]+t*q[0]),m(q[3]+t*q[1]),t};
    int f[]={C,1,2,3,0,1},u[]={q[0],q[1],1};
    for(int i=0;i<3;i++){
      for(int j=0;j<3;j++)f[i+j]=m(f[i+j]-line[i]*line[j]);
      for(int j=0;j<4;j++)f[i+j]=m(f[i+j]-u[i]*product[j]);
    }
    for(int z:f)assert(!z);
    ++rows_done;
  }
}else if constexpr(Fixed){if(count[t]==2){
CPP
s=change(s,' triples=minusarm=reorder=freqmode=fast;',<<'CPP')
 fibmode=p==127?1:2;
 verify_rows=fast==2;
 probes=slopes=rows_done=fallbacks=0;
 triples=minusarm=reorder=freqmode=fast;
CPP
[[127,4,8161],[257,4,17783],[1021,2,1092043],[4093,1,4091693]].each do |p,c,l|
  old="run<#{p},#{c},#{l},true>"
  s=change(s,old,"(completion==1?run<#{p},#{c},#{l},true,1>:completion==2?run<#{p},#{c},#{l},true,2>:#{old})")
end
s=change(s,'if(k<0){++stop[0];','if(k<0){++fallbacks;++stop[0];')
s=change(s,'assert(check(k));if(fast==2)', 'if(verify_rows)fprintf(stderr,"SCAN %d %d %lld %lld %lld %lld\\n",p,completion,probes,slopes,rows_done,fallbacks);assert(check(k));if(fast==2)')
if ARGV.include?('reuse')
  s=change(s,'Mode==1?(3*base_limit+3)/4:Mode==2?(base_limit+3)/4:base_limit','Mode?(3*base_limit+3)/4:base_limit')
  s=change(s,'Mode==1?1:Mode==2?0:Fixed?1:2','Mode?1:Fixed?1:2')
  s=change(s,'if constexpr(Mode==1){','if constexpr(Mode!=0){')
  key='  int xs[]={a,b,c};bool good=true;'
  s=change(s,key,<<'CPP')
  if constexpr(Mode==2){
    int first=ids[t][0],second=ids[t][1],ib=std::abs(second)-1;
    if(count[t]==3){
      assert(c==poles[std::abs(ids[t][2])-1].x);
      for(int i=0;i<3;i++){int e=ids[t][i];row.add(std::abs(e)-1,e<0?-1:1);}
    }else{
      int third=index[c];
      // The scan excluded branch points. Their coefficient is zero.
      if(!Fixed && third<0)continue;
      if(third>=base_limit)continue;
      if(third<0 && root[m(1LL*m(c*c)*m(c*c)*c+3LL*m(c*c)*c+2*c*c+c+C)]!=0)continue;
      bool repeat_a=c==a,repeat_b=c==b;
      row.add(ia,(first<0?-1:1)*(1+repeat_a));
      row.add(ib,(second<0?-1:1)*(1+repeat_b));
      if(!repeat_a && !repeat_b){
        int u=m(c*c+q[1]*c+q[0]);if(!u)continue;
        if(third>=0){
          int y=m(q[2]+q[3]*c+1LL*t*u);
          row.add(third,poles[third].y==y?-1:1);
        }
      }
    }
  }else{
  int xs[]={a,b,c};bool good=true;
CPP
  s=change(s,'  if(!good)continue;','  if(!good)continue;}')
end
puts s
