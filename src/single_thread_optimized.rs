use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fs::File;
use std::io::{BufReader, Read};
use std::mem;

enum ParseState {
    CityName {
        city_name: Vec<u8>,
        temp_buf: VecDeque<u8>,
    },
    Temperature {
        city_name: Vec<u8>,
        temp_buf: VecDeque<u8>,
    },
}

impl ParseState {
    fn feed(&mut self, bytes: &mut VecDeque<u8>, data: &mut HashMap<Vec<u8>, TemperatureStats>) {
        'outer: loop {
            match self {
                ParseState::CityName {
                    city_name,
                    temp_buf,
                } => {
                    while let Some(b) = bytes.pop_front() {
                        if b == b';' {
                            *self = ParseState::Temperature {
                                city_name: mem::take(city_name),
                                temp_buf: mem::take(temp_buf),
                            };

                            continue 'outer;
                        } else {
                            city_name.push(b);
                        }
                    }
                    break 'outer;
                }
                ParseState::Temperature {
                    city_name,
                    temp_buf,
                } => {
                    while let Some(b) = bytes.pop_front() {
                        if b == b'\n' {
                            let result = parse_temp(&*temp_buf);
                            match data.get_mut(city_name) {
                                None => {
                                    data.insert(city_name.clone(), TemperatureStats::new(result));
                                }
                                Some(stats) => {
                                    stats.update(result);
                                }
                            }
                            city_name.clear();
                            temp_buf.clear();
                            *self = ParseState::CityName {
                                city_name: mem::take(city_name),
                                temp_buf: mem::take(temp_buf),
                            };

                            continue 'outer;
                        } else {
                            temp_buf.push_back(b);
                        }
                    }
                    break 'outer;
                }
            }
        }
    }
}

fn parse_temp(buf: &VecDeque<u8>) -> isize {
    let mut result: isize = 0;
    let mut negative = false;
    for &b in buf {
        match b {
            b'-' => negative = true,
            b'0'..=b'9' => result = result * 10 + (b - b'0') as isize,
            _ => {}
        }
    }
    if negative { -result } else { result }
}

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
    let mut f = BufReader::new(File::open(file_path).unwrap());

    let mut data: HashMap<Vec<u8>, TemperatureStats> = HashMap::new();

    let mut current_buf = VecDeque::with_capacity(8192);
    let mut buf = [0; 4096];

    let mut state = ParseState::CityName {
        city_name: Vec::with_capacity(128),
        temp_buf: VecDeque::with_capacity(16),
    };

    while let Ok(n) = f.read(&mut buf[..]) {
        current_buf.extend(buf[..n].iter().cloned());
        state.feed(&mut current_buf, &mut data);
        if n == 0 {
            break;
        }
    }

    let sorted_by_city: BTreeMap<_, _> = data
        .into_iter()
        .map(|(k, v)| {
            let key = unsafe { String::from_utf8_unchecked(k) };
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
