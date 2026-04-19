use rand::{Rng, RngExt};
use crate::datasave::{self, save_energies, save_simulation, save_spins};

pub struct d1system
{
    pub spins: Vec<f64>,
    pub J: f64,
    pub H: f64,
    pub T: f64,
    pub energy: f64,
    pub magnetic_moment: f64,
    pub heat_capacity: f64,
}
impl d1system
{
    pub fn new(n: usize, J: f64, H: f64, T: f64) -> Self {
        let mut spins = vec![1.0; n];
        d1system {
             spins, J, H, T,
             energy: 0.0,
             magnetic_moment: 0.0,
             heat_capacity: 0.0 }
    }
    pub fn energy(&self, spins: &[f64]) -> f64 {
        let N = spins.len();
        let mut energy = 0.0;
        for i in 0..N {
            if i == N-1 {
                energy += -self.J * (spins[i] * spins[0]);
            } else {
                energy += -self.J * (spins[i] * spins[i+1]);
            }
            energy += -self.H * spins[i];
        }
        return energy;
    }
    fn magnetic_moment(&self) -> f64 {
        let N = self.spins.len();
        let mut magnetic_momentum = 0.0;
        for i in 0..N {
            magnetic_momentum += self.spins[i];
        }
        return  magnetic_momentum;

    }
    fn Dzudi(&mut self) {
        let spins = &self.spins;
        let N = spins.len();
        let mut rng = rand::rng();
        let num = rng.random_range(0..N);
        let mut new_spins = vec![1.0; N];
        new_spins.copy_from_slice(&spins);
        new_spins[num] = -spins[num];
        let energy = self.energy(&spins);
        let new_energy = self.energy(&new_spins);
        if new_energy < energy {
            self.spins.copy_from_slice(&new_spins);
        } else {
            let R = (-(new_energy - energy) / self.T).exp();
            let r = rng.random_range(0.0..1.0);
            if R > r {
                self.spins.copy_from_slice(&new_spins);
            }
        }
    }
    pub fn simulate_debug(&mut self, energy_file: &str, spins_file: &str, steps:usize) {
        let mut energies = vec![0.0; steps];
        for i in 0..steps {
            if i == 0 {
                save_spins(i, &self.spins, spins_file, true);
            } else {
                save_spins(i, &self.spins, spins_file, false);
            }
            energies[i] = self.energy(&self.spins);
            self.Dzudi();
        }
        save_spins(steps, &self.spins, spins_file, false);
        save_energies(&energies, energy_file);
    }
    pub fn simulate(&mut self, data_file: &str, T_min: f64, T_max: f64, T_steps: usize) {
        let N = self.spins.len();
        let real_steps = 10 * self.spins.len();
        let false_steps = 10 * self.spins.len();
        let dT = (T_max - T_min) / (T_steps as f64);
        let mut avr_en = vec![0.0; T_steps];
        let mut avr_en_sq = vec![0.0; T_steps];
        let mut avr_mm = vec![0.0; T_steps];
        let mut avr_hc = vec![0.0; T_steps];
        let mut T = vec![0.0; T_steps];
        let percent = T_steps / 100;
        println!("1% = {}", percent);
        for i in 0..T_steps {
            if i % (10 * percent) == 0 {
                println!("passed {}%", i / percent);
            }
            self.T = T_min +  dT * i as f64;
            T[i] = self.T;
            for _ in 0..false_steps {
                self.Dzudi();
            }
            let mut k: usize = 0;
            for j in 0..real_steps {
                self.Dzudi();
                if j % 100 == 0 {
                    k += 1;
                    let e = self.energy(&self.spins);
                    avr_en[i] += e;
                    avr_en_sq[i] += e * e;
                    avr_mm[i] += self.magnetic_moment();
                }
            }
            avr_en[i] /= k as f64;
            avr_en_sq[i] /= k as f64;
            avr_mm[i] /= k as f64;
        }
        for i in 0..T_steps {
            avr_hc[i] = 1.0 / (N as f64) * 
            (avr_en_sq[i] - avr_en[i]*avr_en[i]) / (T[i]*T[i]);
            avr_en[i] /= N as f64;
            avr_mm[i] /= N as f64;
        }
        println!("passed 100%");
        save_simulation(data_file, &T, &avr_en, &avr_mm, &avr_hc);
    }
}