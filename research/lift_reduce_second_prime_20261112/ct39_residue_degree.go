// CT39 independent Go point enumeration and Frobenius-matrix field-degree bounds.
package main
import("encoding/json";"fmt")
type Case struct{a,b,r,n101,n103,f int}
type Mat [2][2]int
func mod(x,p int)int{x%=p;if x<0{x+=p};return x}
func multiply(A,B Mat,p int) Mat{var C Mat;for i:=0;i<2;i++{for j:=0;j<2;j++{for k:=0;k<2;k++{C[i][j]=mod(C[i][j]+A[i][k]*B[k][j],p)}}};return C}
func count(a,b,p int)int{n:=1;for x:=0;x<p;x++{for y:=0;y<p;y++{if mod(y*y-x*x*x-a*x-b,p)==0{n++}}};return n}
func need(b bool,s string){if !b{panic(s)}}
func main(){
  cases:=[]Case{{1,46,109,109,112,27},{44,21,113,113,99,4256}}
  rows:=make([]map[string]int,0,2)
  for _,c:=range cases{
    d:=-16*(4*c.a*c.a*c.a+27*c.b*c.b)
    need(mod(d,101)!=0&&mod(d,103)!=0,"singular reduction")
    n101,n:=count(c.a,c.b,101),count(c.a,c.b,103)
    need(n101==c.n101&&n==c.n103,"unexpected group order")
    t:=104-n
    F:=Mat{{0,mod(-103,c.r)},{1,mod(t,c.r)}}
    M:=Mat{{1,0},{0,1}}
    hits:=[]int{}
    for f:=1;f<=2*c.f;f++{
      M=multiply(M,F,c.r)
      determinant:=mod((1-M[0][0])*(1-M[1][1])-M[0][1]*M[1][0],c.r)
      if determinant==0{hits=append(hits,f)}
    }
    need(len(hits)==2&&hits[0]==c.f&&hits[1]==2*c.f,"wrong field floor")
    need(n%c.r!=0,"target already has rational r torsion")
    rows=append(rows,map[string]int{"a":c.a,"b":c.b,"r":c.r,
     "n101":n101,"n103":n,"t103":t,"min_residue_degree":c.f,
     "number_field_degree_floor":c.f,"discriminant103":mod(d,103)})
  }
  output,e:=json.Marshal(map[string]interface{}{"cases":rows,"status":"[pcc:unverified]; fixed Q-models only"})
  if e!=nil{panic(e)};fmt.Println(string(output))
}
