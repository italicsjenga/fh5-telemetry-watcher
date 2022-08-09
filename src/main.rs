use fh5_common::{Filename, Telemetry};

use std::fs::OpenOptions;
use std::net::UdpSocket;
#[cfg(target_family = "unix")]
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::{fs, net};

use bincode;
use chrono::Local;
use clap::Parser;
use tempfile::tempfile;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "")]
    folder: String,

    #[clap(short, long, default_value_t = 9999)]
    port: i32,

    #[clap(short, long)]
    verbose: bool,
}

struct Status {
    inrace: bool,
    position: u8,
}

fn listen(socket: &net::UdpSocket, mut buffer: &mut [u8]) -> usize {
    let number_of_bytes = match socket.recv_from(&mut buffer) {
        Ok((num, _)) => num,
        // skip packets if they're too big
        Err(e) => {
            println!("Network error: {}", e);
            0
        }
    };
    number_of_bytes
}

fn main() {
    let args = Args::parse();
    let ip = format!("0.0.0.0:{}", args.port.to_string());

    if args.folder != "" {
        println!("Saving logs to {}", args.folder);
    } else {
        println!("No folder specified - saving to current directory");
    }
    let folder_path = Path::new(&args.folder);
    fs::create_dir_all(folder_path).expect("couldnt create log directory!");

    let socket = UdpSocket::bind(ip).expect("couldnt bind");
    println!("Listening on port {}", args.port);
    let mut buf = [0; 2048];

    let mut writer = csv::Writer::from_writer(tempfile().expect("couldnt open tempfile"));

    let mut status = Status {
        inrace: false,
        position: 0,
    };

    'listener: while listen(&socket, &mut buf) != 0 {
        let deserialised: Telemetry = bincode::deserialize(&buf).expect("error parsing packet");
        if deserialised.is_race_on == 0 {
            continue 'listener;
        }

        if status.position != deserialised.race_position {
            status.position = deserialised.race_position;
            if args.verbose {
                println!("now position {}", status.position);
            }
        }

        if status.inrace {
            if deserialised.race_position == 0 {
                // coming out of race
                status.inrace = false;
                writer.flush().expect("couldnt flush to file");
                if args.verbose {
                    println!(
                        "{}: no longer in race",
                        &Local::now().format("%H:%M:%S").to_string()
                    );
                }
                continue 'listener;
            } else {
                // still in race
            }
        } else {
            if deserialised.race_position > 0 {
                // getting into race
                status.inrace = true;
                if args.verbose {
                    println!(
                        "{}: entering race",
                        &Local::now().format("%H:%M:%S").to_string()
                    );
                    println!("car class: {}", deserialised.car_performance_index);
                }
                let mut options = OpenOptions::new().write(true).create_new(true).clone();
                // open file with wide open permissions on unix systems
                if cfg!(target_family = "unix") {
                    options.mode(0o777);
                }
                // open file for this race
                // filename format: timestamp _ car class _ car ordinal
                let name = Filename::new_filename(
                    deserialised.car_performance_index,
                    deserialised.car_ordinal,
                );
                writer = csv::Writer::from_writer(
                    options
                        .open(folder_path.join(format!("{}{}", name.get_string(), ".csv",)))
                        .expect("couldnt open file"),
                );
            } else {
                // still not in race
                continue 'listener;
            }
        }
        writer.serialize(deserialised).expect("couldnt serialise");
    }
}
