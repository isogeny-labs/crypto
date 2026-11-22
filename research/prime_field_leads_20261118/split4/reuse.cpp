#include <bit>
#include <cstdint>
#include <limits>

constexpr std::size_t targets_per_cell=24;
constexpr std::size_t repetitions=3;

enum class Mode { point_hash, x_hash, packed_sorted, packed_buckets };
constexpr std::array mode_names{"point_hash","x_hash","packed_sorted","packed_buckets"};

std::int64_t elapsed_ns(std::chrono::steady_clock::time_point from) {
    return std::chrono::duration_cast<std::chrono::nanoseconds>(
        std::chrono::steady_clock::now()-from).count();
}

Word point_key(Point v) { return v.x*p+v.y; }

class Support {
    Mode mode;
    std::unordered_map<Word,Word> map;
    std::vector<std::uint32_t> packed;
    std::vector<std::uint32_t> starts;
    unsigned shift=0;

public:
    Support(const Bases& base, Mode kind): mode(kind) {
        bool point_keys=mode==Mode::point_hash;
        if (mode==Mode::point_hash || mode==Mode::x_hash)
            map.reserve((point_keys?4:2)*base[0].size()*base[1].size());
        else
            packed.reserve(2*base[0].size()*base[1].size());
        Row row;
        for (auto pt:base[0]) {
            row.roots(pt,base[1],point_keys,[&](Point q) {
                switch (mode) {
                    case Mode::point_hash:
                        ++map[point_key(q)];
                        ++map[point_key(neg(q))];
                        break;
                    case Mode::x_hash:
                        ++map[q.x];
                        break;
                    default:
                        packed.push_back(static_cast<std::uint32_t>(q.x));
                }
            });
        }

        if (mode==Mode::packed_sorted || mode==Mode::packed_buckets)
            std::sort(packed.begin(),packed.end());
        if (mode!=Mode::packed_buckets) return;
        unsigned bits=std::bit_width(std::max<std::size_t>(1, packed.size()/8))-1;
        shift=std::bit_width(p-1)-bits;
        starts.resize(((p-1)>>shift)+2);
        std::size_t i=0;
        for (std::size_t bucket=0;bucket<starts.size();++bucket) {
            starts[bucket]=static_cast<std::uint32_t>(i);
            while (i<packed.size() && (packed[i]>>shift)==bucket) ++i;
        }
        assert(i==packed.size());
    }

    Word match(Point pt, Word& comparisons) const {
        if (mode==Mode::point_hash || mode==Mode::x_hash) {
            auto it=map.find(mode==Mode::point_hash?point_key(pt):pt.x);
            return it==map.end()?0:it->second;
        }
        if (mode==Mode::packed_buckets) {
            auto bucket=pt.x>>shift;
            Word hits=0;
            for (auto i=starts[bucket];i<starts[bucket+1];++i) {
                ++comparisons;
                hits+=packed[i]==pt.x;
            }
            return hits;
        }
        auto first=std::lower_bound(packed.begin(),packed.end(),pt.x,[&](Word a, Word b) {
            ++comparisons;
            return a<b;
        });
        Word hits=0;
        for (;first!=packed.end();++first) {
            ++comparisons;
            if (*first!=pt.x) break;
            ++hits;
        }
        return hits;
    }

    bool needs_y() const { return mode==Mode::point_hash; }
    Word keys() const { return packed.empty()?map.size():packed.size(); }
    Word key_bytes() const {
        return packed.empty()?map.size()*sizeof(std::pair<const Word,Word>):
                              packed.capacity()*sizeof(std::uint32_t);
    }
    Word index_bytes() const {
        return packed.empty()?map.bucket_count()*sizeof(void*):
                              starts.capacity()*sizeof(std::uint32_t);
    }
};

std::vector<Point> prepare_right(const Bases& base) {
    std::vector<Point> right;
    right.reserve(2*base[2].size()*base[3].size());
    Row roots;
    for (auto pt:base[2])
        roots.roots(pt,base[3],true,[&](Point q){right.push_back(q);});
    return right;
}

std::array<Point,targets_per_cell> target_panel(const Bases& base) {
    constexpr std::array<Word,6> fixed{2,17,41,20261117,8675309,13337};
    std::array<Point,targets_per_cell> targets;
    targets[1]=add(base[2][0],base[3][0]);
    targets[0]=add(add(base[0][0],base[1][0]),targets[1]);
    assert(targets[0]!=infinity && targets[1]!=infinity);
    for (std::size_t i=0;i<fixed.size();++i) targets[i+2]=scalar(fixed[i]);
    Word state=0x9de2ed3b2dd8bfe1ULL;
    for (std::size_t i=2+fixed.size();i<targets.size();++i) {
        state^=state>>12;
        state^=state<<25;
        state^=state>>27;
        targets[i]=scalar(1+(state*2685821657736338717ULL)%(p-1));
    }
    return targets;
}

