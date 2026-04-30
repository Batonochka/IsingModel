use std::{fs::OpenOptions, io::{BufWriter, Write}};

use crate::{
    datasave::{save_energies, save_simulation, save_spins},
    matrix::Matrix,
};
use rand::{RngExt};
use rayon::prelude::*;

pub struct D1system {
    spins: Vec<f64>,
    j: f64,
    field: f64,
    temp: f64,
}
impl D1system {
    pub fn new(n: usize, j: f64, field: f64, temp: f64) -> Self {
        let spins = vec![1.0; n];
        D1system {
            spins,
            j,
            field,
            temp,
        }
    }

    pub fn energy(&self, spins: &[f64]) -> f64 {
        let num = spins.len();
        let mut energy = 0.0;
        for i in 0..num {
            if i == num - 1 {
                energy += -self.j * (spins[i] * spins[0]);
            } else {
                energy += -self.j * (spins[i] * spins[i + 1]);
            }
            energy += -self.field * spins[i];
        }
        energy
    }

    fn magnetic_moment(&self) -> f64 {
        let num = self.spins.len();
        let mut magnetic_momentum = 0.0;
        for i in 0..num {
            magnetic_momentum += self.spins[i];
        }
        magnetic_momentum
    }

    fn dzudi(&mut self) {
        let spins = &self.spins;
        let spins_num = spins.len();
        let mut rng = rand::rng();
        let num = rng.random_range(0..spins_num);
        let mut new_spins = vec![1.0; spins_num];
        new_spins.copy_from_slice(&spins);
        new_spins[num] = -spins[num];
        let energy = self.energy(&spins);
        let new_energy = self.energy(&new_spins);
        if new_energy < energy {
            self.spins.copy_from_slice(&new_spins);
        } else {
            let change_pos = (-(new_energy - energy) / self.temp).exp();
            let r = rng.random_range(0.0..1.0);
            if change_pos > r {
                self.spins.copy_from_slice(&new_spins);
            }
        }
    }

