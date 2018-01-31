//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

macro_rules! engine_exhaust_gas_temperature_raw {
   () => (3;);
}

macro_rules! engine_exhaust_gas_temperature_scaled {
//~^ ERROR Non-compliant - variable name shadows engine_exhaust_gas_temperature_raw
   () => (4;);
}

fn main() {
    let _ = engine_exhaust_gas_temperature_raw!();
    let _ = engine_exhaust_gas_temperature_scaled!();
}
