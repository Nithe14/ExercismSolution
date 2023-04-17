#[derive(Debug)]
pub struct Clock {
    m: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            m: hours * 60 + minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            m: self.m + minutes,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{:02}:{:02}",
            modulo((self.m as f32 / 60 as f32).floor() as i32, 24),
            modulo(self.m, 60)
        )
    }
}

impl Eq for Clock {}
impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        if self.to_string() == other.to_string() {
            true
        } else {
            false
        }
    }
}

fn modulo(num: i32, m: i32) -> i32 {
    if num % m < 0 {
        m + num % m
    } else {
        num % m
    }
}