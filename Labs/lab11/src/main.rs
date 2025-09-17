// //Lab 1: Creating Custom Iterators
// struct Fibonacci {
//     current: u64,
//     next: u64
// }

// impl Fibonacci {
//     fn new() -> Fibonacci {
//         Fibonacci { current: 0, next: 1}
//     }
// }

// impl Iterator for Fibonacci {
//     type Item = u64;

//     fn next(&mut self) -> Option<Self::Item> {
//         let current = self.current;

//         self.current = self.next;
//         self.next = current + self.next;
//         Some(current)
//     }
// }

// fn main() {
//     let fib = Fibonacci::new();
//     for (i, val) in fib.take(10).enumerate() {
//         println!("Fibonacci {}: {}", i + 1, val)
//     }
// }

//Lab 2: Weather Data Analysis
#[derive(Debug)]
struct WeatherData {
    temperature: f64,
    humidity: u32,
    rain: bool
}

fn warmest_period(data: &[WeatherData]) -> &[WeatherData] {
    data.windows(3)
        .max_by(|a, b| {let avg_a = (a[0].temperature + a[1].temperature + a[2].temperature) / 3.0;
    let avg_b = (b[0].temperature + b[1].temperature + b[2].temperature) / 3.0;
    avg_a.partial_cmp(&avg_b).unwrap()})
        .unwrap_or(&[])
}

fn coldest_day(data: &[WeatherData]) -> usize {
    let mut coldest_index = 0;
    let mut coldest_temp = data[0].temperature;

    for (i, day) in data.iter().enumerate(){
        if day.temperature < coldest_temp{
            coldest_temp = day.temperature;
            coldest_index = i;
        }
    }
    
    coldest_index
}

fn count_rainy_days(data: &[WeatherData]) -> usize {
    let mut raining_days = 0;
    for i in data.iter(){
        if i.rain == true{
            raining_days += 1
        }
    }

    raining_days
}

fn predict_rain(day_data: &WeatherData) -> bool {
    let mut will_rain = false;
    if 0.005 * (day_data.humidity as f64) + 0.02 * day_data.temperature - 1.0 > 0.5{
        will_rain = true
    }

    will_rain
}

fn main(){
    let weather_data: Vec<WeatherData> = vec![WeatherData { temperature: 25.0, humidity: 65, rain: false }, 
    WeatherData { temperature: 26.2, humidity: 70, rain: false }, WeatherData { temperature: 24.8, humidity: 62, rain: false }, 
    WeatherData { temperature: 23.5, humidity: 78, rain: true }, WeatherData { temperature: 22.1, humidity: 82, rain: true },
    WeatherData { temperature: 20.7, humidity: 85, rain: true }, WeatherData { temperature: 21.3, humidity: 80, rain: true },
    WeatherData { temperature: 22.8, humidity: 73, rain: false }, WeatherData { temperature: 24.0, humidity: 68, rain: false },
    WeatherData { temperature: 25.5, humidity: 60, rain: false }, WeatherData { temperature: 27.1, humidity: 55, rain: false },
    WeatherData { temperature: 28.3, humidity: 52, rain: false }, WeatherData { temperature: 27.9, humidity: 58, rain: false },
    WeatherData { temperature: 26.6, humidity: 64, rain: false }, WeatherData { temperature: 25.2, humidity: 70, rain: true },
    WeatherData { temperature: 23.8, humidity: 75, rain: true }, WeatherData { temperature: 22.4, humidity: 80, rain: true },
    WeatherData { temperature: 21.0, humidity: 83, rain: true }, WeatherData { temperature: 20.5, humidity: 86, rain: true },
    WeatherData { temperature: 21.8, humidity: 82, rain: true }, WeatherData { temperature: 23.2, humidity: 77, rain: false },
    WeatherData { temperature: 24.5, humidity: 70, rain: false }, WeatherData { temperature: 25.8, humidity: 63, rain: false },
    WeatherData { temperature: 26.4, humidity: 58, rain: false }, WeatherData { temperature: 27.0, humidity: 53, rain: false },
    WeatherData { temperature: 26.7, humidity: 56, rain: false }, WeatherData { temperature: 25.3, humidity: 62, rain: false },
    WeatherData { temperature: 24.9, humidity: 68, rain: true },WeatherData { temperature: 23.1, humidity: 74, rain: true },
    WeatherData { temperature: 21.7, humidity: 79, rain: true },
];
    println!("Warmest 3-day period:{:?}",warmest_period(&weather_data).iter().map(|x| (x.temperature, x.humidity, x.rain)).collect::<Vec<_>>());

    println!("Coldest day: {}", coldest_day(&weather_data));
    
    println!("Number of rainy days: {}", count_rainy_days(&weather_data));
    
    println!("Will it rain on the first day? {}", predict_rain(&weather_data[0]))
}
