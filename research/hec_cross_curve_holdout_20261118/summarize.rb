cells = {}
File.foreach("CONTROL.txt") do |line|
  next unless line.start_with?("CELL ", "WALL ")
  a = line.scan(/(\w+)=(\w+)/).to_h
  key = a.values_at("g","p","case").map(&:to_i)
  c = ((cells[key] ||= {})[a.fetch("arm").to_i] ||= {times: []})
  if line.start_with?("CELL ")
    c[:ok] = a.fetch("solved")=="true"
    c[:red] = a.fetch("base").to_i+a.fetch("subgroup").to_i
  else
    c[:times] << a.fetch("ns").to_i
  end
end
rows = cells.map do |key,arms|
  c = (0..3).map{|i|arms.fetch(i)}
  t = c.map{|v| a=v[:times].sort; raise unless a.size==12; (a[5]+a[6])*0.5}
  ratios = (0..2).map{|i|t[i]/t[3] if c[i][:ok] && c[3][:ok]}
  key+c.map{|v|v[:ok]}+t+ratios
end
File.open("COMPARISON.tsv","w") do |f|
  f.puts %w[g p case original_ok lifted_ok bucket_ok combined_ok original_ns lifted_ns bucket_ns combined_ns original_over_combined lifted_over_combined bucket_over_combined].join("\t")
  rows.each{|r|f.puts r.join("\t")}
end
rows.group_by(&:first).each do |g,v|
  r=v.map{|a|a[11]}.compact
  p [g,v.size,(3..6).map{|i|v.count{|a|a[i]}},r.minmax,r.count{|x|x>=10},v.count{|a|a[12] && a[12]>1}]
end
