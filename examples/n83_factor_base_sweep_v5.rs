//! Native, constant-space catalog of the missing N83 symmetry combinations.
//!
//! This is an addressable design, not a relation producer or runtime result.

use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const STUDY: &str = "research/koblitz_n83_factor_base_sweep_20261008";
const V2_SHA256: &str = "7526b6218dfe0b074368901f3869ddf4a978cb0bacffe8654056056700f8a972";
const V3_SHA256: &str = "f7b860c9a652154541c4e6cf7d52ad13bd5a7436a41d38855d206cde36d4b0ac";
const V4_SHA256: &str = "8e512b8dbc6577093f8e31f4afb625894cbdedd9f9f3e87dc4f36a7df52344c2";
const V2_COUNT: u64 = 57_024_000;
const V3_COUNT: u64 = 518_918_400;
const V4_COUNT: u64 = 26_939_278_080;
const BASE_COUNT: u64 = 1_320;
const NESTED_PAIR_COUNT: u64 = 2_970;
const CROSS_PAIR_COUNT: u64 = 169_884;
const NON_SYMMETRY_SOLVERS: u64 = 8_640;
const RESIDUAL_SHAPES: u64 = 3;
const COMPONENTS: [&str; 5] = [
    "ordered_summands",
    "signed_frobenius",
    "two_torsion",
    "four_torsion",
    "ordered_first_pair",
];
const AXES: [&str; 8] = [
    "summands",
    "split_bits",
    "symmetry",
    "backend",
    "enumeration",
    "max_large_primes",
    "threads",
    "linear_algebra",
];
const SIZES: [u64; 11] = [32, 64, 128, 256, 600, 900, 1182, 2048, 4096, 8192, 16627];

struct Parents {
    v2: Value,
    v3: Value,
    v4: Value,
}

fn invalid(message: impl Into<String>) -> Box<dyn Error> {
    std::io::Error::new(std::io::ErrorKind::InvalidData, message.into()).into()
}

fn number(value: &Value, name: &str) -> Result<u64> {
    value
        .as_u64()
        .ok_or_else(|| invalid(format!("missing or invalid {name}")))
}

fn array<'a>(value: &'a Value, name: &str) -> Result<&'a [Value]> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| invalid(format!("missing or invalid {name}")))
}

fn sha256(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn read_pinned(path: &Path, expected_hash: &str) -> Result<Value> {
    let bytes = fs::read(path)?;
    let actual = sha256(&bytes);
    if actual != expected_hash {
        return Err(invalid(format!("{} changed: {actual}", path.display())));
    }
    Ok(serde_json::from_slice(&bytes)?)
}

fn load_parents(root: &Path) -> Result<Parents> {
    let directory = root.join(STUDY);
    let v2 = read_pinned(&directory.join("size-frontier-v2.json"), V2_SHA256)?;
    let v3 = read_pinned(&directory.join("paired-frontier-v3.json"), V3_SHA256)?;
    let v4 = read_pinned(&directory.join("paired-frontier-v4.json"), V4_SHA256)?;
    for (value, schema, count) in [
        (&v2, "n83.factor-base-design/v2-size-frontier", V2_COUNT),
        (&v3, "n83.factor-base-design/v3-paired-frontier", V3_COUNT),
        (
            &v4,
            "n83.factor-base-design/v4-cross-stream-frontier",
            V4_COUNT,
        ),
    ] {
        if value["schema"] != schema || number(&value["total_tuple_count"], "tuple count")? != count
        {
            return Err(invalid(format!("frozen {schema} dimensions changed")));
        }
    }
    if number(&v2["base_spec_count"], "base count")? != BASE_COUNT
        || array(&v2["base_specs"], "base specs")?.len() as u64 != BASE_COUNT
        || number(&v2["solver_tuple_count"], "v2 solvers")? != 43_200
        || number(&v3["pair_spec_count"], "nested pairs")? != NESTED_PAIR_COUNT
        || array(&v3["pair_specs"], "pair specs")?.len() as u64 != NESTED_PAIR_COUNT
        || number(&v3["solver_tuple_count"], "v3 solvers")? != 51_840
        || number(&v4["cross_stream_pair_count"], "cross pairs")? != CROSS_PAIR_COUNT
        || number(&v4["solver_tuple_count"], "v4 solvers")? != 51_840
        || array(&v4["streams"], "streams")?.len() != 54
    {
        return Err(invalid("frozen N83 parent dimensions changed"));
    }
    let v2_symmetry = array(&v2["solver_axes"]["symmetry"], "v2 symmetries")?;
    let v3_symmetry = array(&v3["solver_axes"]["symmetry"], "v3 symmetries")?;
    if v2_symmetry.len() != 5
        || v3_symmetry.len() != 6
        || v2_symmetry[0] != "none"
        || v3_symmetry[0] != "none"
        || COMPONENTS
            .iter()
            .enumerate()
            .any(|(i, component)| v3_symmetry[i + 1] != *component)
    {
        return Err(invalid("frozen symmetry axis changed"));
    }
    Ok(Parents { v2, v3, v4 })
}

fn masks(single_base: bool) -> Vec<u8> {
    (1..32)
        .filter(|mask: &u8| mask.count_ones() >= 2 || (single_base && *mask == 16))
        .collect()
}

fn components(mask: u8) -> Vec<&'static str> {
    COMPONENTS
        .iter()
        .enumerate()
        .filter_map(|(bit, name)| (mask & (1 << bit) != 0).then_some(*name))
        .collect()
}