using Panel=std::array<Point,targets_per_cell>;
using Answers=std::array<Word,targets_per_cell>;

Answers independent_counts(const Bases& base, const Panel& targets) {
    std::unordered_map<Word,Word> full;
    for (Point a:base[0]) for (Point b:base[1])
        for (Point s:{a,neg(a)}) for (Point t:{b,neg(b)}) {
            auto value=add(s,t);
            assert(value!=infinity && value.y);
            ++full[point_key(value)];
        }
    Answers counts{};
    for (Point a:base[2]) for (Point b:base[3])
        for (Point s:{a,neg(a)}) for (Point t:{b,neg(b)}) {
            auto right=add(s,t);
            assert(right!=infinity);
            for (std::size_t target=0;target<targets.size();++target) {
                auto left=add(targets[target],neg(right));
                if (left==infinity) continue;
                auto it=full.find(point_key(left));
                if (it!=full.end()) {
                    assert(add(left,right)==targets[target]);
                    counts[target]+=it->second;
                }
            }
        }
    if (base[0].size()==8) {
        for (std::size_t i=0;i<targets.size();++i)
            assert(counts[i]==brute_small(base,targets[i]));
    }
    assert(counts[0]>0);
    return counts;
}

void row_csv(std::string_view phase,std::size_t b,std::string_view law,std::size_t repeat,
             std::string_view mode,int target,Point pt,Word hits,Word keys,Word bytes,
             Word index,Word lookups,Word comparisons,std::int64_t ns) {
    std::cout<<phase<<','<<b<<','<<law<<','<<repeat<<','<<mode<<','<<target<<','
             <<pt.x<<','<<pt.y<<','<<hits<<','<<keys<<','<<bytes<<','<<index<<','
             <<work.mults<<','<<work.inversions<<','<<work.root_visits<<','
             <<work.complement_visits<<','<<lookups<<','<<comparisons<<','<<ns<<'\n';
}

int main() {
    std::cout<<"phase,B,base,repeat,mode,target,rx,ry,signed_hits,left_keys,"
                "payload_bytes,index_bytes,field_mults,field_inversions,root_visits,"
                "complement_visits,lookups,lookup_key_comparisons,wall_ns\n";
    for (std::size_t b:{8,32,128,256}) for (bool random:{false,true}) {
        auto base=make_bases(b,random);
        validate_root_grid(base);
        const auto targets=target_panel(base);
        const auto oracle=independent_counts(base,targets);
        const std::string_view law=random?"random":"interval";
        for (std::size_t rep=0;rep<repetitions;++rep) {
            work={};
            auto from=std::chrono::steady_clock::now();
            auto right=prepare_right(base);
            auto ns=elapsed_ns(from);
            row_csv("right",b,law,rep,"all",-1,infinity,0,0,right.capacity()*sizeof(Point),
                    0,0,0,ns);
            for (std::size_t arm=0;arm<mode_names.size();++arm) {
                std::size_t kind=(arm+rep)%mode_names.size();
                work={};
                from=std::chrono::steady_clock::now();
                const Support table(base,static_cast<Mode>(kind));
                ns=elapsed_ns(from);
                row_csv("left",b,law,rep,mode_names[kind],-1,infinity,0,table.keys(),
                        table.key_bytes(),table.index_bytes(),0,0,ns);
                for (std::size_t target=0;target<targets.size();++target) {
                    Row shifts;
                    Word hits=0,lookups=0,comparisons=0;
                    work={};
                    from=std::chrono::steady_clock::now();
                    for (std::size_t row=0;row<b;++row)
                        shifts.shift(targets[target],std::span<const Point>(right).subspan(row*2*b,2*b),
                                     table.needs_y(),[&](Point q){
                            if (q==infinity) return;
                            ++lookups;
                            hits+=table.match(q,comparisons);
                        });
                    ns=elapsed_ns(from);
                    assert(hits==oracle[target]);
                    row_csv("query",b,law,rep,mode_names[kind],static_cast<int>(target),
                            targets[target],hits,table.keys(),table.key_bytes(),table.index_bytes(),
                            lookups,comparisons,ns);
                }
            }
        }
    }
}
