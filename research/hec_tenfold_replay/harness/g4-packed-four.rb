require 'open3'
def generated(*args)
  text,status=Open3.capture2('ruby',*args)
  raise args.inspect unless status.success?
  text
end
def replace(source,from,to)
  raise "unexpected kernel: #{from}" unless source.scan(from).size==1
  source.sub(from){to}
end
fixtures=ARGV.fetch(1)
rows=File.readlines(fixtures).map{|s|s.split.map(&:to_i)}
source=generated('harness/g4-transfer.rb','library',fixtures)
four=source.split('namespace fourth_family {').last.split('static int coefficient,mode;').first
emission=four[/if constexpr\(Mask&8\)\{.*?\n\}\n(?=if constexpr\(Mask&1\))/m] or raise
emission=emission.sub('if constexpr(Mask&8)','').sub('bin.degree+=mult;', 'degree[3][b]+=mult;')
kernel=source.split('namespace screened {').last.split('namespace fourth_family {').first
kernel=replace(kernel,'add(0,a5);',emission+'add(0,a5);')
kernel=replace(kernel,'Bucket,P>,3>','Bucket,P>,4>')
kernel=replace(kernel,'char,padded>,3> degree{};', 'char,padded>,4> degree{};degree[3].fill(1);')
kernel=replace(kernel,'arm<3;arm++','arm<4;arm++')
kernel=replace(kernel,'3*step+arm','4*step+arm')
kernel=replace(kernel,'row[pole.col]=target+base;', 'row[pole.col]=arm==3?4:target+base;')
source=replace(source,'static int coefficient,mode;', "namespace packed_four {#{kernel}\nstatic int coefficient,mode;")
rows.each do |p,c,l,_|
  key="run<#{p},#{c},#{l},true,true>"
  source=replace(source,"if(mode==0)original::#{key}", "if(mode==0)fourth_family::run<#{p},#{c},#{l},true,true,15>")
  source=replace(source,"if(mode==1)screened::#{key}", "if(mode==1)packed_four::#{key}")
  source=replace(source,"if(mode==2)fourth_family::run<#{p},#{c},#{l},true,true,15>", "if(mode==2)screened::#{key}")
end
if ARGV.first=='audit'
  source=source.gsub('if(audit){auto &ref=reference[P].at(accepted);assert(ref.pos==pos && ref.row==row);}', 'if(audit)reference[P].push_back({pos,row});')
  puts source
  puts 'int main(){'
  rows.each do |p,c,l,_|
    puts "{int y[67]{},v[67]{},a=-1;"
    %w[fourth_family packed_four].each do |ns|
      params=[p,c,l,true,true];params<<15 if ns=='fourth_family'
      puts "#{ns}::reference[#{p}].clear();#{ns}::run<#{params.join(',')}>(true,y,v,a);"
    end
    puts "auto &r=fourth_family::reference[#{p}];auto &s=packed_four::reference[#{p}];assert(r.size()==s.size());for(unsigned j=0;j<r.size();j++)assert(r[j].pos==s[j].pos && r[j].row==s[j].row);printf(\"ROWS #{p} #{c} %zu\\n\",r.size());}"
  end
  puts '}'
else
  puts source
end
