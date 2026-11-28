runs = (1..2).map do |run|
  rows = {}
  File.foreach("WALL-#{run}.txt").grep(/^WALL /).each do |line|
    g,p,target,rep,arm,ns = line.split.drop(1).map(&:to_i)
    key = [g,p,target,rep,arm]
    raise line if rows.key?(key) || ns <= 0
    rows[key] = ns
  end
  raise unless rows.size == 48*24*3
  rows
end
puts %w[genus p target prior1 original1 prior2 original2].join("\t")
keys = runs.first.keys.map { |key| key.take(3) }.uniq.sort
keys.each do |key|
  ratios = runs.flat_map do |rows|
    [5,0].map do |arm|
      values = (0...24).map { |rep| rows.fetch(key+[rep,arm]).fdiv(rows.fetch(key+[rep,6])) }.sort
      values[11,2].sum / 2
    end
  end
  puts (key+ratios).join("\t")
end
