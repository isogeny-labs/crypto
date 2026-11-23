cells = {}
File.foreach("CONTROL.txt") do |line|
  next unless line.start_with?("CELL ", "WALL ")
  a = line.scan(/(\w+)=(\w+)/).to_h
  key = a.values_at("g","p","case").map(&:to_i)
  arm = a.fetch("arm").to_i
  cell = ((cells[key] ||= {})[arm] ||= {times: []})
  if line.start_with?("CELL ")
    raise unless a.fetch("solved") == "true"
    cell[:red] = a.fetch("base").to_i + a.fetch("subgroup").to_i
  else
    cell[:times] << a.fetch("ns").to_i
  end
end
raise unless cells.size == 44
rows = cells.map do |key, arms|
  med = (0..3).map do |arm|
    t = arms.fetch(arm)[:times].sort
    raise unless t.size == 12
    (t[5]+t[6])/2.0
  end
  key + med.first(3).map{|m| m/med.last} + (0..2).map{|i| arms[i][:red].fdiv(arms[3][:red])}
end
File.open("COMPARISON.tsv","w") do |f|
  f.puts %w[g p case original_wall deferred_wall generic_wall original_redc deferred_redc generic_redc].join("\t")
  rows.each{|r| f.puts r.join("\t")}
end
rows.group_by(&:first).each do |g,rs|
  p [g,(3..8).map{|i|rs.map{|r|r[i]}.minmax.map{|x|x.round(3)}},(3..8).map{|i|rs.count{|r|r[i]>1}}]
end
