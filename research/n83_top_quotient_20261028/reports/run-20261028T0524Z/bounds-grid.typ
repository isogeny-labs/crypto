#set page(width:600pt,height:285pt,margin:12pt)
#set text(size:10pt)
#let rows=json("../../evidence/summary.json").rows
#let cell(m,f)={
 let rs=("planted","natural").map(t=>rows.find(r=>r.m==m and r.family==f and r.target_kind==t))
 let positive=rs.at(0).status=="positive_dimensional_top_ideal"
 block(width:100%,height:48pt,inset:4pt,fill:if positive{rgb("f7e6cb")}else{rgb("dbeaf5")})[
 #for (i,r) in rs.enumerate(){
  [#if i==0{[P:]}else{[N:]} #if r.status=="finite_interval"{str(r.dreg_lower_bound)+"–"+str(r.dreg_upper_bound)}else{[dim ≥ 1]} #linebreak()]
 }
 ]
}
*Certified top-form regularity intervals / positive dimension*
#v(9pt)
#grid(columns:(65pt,..range(8).map(_=>1fr)),gutter:3pt,
 [family],..range(2,10).map(m=>[*m=#m*]),
 ..("koblitz","binary","prime").map(f=>([#f],..range(2,10).map(m=>cell(m,f)))).flatten())
#v(9pt)
Blue: closed degree interval, not an exact regularity measurement.
#linebreak()
Amber: top quotient has positive Krull dimension; no finite graded vanishing degree.
#linebreak()
P/N: planted/natural. 48 exact saved input systems; no statistical interval.
#linebreak()
Run n83-top-quotient-20261028-r1; values from native quotient certificates.
