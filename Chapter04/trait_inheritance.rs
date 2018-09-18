// trait_inheritance.rs

trait Vehicle {
    fn get_price(&self) -> u64;
}

trait Car: Vehicle {
    fn model(&self) -> String;
}

struct TeslaRoadster {
    model: String,
    release_date: u16
}

impl Car for TeslaRoadster {
    fn model(&self) -> String {
        "Tesla Roadster I".to_string()
    }
}

impl Vehicle for TeslaRoadster {
    fn get_price(&self) -> u64 {
        200_000
    }
}

fn main() {
    let my_roadster = TeslaRoadster { model: "Tesla Roadster II".to_string(), release_date: 2020 };
    println!("{} is priced at ${}", my_roadster.model, my_roadster.get_price());
}
