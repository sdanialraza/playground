use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Bill {
    pub price_range: (RangeInclusive<u32>, f32),
    pub usage_hours: Vec<u32>,
}

impl Bill {
    pub const fn new() -> Self {
        Bill {
            price_range: (1..=4, 2.5),
            usage_hours: Vec::new(),
        }
    }

    pub fn change_usage_hours(&mut self, usage_hours: Vec<u32>) -> &mut Self {
        self.usage_hours = usage_hours;
        self
    }

    pub fn change_price_range(&mut self, price_range: (RangeInclusive<u32>, f32)) -> &mut Self {
        self.price_range = price_range;
        self
    }

    pub fn resolve(&self) -> f32 {
        let mut power_bill = 0.0;

        for usage_hours in self.usage_hours.iter() {
            match usage_hours {
                1..=4 => power_bill += *usage_hours as f32 * self.price_range.1,
                _ => power_bill += *usage_hours as f32 * 3.5,
            }
        }

        power_bill
    }
}
