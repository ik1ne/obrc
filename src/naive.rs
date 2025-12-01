use std::collections::BTreeMap;

struct TemperatureStats {
    total: f64,
    count: usize,
    max: f64,
    min: f64,
}

impl TemperatureStats {
    fn new() -> Self {
        TemperatureStats {
            total: 0.0,
            count: 0,
            max: f64::MIN,
            min: f64::MAX,
        }
    }

    fn update(&mut self, temp: f64) {
        self.total += temp;
        self.count += 1;
        if temp > self.max {
            self.max = temp;
        }
        if temp < self.min {
            self.min = temp;
        }
    }
}

pub fn run(file_path: &str) {
    let s = std::fs::read_to_string(file_path).expect("Failed to read file");
    let mut total: BTreeMap<String, TemperatureStats> = BTreeMap::new();

    for line in s.lines() {
        let mut parts = line.split(";");
        let name = parts.next().unwrap();
        let temp: f64 = parts.next().unwrap().parse().unwrap();

        total
            .entry(name.to_string())
            .or_insert_with(TemperatureStats::new)
            .update(temp);
    }

    let output = total
        .iter()
        .map(|(name, stats)| {
            let average = stats.total / stats.count as f64;
            format!("{};{:.2};{:.2};{:.2}", name, average, stats.max, stats.min)
        })
        .collect::<Vec<String>>()
        .join(", ");

    println!("{{{}}}", output);
}
