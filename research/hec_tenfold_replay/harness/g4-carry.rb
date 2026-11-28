require 'open3'
def source(*args)
  s,status=Open3.capture2('ruby',*args)
  raise args.inspect unless status.success?
  s
end
def change(s,a,b)
  raise a unless s.scan(a).size==1
  s.sub(a){b}
end
s=source('harness/g4-hermite-quotient.rb')
packed=source('harness/g4-packed-four.rb','library','results/g4-packed-four-curves.txt').split('namespace packed_four {').last.split('static int coefficient,mode;').first
# Retain the packed four-family collector and its descending-pivot elimination.
a=packed.index('std::pair<int,int> run(')
b=packed.index('return {pos,rank};',packed.index('if(audit)assert(accepted'))+'return {pos,rank};'.size
collector=packed[a...b]
replay=ARGV.include?('replay')
if replay
  collector=change(collector,'if(audit){auto &ref=', 'if(record)history.push_back(row);if(audit){auto &ref=')
else
  collector=change(collector,'if(audit)assert(accepted', 'history.assign(piv.begin(),piv.begin()+n);if(audit)assert(accepted')
end
a=s.index('std::pair<int,int> run(');b=s.index('return {pos,rank};',s.index('if(audit)assert(accepted'))+'return {pos,rank};'.size
s[a...b]=collector
if replay
  puts s
  exit
end
s=change(s,'for(int j=0;j<n;j++)if(int v=mod(row[j],L))','for(int j=n-1;j>=0;j--)if(int v=mod(row[j],L))')
s=s.gsub('for(int k=j;k<n;k++)row[k]','for(int k=0;k<=j;k++)row[k]')
s=change(s,'for(auto &r:history)insert(std::vector<int>(r.begin(),r.begin()+n));',<<'CPP')
 assert(int(history.size())==n);
 for(int j=0;j<n;j++)if(history[j][j]){
   assert(history[j][j]==1);
   basis[j].assign(history[j].begin(),history[j].begin()+n);rank++;
 }
CPP
s=change(s,'std::vector<int> v(n);int free=-1;
   for(int j=n-1;j>=0;j--){','std::vector<int> v(n);int free=-1;
   for(int j=0;j<n;j++){')
s=change(s,'for(int k=j+1;k<n;k++)v[j]', 'for(int k=0;k<j;k++)v[j]')
puts s
