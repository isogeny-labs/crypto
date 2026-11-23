require 'open3'
s,status=Open3.capture2('ruby','harness/g4-carry.rb');raise unless status.success?
marker='// Start with the complete four-family matrix'
s=s.sub(marker){File.read('harness/normalized-cubic.inc')+File.read('harness/g4-resolvent.inc')+marker}
s=s.sub(' int pairs=0,tested=0,split=0;', ' Quartics<P> quartics;int pairs=0,tested=0,split=0;')
cursor=0
2.times do
  a=s.index('       int roots_found[4]{},nr=0;',cursor)
  b=s.index('       std::vector<int> row(n);',a)
  s[a...b]="       int roots_found[4];int nr=quartics.split4(q,roots_found);if(!nr)continue;\n"
  cursor=a+1
end
# The resolvent path needs no table of powers at rational abscissae.
s=s.gsub(/   std::vector<std::array<int,4>> coordinates;.*?\n   \}\n/m,'')
s=s.sub(' std::array<int,P> sqrt_table{},inv_table{},by_x{},cols{};', ' std::array<int,P> by_x{},cols{};')
s=s.sub(/ sqrt_table.fill\(-1\);by_x.fill\(-1\);.*?\n int next=/m," by_x.fill(-1);\n int next=")
puts s
