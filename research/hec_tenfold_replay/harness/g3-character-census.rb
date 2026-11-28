# Generate an exhaustive two-pencil census for the reducible transfer curves.
require 'open3'
source, status = Open3.capture2('ruby', 'harness/g3-cubic-pencil.rb')
raise unless status.success?
source = source.split('int go(int p,int*y').first
source.sub!('namespace fused {', 'namespace fused { static int mode, size, extra;')
start = source.index(' for(int x=0;x<P;x++){', source.index('std::vector<Point> pts,poles;'))
source.insert(start, <<'CPP')
 auto positive=[&](int x){
   int g;
   if constexpr(P==127)g=x+56;
   else if constexpr(P==257 && C==54)g=((x+207)*x+243)*x+18;
   else if constexpr(P==257)g=((x+80)*x+165)*x+200;
   else if constexpr(C==42)g=(x+378)*x+218;
   else g=x+113;
   return root[m(g)]>0;
 };
 int quota=0, negatives=0;
 for(int x=0;x<P;x++){
   int b=m(x*x*x);int y=root[m(m(b*b)*x+b+C*x+1)];
   quota+=y>0 && positive(x);
 }
CPP
source.sub!('if(y<0)continue;', 'if(y<0 || (y>0 && ((mode==1 && int(poles.size())>=quota) || (mode==2 && !positive(x) && negatives++>=extra))))continue;')
source.sub!(' int n=poles.size();',' int n=poles.size();size=n;std::vector<std::vector<int>> basis(n);')
a=source.index(' if constexpr(Adaptive){', source.index('if(validation){last_poles='))
b=source.index(" return rep;\n}",source.index('return rep;',a))
raise unless a && b
source[a...b]=<<'CPP'
   for(int j=0;j<n;j++){
     int v=(work[j]%L+L)%L;
     if(!v)continue;
     if(basis[j].empty()){
       int scale=inverse(v,L);basis[j].resize(n);
       for(int k=j;k<n;k++)basis[j][k]=(work[k]%L+L)%L*scale%L;
       ++rep.rank;break;
     }
     for(int k=j;k<n;k++)work[k]=(work[k]-1LL*v*basis[j][k])%L;
   }
 }}

 // Recover a nonzero null vector of the full catalogue, then check its
 // restriction against every pivot of each independently reduced submatrix.
 static int full_coordinate[P]{};
 std::vector<int> coordinate(n);
 if(mode==0){
   assert(rep.rank==n-1);
   for(int j=n-1;j>=0;j--){
     if(basis[j].empty())coordinate[j]=1;
     else{
       long long sum=0;
       for(int k=j+1;k<n;k++)sum=(sum+1LL*basis[j][k]*coordinate[k])%L;
       coordinate[j]=(L-sum)%L;
     }
     full_coordinate[poles[j].x]=coordinate[j];
   }
 }else{
   for(int j=0;j<n;j++)coordinate[j]=full_coordinate[poles[j].x];
   assert(std::any_of(coordinate.begin(),coordinate.end(),[](int x){return x!=0;}));
 }
 for(int j=0;j<n;j++)if(!basis[j].empty()){
   long long sum=0;
   for(int k=j;k<n;k++)sum=(sum+1LL*basis[j][k]*coordinate[k])%L;
   assert(sum==0);
 }
CPP
puts source
puts '}'
puts 'int main(){ int y[383]{},v[383]{},a=-1;'
File.foreach('results/g3-transfer-curves.txt') do |line|
  p,c,l=line.split.map(&:to_i)
  next unless [[127,15],[257,54],[257,78],[383,42],[383,80]].include?([p,c])
  puts "for(int variant=0;variant<=14;variant++){ int arm=std::min(variant,2);fused::extra=variant-arm; fused::mode=arm;fused::validation=true;fused::dual=true;auto r=fused::run<#{p},#{c},#{l},true,true,true>(nullptr,y,v,&a,true,nullptr,nullptr,nullptr,nullptr);printf(\"#{p} #{c} %d %d %d %d\\n\",variant,fused::size,r.rows,r.rank); }"
end
puts '}'
