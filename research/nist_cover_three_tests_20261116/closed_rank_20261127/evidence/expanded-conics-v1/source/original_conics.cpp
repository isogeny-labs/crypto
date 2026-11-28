#include "sections_arithmetic.hpp"
#include <optional>

using CV=std::array<F,6>;
using Anchors=std::array<int,4>;
struct CPoint {SPoint value;CV monomial,derivative;};
struct CSection {CV coefficients;std::map<int,int> support;SCRow row;};
F cdot(const CV&a,const CV&b){F v;for(size_t i=0;i<6;++i)v=v+a[i]*b[i];return v;}
CV cnormalize(CV q){size_t i=0;while(i<6&&q[i]==F(0))++i;require(i<6,"nonzero conic");F inverse=q[i].inverse();for(auto&v:q)v=v*inverse;return q;}
std::string ckey(const CV&q){std::string out;for(const auto&v:q)out+=decimal(v)+":";return out;}
BP cbp(const CV&q){BP p;const std::array<Exponent,6> e{{{0,0},{1,0},{0,1},{2,0},{1,1},{0,2}}};for(size_t i=0;i<6;++i)p.term(e[i],q[i]);return p;}
bool cgeneral(const Anchors&a,const std::vector<CPoint>&points){
 for(size_t i=0;i<4;++i)for(size_t j=i+1;j<4;++j)for(size_t k=j+1;k<4;++k){const auto&p=points.at(size_t(a[i])).value;const auto&q=points.at(size_t(a[j])).value;const auto&r=points.at(size_t(a[k])).value;if((q.x-p.x)*(r.y-p.y)==(q.y-p.y)*(r.x-p.x))return false;}
 return true;
}
std::array<CV,2> cbasis(const Anchors&a,const std::vector<CPoint>&points){
 FM matrix;for(int i:a){const auto&v=points.at(size_t(i)).monomial;matrix.emplace_back(v.begin(),v.end());}
 auto r=fs_rref(matrix);require(r.pivots.size()==4,"four independent conic constraints");
 std::vector<size_t> free;for(size_t j=0;j<6;++j)if(std::find(r.pivots.begin(),r.pivots.end(),j)==r.pivots.end())free.push_back(j);
 require(free.size()==2,"two-dimensional conic pencil");std::array<CV,2> basis;
 for(size_t j=0;j<2;++j){basis[j][free[j]]=F(1);for(size_t i=0;i<4;++i)basis[j][r.pivots[i]]=-r.a[i][free[j]];for(int i:a)require(cdot(basis[j],points.at(size_t(i)).monomial)==F(0),"pencil basis incidence");}
 return basis;
}
CV cthrough(const std::array<CV,2>&basis,const CV&constraint){F u=cdot(basis[0],constraint),v=cdot(basis[1],constraint);require(u!=F(0)||v!=F(0),"no extra pencil base condition");CV q;for(size_t i=0;i<6;++i)q[i]=v*basis[0][i]-u*basis[1][i];return cnormalize(q);}
int main(int argc,char**argv){try{
 require(argc==6||argc==7,"usage: conics QUARTIC BACKMAP BASELINE TANGENTS LIFTED_SCAN [fibres]");
 bool fibre_mode=argc==7;require(!fibre_mode||std::string(argv[6])=="fibres","explicit fibre extension");
 auto model=read_json(argv[1]),back=read_json(argv[2]),baseline=read_json(argv[3]),tangents=read_json(argv[4]),scan=read_json(argv[5]);
 ss_private(baseline,tangents);auto c=read_bp(model.at("terms"),true);std::array<BP,4> map;size_t h=0;
 for(const auto*key:{"X_numerator","X_denominator","Y_numerator","Y_denominator"})map[h++]=read_bp(back.at(key));
 require(global_identity(c,map),"actual elliptic map identity");
 std::vector<F> ic(5);for(const auto&[e,a]:c.t)if(e[0]+e[1]==4)ic[size_t(e[1])]=a;
 Poly inf(std::move(ic)),z{F(0),F(1)};require(gcd(inf,wide_power(z,prime(),inf)-z).degree()==0,"no omitted rational infinity");
 std::vector<Point> base;for(const auto&q:baseline.at("projected_base").arr)base.push_back(point(q));require(base.size()==225,"fixed old base width");
 std::vector<CPoint> points;std::set<std::string> coordinates;
 for(const auto&q:scan.at("points").arr){
  F x=coefficient(q.at("x")),y=coefficient(q.at("y"));require(coordinates.insert(sp_key(x,y)).second&&fs_eval(c,x,y)==F(0),"unique actual quartic point");
  auto mapped=fs_image(c,map,x,y);require(mapped.value==point(q.at("image")),"fresh actual image binding");int index=q.at("index").integer(),sign=q.at("sign").integer();
  if(index<0)require(index==-1&&sign==0&&mapped.value.zero,"identity image label");
  else{require(index<225&&(sign==1||sign==-1),"old signed image label");Point expected=base[size_t(index)];if(sign<0)expected.y=-expected.y;require(mapped.value==expected,"signed old image coordinates");}
  F fx=fs_eval(c.partial(0),x,y),fy=fs_eval(c.partial(1),x,y);require(fx!=F(0)||fy!=F(0),"smooth point");
  points.push_back({{x,y,mapped.value,index,sign},{F(1),x,y,x*x,x*y,y*y},{F(0),fy,-fx,F(2)*x*fy,y*fy-x*fx,-F(2)*y*fx}});
 }
 require(points.size()==652,"frozen lifted pool size");
 for(size_t i=0;i<226;++i){const auto&q=baseline.at("pool").at("points").arr[i];require(decimal(points[i].value.x)==q.at("x").scalar&&decimal(points[i].value.y)==q.at("y").scalar&&points[i].value.image==point(baseline.at("images").arr[i].at("point")),"original pool prefix unchanged");}
 std::vector<Anchors> known;std::vector<BP> lines;
 for(const auto&f:baseline.at("pool").at("split_fibers").arr){Anchors a;require(f.at("point_indices").arr.size()==4,"known split degree four");for(size_t i=0;i<4;++i)a[i]=f.at("point_indices").arr[i].integer();known.push_back(a);const auto&p=points.at(size_t(a[0])).value;const auto&q=points.at(size_t(a[1])).value;BP line;line.term({0,0},q.y*p.x-q.x*p.y);line.term({1,0},p.y-q.y);line.term({0,1},q.x-p.x);for(int i:a)require(fs_eval(line,points.at(size_t(i)).value.x,points.at(size_t(i)).value.y)==F(0),"known complete line incidence");lines.push_back(std::move(line));}
 require(known.size()==7,"all seven original sections");
 SCRow reference(225);Point kappa;for(int i:known[0]){const auto&p=points.at(size_t(i)).value;kappa=add(kappa,p.image);if(p.index>=0)reference[size_t(p.index)]+=p.sign;}
 std::vector<std::array<int,2>> linepairs;for(int i=0;i<7;++i)for(int j=i+1;j<7;++j)linepairs.push_back({i,j});
 const std::array<std::array<int,2>,6> choices{{{0,1},{0,2},{0,3},{1,2},{1,3},{2,3}}};
 std::vector<Anchors> pencils;std::set<Anchors> seen_anchors;
 auto pencil=[&](Anchors a){std::sort(a.begin(),a.end());require(std::adjacent_find(a.begin(),a.end())==a.end()&&cgeneral(a,points),"general-position anchors");if(seen_anchors.insert(a).second)pencils.push_back(a);};
 for(int k=0;k<32;++k){auto ij=linepairs[size_t(k%21)];auto selection=choices[size_t(k/21)];pencil({known[size_t(ij[0])][size_t(selection[0])],known[size_t(ij[0])][size_t(selection[1])],known[size_t(ij[1])][size_t(selection[0])],known[size_t(ij[1])][size_t(selection[1])]});}
 require(pencils.size()==32,"32 distinct qualification pencils");
 std::vector<int> bank;
 for(size_t i=0;i<points.size()&&bank.size()<8;++i){bool ok=true;for(size_t j=0;j<bank.size();++j)for(size_t k=j+1;k<bank.size();++k){const auto&p=points[size_t(bank[j])].value;const auto&q=points[size_t(bank[k])].value;const auto&r=points[i].value;if((q.x-p.x)*(r.y-p.y)==(q.y-p.y)*(r.x-p.x))ok=false;}if(ok)bank.push_back(int(i));}
 require(bank.size()==8,"eight-point general-position bank");
 for(size_t i=0;i<8&&pencils.size()<64;++i)for(size_t j=i+1;j<8&&pencils.size()<64;++j)for(size_t k=j+1;k<8&&pencils.size()<64;++k)for(size_t l=k+1;l<8&&pencils.size()<64;++l)pencil({bank[i],bank[j],bank[k],bank[l]});
 require(pencils.size()==64,"frozen original pencil count");
 size_t fibre_groups=0,degenerate_fibre_groups=0;
 if(fibre_mode){
  std::map<std::array<int,2>,std::vector<int>> groups;
  for(size_t i=0;i<points.size();++i){const auto&p=points[i].value;groups[{p.index,p.sign}].push_back(int(i));}
  for(const auto&[image,ids]:groups){(void)image;require(ids.size()<=4,"recorded signed fibre maximum");if(ids.size()!=4)continue;++fibre_groups;Anchors a{ids[0],ids[1],ids[2],ids[3]};if(!cgeneral(a,points)){++degenerate_fibre_groups;continue;}pencil(a);}
  require(fibre_groups==25&&pencils.size()<=89,"all recorded four-point image groups");
 }
 std::map<int,Jet> jets;precision=9;
 auto multiplicity=[&](const CV&q,int index){const auto&p=points.at(size_t(index));require(cdot(q,p.monomial)==F(0),"local conic incidence");if(cdot(q,p.derivative)!=F(0))return 1;auto at=jets.find(index);if(at==jets.end())at=jets.emplace(index,lift(homogenize(c),{R{K(p.value.x)},R{K(p.value.y)},R{K(1)}},2)).first;int n=order(evaluate(homogenize(cbp(q),2),at->second.xyz));require(n>=2&&n<=8,"bounded exact tangent order");return n;};
 auto section=[&](const CV&q,const std::vector<int>&ids)->std::optional<CSection>{
  int degree=0;std::map<int,int> support;SCRow row(225);Point sum;
  for(int i:ids){int m=multiplicity(q,i);require(support.emplace(i,m).second,"unique conic support");degree+=m;const auto&p=points.at(size_t(i)).value;for(int n=0;n<m;++n)sum=add(sum,p.image);if(p.index>=0)row[size_t(p.index)]+=m*p.sign;}
  require(degree<=8,"proper quartic-conic Bezout degree");if(degree!=8)return {};
  Point negative=kappa;if(!negative.zero)negative.y=-negative.y;sum=add(add(sum,negative),negative);require(sum.zero,"actual degree-eight elliptic row");
  for(size_t i=0;i<225;++i){row[i]-=2*reference[i];}
  return CSection{q,std::move(support),std::move(row)};
 };
 std::map<std::string,CSection> sections;size_t candidate_count=0;
 for(size_t pindex=0;pindex<pencils.size();++pindex){
  const auto&a=pencils[pindex];auto basis=cbasis(a,points);std::map<std::string,std::pair<CV,std::vector<int>>> candidates;
  for(size_t i=0;i<points.size();++i){if(std::find(a.begin(),a.end(),int(i))!=a.end())continue;CV q=cthrough(basis,points[i].monomial);auto[it,inserted]=candidates.emplace(ckey(q),std::make_pair(q,std::vector<int>{}));(void)inserted;it->second.second.push_back(int(i));}
  for(int i:a){CV q=cthrough(basis,points[size_t(i)].derivative);candidates.emplace(ckey(q),std::make_pair(q,std::vector<int>{}));}
  candidate_count+=candidates.size();
  for(const auto&[key,value]:candidates){if(sections.count(key))continue;std::vector<int> ids(a.begin(),a.end());ids.insert(ids.end(),value.second.begin(),value.second.end());require(ids.size()<=8,"distinct conic intersection degree bound");bool tangent=false;for(int i:ids){require(cdot(value.first,points[size_t(i)].monomial)==F(0),"complete parameter bucket incidence");if(cdot(value.first,points[size_t(i)].derivative)==F(0))tangent=true;}if(ids.size()<8&&!tangent)continue;auto result=section(value.first,ids);if(result){sections.emplace(key,std::move(*result));require(sections.size()<=4096,"supported conic bound");}}
  std::cerr<<"completed pencil "<<pindex<<" candidates "<<candidates.size()<<" accepted "<<sections.size()<<'\n';
 }
 auto vector_for=[&](const BP&q){CV v;v[0]=fs_eval(q,F(0),F(0));const std::array<Exponent,6> e{{{0,0},{1,0},{0,1},{2,0},{1,1},{0,2}}};for(size_t i=0;i<6;++i){auto at=q.t.find(e[i]);v[i]=at==q.t.end()?F(0):at->second;}return cnormalize(v);};
 SCMat control_rows;
 for(auto ij:linepairs){CV q=vector_for(lines[size_t(ij[0])]*lines[size_t(ij[1])]);auto hit=sections.find(ckey(q));require(hit!=sections.end(),"all 21 known line-product conics recovered");control_rows.push_back(hit->second.row);}
 require(sc_rank(control_rows)==6,"nonempty qualification spans all six baseline rows");
 CV repeated=vector_for(lines[0]*lines[0]);std::vector<int> refids(known[0].begin(),known[0].end());auto square=section(repeated,refids);require(bool(square)&&square->support.size()==4,"repeated reference conic detected");for(const auto&[i,m]:square->support){(void)i;require(m==2,"repeated reference multiplicity two");}for(const auto&v:square->row)require(v==0,"repeated reference projected zero");
 CV changed=repeated;changed[0]=changed[0]+F(1);fs_reject([&]{(void)section(changed,refids);});refids.pop_back();require(!section(repeated,refids),"missing support is incomplete");
 SCMat old(6,SCRow(225));for(size_t i=0;i<6;++i)for(const auto&t:baseline.at("rows").arr[i].arr)old[i].at(size_t(t.arr[0].integer()))+=SCBig(t.arr[1].scalar);
 SCMat combined=old,full=sc_read_matrix(tangents.at("matrix"));for(const auto&[key,s]:sections){(void)key;combined.push_back(s.row);SCRow row(429);std::copy(s.row.begin(),s.row.end(),row.begin());full.push_back(std::move(row));}
 size_t rank=sc_rank(combined),frank=sc_rank(full);SCMat aux;for(const auto&r:full)aux.emplace_back(r.begin()+225,r.end());size_t arank=sc_rank(aux);require(arank==102&&frank-arank==rank,"separate full/auxiliary/old closure equality");
 std::cout<<"{\"scope\":\""<<(fibre_mode?"reference-and-same-image-conic-pencils":"64-fixed-conic-pencils-on-old-base-lift")<<"\",\"pool_size\":652,\"pencil_count\":"<<pencils.size()<<",\"four_point_image_groups\":"<<fibre_groups<<",\"degenerate_image_groups_excluded\":"<<degenerate_fibre_groups<<",\"candidate_occurrences\":"<<candidate_count<<",\"supported_conics\":"<<sections.size()<<",\"known_line_products_recovered\":21,\"qualification_rank\":6,\"full_rank\":"<<frank<<",\"auxiliary_rank\":"<<arank<<",\"closed_rank\":"<<rank<<",\"candidate_gain\":"<<rank-6<<",\"controls_pass\":true,\"positive_gain_established\":false,\"pencils\":[";
 for(size_t i=0;i<pencils.size();++i){if(i)std::cout<<',';std::cout<<'[';for(size_t j=0;j<4;++j)std::cout<<(j?",":"")<<pencils[i][j];std::cout<<']';}
 std::cout<<"],\"sections\":[";bool comma=false;for(const auto&[key,s]:sections){(void)key;if(comma)std::cout<<',';comma=true;std::cout<<"{\"coefficients\":[";for(size_t i=0;i<6;++i)std::cout<<(i?",":"")<<'"'<<decimal(s.coefficients[i])<<'"';std::cout<<"],\"support\":[";bool sep=false;for(const auto&[i,m]:s.support){if(sep)std::cout<<',';sep=true;std::cout<<'['<<i<<','<<m<<']';}std::cout<<"],\"row\":";sc_emit_row(std::cout,s.row);std::cout<<'}';}
 std::cout<<"],\"standing\":\"producer-unverified; positive candidates require actual KM and separate elliptic/rank replay\"}\n";
 return 0;
}catch(const std::exception&e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
