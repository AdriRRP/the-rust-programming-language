fn main() {
    println!("Hello, world!");
}

fn far2cel(val: f32, unit: String) -> f32 {
    if val == "farenheit" {
        (val - 32) / 1.8
    } else {
	(val * 1.8) + 32
    }
}