fn count(specs: u64, residuals: u64, mask_count: u64) -> Result<u64> {
    specs
        .checked_mul(residuals)
        .and_then(|x| x.checked_mul(mask_count))
        .and_then(|x| x.checked_mul(NON_SYMMETRY_SOLVERS))
        .ok_or_else(|| invalid("v5 tuple count overflow"))
}

fn design(parents: &Parents) -> Result<Value> {
    let single_masks = masks(true);
    let paired_masks = masks(false);
    if single_masks.len() != 27 || paired_masks.len() != 26 {
        return Err(invalid("symmetry subset coverage changed"));
    }
    let mut axes = Map::new();
    let mut radices = Vec::new();
    for name in AXES.into_iter().filter(|name| *name != "symmetry") {
        let values = array(&parents.v3["solver_axes"][name], name)?;
        radices.push(values.len() as u64);
        axes.insert(name.to_owned(), Value::Array(values.to_vec()));
    }
    if radices != [5, 4, 8, 2, 3, 3, 3] || radices.iter().product::<u64>() != NON_SYMMETRY_SOLVERS {
        return Err(invalid("non-symmetry solver axis changed"));
    }
    let single = count(BASE_COUNT, 1, single_masks.len() as u64)?;
    let nested = count(
        NESTED_PAIR_COUNT,
        RESIDUAL_SHAPES,
        paired_masks.len() as u64,
    )?;
    let cross = count(CROSS_PAIR_COUNT, RESIDUAL_SHAPES, paired_masks.len() as u64)?;
    let total = V4_COUNT
        .checked_add(single)
        .and_then(|x| x.checked_add(nested))
        .and_then(|x| x.checked_add(cross))
        .ok_or_else(|| invalid("v5 total overflow"))?;
    Ok(json!({
        "schema": "n83.factor-base-design/v5-symmetry-subsets",
        "study": parents.v4["study"],
        "parent_v2_sha256": V2_SHA256,
        "parent_v3_sha256": V3_SHA256,
        "parent_v4_sha256": V4_SHA256,
        "v4_tuple_count_preserved": V4_COUNT,
        "symmetry_components": COMPONENTS,
        "single_base_new_masks": single_masks,
        "paired_base_new_masks": paired_masks,
        "mask_semantics": "bit i selects symmetry_components[i]; multi-policy execution requires a verified composition adapter",
        "non_symmetry_solver_axis_order": AXES.into_iter().filter(|name| *name != "symmetry").collect::<Vec<_>>(),
        "non_symmetry_solver_axes": axes,
        "non_symmetry_solver_radices": radices,
        "non_symmetry_solver_tuple_count": NON_SYMMETRY_SOLVERS,
        "single_base_spec_count": BASE_COUNT,
        "nested_pair_spec_count": NESTED_PAIR_COUNT,
        "cross_stream_pair_spec_count": CROSS_PAIR_COUNT,
        "residual_shape_count_for_pairs": RESIDUAL_SHAPES,
        "new_single_base_tuple_count": single,
        "new_nested_pair_tuple_count": nested,
        "new_cross_stream_pair_tuple_count": cross,
        "new_tuple_count": single + nested + cross,
        "total_tuple_count": total,
        "execution_status": "catalog_only_composition_adapters_unavailable",
        "selected_best_total_runtime": null
    }))
}

