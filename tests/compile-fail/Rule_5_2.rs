#[allow(unused_variables)]

fn main() {
    let engine_exhaust_gas_temperature_raw: i32 = 0;
    let engine_exhaust_gas_temperature_scaled: i32 = 1;
    //~^ ERROR Non-compliant - variable name shadows engine_exhaust_gas_temperature_raw
}
