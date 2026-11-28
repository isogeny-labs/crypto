prior = {}
ref = {}
anchors = {}
File.foreach("../hec_singleton_recipes_20261118/CONTROL.txt") do |line|
  a = line.scan(/(\w+)=(\w+)/).to_h
  key = a.values_at("g","p","case")
  if line.start_with?("COUNT") && a["arm"]=="3"
    prior[key+[a["modulus"]]]=line[/vector=.*/]
  elsif line.start_with?("CELL")
    ref[key]=a["base"].to_i+a["subgroup"].to_i if a["arm"]=="0"
    anchors[key]=a.fetch("anchors") if a["arm"]=="3"
  end
end
cells = {}
key = nil
File.foreach("CONTROL.txt") do |line|
  a = line.scan(/(\w+)=(\w+)/).to_h
  case line
  when /^BEGIN/
    key=a.values_at("g","p","case")
    cells[key]=Hash.new(0)
  when /^PHASE/
    cells[key][a.fetch("name")]+=line[/\[([^\]]+)\]/,1].split(",").fetch(8).to_i
  when /^TOTAL/
    raise unless prior.delete(key+[a.fetch("modulus")])==line[/vector=.*/]
  when /^END/
    raise unless a.fetch("solved")=="true" && a.fetch("anchors")==anchors.fetch(key)
  end
end
raise unless cells.size==48 && prior.empty?
names=cells.values.flat_map(&:keys).uniq.sort
rows=cells.map do |k,h|
  total=h.values.sum
  remaining=total
  removed=0
  order=h.sort_by{|name,cost|[-cost,name]}
  while remaining*10>ref.fetch(k)
    remaining-=order.fetch(removed)[1]
    removed+=1
  end
  k+[ref[k],total,order.first[0],removed]+names.map{|n|h[n]}
end
File.open("BUDGET.tsv","w"){|f|f.puts (%w[g p case original_redc current_redc largest phases_needed]+names).join("\t");rows.each{|r|f.puts r.join("\t")}}
rows.group_by(&:first).each do |g,rs|
  t=rs.sum{|r|r[4]}
  p [g,rs.group_by{|r|r[5]}.transform_values(&:size),rs.group_by{|r|r[6]}.transform_values(&:size),names.map.with_index{|n,i|[n,(100.0*rs.sum{|r|r[7+i]}/t).round(1)]}.sort_by{|_,s|-s}]
end
