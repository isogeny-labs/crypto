def ratios(path)
  runs = {}
  File.foreach(path).grep(/^WALL /).each do |line|
    g, p, target, rep, arm, ns = line.split.drop(1).map(&:to_i)
    times = (runs[[g,p,target]] ||= {})
    raise line if times.key?([rep,arm]) || ns <= 0
    times[[rep,arm]] = ns
  end
  raise path unless runs.size == 48
  runs.transform_values do |times|
    raise path unless times.size == 48
    ordered = (0...24).map { |rep| times.fetch([rep,0]).fdiv(times.fetch([rep,3])) }.sort
    ordered[11,2].sum / 2
  end
end

a, b = %w[WALL-1.txt WALL-2.txt].map { |path| ratios(path) }
raise unless a.keys.sort == b.keys.sort
puts %w[genus p target run1_ratio run2_ratio].join("\t")
a.each { |key,v| puts [*key,v,b.fetch(key)].join("\t") }
