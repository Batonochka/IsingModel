use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
};

pub fn save_energies(energies: &[f64], filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    writeln!(file, "step,energy")?;
    for (step, energy) in energies.iter().enumerate() {
        writeln!(file, "{},{}", step, energy)?;
    }
    Ok(())
}
pub fn save_spins(
    step: usize,
    spins: &[f64],
    filename: &str,
    overwrite: bool,
) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(overwrite)
        .append(!overwrite)
        .open(filename)?;

    let mut writer = BufWriter::new(file);

    if overwrite {
        writeln!(writer, "steps,spins");
    }
    let spins_str = spins
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<_>>()
        .join(",");
    writeln!(writer, "{},\"{}\"", step, spins_str)?;
    writer.flush()?;
    Ok(())
}
pub fn save_simulation(
    filename: &str,
    temp: &[f64],
    energies: &[f64],
    magnetics: &[f64],
    heatcap: &[f64],
) -> std::io::Result<()> {
    println!("Saving");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(filename)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "temperature,energy,magnetic_momentum,heat_capacity")?;
    for i in 0..temp.len() {
        writeln!(writer, "{},{},{},{}", temp[i], energies[i], magnetics[i], heatcap[i])?;
    }
    writer.flush()?;
    Ok(())
}