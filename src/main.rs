use crate::zveropolis::{d1system, d2system};

mod zveropolis;
mod visuals;
mod datasave;
mod matrix;

fn main() {
    let J = 1.0;
    let H = 0.0;
    let T = 1.0;
    let N: usize = 2000;
    let steps: usize = 40000;
    // опытным путем множитель х20 гарантрует сходимость

    let Nx: usize = 100;
    let Ny: usize = 100;
    let mut system2d = d2system::new(Nx, Ny, J, H, T);
    let mut system = d1system::new(N, J, H, T);
    let spins_file = "src/Spins.csv";
    let energy_file = "src/Energies.csv";

    // system.simulate_debug(energy_file, spins_file, steps);
    // let data_file = "src/systemdataH=1.csv";
    let data2d_file = "systemdata2d.csv";
    let T_min = 0.1;
    let T_max = 5.0;
    let T_steps: usize = 1000;
    // system.simulate(data_file, T_min, T_max, T_steps);
    system2d.simulate(data2d_file, T_min, T_max, T_steps);
}