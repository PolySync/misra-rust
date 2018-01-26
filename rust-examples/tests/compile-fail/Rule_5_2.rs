#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let engine_exhaust_gas_temperature_raw: i32 = 0;
    let engine_exhaust_gas_temperature_scaled: i32 = 1;

    println!("{} - {}",
    engine_exhaust_gas_temperature_raw,
    engine_exhaust_gas_temperature_scaled);
}