fn solver(axes: &Value, ordinal: u64, include_symmetry: bool) -> Result<Value> {
    let names: Vec<&str> = AXES
        .into_iter()
        .filter(|name| include_symmetry || *name != "symmetry")
        .collect();
    let mut remaining = ordinal;
    let mut selected = Map::new();
    for name in names.into_iter().rev() {
        let values = array(&axes[name], name)?;
        let index = (remaining % values.len() as u64) as usize;
        remaining /= values.len() as u64;
        selected.insert(name.to_owned(), values[index].clone());
    }
    if remaining != 0 {
        return Err(invalid("solver index outside frozen radix product"));
    }
    Ok(Value::Object(selected))
}

fn cross_pair(parents: &Parents, index: u64) -> Result<Value> {
    let pairs_per_arm = 27 * 26 * 11 * 11;
    let arm = index / pairs_per_arm;
    let mut local = index % pairs_per_arm;
    let source_stream = local / (26 * 11 * 11);
    local %= 26 * 11 * 11;
    let other_slot = local / (11 * 11);
    local %= 11 * 11;
    let pool_stream = other_slot + u64::from(other_slot >= source_stream);
    let source_size = (local / 11) as usize;
    let pool_size = (local % 11) as usize;
    let streams = array(&parents.v4["streams"], "streams")?;
    let source = &streams[(arm * 27 + source_stream) as usize];
    let pool = &streams[(arm * 27 + pool_stream) as usize];
    let source_k = SIZES[source_size];
    let pool_k = SIZES[pool_size];
    let source_index = &source["base_indices"][source_k.to_string()];
    let pool_index = &pool["base_indices"][pool_k.to_string()];
    if source_index.is_null() || pool_index.is_null() {
        return Err(invalid("cross-stream base index missing"));
    }
    Ok(json!({
        "curve_a": arm,
        "base": {"policy": source["policy"], "seed": source["seed"],
                 "closure": source["closure"], "orbits": source_k,
                 "parent_base_index": source_index},
        "pool": {"policy": pool["policy"], "seed": pool["seed"],
                 "closure": pool["closure"], "orbits": pool_k,
                 "parent_base_index": pool_index}
    }))
}

