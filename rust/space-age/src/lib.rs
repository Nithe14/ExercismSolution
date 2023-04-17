// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const ONESEC: f64 = 3.16887646e-8;
const MERCURY_YEAR: f64 = 0.2408467;
const VENUS_YEAR: f64 = 0.61519726;
const MARS_YEAR: f64 = 1.8808158;
const JUPITER_YEAR: f64 = 11.862615;
const SATURN_YEAR: f64 = 29.447498;
const URANUS_YEAR: f64 = 84.016846;
const NEPTUNE_YEAR: f64 = 164.79132;

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 * ONESEC
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

macro_rules! planet_year {
    ($planet: ident, $orbit:expr) => {
        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 {
                d.0 as f64 * ONESEC / $orbit
            }
        }
    };
}

impl Planet for Earth {} //default trait for earth
planet_year!(Mercury, MERCURY_YEAR);
planet_year!(Venus, VENUS_YEAR);
planet_year!(Mars, MARS_YEAR);
planet_year!(Jupiter, JUPITER_YEAR);
planet_year!(Saturn, SATURN_YEAR);
planet_year!(Uranus, URANUS_YEAR);
planet_year!(Neptune, NEPTUNE_YEAR);
