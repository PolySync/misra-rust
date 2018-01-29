#![deny(clippy)]
#[deny(warnings)]

macro_rules! engine_exhaust_gas_temperature_raw {
   () => (3;);
}

macro_rules! engine_exhaust_gas_temperature_scaled {
   () => (4;);
}

fn main() {
    let _ = engine_exhaust_gas_temperature_raw!();
    let _ = engine_exhaust_gas_temperature_scaled!();
}
