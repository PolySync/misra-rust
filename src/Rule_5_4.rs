macro_rules! engine_exhaust_gas_temperature_raw {
   () => (3;);
}

macro_rules! engine_exhaust_gas_temperature_scaled {
   () => (4;);
}

fn main() {
    println!(
        "This program uses macro identifiers that are not MISRA-C \
        distinct. {} -- {}",
        engine_exhaust_gas_temperature_raw!(),
        engine_exhaust_gas_temperature_scaled!());

}
