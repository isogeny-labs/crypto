s=File.read('variants/g3/kernel.cpp')
def change(s,key,value)
  raise "source mismatch" unless s.scan(key).size==1
  s.sub(key){value}
end
puts '#include <unordered_map>'
s=s.gsub('std::array<Term,5>','std::array<Term,6>').gsub('assert(size<5)','assert(size<6)')
s=change(s,' Report rep{};',<<'CPP')
 struct Partial {int key,orient;CompactRow row;};
 std::unordered_map<int,Partial> table;
 Report rep{};
CPP
key='if(distinct<3)continue;'
s=change(s,key,<<'CPP')
 CompactRow joined{};
 if(distinct<3){
   int need=b?2:1;
   if(!bin.count || bin.count>need)continue;
   int mult=need==2 && bin.count==1 ? 2 : 1;
   if(mult==2){
     auto q=pts[bin.point[0]];int z=m(q.x-h);
     bool tangent=z?(q.sg && m(q.slope-s1-2*s2*z-3*s3*z2[z]-4*b*z3[z])==0):s5==0;
     if(!tangent)continue;
   }
   int al,be;
   if(b){
     int scale=m(-inv[b]*inv[b]);
     al=m(m(1-2*b*s3)*scale);be=m(m(7*h-s3*s3-2*b*s2)*scale);
   }else{al=m(7*h-s3*s3);be=m(21*h*h-2*s3*s2);}
   for(int i=0;i<bin.count;i++){
     int z=m(pts[bin.point[i]].x-h);
     for(int j=0;j<mult;j++){al=m(al+z);be=m(be+al*z);}
   }
   if(root[m(al*al-4*be)]>=0)continue;
   int A=m(al-2*h),B=m(be-al*h+h*h),u=b,w=0;
   for(int k=3;k>=0;k--){int next=m(u-al*w);u=m(s[k]-be*w);w=next;}
   int v=m(u-h*w);
   int orient=(w||v)?(w*P+v<m(-w)*P+m(-v)?1:-1):0;
   std::fill(work.begin(),work.end(),0);
   work[pole.col]=4;
   for(int i=0;i<bin.count;i++){auto q=pts[bin.point[i]];if(q.sg)work[q.col]+=mult*q.sg;}
   int key=A*P+B;
   if(orient){
     auto pos=table.find(key);
     if(pos==table.end()){
       CompactRow row;
       row.add(pole.col, work[pole.col]);
       for(int i=0;i<bin.count;i++){auto q=pts[bin.point[i]];if(q.sg)row.add(q.col,work[q.col]);}
       table.emplace(key,Partial{key,orient,row});
       continue;
     }
     auto &entry=pos->second;
     for(int i=0;i<entry.row.size;i++){auto t=entry.row.terms[i];work[t.col]-=orient*entry.orient*t.value;}
   }
   for(int j=0;j<n;j++)joined.add(j,work[j]);
   if(!joined.size)continue;
   ++rep.rows;
 }else{
CPP
s=change(s,' if constexpr(Batch){'+ "\n" +'   input.emplace_back();',' } if constexpr(Batch){'+ "\n" +'   input.emplace_back();')
s=change(s,'   row.add(pole.col,work[pole.col]);','   if(joined.size)row=joined;else{row.add(pole.col,work[pole.col]);')
s=change(s,'   if(rep.rows>=n-1)','   } if(rep.rows>=n-1)')
puts s.lines.select{|line|line.start_with?('#include')}.uniq
puts 'namespace lifted {'
puts s.split('extern "C" int quartic').first
puts 'int go(int p,int*y,int*v,unsigned long long*inv,int*root,unsigned long long*xs,int*len){int a=-1;switch(p){'
[[73,22,73277],[127,12,2214137],[257,15,5561089],[383,35,19226639]].each do |p,c,l|
  puts "case #{p}:run<#{p},#{c},#{l},true,true,true>(nullptr,y,v,&a,true,inv,root,xs,len);break;"
end
puts '}return a;}}'
puts 'extern "C" int original_quartic(int,int*,int*,int,unsigned long long*,int*,unsigned long long*,int*);'
puts 'static bool enabled; extern "C" void select_quadratics(int x){enabled=x;}'
puts 'extern "C" int quartic(int p,int*y,int*v,int cover,unsigned long long*i,int*r,unsigned long long*x,int*n){return enabled?lifted::go(p,y,v,i,r,x,n):original_quartic(p,y,v,cover,i,r,x,n);}'
