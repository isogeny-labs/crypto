#pragma once
#include "../native/incidence.hpp"
#include <memory>

namespace paper_ec::accelerated {
enum class Generator { LegacyAffine, Straus, BatchedWindow };
enum class Norm { Composition, StreamedPairs, StreamedPairsBloom };
struct GenerationCounts { Word draws=0, rejected_infinity=0, rejected_coordinate=0; };
struct FastCounts {
    Word batch_inversions=0, exceptional_doubling_inversions=0, ordinary_additions=0,
         table_entries=0, combinations=0, pair_coordinates=0, exact_lookups=0, uninformative=0;
};
// Each context belongs to one public target. Construction/subgroup checks and
// fixed-window table creation must be charged; never amortize across targets.
class FastContext {
public:
    explicit FastContext(const LabelledPools&, bool prepare_window=false);
    ~FastContext();
    FastContext(const FastContext&)=delete;
    FastContext& operator=(const FastContext&)=delete;
    std::vector<Point> combine(const std::vector<std::array<Word,2>>&,bool batch);
    IncidenceAnswer pairs(const LabelledPools&,bool bloom);
    // Does not return a GCD: traversal stops at the first informative witness.
    IncidenceAnswer first_witness(const LabelledPools&,bool bloom);
    FastCounts counts() const;
private:
    friend LabelledPools generate(LabelledPools,std::size_t,Word,Generator,GenerationCounts&,FastContext*);
    IncidenceAnswer pair_search(const LabelledPools&,bool bloom,bool complete);
    void* context_=nullptr;
    Curve curve_;
    Word order_;
    Point generator_,target_;
    void require_matching(const LabelledPools&) const;
};
LabelledPools generate(LabelledPools public_input,std::size_t B,Word seed,
                       Generator,GenerationCounts&,FastContext*);
// Validates public inputs, points and global coordinate distinctness; labels
// are not recomputed individually (same contract as the frozen native solver).
void validate(const LabelledPools&);
Coefficients composition_gcd(const LabelledPools&,Stats* = nullptr);
IncidenceAnswer recover(const LabelledPools&,const Coefficients&,Stats* = nullptr);
IncidenceAnswer solve(const LabelledPools&,Norm,FastContext*,Stats* = nullptr);
} // namespace paper_ec::accelerated
