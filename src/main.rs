use clap::{crate_description, crate_name, crate_version, Arg, ArgAction, Command};
use std::{
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    thread::{sleep, spawn},
    time::{Duration, Instant},
};
use sui_keys::key_derive::generate_new_key;
use sui_sdk::types::crypto::SignatureScheme;

fn main() {
    let num_cpus_string = num_cpus::get().to_string();
    let matches = Command::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .arg({
            Arg::new("threads")
                .help("Number of threads for lookup")
                .short('t')
                .long("threads")
                .default_value(num_cpus_string)
        })
        .arg({
            Arg::new("exit")
                .help("Exit on first match")
                .short('e')
                .long("exit")
                .action(ArgAction::SetTrue)
        })
        .arg({
            Arg::new("stat")
                .help("Print genrate stats every X seconds")
                .short('s')
                .long("stat")
                .default_value("10")
        })
        .arg(
            Arg::new("num")
                .help("Num of zero prefix")
                .index(1)
                .required(true),
        )
        .get_matches();

    let threads = matches
        .get_one::<String>("threads")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let exit = matches.get_flag("exit");
    let stat = Some(
        matches
            .get_one::<String>("stat")
            .unwrap()
            .parse::<u64>()
            .unwrap(),
    );
    let num = matches
        .get_one::<String>("num")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let word = format!("{}{}", "0x", "0".repeat(num));
    println!("searching prefix: {:?}", word);

    let exit_flag = Arc::new(AtomicBool::new(false));

    let perf_count = Arc::new(AtomicUsize::new(0));
    let mut perf_ts = Instant::now();

    let mut threads = (0..threads)
        .map(|_| {
            let word = word.clone();
            let exit_flag = Arc::clone(&exit_flag);
            let perf_count = Arc::clone(&perf_count);
            spawn(move || {
                while !exit_flag.load(Ordering::Relaxed) {
                    let chunk = 10;
                    for _ in 0..chunk {
                        if generate(&word) && exit {
                            exit_flag.store(true, Ordering::Relaxed);
                        }
                    }

                    perf_count.fetch_add(chunk, Ordering::AcqRel);
                }
            })
        })
        .collect::<Vec<_>>();

    if let Some(sleep_time) = stat {
        let sleep_time = Duration::from_secs(sleep_time);
        threads.push(spawn(move || loop {
            let sts = Instant::now();
            while sts.elapsed() < sleep_time {
                sleep(Duration::from_millis(50));
                if exit_flag.load(Ordering::Relaxed) {
                    return;
                }
            }

            let elapsed = perf_ts.elapsed().as_micros() as f64;
            let perf_total = perf_count.swap(0, Ordering::AcqRel) as f64;
            perf_ts = Instant::now();

            eprintln!("Genrate: {:.2?} op/s", perf_total * 1_000_000.0 / elapsed);
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
}

fn generate(word: &str) -> bool {
    let kp = generate_new_key(SignatureScheme::ED25519, None, None).unwrap();

    if kp.0.to_string().starts_with(word) {
        println!("{:#?}", kp);
        return true;
    } else {
        return false;
    }
}
