// Reuse the full, already-qualified cold boundary and all reference arms.
// Only the block planner/program/executor implementation is substituted.
// Include both headers before the local substitution so frozen declarations
// are not rewritten. No frozen source file is edited.
#include "block_compact.hpp"
namespace cmcompact {using Metrics=cmblock::Metrics;}
#define cmblock cmcompact
#include "block_eval.cpp"
#undef cmblock
