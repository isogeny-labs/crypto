// CT38: independent simple generalized-Weierstrass group arithmetic.
package main
import ("encoding/json";"fmt")
type Pt struct {x,y int; inf bool}
var O=Pt{inf:true}
func mod(a,p int)int{a%=p;if a<0{a+=p};return a}
func valid(P Pt,p int)bool{if P.inf{return true};x,y:=P.x,P.y;return mod(y*y-y-x*x*x+x*x,p)==0}
func inverse(a,p int)int{for x:=1;x<p;x++{if mod(a*x,p)==1{return x}};panic("nonunit")}
func add(A,B Pt,p int)Pt{
 if A.inf{return B};if B.inf{return A}
 if !valid(A,p)||!valid(B,p){panic("off curve")}
 x,y,u,v:=A.x,A.y,B.x,B.y
 if x==u&&mod(y+v-1,p)==0{return O}
 var s,t int
 if A==B{d:=inverse(2*y-1,p);s=mod((3*x*x-2*x)*d,p);t=mod((-x*x*x+y)*d,p)
 }else{d:=inverse(u-x,p);s=mod((v-y)*d,p);t=mod((y*u-v*x)*d,p)}
 X:=mod(s*s+1-x-u,p);Y:=mod(1-s*X-t,p);R:=Pt{x:X,y:Y}
 if !valid(R,p){panic("bad sum")};return R
}
func mult(k int,A Pt,p int)Pt{R:=O;for k>0{if k&1>0{R=add(R,A,p)};A=add(A,A,p);k>>=1};return R}
func main(){
 primes:=[]int{7,13,17,101,103};counts:=map[int]int{}
 tor:=[]Pt{O,{x:0,y:0},{x:1,y:1},{x:1,y:0},{x:0,y:1}}
 for _,p:=range primes{
  cnt:=1;for x:=0;x<p;x++{for y:=0;y<p;y++{if valid(Pt{x:x,y:y},p){cnt++}}}
  counts[p]=cnt
  if cnt%5!=0{panic("cardinality")}
  for k:=0;k<5;k++{if mult(k,tor[1],p)!=tor[k]{panic("torsion")}}
  if mult(5,tor[1],p)!=O{panic("order")}
  for i:=0;i<5;i++{for j:=0;j<5;j++{if add(tor[i],tor[j],p)!=tor[(i+j)%5]{panic("law")}}}
 }
 if !valid(Pt{x:2,y:29},101)||valid(Pt{x:2,y:29},103){panic("negative")}
 out,_:=json.Marshal(map[string]interface{}{"point_counts":counts,"torsion_order":5,
  "negative_control":[]int{2,29},"standing":"[pcc:unverified]"})
 fmt.Println(string(out))
}
