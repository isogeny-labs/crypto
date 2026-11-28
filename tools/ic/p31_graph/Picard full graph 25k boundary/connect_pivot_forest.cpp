// Multi-source Dijkstra plus Kruskal on the Voronoi boundary graph. Each
// selected connector is expanded to original p31 core edge witnesses.
#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <queue>
#include <stdexcept>
#include <tuple>
#include <unordered_map>
#include <vector>

struct Edge{uint32_t ordinal,x0,x1;};
struct Chain{uint32_t x0,x1,offset,length;};
template<class T>std::vector<T>load(const char*name){std::ifstream f(name,std::ios::binary|std::ios::ate);if(!f||f.tellg()%sizeof(T))throw std::runtime_error("bad input");size_t n=size_t(f.tellg())/sizeof(T);f.seekg(0);std::vector<T>v(n);if(!f.read(reinterpret_cast<char*>(v.data()),n*sizeof(T)))throw std::runtime_error("short input");return v;}
struct Arc{uint32_t to,chain;};
int main(int argc,char**argv){
 if(argc!=5)throw std::runtime_error("usage: connect_pivots core.bin chains.bin paths.bin pivots.txt");
 auto edges=load<Edge>(argv[1]);auto chains=load<Chain>(argv[2]);auto paths=load<uint32_t>(argv[3]);
 std::vector<uint32_t>labels;for(auto c:chains){labels.push_back(c.x0);labels.push_back(c.x1);}
 std::sort(labels.begin(),labels.end());labels.erase(std::unique(labels.begin(),labels.end()),labels.end());
 std::unordered_map<uint32_t,uint32_t>id;id.reserve(labels.size());for(uint32_t i=0;i<labels.size();++i)id.emplace(labels[i],i);
 std::vector<std::vector<Arc>>graph(labels.size());
 for(uint32_t k=0;k<chains.size();++k){auto c=chains[k];uint32_t a=id.at(c.x0),b=id.at(c.x1);graph[a].push_back({b,k});if(a!=b)graph[b].push_back({a,k});}
 std::ifstream sources(argv[4]);if(!sources)throw std::runtime_error("pivots unavailable");
 std::vector<uint32_t>source_cycle(labels.size(),UINT32_MAX),source_id(labels.size(),UINT32_MAX);
 std::vector<uint32_t>parent(labels.size(),UINT32_MAX),parent_chain(labels.size(),UINT32_MAX);
 std::vector<uint64_t>dist(labels.size(),UINT64_MAX);
 std::vector<uint8_t>settled(labels.size());
 using Item=std::pair<uint64_t,uint32_t>;
 std::priority_queue<Item,std::vector<Item>,std::greater<Item>>heap;
 uint32_t cycle,x,count=0;
 struct Duplicate{uint32_t representative,cycle,x;};
 std::vector<Duplicate>duplicates;
 while(sources>>cycle>>x){uint32_t v=id.at(x);if(dist[v]!=UINT64_MAX){duplicates.push_back({source_cycle[v],cycle,x});++count;continue;}dist[v]=0;source_id[v]=v;source_cycle[v]=cycle;heap.push({0,v});++count;}
 if(count<2)throw std::runtime_error("too few pivots");
 struct Boundary{uint64_t length;uint32_t u,v,chain;};
 std::vector<Boundary>boundary;
 while(!heap.empty()){
  auto [d,u]=heap.top();heap.pop();if(d!=dist[u]||settled[u])continue;settled[u]=1;
  for(auto a:graph[u]){
   uint32_t v=a.to;uint64_t step=chains[a.chain].length;
   if(settled[v]&&source_id[v]!=source_id[u])boundary.push_back({d+step+dist[v],u,v,a.chain});
   if(!settled[v]&&d+step<dist[v]){dist[v]=d+step;source_id[v]=source_id[u];source_cycle[v]=source_cycle[u];parent[v]=u;parent_chain[v]=a.chain;heap.push({dist[v],v});}
  }
 }
 std::sort(boundary.begin(),boundary.end(),[](auto a,auto b){return a.length!=b.length?a.length<b.length:a.chain<b.chain;});
 std::vector<uint32_t>uf(labels.size()),sz(labels.size(),1);for(uint32_t i=0;i<uf.size();++i)uf[i]=i;
 auto root=[&](uint32_t v){while(uf[v]!=v){uf[v]=uf[uf[v]];v=uf[v];}return v;};
 auto trace=[&](uint32_t v){std::vector<uint32_t>p;while(parent[v]!=UINT32_MAX){p.push_back(parent_chain[v]);v=parent[v];}return p;};
 std::cout<<"{\"method\":\"multi_source_dijkstra_voronoi_kruskal\",\"pivot_count\":"<<count<<",\"boundary_edges\":"<<boundary.size()<<",\"connectors\":[";
 uint32_t selected=0;
 for(auto duplicate:duplicates){
  if(selected++)std::cout<<',';
  std::cout<<"{\"cycle_a\":"<<duplicate.representative<<",\"cycle_b\":"<<duplicate.cycle
           <<",\"pivot_a\":"<<duplicate.x<<",\"pivot_b\":"<<duplicate.x
           <<",\"connector_core_edges\":0,\"connector_kernel_edges\":0,\"core_edge_indexes\":[],\"vertex_x_path\":["<<duplicate.x<<"]}";
 }
 for(auto item:boundary){
 uint32_t best_u=item.u,best_v=item.v,best_chain=item.chain;
 uint32_t ra=root(source_id[best_u]),rb=root(source_id[best_v]);if(ra==rb)continue;
 if(sz[ra]<sz[rb])std::swap(ra,rb);uf[rb]=ra;sz[ra]+=sz[rb];
 auto left=trace(best_u),right=trace(best_v);
 std::reverse(left.begin(),left.end());
 std::vector<uint32_t>cp=left;cp.push_back(best_chain);cp.insert(cp.end(),right.begin(),right.end());
 uint32_t start=labels[source_id[best_u]],end=labels[source_id[best_v]],current=start;
 std::vector<uint32_t>ep,vp{start};
 for(uint32_t k:cp){auto c=chains[k];bool forward=current==c.x0;if(!forward&&current!=c.x1)throw std::runtime_error("connector chain disconnected");
  for(uint32_t j=0;j<c.length;++j){uint32_t pos=forward?c.offset+j:c.offset+c.length-1-j;uint32_t ei=paths[pos];auto e=edges.at(ei);current=current==e.x0?e.x1:current==e.x1?e.x0:UINT32_MAX;if(current==UINT32_MAX)throw std::runtime_error("connector edge disconnected");ep.push_back(ei);vp.push_back(current);}
 }
 if(current!=end||ep.size()!=item.length)throw std::runtime_error("connector length mismatch");
 if(selected++)std::cout<<',';
 std::cout<<"{\"cycle_a\":"<<source_cycle[best_u]<<",\"cycle_b\":"<<source_cycle[best_v]<<",\"pivot_a\":"<<start<<",\"pivot_b\":"<<end<<",\"connector_core_edges\":"<<ep.size()<<",\"connector_kernel_edges\":"<<cp.size()<<",\"core_edge_indexes\":[";
 for(size_t i=0;i<ep.size();++i){if(i)std::cout<<',';std::cout<<ep[i];}std::cout<<"],\"vertex_x_path\":[";
 for(size_t i=0;i<vp.size();++i){if(i)std::cout<<',';std::cout<<vp[i];}std::cout<<"]}";
 }
 std::cout<<"],\"selected_connectors\":"<<selected<<"}\n";
}
