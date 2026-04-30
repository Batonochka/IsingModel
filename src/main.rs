use crate::zveropolis::{D1system, D2system};

mod datasave;
mod matrix;
mod visuals;
mod zveropolis;

fn main() {
    let integral = 1.0;
    let mut field = 5.0;
    let temp = 1.0;
    // let spins_num: usize = 2000;
    // let steps: usize = 80000;

    let temp_min = 0.1;
    let temp_max = 5.0;
    let temp_steps: usize = 100;
    // опытным путем множитель х20 гарантрует сходимость

    // let mut system = D1system::new(spins_num, integral, field, temp);
    
    // let spins_file = "SpinsH=2.0.csv";
    // let energy_file = "EnergiesH=2.0.csv";
    // let data_file = "systemdataH=2.0.csv";
    
    // let _ = system.simulate_debug(energy_file, spins_file, steps);
    // system.simulate(data_file, temp_min, temp_max, temp_steps);
    
    
    let nx: usize = 30;
    let ny: usize = 30;
    field = 0.1;
    let mut system2d = D2system::new(nx, ny, integral, field, temp);
    let mut data2d_file = "systemdata32dH=0.5.csv";
    // let _ = system2d.simulate(data2d_file, temp_min, temp_max, temp_steps);
    // data2d_file = "systemdata22dH=0.5.csv";
    // field = 0.5;
    // system2d = D2system::new(nx, ny, integral, field, temp);
    system2d.parallel_simulate(data2d_file, temp_min, temp_max, temp_steps);
    // let _ = system2d.simulate(data2d_file, temp_min, temp_max, temp_steps);
}
