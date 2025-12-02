use memmap::MmapOptions;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::str;

/// total/max/min == actual temp multiplied by 10
struct TemperatureStats {
    total: isize,
    count: usize,
    max: isize,
    min: isize,
}

impl TemperatureStats {
    fn new(temp: isize) -> Self {
        TemperatureStats {
            total: temp,
            count: 1,
            max: temp,
            min: temp,
        }
    }

    fn update(&mut self, temp: isize) {
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
    let f = unsafe { MmapOptions::new().map(&File::open(file_path).unwrap()) }.unwrap();

    let mut data: HashMap<&[u8], TemperatureStats> = HashMap::new();

    for line in f.split(|&b| b == b'\n') {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.splitn(2, |&b| b == b';');
        let city = parts.next().unwrap();
        let temp_str = parts.next().unwrap();
        let temp: isize = parse_temp(temp_str);
        data.entry(city)
            .and_modify(|stats| stats.update(temp))
            .or_insert_with(|| TemperatureStats::new(temp));
    }

    let sorted_by_city: BTreeMap<_, _> = data
        .into_iter()
        .map(|(k, v)| {
            let key = unsafe { str::from_utf8_unchecked(k) };
            (key, v)
        })
        .collect();

    print!("{{");
    let mut first = true;
    for (city, stats) in sorted_by_city {
        if !first {
            print!(", ");
        }
        first = false;
        let avg = stats.total as f64 / stats.count as f64 / 10.0;
        let max = stats.max as f64 / 10.0;
        let min = stats.min as f64 / 10.0;
        print!("{}={:.1}/{:.1}/{:.1}", city, avg, max, min);
    }
    println!("}}");
}

fn parse_temp(s: &[u8]) -> isize {
    // we leverage the fact the temperatures are always between -99.9 and 99.9
    let negative = s[0] == b'-';
    let mut temp: isize = 0;
    for &b in s.iter().filter(|&&b| b != b'.' && b != b'-') {
        temp = temp * 10 + (b - b'0') as isize;
    }
    if negative { -temp } else { temp }
}
