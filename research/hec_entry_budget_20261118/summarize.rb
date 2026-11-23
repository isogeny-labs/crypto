cells = {}
File.foreach("CONTROL.txt") do |line|
  next unless line.start_with?("WALL ", "CELL ")
  a = line.scan(/(\w+)=(\w+)/).to_h
  key = a.values_at("g","p","case").map(&:to_i)
  arm = a.fetch("arm").to_i
  c = ((cells[key] ||= {})[arm] ||= {wall: []})
  if line.start_with?("WALL ")
    c[:wall] << a.fetch("ns").to_i
  else
    raise unless (a.fetch("solved") == "true") == (arm < 2)
    c[:red] = a.fetch("base").to_i + a.fetch("subgroup").to_i
  end
end
raise unless cells.size == 44
rows = cells.map do |key, arms|
  med = (0..4).map do |i|
    t = arms.fetch(i)[:wall].sort
    raise unless t.size == 15
    t[7]
  end
  red = (0..4).map{|i| arms.fetch(i).fetch(:red)}
  key + med + red
end
File.open("BUDGET.tsv","w") do |f|
  f.puts %w[g p case original_ns fused_ns eligible_ns prepared_ns verified_ns original_red fused_red eligible_red prepared_red verified_red].join("\t")
  rows.each{|r| f.puts r.join("\t")}
end
rows.group_by(&:first).each do |g, rs|
  p [g,(5..7).map{|i|rs.count{|r|10*r[i]>=r[3]}},(10..12).map{|i|rs.count{|r|10*r[i]>=r[8]}},rs.map{|r|100.0*r[7]/r[4]}.minmax,rs.map{|r|r[3].fdiv(r[4])}.minmax]
end
