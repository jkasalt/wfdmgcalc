struct BaseDamage {
    slash: f32,
    impact: f32,
    puncture: f32,
}

impl BaseDamage {
    fn total_damage(&self) -> f32 {
        self.slash + self.impact + self.puncture
    }

    fn scale(&self) -> f32 {
        self.total_damage() / 16.0
    }

    fn scale_one(&self, damage: f32) -> f32 {
        f32::round(damage / self.scale()) * self.scale()
    }

    pub fn scaled_damage(&self) -> Self {
        Self {
            slash: self.scale_one(self.slash),
            impact: self.scale_one(self.impact),
            puncture: self.scale_one(self.puncture),
        }
    }
}

struct BaseDamageBonus {
    slash: f32,
    impact: f32,
    puncture: f32,
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;

    #[test]
    fn correct_damage_scaling_calculations() {
        let damage = BaseDamage {
            slash: 40,
            impact: 30,
            puncture: 30,
        };
        let scaled_damage = damage.scaled_damage();
        assert_ulps_eq!(scaled_damage.slash, 37.5);
        assert_ulps_eq!(scaled_damage.impact, 31.25);
        assert_ulps_eq!(scaled_damage.puncture, 31.25);
    }
}
