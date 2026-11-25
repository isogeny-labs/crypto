// Suppress every degree-two chain of the exact p31 PP graph 2-core.
// Chain records preserve oriented core-edge indexes for geometric replay.
#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <unordered_map>
#include <vector>

struct Edge { uint32_t ordinal, x0, x1; };
struct Arc { uint32_t to, edge, next; };
struct Chain { uint32_t x0, x1, offset, length; };
static_assert(sizeof(Edge)==12 && sizeof(Chain)==16);

int main(int argc,char**argv) {
  if(argc!=4)throw std::runtime_error("usage: compress_kernel core.bin chains.bin paths.bin");
  std::ifstream in(argv[1],std::ios::binary|std::ios::ate);
  if(!in || in.tellg()%sizeof(Edge))throw std::runtime_error("bad core file");
  size_t m=size_t(in.tellg())/sizeof(Edge);in.seekg(0);
  std::vector<Edge> edges(m);
  if(!in.read(reinterpret_cast<char*>(edges.data()),m*sizeof(Edge)))throw std::runtime_error("short core file");
  std::vector<uint32_t> labels;labels.reserve(2*m);
  for(const auto&e:edges){labels.push_back(e.x0);labels.push_back(e.x1);}
  std::sort(labels.begin(),labels.end());labels.erase(std::unique(labels.begin(),labels.end()),labels.end());
  uint32_t n=labels.size();
  std::unordered_map<uint32_t,uint32_t> id;id.reserve(n);
  for(uint32_t i=0;i<n;++i)id.emplace(labels[i],i);
  std::vector<uint32_t> degree(n),head(n,UINT32_MAX);
  std::vector<Arc> arcs;arcs.reserve(2*m);
  for(uint32_t k=0;k<m;++k){
    uint32_t a=id.at(edges[k].x0),b=id.at(edges[k].x1);
    if(a==b)throw std::runtime_error("self-loop unsupported");
    arcs.push_back({b,k,head[a]});head[a]=arcs.size()-1;++degree[a];
    arcs.push_back({a,k,head[b]});head[b]=arcs.size()-1;++degree[b];
  }
  std::vector<uint8_t> seen(m);
  std::vector<Chain> chains;chains.reserve(m/20);
  std::vector<uint32_t> paths;paths.reserve(m);
  auto walk=[&](uint32_t start,uint32_t first){
    uint32_t at=start,arc=first,previous=UINT32_MAX;
    uint32_t offset=paths.size();
    for(;;){
      Arc a=arcs[arc];
      if(seen[a.edge])throw std::runtime_error("duplicate path edge");
      seen[a.edge]=1;paths.push_back(a.edge);
      previous=a.edge;at=a.to;
      if(degree[at]!=2 || (at==start && paths.size()>offset))break;
      uint32_t next=head[at];
      if(arcs[next].edge==previous)next=arcs[next].next;
      if(next==UINT32_MAX||arcs[next].edge==previous)throw std::runtime_error("broken degree-two chain");
      arc=next;
      if(paths.size()-offset>m)throw std::runtime_error("nonterminating chain");
    }
    chains.push_back({labels[start],labels[at],offset,uint32_t(paths.size()-offset)});
  };
  for(uint32_t v=0;v<n;++v)if(degree[v]!=2){
    if(degree[v]<2)throw std::runtime_error("not a 2-core");
    for(uint32_t arc=head[v];arc!=UINT32_MAX;arc=arcs[arc].next)
      if(!seen[arcs[arc].edge])walk(v,arc);
  }
  uint32_t pure_cycles=0;
  for(uint32_t k=0;k<m;++k)if(!seen[k]){
    uint32_t before=chains.size();walk(id.at(edges[k].x0),2*k);
    if(chains[before].x0!=chains[before].x1)throw std::runtime_error("pure cycle did not close");
    ++pure_cycles;
  }
  if(paths.size()!=m)throw std::runtime_error("edge partition incomplete");
  std::ofstream out_chains(argv[2],std::ios::binary|std::ios::trunc);
  std::ofstream out_paths(argv[3],std::ios::binary|std::ios::trunc);
  out_chains.write(reinterpret_cast<const char*>(chains.data()),chains.size()*sizeof(Chain));
  out_paths.write(reinterpret_cast<const char*>(paths.data()),paths.size()*sizeof(uint32_t));
  if(!out_chains||!out_paths)throw std::runtime_error("output failed");
  uint64_t kernel_vertices=0,degree2_vertices=0;
  for(auto d:degree){kernel_vertices+=d>2;degree2_vertices+=d==2;}
  std::cout<<"{\"core_vertices\":"<<n<<",\"core_edges\":"<<m
           <<",\"kernel_vertices\":"<<kernel_vertices<<",\"degree_two_vertices\":"<<degree2_vertices
           <<",\"kernel_edges\":"<<chains.size()-pure_cycles<<",\"pure_degree_two_cycles\":"<<pure_cycles
           <<",\"path_edge_records\":"<<paths.size()<<"}\n";
}
