cells = {}
File.foreach("CONTROL.txt") do |line|
  next unless line.start_with?("CELL ", "WALL ")
  a = line.scan(/(\w+)=(\w+)/).to_h
  key = a.values_at("g","p","case").map(&:to_i)
  arm = a.fetch("arm").to_i
  c = ((cells[key] ||= {})[arm] ||= {times: []})
  if line.start_with?("CELL ")
    c[:solved] = a.fetch("solved") == "true"
    c[:width]=a.fetch('small').to_i
    c[:anchors],c[:unready] = a.values_at("anchors","unready").map(&:to_i)
    c[:red] = a.fetch("base").to_i + a.fetch("subgroup").to_i
  else
    c[:times] << a.fetch("ns").to_i
  end
end
raise unless cells.size == 44
rows = cells.flat_map do |key,arms|
  med = (0..4).map do |i|
    t=arms.fetch(i).fetch(:times).sort
    raise unless t.size == 10
    (t[4]+t[5])*0.5
  end
  raise unless (0..1).all?{|i|arms[i][:solved]}
  [2,3,4].map do |i|
    c=arms[i]
    ratios = c[:solved] ? [med[0]/med[i], med[1]/med[i], arms[1][:red].fdiv(c[:red])] : [nil]*3
    key+[i,c[:solved],c[:anchors],c[:unready],arms[1][:anchors]]+ratios+[arms[1][:width],c[:width]]
  end
end
File.open("COMPARISON.tsv","w") do |f|
  f.puts %w[g p case arm solved anchors unready prior_anchors original_wall prior_wall prior_redc prior_width width].join("\t")
  rows.each{|r|f.puts r.join("\t")}
end
rows.group_by{|r|r.values_at(0,3)}.each do |k,all|
  rs=all.select{|r|r[4]}
  distinct=rs.select{|r|r[11]!=r[12]}; p [distinct.size,distinct.count{|r|r[9]>1},rs.count{|r|r[8]>=10},k,rs.size,rs.count{|r|r[9]>1},rs.count{|r|r[5]<r[7]},(8..10).map{|i|rs.map{|r|r[i]}.minmax.compact.map{|n|n.round(3)}}]
end
