// Inspect the exact 2-core and its degree-2 suppressed kernel.
#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <unordered_map>
#include <vector>

struct Edge { uint32_t ordinal, x0, x1; };
static_assert(sizeof(Edge) == 12);

int main(int argc, char** argv) {
  if (argc != 2) throw std::runtime_error("usage: core_topology core_edges.bin");
  std::ifstream in(argv[1], std::ios::binary | std::ios::ate);
  if (!in || in.tellg() % sizeof(Edge)) throw std::runtime_error("bad core file");
  const size_t m = size_t(in.tellg()) / sizeof(Edge);
  in.seekg(0);
  std::vector<Edge> edges(m);
  if (!in.read(reinterpret_cast<char*>(edges.data()), m * sizeof(Edge))) throw std::runtime_error("short core file");
  std::vector<uint32_t> labels;
  labels.reserve(2*m);
  for (const auto& e : edges) { labels.push_back(e.x0); labels.push_back(e.x1); }
  std::sort(labels.begin(), labels.end());
  labels.erase(std::unique(labels.begin(), labels.end()), labels.end());
  const uint32_t n = labels.size();
  std::unordered_map<uint32_t,uint32_t> id;
  id.reserve(n);
  for (uint32_t i=0;i<n;++i) id.emplace(labels[i],i);
  std::vector<uint32_t> degree(n), parent(n), size(n,1);
  for (uint32_t i=0;i<n;++i) parent[i]=i;
  auto root=[&](uint32_t v){while(parent[v]!=v){parent[v]=parent[parent[v]];v=parent[v];}return v;};
  uint64_t components=n, loops=0, multiedge_pairs=0;
  uint32_t previous_ordinal=0;
  for (size_t i=0;i<m;++i) {
    auto& e=edges[i];
    if (i && e.ordinal<=previous_ordinal) throw std::runtime_error("core ordinals unsorted");
    previous_ordinal=e.ordinal;
    uint32_t a=id.at(e.x0),b=id.at(e.x1);
    ++degree[a];++degree[b];
    if(a==b){++loops;continue;}
    a=root(a);b=root(b);
    if(a!=b){if(size[a]<size[b])std::swap(a,b);parent[b]=a;size[a]+=size[b];--components;}
  }
  uint64_t deg2=0,deg3plus=0,degree_excess=0,core_components_with_cycles=0;
  uint32_t maximum_degree=0;
  for(uint32_t i=0;i<n;++i){
    if(degree[i]<2)throw std::runtime_error("not a 2-core");
    deg2+=degree[i]==2;deg3plus+=degree[i]>=3;
    degree_excess+=degree[i]-2;
    maximum_degree=std::max(maximum_degree,degree[i]);
  }
  std::cout<<"{\"core_vertices\":"<<n<<",\"core_edges\":"<<m
           <<",\"components\":"<<components<<",\"cycle_rank\":"<<m-n+components
           <<",\"degree_two_vertices\":"<<deg2<<",\"kernel_vertices\":"<<deg3plus
           <<",\"degree_excess\":"<<degree_excess<<",\"max_degree\":"<<maximum_degree
           <<",\"loops\":"<<loops<<"}\n";
}
