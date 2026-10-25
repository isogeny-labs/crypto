// Metadata registration delegates to Rust: legacy JSON integers can exceed 2^53.
// The original v1 implementation remains in the frozen pre-measurement commits.
import {spawnSync} from 'node:child_process';
const result=spawnSync('cargo',['run','--release','--locked','--example',
  'conductor_gap_registry','--',...process.argv.slice(2)],{stdio:'inherit'});
if(result.error)throw result.error;
process.exit(result.status??1);
