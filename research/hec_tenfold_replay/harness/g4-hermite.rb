require 'open3'
def gen(*args)
  s,status=Open3.capture2('ruby',*args)
  raise args.inspect unless status.success?
  s
end
modes=[%w[tangent],%w[tangent osculate],%w[screen]]
sources=modes.map{|options|gen('harness/g4-rank-panel.rb','extend',*options)}
def loop_bounds(s)
  [s.index(' for(int aj=0;'),s.index(' printf("EXT ')]
end
s=sources.last.dup
start,finish=loop_bounds(s)
s[start...finish]=sources.map.with_index do |src,i|
  a,b=loop_bounds(src)
  src[a...b]+%Q{ if(report)printf("STAGE %d %d #{i} %d %d %d %d\\n",P,C,rank,pairs,tested,split);\n}
end.join
s=s.sub('template<int P> void extend','static bool report=true;template<int P> void extend')
puts s
