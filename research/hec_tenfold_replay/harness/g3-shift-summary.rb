# The audit logs record the two input coordinates for each of three catalogue arms.
directory=ARGV.fetch(0)
audit=File.readlines(File.join(directory,'panel-forward.audit')).map(&:split)
metrics=File.readlines(File.join(directory,'metrics')).drop(1).map(&:split)
def median(values)
  sorted=values.sort
  (sorted[(sorted.size-1)/2]+sorted[sorted.size/2])/2
end
puts 'p c retries recovered median_forward median_reverse tenfold_before tenfold_after affected_forward affected_reverse'
metrics.group_by{|row|row.take(2)}.each do |curve,records|
  misses=0
  recovered=0
  affected=[]
  8.times do |index|
    3.times do |arm|
      2.times do
        p,ok=audit.shift
        raise unless p==curve.first && %w[0 1].include?(ok)
        if arm==2 && ok=='0'
          tag,p,hit=audit.shift
          raise unless tag=='RETRY' && p==curve.first && %w[0 1].include?(hit)
          affected << index
          misses+=1
          recovered+=hit.to_i
        end
      end
    end
  end
  ratios=%w[forward reverse].map do |order|
    samples=records.select{|r|r[3]==order}.map{|r|Float(r[7])}
    raise unless samples.size==8
    '%.3f'%median(samples)
  end
  wins=[4,5].map do |column|
    records.group_by{|r|r[2]}.count{|_,pair|pair.all?{|r|Float(r[column])>=10}}
  end
  subgroup=%w[forward reverse].map do |order|
    samples=records.select{|r|r[3]==order && affected.include?(r[2].to_i)}.map{|r|Float(r[7])}
    samples.empty? ? '-' : '%.3f'%median(samples)
  end
  puts [*curve,misses,recovered,*ratios,*wins,*subgroup].join(' ')
end
raise unless audit.empty?
