use colored::*;

fn scan() {
    let (mut ctl, mut reader) = rtlsdr_mt::open(0).unwrap();

    ctl.enable_agc().unwrap();
    ctl.set_ppm(-2).unwrap();
    ctl.set_center_freq(82_200_000).unwrap();

    std::thread::spawn(move || {
        loop {
            let next = ctl.center_freq() + 1000;
            ctl.set_center_freq(next).unwrap();

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    reader.read_async(4, 32768, |bytes| {
        println!("i[0] = {}", bytes[0]);
        println!("q[0] = {}", bytes[1]);
    }).unwrap();
}

fn main() {
    let devices: Vec<_> = rtlsdr_mt::devices().collect();

    println!("{}", format!("Found {} device(s):", devices.len()).bright_green().bold());
    for device in devices {
        println!("  {:?}", device);
    }
    scan();
}