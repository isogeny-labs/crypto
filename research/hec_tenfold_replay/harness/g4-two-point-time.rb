require 'open3'
inputs=File.readlines('results/g4-two-point-rank.txt').map{|line|line.split[1,3].join(' ')}.join("\n")+"\n"
reference=nil
8.times do |rep|
  order=rep.even? ? ARGV : ARGV.reverse
  order.each do |binary|
    start=Process.clock_gettime(Process::CLOCK_MONOTONIC,:nanosecond)
    out,status=Open3.capture2(binary,stdin_data:inputs)
    elapsed=Process.clock_gettime(Process::CLOCK_MONOTONIC,:nanosecond)-start
    raise unless status.success?
    vectors=out.lines.grep(/^VALUES /)
    reference||=vectors
    raise unless vectors.size==21 && vectors==reference
    puts [rep,File.basename(binary),elapsed].join(' ')
  end
end
