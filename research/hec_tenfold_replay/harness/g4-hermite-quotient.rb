require 'open3'
s,status=Open3.capture2('ruby','harness/g4-hermite.rb')
raise unless status.success?
start=s.index(' int pairs=0,tested=0,split=0;')
s.insert(start,<<'CPP')
 std::array<int,P> sqrt_table{},inv_table{},by_x{},cols{};
 sqrt_table.fill(-1);by_x.fill(-1);
 for(int i=0;i<=P/2;i++)sqrt_table[i*i%P]=i;
 for(int i=1;i<P;i++)inv_table[i]=inverse(i,P);
 int next=0;for(size_t j=0;j<xs.size();j++){by_x[xs[j]]=j;cols[j]=next;next+=ys[j]!=0;}
CPP
cursor=start
[2,3].each do |contact|
  at=s.index('   int inv2y=',cursor)
  s.insert(at,<<'CPP')
   std::vector<std::array<int,4>> coordinates;coordinates.reserve(xs.size());
   for(int x:xs){
     int z=mod(x-h,P),z2=z*z%P;
     coordinates.push_back({z,z2,z*z2%P,z2*z2%P});
   }
CPP
  a=s.index('       auto residual=f;',at)
  key='if(!degree){split++;insert(row);}'
  b=s.index(key,a)+key.size
  audit=ARGV.include?('audit') ? <<'CPP' : ''
       int fixed[7]{};fixed[3]=1;
       for(int j=0;j<mB;j++)for(int i=4+j;i>=3;i--)fixed[i]=mod(fixed[i-1]-t*fixed[i],P);
       for(int k=0;k<=10;k++){
         int val=0;
         for(int i=0;i<=4;i++)if(k-i>=0 && k-i<=6)val+=q[i]*fixed[k-i];
         for(int i=0;i<=wdeg;i++)if(k-i>=0 && k-i<=wdeg)val+=w[i]*w[k-i];
         assert(mod(val,P)==(k<=9?f[k]:0));
       }
CPP
  s[a...b]=File.read('harness/g4-hermite-quotient.inc').sub('CONTACT',contact.to_s).sub('QUOTIENT_AUDIT'){audit}
  cursor=a+1
end
if ARGV.include?('rare')
  pos=s.index(' int pairs=0,tested=0,split=0;')
  s.insert(pos,<<'CPP')
 std::vector<int> frequency(n),permutation(n);
 for(const auto &row:history)for(int j=0;j<n;j++)frequency[j]+=(row[j]!=0);
 for(int j=0;j<n;j++)permutation[j]=j;
 std::stable_sort(permutation.begin(),permutation.end(),[&](int a,int b){return frequency[a]<frequency[b];});
CPP
  s=s.gsub('for(int aj=0;aj<n && rank<n-1;aj++){','for(int ai=0;ai<n && rank<n-1;ai++){int aj=permutation[ai];')
  s=s.gsub('for(int bj=0;bj<n && rank<n-1;bj++)if(aj!=bj){','for(int bi=0;bi<n && rank<n-1;bi++)if(ai!=bi){int bj=permutation[bi];')
  s=s.gsub('for(int bj=0;bj<n && rank<n-1;bj++)if(aj<bj){','for(int bi=0;bi<n && rank<n-1;bi++)if(ai<bi){int bj=permutation[bi];')
end
if ARGV.include?('reference')
  s,status=Open3.capture2('ruby','harness/g4-hermite.rb')
  raise unless status.success?
end
if ARGV.include?('rows')
  s=s.gsub('split++;insert(row);','split++;printf("ROW %d %d %d %d %d",P,C,aj,bj,tested);for(int v:row)printf(" %d",v);puts("");insert(row);')
end
puts s
