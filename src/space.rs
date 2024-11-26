#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Duration(seconds as f64 / 31_557_600.0f64)
    }
}

pub trait Planet {
    const PERIOD_METRIC: f64;

    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::PERIOD_METRIC
    }
}

macro_rules! planet {
    ($n:ident, $p:expr) => {
        pub struct $n; impl Planet for $n { const PERIOD_METRIC: f64 = $p; }
    }
}

planet!(Earth, 1.0);
planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);

pub fn main() {
    let duration = Duration::from(1_000_000_000u64);
    println!("Years on Mercury: {}", Mercury::years_during(&duration));
    println!("Years on Venus: {}", Venus::years_during(&duration));
    println!("Years on Earth: {}", Earth::years_during(&duration));
    println!("Years on Mars: {}", Mars::years_during(&duration));
    println!("Years on Jupiter: {}", Jupiter::years_during(&duration));
    println!("Years on Saturn: {}", Saturn::years_during(&duration));
    println!("Years on Uranus: {}", Uranus::years_during(&duration));
    println!("Years on Neptune: {}", Neptune::years_during(&duration));
}