    pub fn simulate_debug(&mut self, energy_file: &str, spins_file: &str, steps: usize) -> std::io::Result<()> {
        let mut energies = vec![0.0; steps];
        for i in 0..steps {
            if i == 0 {
                save_spins(i, &self.spins, spins_file, true)?;
            } else {
                save_spins(i, &self.spins, spins_file, false)?;
            }
            energies[i] = self.energy(&self.spins);
            self.dzudi();
        }
        save_spins(steps, &self.spins, spins_file, false)?;
        save_energies(&energies, energy_file)?;
        Ok(())
    }
    pub fn simulate(&mut self, data_file: &str, temp_min: f64, temp_max: f64, temp_steps: usize) {
        let spins_num = self.spins.len();
        let real_steps = 20 * self.spins.len();
        let false_steps = 20 * self.spins.len();
        let temp_diff = (temp_max - temp_min) / (temp_steps as f64);
        let mut avr_en = vec![0.0; temp_steps];
        let mut avr_en_sq = vec![0.0; temp_steps];
        let mut avr_mm = vec![0.0; temp_steps];
        let mut avr_hc = vec![0.0; temp_steps];
        let mut temp = vec![0.0; temp_steps];
        let percent = temp_steps / 100;
        println!("1% = {}", percent);
        for i in 0..temp_steps {
            if i % (10 * percent) == 0 {
                println!("passed {}%", i / percent);
            }
            self.temp = temp_min + temp_diff * i as f64;
            temp[i] = self.temp;
            for _ in 0..false_steps {
                self.dzudi();
            }
            let mut k: usize = 0;
            for j in 0..real_steps {
                self.dzudi();
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
        for i in 0..temp_steps {
            avr_hc[i] = 1.0 / (spins_num as f64) * (avr_en_sq[i] - avr_en[i] * avr_en[i]) / (temp[i] * temp[i]);
            avr_en[i] /= spins_num as f64;
            avr_mm[i] /= spins_num as f64;
        }
        println!("passed 100%");
        let _ = save_simulation(data_file, &temp, &avr_en, &avr_mm, &avr_hc);
    }
}

#[derive(Clone)]
pub struct D2system {
    pub spins: Matrix,
    pub j: f64,
    pub field: f64,
    pub temp: f64,
}
impl D2system {
    pub fn new(n_x: usize, n_y: usize, j: f64, field: f64, temp: f64) -> Self {
        D2system {
            spins: Matrix::new1(n_y, n_x),
            j: j,
            field: field,
            temp: temp,
        }
    }
    pub fn energy(&self, spins: &Matrix) -> f64 {
        let cols = spins.cols;
        let rows = spins.rows;
        let mut energy = 0.0;
        for i in 0..rows {
            for j in 0..cols {
                if i == rows - 1 {
                    energy += -self.j * spins[(i, j)] * spins[(0, j)];
                } else {
                    energy += -self.j * spins[(i, j)] * spins[(i + 1, j)];
                }
                if j == cols - 1 {
                    energy += -self.j * spins[(i, j)] * spins[(i, 0)];
                } else {
                    energy += -self.j * spins[(i, j)] * spins[(i, j + 1)];
                }
                energy += -self.field * spins[(i, j)];
            }
        }
        energy
    }
    pub fn magnetic_moment(&self) -> f64 {
        let cols = self.spins.cols;
        let rows = self.spins.rows;
        let mut momentum = 0.0;
        for i in 0..cols * rows {
            momentum += self.spins.data[i];
        }
        momentum
    }
    fn dzudi(&mut self) {
        let spins = &self.spins;
        let rows = spins.rows;
        let cols = spins.cols;
        let spins_num = cols * rows;
        let mut rng = rand::rng();
        let num = rng.random_range(0..spins_num);
        let mut new_spins = Matrix::new(rows, cols);
        new_spins.data.copy_from_slice(&self.spins.data);
        new_spins.data[num] = -new_spins.data[num];
        let energy = self.energy(spins);
        let new_energy = self.energy(&new_spins);
        if new_energy < energy {
            self.spins.data.copy_from_slice(&new_spins.data);
        } else {
            let change_pos = (-(new_energy - energy) / self.temp).exp();
            let r = rng.random_range(0.0..1.0);
            if change_pos > r {
                self.spins.data.copy_from_slice(&new_spins.data);
            }
        }
    }
    pub fn simulate(
        &mut self,
        data_file: &str,
        temp_min: f64,
        temp_max: f64,
        temp_steps: usize,
    ) -> std::io::Result<()> {
        let cols = self.spins.cols;
        let rows = self.spins.rows;
        let spins_num = cols * rows;
        let steps = 5000 * spins_num;
        let temp_diff = (temp_max - temp_min) / (temp_steps as f64);
        let mut avr_en = vec![0.0; temp_steps];
        let mut avr_en_sq = vec![0.0; temp_steps];
        let mut avr_mm = vec![0.0; temp_steps];
        let mut avr_hc = vec![0.0; temp_steps];
        let mut temp = vec![0.0; temp_steps];
        let percent = temp_steps / 100;
        for i in 0..temp_steps {
            if i % (10 * percent) == 0 {
                println!("passed {}%", i / percent);
            }
            self.temp = temp_min + temp_diff * i as f64;
            temp[i] = self.temp;
            for _ in 0..steps {
                self.dzudi();
            }
            let mut k: usize = 0;
            for j in 0..steps {
                self.dzudi();
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
        for i in 0..temp_steps {
            avr_hc[i] = 1.0 / (spins_num as f64) * (avr_en_sq[i] - avr_en[i] * avr_en[i]) / (temp[i] * temp[i]);
            avr_en[i] /= spins_num as f64;
            avr_mm[i] /= spins_num as f64;
        }
        println!("passed 100%");
        let _ = save_simulation(data_file, &temp, &avr_en, &avr_mm, &avr_hc)?;
        println!("Saved");
        Ok(())
    }
    pub fn parallel_simulate(
        &mut self,
        data_file: &str,
        temp_min: f64,
        temp_max: f64,
        temp_steps: usize
    ) -> std::io::Result<()> {
        let cols = self.spins.cols;
        let rows = self.spins.rows;
        let spins_num = cols * rows;
        let steps = 1000 * spins_num;
        let temp_diff = (temp_max - temp_min) / (temp_steps as f64);
        let temperatures: Vec<f64> = (0..temp_steps)
            .map(|i| temp_min + temp_diff * i as f64)
            .collect();
        let results: Vec<_> = temperatures
            .par_iter()
            .map(|&t| {
                let mut system = self.clone();
                system.temp = t;
                for _ in 0..steps {
                    system.dzudi();
                }
                let mut k: usize = 0;
                let mut sum_e = 0.0;
                let mut sum_e2 = 0.0;
                let mut sum_m = 0.0;
                for j in 0..steps {
                    system.dzudi();
                    if j % 100 == 0 {
                        k += 1;
                        let e = system.energy(&system.spins);
                        sum_e += e;
                        sum_e2 += e * e;
                        sum_m += system.magnetic_moment();
                    }
                }
                let mean_e = sum_e / k as f64;
                let mean_e2 = sum_e2 / k as f64;
                let mean_m = sum_m / k as f64;
                let hc = (mean_e2 - mean_e * mean_e) / (t * t * spins_num as f64);
                (t, mean_e / spins_num as f64, mean_m / spins_num as f64, hc)
            }).collect();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(data_file)?;
        let mut writer = BufWriter::new(file);
        writeln!(writer, "temperature,energy,magnetic_momentum,heat_capacity")?;
        for (t,e,m,hc) in results {
            writeln!(writer, "{},{},{},{}", t,e,m,hc)?;
        }
        Ok(())
    }
}
