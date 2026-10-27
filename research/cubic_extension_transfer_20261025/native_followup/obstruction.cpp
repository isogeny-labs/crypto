// Native port of the exact theta/chi18 evaluation. This is a finite evaluator,
// not a symbolic modular-function proof. Reference: Tian; Kirschmer et al.
#include "ff.hpp"
#include <chrono>
#include <set>
using Sections=std::array<V3,9>;using Actions=std::array<M3,9>;using Tensor=std::array<F,27>;
inline int code(int i,int j){return 3*i+j;}
inline int sumcode(int a,int b){return code((a/3+b/3)%3,(a%3+b%3)%3);}
struct Normal{std::array<Point,9> points;Sections sections;F pairing;};
Normal normal3(const EC&E){
 auto xs=E.three_x();require(xs.size()==4,"full geometric three torsion not found");
 Point B1(xs[0],(xs[0].pow(3)+E.a*xs[0]+E.b).sqrt());Point B2(xs[1],(xs[1].pow(3)+E.a*xs[1]+E.b).sqrt());
 require(E.mul(3,B1).infinity&&E.mul(3,B2).infinity,"torsion order");
 Normal n;Sections sec{};sec[0]={F(1),F(0),F(0)};
 for(int i=0;i<3;i++)for(int j=0;j<3;j++){
  int c=code(i,j);Point P=E.add(E.mul(i,B1),E.mul(j,B2));n.points[c]=P;if(c==0)continue;
  F slope=(F(3)*P.x*P.x+E.a)/(F(2)*P.y);sec[c]={P.y-slope*P.x,slope,F(1)};
 }
 auto ev=[&](int c,const Point&P){require(!P.infinity,"section evaluation at infinity");return dot(sec[c],V3{F(1),P.x,P.y});};
 Point R;
 for(int attempt=0;;attempt++){
  require(attempt<1000,"normal-three safe point budget");R=E.random();bool safe=true;
  for(auto&P:n.points){Point Q=E.add(R,P);if(Q.infinity){safe=false;break;}for(int c=0;c<9;c++)if(!ev(c,Q)){safe=false;break;}}
  if(safe)break;
 }
 auto coc=[&](int c,int d){return ev(c,R)*ev(d,E.add(R,n.points[c]))/ev(sumcode(c,d),R);};
 n.pairing=coc(3,1)/coc(1,3);require(n.pairing!=F(1)&&n.pairing.pow(3)==F(1),"Heisenberg pairing");
 auto d=[&](int c,int e){int exponent=2*((c/3)*(e%3)-(c%3)*(e/3));return n.pairing.pow((exponent%3+3)%3);};
 std::array<F,9> alpha{};alpha[0]=1;
 for(int c:{3,1}){int c2=sumcode(c,c);F u=F(1)/(coc(c,c)*coc(c,c2));F v=ev(c2,R)/(ev(c,R)*ev(c2,E.add(R,n.points[c]))*ev(c,E.neg(R)));alpha[c]=u/v;require(alpha[c].pow(3)==u&&alpha[c].pow(2)==v,"three-normalization powers");alpha[c2]=alpha[c]*alpha[c]*coc(c,c);}
 for(int i:{1,2})for(int j:{1,2}){int c=code(i,0),e=code(0,j);alpha[code(i,j)]=alpha[c]*alpha[e]*coc(e,c)/d(e,c);}
 for(int c=0;c<9;c++){for(int j=0;j<3;j++)n.sections[c][j]=alpha[c]*sec[c][j];for(int e=0;e<9;e++)require(alpha[c]*alpha[e]*coc(c,e)==d(c,e)*alpha[sumcode(c,e)],"cocycle identity");}
 return n;
}
Actions action_matrices(const EC&E,const Normal&n){
 auto safe=[&](){for(int i=0;i<1000;i++){Point R=E.random();bool good=true;for(auto&P:n.points)if(E.add(R,P).infinity){good=false;break;}if(good)return R;}throw std::runtime_error("action point budget");};
 M3 V;std::array<Point,3> samples;
 for(int attempt=0;;attempt++){require(attempt<1000,"action matrix sample budget");for(int i=0;i<3;i++){samples[i]=safe();V[i]={F(1),samples[i].x,samples[i].y};}if(det(V))break;}
 M3 iv=inverse(V);Actions out;
 for(int c=0;c<9;c++){M3 value;for(int i=0;i<3;i++){Point Q=E.add(samples[i],n.points[c]);F scale=dot(n.sections[c],V[i]);value[i]={scale,scale*Q.x,scale*Q.y};}out[c]=matmul(iv,value);}
 for(int k=0;k<2;k++){Point R=safe();V3 v{F(1),R.x,R.y};for(int c=0;c<9;c++){Point Q=E.add(R,n.points[c]);F scale=dot(n.sections[c],v);V3 expected{scale,scale*Q.x,scale*Q.y};for(int j=0;j<3;j++){F got;for(int i=0;i<3;i++)got+=v[i]*out[c][i][j];require(got==expected[j],"holdout Heisenberg action");}}}
 return out;
}
Tensor invariant(const std::array<Actions,3>&a,const json&rows){
 for(int h=0;h<27;h++){
  int hh[3]={h/9,(h/3)%3,h%3};Tensor T{};
  for(int i=0;i<3;i++)for(int j=0;j<3;j++)for(int k=0;k<3;k++){
   int cs[3]={i,j,k},v[6]={};for(int c=0;c<6;c++)for(int l=0;l<3;l++)v[c]=(v[c]+cs[l]*rows[l][c].get<int>())%3;
   for(int u=0;u<3;u++)for(int w=0;w<3;w++)for(int t=0;t<3;t++)T[9*u+3*w+t]+=a[0][code(v[0],v[1])][u][hh[0]]*a[1][code(v[2],v[3])][w][hh[1]]*a[2][code(v[4],v[5])][t][hh[2]];
  }
  if(std::any_of(T.begin(),T.end(),[](const F&x){return bool(x);}))return T;
 }throw std::runtime_error("zero invariant projector");
}
F contract(const Tensor&T,const std::array<V3,3>&v){F z;for(int i=0;i<3;i++)for(int j=0;j<3;j++)for(int k=0;k<3;k++)z+=T[9*i+3*j+k]*v[0][i]*v[1][j]*v[2][k];return z;}
std::array<int,3> bits(int idx){std::array<int,3>c;for(int j=0;j<3;j++)c[j]=((idx>>(j+3))&1)+2*((idx>>j)&1);return c;}
bool odd(int idx){int v=0;for(int j=0;j<3;j++)v+=((idx>>j)&1)*((idx>>(j+3))&1);return v%2;}
long chi(const Tensor&T,const std::vector<F>&roots,const EC&E,const F&z){
 F e1=roots[0],e2=roots[1],e3=roots[2];F a=((e1-e3)/(e1-e2)).sqrt(),b=((e2-e3)/(e2-e1)).sqrt(),c;Point R;Point Q1(e1,F(0));
 for(int attempt=0;;attempt++){require(attempt<1000,"chi normalization point budget");R=E.random();Point Q=E.add(R,Q1);if(!Q.infinity&&R.x!=e3&&Q.x!=e3){c=-a*(R.x-e2)/(R.x-e3)*b*(Q.x-e1)/(Q.x-e3)*(R.x-e3);break;}}
 require(c*c==(e3-e1)*(e2-e3),"two-level normality");
 std::array<F,4>gs{F(1),a,b,c};std::array<V3,4>vs{V3{F(1),e3,F(0)},V3{F(1),e2,F(0)},V3{F(1),e1,F(0)},V3{F(0),F(0),F(-1)}};
 std::array<V3,4>ds{V3{F(0),F(0),F(3)*e3*e3+E.a},V3{F(0),F(0),F(3)*e2*e2+E.a},V3{F(0),F(0),F(3)*e1*e1+E.a},V3{F(0),F(1),F(0)}};
 std::array<F,64>contractions{};F product(1);
 for(int idx=0;idx<64;idx++){
  auto codes=bits(idx);std::array<V3,3>vv;for(int j=0;j<3;j++)for(int l=0;l<3;l++)vv[j][l]=vs[codes[j]][l].frob(j);
  contractions[idx]=contract(T,vv);if(!odd(idx))product=product*contractions[idx];else require(!contractions[idx],"odd theta does not vanish");
 }
 F abc=a.pow(15)*b.pow(15)*c.pow(9);for(int j=0;j<3;j++)product=product*abc.frob(j);
 M3 lines;int odds[3]={63,38,52};for(int row=0;row<3;row++){
  auto codes=bits(odds[row]);std::array<V3,3>vv;for(int j=0;j<3;j++)for(int l=0;l<3;l++)vv[j][l]=vs[codes[j]][l].frob(j);
  for(int j=0;j<3;j++){auto dv=vv;for(int l=0;l<3;l++)dv[j][l]=ds[codes[j]][l].frob(j);lines[row][j]=contract(T,dv);}
 }
 M3 B;for(int i=0;i<3;i++)for(int j=0;j<3;j++)B[i][j]=z.pow(j).frob(i);
 F determinant=(det(lines)*det(B)).pow(18);
 for(int idx:odds){auto codes=bits(idx);for(int j=0;j<3;j++)determinant=determinant*gs[codes[j]].pow(27).frob(j);}
 F denominator(1);for(int idx:{0,4,32,55,62}){auto codes=bits(idx);F v=contractions[idx].pow(2);for(int j=0;j<3;j++)v=v*gs[codes[j]].pow(3).frob(j);denominator=denominator*v.pow(9);}
 F value=product*determinant/denominator;require(value&&value.frob()==value,"chi18 representative does not descend");return value.constant();
}
json screen(const EC&E,const F&z,const json&ks){
 Normal n=normal3(E);Actions a=action_matrices(E,n);std::array<Actions,3>as;
 for(int j=0;j<3;j++)for(int c=0;c<9;c++)for(int u=0;u<3;u++)for(int v=0;v<3;v++)as[j][c][u][v]=a[c][u][v].frob(j);
 auto rt=roots({E.b,E.a,F(0),F(1)});require(rt.size()==3,"two torsion splitting");json signs=json::array();int good=0;long product=1;
 for(auto&k:ks){Tensor T=invariant(as,k);long value=chi(T,rt,E,z);long character=F(value).pow((FP-1)/2).constant();require(character==1||character==long(FP-1),"invalid character");bool positive=character==1;good+=positive;product=product*character%FP;signs.push_back({{"representative",value},{"square",positive}});}
 return{{"usable_count",good},{"chi_product_character",product==1?1:-1},{"kernels",signs}};
}
int main(int argc,char**argv){
 try{
  require(argc>=4,"usage: obstruction HISTORICAL_RESULTS OUT [MAX_COMPONENT_CASES]");std::string dir=argv[1],out=argv[2];int limit=argc>3?std::stoi(argv[3]):10000;
  json kd=read_json(dir+"/kernel_classification.json"),ks;for(auto&m:kd["models"])if(m["q_mod_3"]==1&&m["trace_mod_3"]==1&&m["source_frobenius_type"]=="scalar")ks=m["transverse_kernel_bases"];require(ks.size()==12,"twelve kernel fixture");
  json cases=json::array(),component=read_json(dir+"/neighborhood_component.json");int ci=0;
  for(auto&row:component["curves"]){if(ci++>=limit)break;int expected=0;for(auto&k:row["kernels_tested"])expected+=k["unobstructed"].get<bool>();cases.push_back({{"p",109},{"modulus",std::vector<long>{103,1,0,1}},{"curve",row["curve"]},{"expected",expected},{"label","component-"+std::to_string(ci)}});}
  json forced=read_json(dir+"/forced_two_step_audit.json");cases.push_back({{"p",157},{"modulus",std::vector<long>{152,1,0,1}},{"curve",forced["bad_prepared_curve"]},{"expected",0},{"label","forced-center"}});
  for(auto&n:forced["forced_sources"])cases.push_back({{"p",157},{"modulus",std::vector<long>{152,1,0,1}},{"curve",n["curve"]},{"expected",nullptr},{"label","unprepared-forced-source"}});
  json record={{"status","running"},{"implementation","C++17/FLINT; no Python runtime"},{"scope","Exact finite obstruction evaluator; not a universal modular-function identity"},{"cases",json::array()}};write_json(out,record);
  int done=0;for(auto&row:cases){uint64_t p=row["p"],r=p==109?1293169:3868159;field_start(p,18,20261025+done);
   {auto coeff=row["modulus"].get<std::vector<long>>();std::vector<F>cc;for(auto c:coeff)cc.push_back(F(c));auto zr=roots(cc);require(zr.size()==3,"cubic embedding roots");F z=zr[0];require(z.frob(3)==z&&z.frob()!=z,"cubic embedding degree");auto ab=parse_curve(row["curve"],z);EC E{ab.first,ab.second};
    json result={{"label",row["label"]},{"p",p},{"r",r},{"source_curve",row["curve"]},{"expected_usable_count",row["expected"]},{"neighbors",json::array()}};
    auto xs=E.three_x();std::vector<F>rational;for(auto&x:xs)if(x.frob(3)==x)rational.push_back(x);result["rational_three_lines"]=rational.size();
    if(rational.size()==4){result["screen"]=screen(E,z,ks);require(result["screen"]["usable_count"]==row["expected"],"historical usable-count mismatch");
     if(result["screen"]["usable_count"]==0){int parity=1;for(auto&x:rational){EC E1=E.quotient3(x);require(E1.a.frob(3)==E1.a&&E1.b.frob(3)==E1.b,"Velu codomain descent");auto x1=E1.three_x();int lines=0;for(auto&v:x1)lines+=v.frob(3)==v;json nr={{"rational_three_lines",lines},{"kernel_x",x.coeffs()}};if(lines==4){nr["screen"]=screen(E1,z,ks);parity*=nr["screen"]["chi_product_character"].get<int>();}result["neighbors"].push_back(nr);}result["eligible_neighbor_product_character"]=parity;}
    }else require(row["expected"].is_null()&&rational.size()==1,"unexpected rational three-line count");
    record["cases"].push_back(result);write_json(out,record);std::cout<<"checked "<<++done<<" p="<<p<<" "<<row["label"]<<"\n"<<std::flush;
   }field_stop();
  }record["status"]="passed";record["universal_extra_step_claim"]="unresolved";write_json(out,record);return 0;
 }catch(const std::exception&e){std::cerr<<"ERROR "<<e.what()<<'\n';return 2;}
}