fn case(parents: &Parents, design: &Value, ordinal: u64) -> Result<Value> {
    let total = number(&design["total_tuple_count"], "v5 total")?;
    if ordinal >= total {
        return Err(invalid("case ordinal outside v5 grid"));
    }
    let (region, source, residual, symmetry_mask, choices, disposition) = if ordinal < V2_COUNT {
        let base_index = ordinal / 43_200;
        let solver_index = ordinal % 43_200;
        let base = array(&parents.v2["base_specs"], "base specs")?[base_index as usize].clone();
        let choices = solver(&parents.v2["solver_axes"], solver_index, true)?;
        (
            "v2_parent",
            json!({"base_index": base_index, "base": base}),
            None,
            None,
            choices,
            "refer_to_frozen_v2_disposition",
        )
    } else if ordinal < V3_COUNT {
        let local = ordinal - V2_COUNT;
        let pair_index = local / (RESIDUAL_SHAPES * 51_840);
        let residual = (local / 51_840) % RESIDUAL_SHAPES;
        let solver_index = local % 51_840;
        let pair = array(&parents.v3["pair_specs"], "pair specs")?[pair_index as usize].clone();
        let choices = solver(&parents.v3["solver_axes"], solver_index, true)?;
        (
            "v3_parent",
            json!({"pair_index": pair_index, "pair": pair}),
            Some(residual),
            None,
            choices,
            "refer_to_frozen_v3_disposition",
        )
    } else if ordinal < V4_COUNT {
        let local = ordinal - V3_COUNT;
        let pair_index = local / (RESIDUAL_SHAPES * 51_840);
        let residual = (local / 51_840) % RESIDUAL_SHAPES;
        let solver_index = local % 51_840;
        let pair = cross_pair(parents, pair_index)?;
        let choices = solver(&parents.v3["solver_axes"], solver_index, true)?;
        (
            "v4_parent",
            json!({"pair_index": pair_index, "pair": pair}),
            Some(residual),
            None,
            choices,
            "refer_to_frozen_v4_disposition",
        )
    } else {
        let mut local = ordinal - V4_COUNT;
        let single = number(&design["new_single_base_tuple_count"], "single count")?;
        let nested = number(&design["new_nested_pair_tuple_count"], "nested count")?;
        let single_masks = masks(true);
        let paired_masks = masks(false);
        let (region, source, residual, mask) = if local < single {
            let per_base = single_masks.len() as u64 * NON_SYMMETRY_SOLVERS;
            let base_index = local / per_base;
            let mask_index = (local % per_base) / NON_SYMMETRY_SOLVERS;
            let base = array(&parents.v2["base_specs"], "base specs")?[base_index as usize].clone();
            (
                "v5_single_base",
                json!({"base_index": base_index, "base": base}),
                None,
                single_masks[mask_index as usize],
            )
        } else if local - single < nested {
            local -= single;
            let per_pair = RESIDUAL_SHAPES * paired_masks.len() as u64 * NON_SYMMETRY_SOLVERS;
            let pair_index = local / per_pair;
            let residual = (local % per_pair) / (paired_masks.len() as u64 * NON_SYMMETRY_SOLVERS);
            let mask_index = (local / NON_SYMMETRY_SOLVERS) % paired_masks.len() as u64;
            let pair = array(&parents.v3["pair_specs"], "pair specs")?[pair_index as usize].clone();
            (
                "v5_nested_pair",
                json!({"pair_index": pair_index, "pair": pair}),
                Some(residual),
                paired_masks[mask_index as usize],
            )
        } else {
            local -= single + nested;
            let per_pair = RESIDUAL_SHAPES * paired_masks.len() as u64 * NON_SYMMETRY_SOLVERS;
            let pair_index = local / per_pair;
            let residual = (local % per_pair) / (paired_masks.len() as u64 * NON_SYMMETRY_SOLVERS);
            let mask_index = (local / NON_SYMMETRY_SOLVERS) % paired_masks.len() as u64;
            (
                "v5_cross_stream_pair",
                json!({"pair_index": pair_index,
                "pair": cross_pair(parents, pair_index)?}),
                Some(residual),
                paired_masks[mask_index as usize],
            )
        };
        let solver_index = local % NON_SYMMETRY_SOLVERS;
        let mut choices = solver(&parents.v3["solver_axes"], solver_index, false)?;
        choices["symmetry"] = json!(components(mask));
        let disposition = if mask == 16 {
            "single_base_first_pair_adapter_unavailable"
        } else {
            "composed_symmetry_adapter_unavailable"
        };
        (region, source, residual, Some(mask), choices, disposition)
    };
    Ok(json!({
        "schema": "n83.factor-base-case/v5-symmetry-subsets",
        "ordinal": ordinal,
        "region": region,
        "source": source,
        "residual_summands": residual,
        "symmetry_mask": symmetry_mask,
        "solver": choices,
        "disposition": disposition,
        "executed_complete_pipeline": false,
        "selected_best_total_runtime": null
    }))
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn saved_path(root: &Path) -> PathBuf {
    root.join(STUDY).join("paired-frontier-v5.json")
}

fn main() -> Result<()> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let root = root();
    let parents = load_parents(&root)?;
    let design = design(&parents)?;
    let mut bytes = serde_json::to_vec_pretty(&design)?;
    bytes.push(b'\n');
    let saved = saved_path(&root);
    match arguments.as_slice() {
        [flag] if flag == "--write" => {
            if saved.exists() && fs::read(&saved)? != bytes {
                return Err(invalid("refusing to replace changed v5 design"));
            }
            if !saved.exists() {
                fs::write(&saved, &bytes)?;
            }
        }
        [flag] if flag == "--check" => {
            if fs::read(&saved)? != bytes {
                return Err(invalid("saved v5 design differs from native generator"));
            }
        }
        [flag, index] if flag == "--case" => {
            if fs::read(&saved)? != bytes {
                return Err(invalid("saved v5 design differs from native generator"));
            }
            let ordinal: u64 = index.parse()?;
            println!(
                "{}",
                serde_json::to_string_pretty(&case(&parents, &design, ordinal)?)?
            );
            return Ok(());
        }
        _ => return Err(invalid("usage: --write | --check | --case ORDINAL")),
    }
    println!(
        "{}",
        json!({"status": "PASS", "sha256": sha256(&bytes),
        "total_tuple_count": design["total_tuple_count"]})
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masks_partition_the_frozen_symmetry_axis() {
        let single = masks(true);
        let paired = masks(false);
        assert_eq!(single.len(), 27);
        assert_eq!(paired.len(), 26);
        assert_eq!(
            single
                .iter()
                .filter(|mask| mask.count_ones() == 1)
                .copied()
                .collect::<Vec<_>>(),
            [16]
        );
        assert!(paired.iter().all(|mask| mask.count_ones() >= 2));
        assert!(single.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn exact_counts_and_region_boundaries() -> Result<()> {
        let parents = load_parents(&root())?;
        let design = design(&parents)?;
        assert_eq!(design["new_single_base_tuple_count"], 307_929_600u64);
        assert_eq!(design["new_nested_pair_tuple_count"], 2_001_542_400u64);
        assert_eq!(
            design["new_cross_stream_pair_tuple_count"],
            114_488_225_280u64
        );
        assert_eq!(design["total_tuple_count"], 143_736_975_360u64);
        for (ordinal, region) in [
            (0, "v2_parent"),
            (V2_COUNT - 1, "v2_parent"),
            (V2_COUNT, "v3_parent"),
            (V3_COUNT, "v4_parent"),
            (V4_COUNT - 1, "v4_parent"),
            (V4_COUNT, "v5_single_base"),
            (V4_COUNT + 307_929_600, "v5_nested_pair"),
            (
                V4_COUNT + 307_929_600 + 2_001_542_400,
                "v5_cross_stream_pair",
            ),
            (143_736_975_359, "v5_cross_stream_pair"),
        ] {
            assert_eq!(case(&parents, &design, ordinal)?["region"], region);
        }
        let first_added = case(&parents, &design, V4_COUNT)?;
        assert_eq!(first_added["source"]["base_index"], 0);
        assert_eq!(first_added["symmetry_mask"], 3);
        assert_eq!(first_added["solver"]["enumeration"], "binary");
        let last = case(&parents, &design, 143_736_975_359)?;
        assert_eq!(last["source"]["pair_index"], CROSS_PAIR_COUNT - 1);
        assert_eq!(last["residual_summands"], 2);
        assert_eq!(last["symmetry_mask"], 31);
        assert_eq!(last["solver"]["enumeration"], "gray");
        assert_eq!(last["solver"]["max_large_primes"], 2);
        assert!(case(&parents, &design, 143_736_975_360).is_err());
        Ok(())
    }
}
