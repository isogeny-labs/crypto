# Run from this directory to redraw the SVG from the archived paired medians.
width, height = 760, 450
data = (1..2).map do |run|
  File.readlines("../../results/g2-holdout-#{run}.txt").map do |line|
    p, c, _, _, _, _, ratio = line.split
    [Integer(p), Integer(c), Float(ratio)]
  end
end
svg = [%(<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 #{width} #{height}" font-family="Arial,sans-serif" fill="#173342">)]
text = ->(x,y,value,size,anchor="start") { svg << %(<text x="#{x}" y="#{y}" font-size="#{size}" text-anchor="#{anchor}">#{value}</text>) }
line = ->(x,y,x2,y2,color) { svg << %(<path d="M#{x} #{y} L#{x2} #{y2}" fill="none" stroke="#{color}"/>) }
[127,257,1021,4093].each_with_index do |p,index|
  left,top = 46+380*(index%2),22+212*(index/2)
  x = ->(c) { left+10*c }
  y = ->(r) { top+157-151*Math.log2(r/6.0)/Math.log2(256.0/6.0) }
  text.call(left,top-1,"p = #{p}",16)
  [8,10,32,128,256].each do |v|
    line.call(left,y.call(v),x.call(31),y.call(v),v==10 ? "#a84715" : "#dbe4e6")
    text.call(left-6,y.call(v)+3,"#{v}×",10,"end")
  end
  [0,8,16,24,31].each { |c| text.call(x.call(c),top+173,c,10,"middle") }
  [0,1].each do |r|
    data[r].select { |field,_,_| field==p }.each do |_,c,ratio|
      svg << %(<circle cx="#{x.call(c)+(r==0 ? -1.5 : 1.5)}" cy="#{y.call(ratio)}" r="2.7" fill="#{r==0 ? '#126771' : '#805397'}"/>)
    end
  end
end
text.call(380,437,"Case within field",14,"middle")
File.write("holdout.svg",(svg+["</svg>"]).join("\n"))
