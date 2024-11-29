use derive_builder::Builder;

#[derive(Default, Builder)]
#[builder(default)]
struct Damage {
    slash: f32,
    impact: f32,
    puncture: f32,
    cold: f32,
    electricity: f32,
    fire: f32,
    toxin: f32,
    blast: f32,
    corrosive: f32,
    gas: f32,
    magnetic: f32,
    radiation: f32,
    viral: f32,
}

impl Damage {
    fn total_damage(&self) -> f32 {
        self.slash
            + self.impact
            + self.puncture
            + self.cold
            + self.electricity
            + self.fire
            + self.toxin
            + self.blast
            + self.corrosive
            + self.gas
            + self.magnetic
            + self.radiation
            + self.viral
    }

    fn scale(&self) -> f32 {
        self.total_damage() / 16.0
    }

    fn round(&self, damage: f32) -> f32 {
        f32::round(damage / self.scale()) * self.scale()
    }

    pub fn scaled_damage(&self) -> Self {
        Self {
            slash: self.round(self.slash),
            impact: self.round(self.impact),
            puncture: self.round(self.puncture),
            cold: self.round(self.cold),
            electricity: self.round(self.electricity),
            fire: self.round(self.fire),
            toxin: self.round(self.toxin),
            blast: self.round(self.blast),
            corrosive: self.round(self.corrosive),
            gas: self.round(self.gas),
            magnetic: self.round(self.magnetic),
            radiation: self.round(self.radiation),
            viral: self.round(self.viral),
        }
    }
}

struct Weapon {
    damage: Damage,
    multishot: f32,
    crit_chance: f32,
    crit_multi: f32,
    status: f32,
    fire_rate: f32,
}

impl Weapon {
    fn dps(&self) -> f32 {
        let total_dmg = self.damage.scaled().total();
        let avg_dmg_multi = 1.0 + self.crit_chance * (self.crit_multi - 1.0);
        let avg_dmg = total_dmg * avg_dmg_multi;
        avg_dmg * (1.0 + self.multishot) * self.fire_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;

    #[test]
    fn correct_damage_scaling_calculations() {
        let damage = Damage {
            slash: 40.0,
            impact: 30.0,
            puncture: 30.0,
            ..Default::default()
        };
        let scaled_damage = damage.scaled_damage();
        assert_ulps_eq!(scaled_damage.slash, 37.5);
        assert_ulps_eq!(scaled_damage.impact, 31.25);
        assert_ulps_eq!(scaled_damage.puncture, 31.25);
    }

    #[test]
    fn weapon_dps_calculations() {
        let weapon = Weapon {
            damage: DamageBuilder::default().puncture(10.0).build().unwrap(),
            multishot: 0.0,
            crit_chance: 1.5,
            crit_multi: 2.0,
            status: 0.0,
            fire_rate: 1.0,
        };

        assert_ulps_eq!(weapon.dps(), 25.0);
    }
}
