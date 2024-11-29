use std::{
    collections::HashMap,
    ops::{Deref, Index},
};

use derive_builder::Builder;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum DamageType {
    Impact,
    Puncture,
    Slash,
    Cold,
    Electricity,
    Heat,
    Toxin,
    Blast,
    Corrosive,
    Gas,
    Magnetic,
    Radiation,
    Viral,
}

#[derive(Default)]
struct DamageSet {
    inner: HashMap<DamageType, f32>,
}

impl Index<DamageType> for DamageSet {
    type Output = f32;
    fn index(&self, index: DamageType) -> &Self::Output {
        &self.inner[&index]
    }
}

impl<A> FromIterator<A> for DamageSet
where
    HashMap<DamageType, f32>: FromIterator<A>,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self {
            inner: iter.into_iter().collect(),
        }
    }
}

impl From<(DamageType, f32)> for DamageSet {
    fn from(value: (DamageType, f32)) -> Self {
        Self::from_iter(std::iter::once(value))
    }
}

impl DamageSet {
    pub fn total(&self) -> f32 {
        self.inner.values().sum()
    }

    fn scale(&self) -> f32 {
        self.total() / 16.0
    }

    fn round(&self, damage: f32) -> f32 {
        f32::round(damage / self.scale()) * self.scale()
    }

    pub fn scaled(&self) -> Self {
        Self {
            inner: self
                .inner
                .iter()
                .map(|(k, v)| (*k, self.round(*v)))
                .collect(),
        }
    }
}

struct Weapon {
    damage: DamageSet,
    multishot: f32,
    crit_chance: f32,
    crit_multi: f32,
    status_chance: f32,
    fire_rate: f32,
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            damage: DamageSet::default(),
            multishot: 0.0,
            crit_chance: 0.0,
            crit_multi: 1.0,
            status_chance: 0.0,
            fire_rate: 1.0,
        }
    }
}

impl Weapon {
    fn dps(&self) -> f32 {
        let total_dmg = self.damage.scaled().total();
        let avg_dmg_multi = 1.0 + self.crit_chance * (self.crit_multi - 1.0);
        let avg_dmg = total_dmg * avg_dmg_multi;
        avg_dmg * (1.0 + self.multishot) * self.fire_rate
    }

    fn apply_bonus(&self, bonus: Bonus) -> Self {
        todo!()
    }
}

enum Bonus {
    Cold(f32),
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;

    #[test]
    fn correct_damage_scaling_calculations() {
        use DamageType as DT;
        let damage = DamageSet::from_iter(vec![
            (DT::Slash, 40.0),
            (DT::Impact, 30.0),
            (DT::Puncture, 30.0),
        ]);

        let scaled_damage = damage.scaled();
        assert_ulps_eq!(scaled_damage[DT::Slash], 37.5);
        assert_ulps_eq!(scaled_damage[DT::Impact], 31.25);
        assert_ulps_eq!(scaled_damage[DT::Puncture], 31.25);
    }

    #[test]
    fn weapon_dps_calculations() {
        let weapon = Weapon {
            damage: DamageSet::from((DamageType::Puncture, 10.0)),
            multishot: 0.0,
            crit_chance: 1.5,
            crit_multi: 2.0,
            status_chance: 0.0,
            fire_rate: 1.0,
        };

        assert_ulps_eq!(weapon.dps(), 25.0);
    }

    #[test]
    #[ignore]
    fn weapon_dps_apply_elemental_bonus() {
        let weapon = Weapon {
            damage: DamageSet::from((DamageType::Puncture, 10.0)),
            multishot: 0.0,
            crit_chance: 1.5,
            crit_multi: 2.0,
            status_chance: 0.0,
            fire_rate: 1.0,
        }
        .apply_bonus(Bonus::Cold(0.8));
        assert_ulps_eq!(weapon.dps(), 18.0 * 2.5);
    }
}
