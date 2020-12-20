use std::process::exit;
use colored::*;

mod converters;
mod resamplers;
mod demodulators;
mod sdr;
mod utils;
mod error;

fn main() -> anyhow::Result<()> {
    let matches = utils::get_matches();
    let devices: Vec<_> = rtlsdr_mt::devices().collect();
    let frequency: u32 = matches.value_of("frequency").unwrap_or("98000000").parse()?;
    if devices.len() == 0 {
        eprintln!("{}", "No rtl-sdr devices found. Bye.".bright_red());
        exit(1);
    }
    println!("{}", format!("Found {} device(s):", devices.len()).bright_green().bold());
    for (i, device) in devices.iter().enumerate() {
        println!("{}  {:?}", i, device);
    }
    println!("Demodulating frequency {} using device {}", frequency.to_string().bright_green(), devices[0].to_str()?.bright_green());

    // let source = RtlSdrSource::with_converter(U8F32Converter());

    sdr::fm_play(frequency)?;   
    Ok(())
}