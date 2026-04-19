use crate::zveropolis::d1system;

mod zveropolis;
mod visuals;
mod datasave;

fn main() {
    let J = 1.0;
    let H = 0.0;
    let T = 1.0;
    let N: usize = 2000;
    let steps: usize = 40000;
    // опытным путем множитель х20 гарантрует сходимость

    let mut system = d1system::new(N, J, H, T);
    let spins_file = "src/Spins.csv";
    let energy_file = "src/Energies.csv";

    // system.simulate_debug(energy_file, spins_file, steps);
    let data_file = "src/systemdata.csv";
    let T_min = 0.1;
    let T_max = 5.0;
    let T_steps: usize = 100;
    system.simulate(data_file, T_min, T_max, T_steps);
}