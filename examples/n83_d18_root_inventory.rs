//! Independent native check of the pinned D18 representative bits.
#[path = "../research/n83_d18_shared_bits_20261020/root_inventory.rs"]
mod inventory;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    inventory::main()
}
