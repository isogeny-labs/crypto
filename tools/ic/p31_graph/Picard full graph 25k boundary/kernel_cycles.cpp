// Kruskal minimum-weight forest on the degree-two-suppressed kernel, then
// expand its shortest fundamental cycles to original p31 edge witnesses.
#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <queue>
#include <stdexcept>
#include <unordered_map>
#include <vector>

struct Edge { uint32_t ordinal,x0,x1; };
struct Chain { uint32_t x0,x1,offset,length; };
struct Candidate { uint64_t length; uint32_t chain; };
static_assert(sizeof(Edge)==12 && sizeof(Chain)==16);
template<class T> std::vector<T> load(const char*name){
  std::ifstream in(name,std::ios::binary|std::ios::ate);
  if(!in||in.tellg()%sizeof(T))throw std::runtime_error("bad input file");
  size_t n=size_t(in.tellg())/sizeof(T);in.seekg(0);
  std::vector<T> out(n);
  if(!in.read(reinterpret_cast<char*>(out.data()),n*sizeof(T)))throw std::runtime_error("short input");
  return out;
}
int main(int argc,char**argv){
  if(argc!=5 && argc!=6)throw std::runtime_error("usage: kernel_cycles core.bin chains.bin paths.bin count [start]");
  auto edges=load<Edge>(argv[1]);auto chains=load<Chain>(argv[2]);auto paths=load<uint32_t>(argv[3]);
  uint32_t top=std::stoul(argv[4]);if(!top||top>1000)throw std::runtime_error("top_n out of range");
  uint32_t first=argc==6?std::stoul(argv[5]):0;
  std::vector<uint32_t> labels;labels.reserve(chains.size()*2);
  for(auto c:chains){labels.push_back(c.x0);labels.push_back(c.x1);}
  std::sort(labels.begin(),labels.end());labels.erase(std::unique(labels.begin(),labels.end()),labels.end());
  std::unordered_map<uint32_t,uint32_t> id;id.reserve(labels.size());
  for(uint32_t i=0;i<labels.size();++i)id.emplace(labels[i],i);
  std::vector<uint32_t> order(chains.size()),uf(labels.size()),sz(labels.size(),1);
  for(uint32_t i=0;i<order.size();++i)order[i]=i;
  std::sort(order.begin(),order.end(),[&](auto a,auto b){
    return chains[a].length!=chains[b].length?chains[a].length<chains[b].length:a<b;
  });
  for(uint32_t i=0;i<uf.size();++i)uf[i]=i;
  auto root=[&](uint32_t x){while(uf[x]!=x){uf[x]=uf[uf[x]];x=uf[x];}return x;};
  std::vector<std::vector<std::pair<uint32_t,uint32_t>>> tree(labels.size());
  std::vector<uint32_t> closure;
  for(uint32_t k:order){
    auto c=chains[k];uint32_t a=id.at(c.x0),b=id.at(c.x1),ra=root(a),rb=root(b);
    if(ra==rb){closure.push_back(k);continue;}
    if(sz[ra]<sz[rb])std::swap(ra,rb);
    uf[rb]=ra;sz[ra]+=sz[rb];
    tree[a].push_back({b,k});tree[b].push_back({a,k});
  }
  const uint32_t n=labels.size();
  constexpr uint32_t LOG=18;
  std::vector<std::vector<uint32_t>> up(LOG,std::vector<uint32_t>(n));
  std::vector<uint32_t> depth(n),parent_edge(n,UINT32_MAX),component(n,UINT32_MAX);
  std::vector<uint64_t> dist(n);
  std::vector<uint32_t> queue;queue.reserve(n);
  for(uint32_t r=0;r<n;++r)if(component[r]==UINT32_MAX){
    component[r]=r;up[0][r]=r;queue.push_back(r);
    for(size_t q=queue.size()-1;q<queue.size();++q){
      uint32_t v=queue[q];
      for(auto [w,k]:tree[v])if(component[w]==UINT32_MAX){
        component[w]=r;up[0][w]=v;parent_edge[w]=k;
        depth[w]=depth[v]+1;dist[w]=dist[v]+chains[k].length;queue.push_back(w);
      }
      if(q+1==queue.size())break;
    }
  }
  for(uint32_t j=1;j<LOG;++j)for(uint32_t v=0;v<n;++v)up[j][v]=up[j-1][up[j-1][v]];
  auto lca=[&](uint32_t a,uint32_t b){
    if(component[a]!=component[b])throw std::runtime_error("closure crosses components");
    if(depth[a]<depth[b])std::swap(a,b);
    uint32_t delta=depth[a]-depth[b];
    for(uint32_t j=0;j<LOG;++j)if(delta&(1u<<j))a=up[j][a];
    if(a==b)return a;
    for(int j=int(LOG)-1;j>=0;--j)if(up[j][a]!=up[j][b]){a=up[j][a];b=up[j][b];}
    return up[0][a];
  };
  std::vector<Candidate> candidates;candidates.reserve(closure.size());
  for(uint32_t k:closure){
    auto c=chains[k];uint32_t a=id.at(c.x0),b=id.at(c.x1),w=lca(a,b);
    candidates.push_back({uint64_t(c.length)+dist[a]+dist[b]-2*dist[w],k});
  }
  std::sort(candidates.begin(),candidates.end(),[](auto a,auto b){
    return a.length!=b.length?a.length<b.length:a.chain<b.chain;
  });
  if(candidates.size()!=37445)throw std::runtime_error("cycle rank changed");
  if(first>=candidates.size() || uint64_t(first)+top>candidates.size())throw std::runtime_error("cycle slice out of range");
  std::cout<<"{\"method\":\"minimum_chain_length_spanning_forest\",\"kernel_vertices\":"<<n
           <<",\"kernel_edges\":"<<chains.size()<<",\"cycle_rank\":"<<candidates.size()
           <<",\"first_cycle_rank\":"<<first
           <<",\"shortest_cycle_edges\":"<<candidates[0].length
           <<",\"median_cycle_edges\":"<<candidates[candidates.size()/2].length
           <<",\"top_cycles\":[";
  for(uint32_t i=0;i<top;++i){
    auto candidate=candidates[first+i];auto close=chains[candidate.chain];
    uint32_t a=id.at(close.x0),b=id.at(close.x1),w=lca(a,b);
    std::vector<uint32_t> chain_path{candidate.chain};
    uint32_t at=b;
    while(at!=w){chain_path.push_back(parent_edge[at]);at=up[0][at];}
    std::vector<uint32_t> tail;
    at=a;while(at!=w){tail.push_back(parent_edge[at]);at=up[0][at];}
    std::reverse(tail.begin(),tail.end());chain_path.insert(chain_path.end(),tail.begin(),tail.end());
    std::vector<uint32_t> edge_path,vertex_path{close.x0};
    uint32_t current=close.x0;
    for(uint32_t k:chain_path){
      auto c=chains[k];bool forward=current==c.x0;
      if(!forward&&current!=c.x1)throw std::runtime_error("chain path disconnected");
      for(uint32_t j=0;j<c.length;++j){
        uint32_t pos=forward?c.offset+j:c.offset+c.length-1-j;
        uint32_t edge_index=paths[pos];
        if(edge_index>=edges.size())throw std::runtime_error("path edge out of range");
        Edge e=edges[edge_index];
        current=current==e.x0?e.x1:current==e.x1?e.x0:UINT32_MAX;
        if(current==UINT32_MAX)throw std::runtime_error("edge path disconnected");
        edge_path.push_back(edge_index);vertex_path.push_back(current);
      }
    }
    if(current!=close.x0||edge_path.size()!=candidate.length)throw std::runtime_error("cycle does not close");
    if(i)std::cout<<',';
    std::cout<<"{\"closing_chain\":"<<candidate.chain<<",\"cycle_edges\":"<<candidate.length
             <<",\"kernel_edges\":"<<chain_path.size()<<",\"core_edge_indexes\":[";
    for(size_t j=0;j<edge_path.size();++j){if(j)std::cout<<',';std::cout<<edge_path[j];}
    std::cout<<"],\"vertex_x_path\":[";
    for(size_t j=0;j<vertex_path.size();++j){if(j)std::cout<<',';std::cout<<vertex_path[j];}
    std::cout<<"]}";
  }
  std::cout<<"]}\n";
}
