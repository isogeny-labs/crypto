#pragma once
#include "partial_arithmetic.hpp"

using GO=std::vector<std::pair<int,int>>;
struct GE {GO old;std::array<int,2> vertex{-1,-1},coefficient{0,0};int kind=0,i=0,j=0;};
struct GW {std::vector<int> edges;SCRow combination,old;};
struct GP {int edge,vertex,coefficient;};
struct GR {size_t auxiliary_rank=0,closed_rank=0,full_rank=0,peeled=0,core_rows=0,core_components=0,largest_core_rows=0,zero_aux_rows=0;std::string peel_hash;std::vector<GW> new_rows;std::vector<GP> peel;};
GE gmake(GO old,int u,int a,int v,int b,int kind,int i,int j){std::map<int,int> aux;if(a)aux[u]+=a;if(b)aux[v]+=b;GE e;e.old=std::move(old);e.kind=kind;e.i=i;e.j=j;size_t k=0;for(auto[id,c]:aux)if(c){require(id>=0&&k<2&&c>=-2&&c<=2,"canonical graph endpoints");e.vertex[k]=id;e.coefficient[k]=c;++k;}return e;}
SCRow gold(const GO&old){SCRow row(225);for(auto[i,c]:old){require(i>=0&&i<225&&c>=-16&&c<=16,"small old graph coefficient");row[size_t(i)]+=c;}return row;}
struct GB {
 std::map<size_t,SCRow> rows;
 bool add(SCRow row){require(row.size()==225,"old basis width");for(auto&v:row)v=sc_reduce(v);for(const auto&[p,r]:rows){SCBig c=row[p];if(c!=0)for(size_t j=p;j<225;++j)row[j]=sc_reduce(row[j]-c*r[j]);}size_t p=0;while(p<225&&row[p]==0)++p;if(p==225)return false;SCBig inverse;require(mpz_invert(inverse.get_mpz_t(),row[p].get_mpz_t(),sc_n.get_mpz_t())==1,"old graph pivot inverse");for(size_t j=p;j<225;++j)row[j]=sc_reduce(row[j]*inverse);rows.emplace(p,std::move(row));return true;}
};
std::vector<char> gverify_peel(const std::vector<GE>&edges,size_t vertices,const std::vector<GP>&proof){
 std::vector<char> active(edges.size(),false);std::vector<size_t> degree(vertices);
 for(size_t i=0;i<edges.size();++i)for(size_t k=0;k<2;++k)if(edges[i].coefficient[k]){int v=edges[i].vertex[k];require(v>=0&&size_t(v)<vertices,"peel endpoint range");active[i]=true;++degree[size_t(v)];}
 for(auto p:proof){require(p.edge>=0&&size_t(p.edge)<edges.size()&&p.vertex>=0&&size_t(p.vertex)<vertices&&active[size_t(p.edge)]&&degree[size_t(p.vertex)]==1,"active singleton owner");const auto&e=edges[size_t(p.edge)];bool matches=false;for(size_t k=0;k<2;++k)if(e.vertex[k]==p.vertex&&e.coefficient[k]==p.coefficient&&p.coefficient!=0)matches=true;require(matches,"exact singleton coefficient");active[size_t(p.edge)]=false;for(size_t k=0;k<2;++k)if(e.coefficient[k]){require(degree[size_t(e.vertex[k])]>0,"positive singleton incidence");--degree[size_t(e.vertex[k])];}}
 return active;
}
void gcombination(const std::vector<GE>&edges,const std::vector<int>&ids,const SCRow&combination,const SCRow&old){
 require(ids.size()==combination.size()&&old.size()==225,"graph combination dimensions");SCRow row(225);std::map<int,SCBig> aux;
 for(size_t i=0;i<ids.size();++i)if(combination[i]!=0){const auto&e=edges.at(size_t(ids[i]));for(auto[j,c]:e.old)row[size_t(j)]=sc_reduce(row[size_t(j)]+combination[i]*c);for(size_t k=0;k<2;++k)if(e.coefficient[k])aux[e.vertex[k]]=sc_reduce(aux[e.vertex[k]]+combination[i]*e.coefficient[k]);}
 for(const auto&[v,c]:aux){(void)v;require(c==0,"all component auxiliary coefficients canceled");}for(size_t i=0;i<225;++i)require(row[i]==sc_reduce(old[i]),"exact old component combination");
}
GR gclose(const std::vector<GE>&edges,size_t vertices,const SCMat&baseline){
 require(edges.size()<=250000&&vertices<=500000,"graph collection resource cap");GB basis;for(const auto&row:baseline)basis.add(row);GR out;
 std::vector<std::vector<int>> adjacent(vertices);std::vector<size_t> degree(vertices);std::vector<char> active(edges.size(),false);
 for(size_t i=0;i<edges.size();++i){bool has_aux=false;for(size_t k=0;k<2;++k)if(edges[i].coefficient[k]){int v=edges[i].vertex[k];require(v>=0&&size_t(v)<vertices,"graph endpoint in range");adjacent[size_t(v)].push_back(int(i));++degree[size_t(v)];has_aux=true;}active[i]=has_aux;if(!has_aux){++out.zero_aux_rows;auto row=gold(edges[i].old);if(basis.add(row))out.new_rows.push_back({{int(i)},SCRow{SCBig(1)},std::move(row)});}}
 std::vector<int> queue;for(size_t v=0;v<vertices;++v)if(degree[v]==1)queue.push_back(int(v));PHash peel_digest;
 for(size_t cursor=0;cursor<queue.size();++cursor){int v=queue[cursor];if(degree[size_t(v)]!=1)continue;int owner=-1;for(int i:adjacent[size_t(v)])if(active[size_t(i)]){require(owner<0,"unique active singleton row");owner=i;}require(owner>=0,"singleton row present");const auto&e=edges[size_t(owner)];int coefficient=0;for(size_t k=0;k<2;++k)if(e.vertex[k]==v)coefficient=e.coefficient[k];require(coefficient!=0,"nonzero singleton coefficient");out.peel.push_back({owner,v,coefficient});peel_digest.add(size_t(owner),size_t(v),std::to_string(coefficient));active[size_t(owner)]=false;++out.peeled;
  for(size_t k=0;k<2;++k)if(e.coefficient[k]){size_t w=size_t(e.vertex[k]);require(degree[w]>0,"active edge count");--degree[w];if(degree[w]==1)queue.push_back(int(w));}
 }
 require(gverify_peel(edges,vertices,out.peel)==active,"complete singleton witness replay");out.peel_hash=peel_digest.finish();out.auxiliary_rank=out.peeled;
 std::vector<char> seen_vertex(vertices,false),seen_edge(edges.size(),false);
 for(size_t start=0;start<edges.size();++start)if(active[start]&&!seen_edge[start]){
  std::vector<int> pending{edges[start].vertex[0]},ids,vs;require(pending[0]>=0,"nonempty active core edge");seen_vertex[size_t(pending[0])]=true;
  for(size_t cursor=0;cursor<pending.size();++cursor){int v=pending[cursor];vs.push_back(v);for(int id:adjacent[size_t(v)])if(active[size_t(id)]&&!seen_edge[size_t(id)]){seen_edge[size_t(id)]=true;ids.push_back(id);for(size_t k=0;k<2;++k)if(edges[size_t(id)].coefficient[k]){int w=edges[size_t(id)].vertex[k];if(!seen_vertex[size_t(w)]){seen_vertex[size_t(w)]=true;pending.push_back(w);}}}}
  std::sort(ids.begin(),ids.end());std::sort(vs.begin(),vs.end());require(ids.size()<=256&&225+vs.size()<=1024,"surviving core exceeds unchanged dense closure cap; inconclusive");std::map<int,size_t> local;for(size_t i=0;i<vs.size();++i)local[vs[i]]=i;SCMat matrix;
  for(int id:ids){const auto&e=edges[size_t(id)];auto row=gold(e.old);row.resize(225+vs.size());for(size_t k=0;k<2;++k)if(e.coefficient[k])row[225+local.at(e.vertex[k])]+=e.coefficient[k];matrix.push_back(std::move(row));}
  auto closure=sc_close(matrix,225,0);out.auxiliary_rank+=closure.new_rank;out.core_rows+=ids.size();++out.core_components;out.largest_core_rows=std::max(out.largest_core_rows,ids.size());
  for(size_t i=0;i<closure.old_rows.size();++i){gcombination(edges,ids,closure.combinations[i],closure.old_rows[i]);if(basis.add(closure.old_rows[i]))out.new_rows.push_back({ids,closure.combinations[i],closure.old_rows[i]});}
 }
 out.closed_rank=basis.rows.size();out.full_rank=out.auxiliary_rank+out.closed_rank;return out;
}
void gcontrols(){
 struct Fixture{size_t vertices;std::vector<GE> edges;size_t gain;};GO a{{0,1}},b{{1,1}},ab{{0,-1},{1,-1}};
 std::vector<Fixture> fixtures{
  {4,{gmake(a,0,1,1,1,0,0,0),gmake(b,2,1,3,1,0,0,0)},0},
  {2,{gmake(a,0,1,1,-1,0,0,0),gmake(b,0,1,1,-1,0,0,0)},1},
  {2,{gmake(a,0,1,1,-1,0,0,0),gmake(b,0,-1,1,1,0,0,0)},1},
  {3,{gmake(a,0,1,1,1,0,0,0),gmake(b,1,1,2,1,0,0,0),gmake(ab,2,1,0,1,0,0,0)},0},
  {3,{gmake(a,0,1,1,-1,0,0,0),gmake(b,1,1,2,-1,0,0,0),gmake(GO{{0,-1}},2,1,0,-1,0,0,0)},1},
  {1,{gmake(a,0,1,0,-1,0,0,0)},1},
  {1,{gmake(a,0,1,0,1,0,0,0),gmake(b,0,1,0,1,0,0,0)},1},
  {2,{gmake(a,0,1,0,1,0,0,0),gmake(b,1,1,1,1,0,0,0),gmake(GO{},0,1,1,-1,0,0,0)},1}
 };
 for(const auto&f:fixtures){auto result=gclose(f.edges,f.vertices,{});SCMat full,aux;for(const auto&e:f.edges){auto row=gold(e.old);row.resize(225+f.vertices);for(size_t k=0;k<2;++k)if(e.coefficient[k])row[225+size_t(e.vertex[k])]+=e.coefficient[k];aux.emplace_back(row.begin()+225,row.end());full.push_back(std::move(row));}require(result.full_rank==sc_rank(full)&&result.auxiliary_rank==sc_rank(aux)&&result.closed_rank==f.gain,"graph controls match full exact dense ranks");}
 auto private_result=gclose(fixtures[0].edges,4,{});require(!private_result.peel.empty(),"nonempty private peel control");auto bad=private_result.peel;bad[0].vertex=2;fs_reject([&]{(void)gverify_peel(fixtures[0].edges,4,bad);});bad=private_result.peel;++bad[0].coefficient;fs_reject([&]{(void)gverify_peel(fixtures[0].edges,4,bad);});
}
