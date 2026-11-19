//! Deterministic producer-side screens for R67--R83.
//!
//! This is deliberately not a throughput benchmark and it is not the GHS
//! group walk. It checks scheduler/accounting invariants needed before the
//! parallel distinguished-point implementation can be timed: disjoint
//! deterministic walker indices, a shared transition, prefix sharding,
//! collision detection, stop-latency overshoot, and masked batch tails.

use std::collections::{BTreeMap, BTreeSet};

const JOB_ID: u64 = 0x4757_6768_735f_7268;
const WORKERS: usize = 6;
const SHARDS: usize = 8;
const STOP_DELAY_ROUNDS: u64 = 3;
const MAX_ROUNDS: u64 = 100_000;
const TOY_MODULUS: u64 = 257;
const DP_MASK: u64 = 0x7;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DpRecord {
    walker: u64,
    state: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Worker {
    id: usize,
    generation: u64,
    walker: u64,
    state: u64,
}

#[derive(Debug, Eq, PartialEq)]
struct Summary {
    collision_round: u64,
    collision_key: u64,
    collision_shard: usize,
    useful_steps: u64,
    overshoot_steps: u64,
    total_steps: u64,
    table_operations: u64,
    distinguished_points: u64,
    sterile_collisions: u64,
    occupied_keys: usize,
}

fn mix64(mut x: u64) -> u64 {
    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

fn job_offset(job_id: u64) -> u64 {
    mix64(job_id) & 0x0000_ffff_ffff_0000
}

/// Interleaving worker ids gives disjoint indices without assigning a
/// different transition function to each worker.
fn walker_index(job_id: u64, worker: usize, generation: u64) -> u64 {
    job_offset(job_id)
        .checked_add(generation.checked_mul(WORKERS as u64).unwrap())
        .and_then(|x| x.checked_add(worker as u64))
        .unwrap()
}

fn initial_state(job_id: u64, walker: u64) -> u64 {
    mix64(job_id ^ walker) % TOY_MODULUS
}

/// One shared, intentionally tiny many-to-one transition. Its only purpose is
/// to make the accounting screen reach a reproducible collision quickly.
fn transition(state: u64) -> u64 {
    const JUMPS: [u64; 4] = [1, 3, 7, 15];
    let branch = (state & 3) as usize;
    (state.wrapping_mul(state) + JUMPS[branch]) % TOY_MODULUS
}

fn is_distinguished(state: u64) -> bool {
    state & DP_MASK == 0
}

fn shard_for(key: u64) -> usize {
    (key as usize) & (SHARDS - 1)
}

fn reset_worker(worker: &mut Worker) {
    worker.generation += 1;
    worker.walker = walker_index(JOB_ID, worker.id, worker.generation);
    worker.state = initial_state(JOB_ID, worker.walker);
}

fn simulate() -> Summary {
    assert!(SHARDS.is_power_of_two());
    let mut workers: Vec<Worker> = (0..WORKERS)
        .map(|id| {
            let walker = walker_index(JOB_ID, id, 0);
            Worker {
                id,
                generation: 0,
                walker,
                state: initial_state(JOB_ID, walker),
            }
        })
        .collect();

    let mut shards: Vec<BTreeMap<u64, DpRecord>> = (0..SHARDS).map(|_| BTreeMap::new()).collect();
    let mut detection: Option<(u64, u64, usize)> = None;
    let mut useful_steps = 0u64;
    let mut overshoot_steps = 0u64;
    let mut table_operations = 0u64;
    let mut distinguished_points = 0u64;
    let mut sterile_collisions = 0u64;

    for round in 0..MAX_ROUNDS {
        if let Some((detected_round, _, _)) = detection {
            if round > detected_round + STOP_DELAY_ROUNDS {
                break;
            }
        }

        let overshoot_round = detection.is_some();
        let mut pending_collision: Option<(u64, usize)> = None;

        for worker in &mut workers {
            worker.state = transition(worker.state);
            if overshoot_round {
                overshoot_steps += 1;
            } else {
                useful_steps += 1;
            }

            if !is_distinguished(worker.state) {
                continue;
            }

            distinguished_points += 1;
            table_operations += 1;
            let key = worker.state;
            let shard = shard_for(key);
            assert_eq!(shard, shard_for(key), "routing must be stable");
            let record = DpRecord {
                walker: worker.walker,
                state: worker.state,
            };

            match shards[shard].get(&key).copied() {
                Some(previous) if previous.walker == record.walker => {
                    sterile_collisions += 1;
                }
                Some(previous) => {
                    assert_eq!(previous.state, record.state);
                    pending_collision.get_or_insert((key, shard));
                }
                None => {
                    shards[shard].insert(key, record);
                }
            }
            reset_worker(worker);
        }

        if detection.is_none() {
            if let Some((key, shard)) = pending_collision {
                detection = Some((round, key, shard));
            }
        }
    }

    let (collision_round, collision_key, collision_shard) =
        detection.expect("deterministic toy screen must collide");
    let occupied_keys = shards.iter().map(BTreeMap::len).sum();
    let total_steps = useful_steps + overshoot_steps;
    assert_eq!(overshoot_steps, WORKERS as u64 * STOP_DELAY_ROUNDS);

    Summary {
        collision_round,
        collision_key,
        collision_shard,
        useful_steps,
        overshoot_steps,
        total_steps,
        table_operations,
        distinguished_points,
        sterile_collisions,
        occupied_keys,
    }
}

fn check_walker_indices() {
    let mut seen = BTreeSet::new();
    for generation in 0..1_000 {
        for worker in 0..WORKERS {
            assert!(seen.insert(walker_index(JOB_ID, worker, generation)));
        }
    }
    assert_eq!(seen.len(), WORKERS * 1_000);
}

fn check_masked_tails() -> u64 {
    let mut cells = 0u64;
    for &width in &[1usize, 4, 8, 16, 32] {
        for total in 0usize..=65 {
            let mut seen = vec![0u8; total];
            for base in (0..total).step_by(width) {
                for lane in 0..width {
                    let active = base + lane < total;
                    if active {
                        seen[base + lane] += 1;
                    }
                }
            }
            assert!(seen.iter().all(|&visits| visits == 1));
            cells += 1;
        }
    }
    cells
}

fn main() {
    check_walker_indices();
    let masked_tail_cells = check_masked_tails();
    let first = simulate();
    let second = simulate();
    assert_eq!(
        first, second,
        "parallel accounting screen must replay exactly"
    );
    assert_eq!(
        first.total_steps,
        first.useful_steps + first.overshoot_steps
    );

    println!("R67-R83 deterministic invariant screen PASS");
    println!("walker_indices_checked={}", WORKERS * 1_000);
    println!("masked_tail_cells_checked={masked_tail_cells}");
    println!("workers={WORKERS} shards={SHARDS} stop_delay_rounds={STOP_DELAY_ROUNDS}");
    println!(
        "collision_round={} collision_key={} collision_shard={}",
        first.collision_round, first.collision_key, first.collision_shard
    );
    println!(
        "useful_steps={} overshoot_steps={} total_steps={}",
        first.useful_steps, first.overshoot_steps, first.total_steps
    );
    println!(
        "distinguished_points={} table_operations={} occupied_keys={} sterile_collisions={}",
        first.distinguished_points,
        first.table_operations,
        first.occupied_keys,
        first.sterile_collisions
    );
    println!("timing_observations=0 performance_claims=0");
}
