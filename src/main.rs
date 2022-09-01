use fh5_common::{Filename, Telemetry};
#[macro_use]
extern crate rocket;
use bincode;
use chrono::Local;
use clap::Parser;
use rocket::State;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::UdpSocket;
#[cfg(target_family = "unix")]
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{fs, net};

macro_rules! verbose_print {
	($args:expr, $($tts:tt)*) => {
		if($args.verbose) {
			println!($($tts)*);
		}
	};
}

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
    next: Option<NextFile>,
    position: u8,
}

struct NextFile {
    writer: csv::Writer<Vec<u8>>,
    name: Filename,
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

struct DataStruct {
    data: Arc<Mutex<[f32; 3]>>,
}

#[rocket::main]
async fn main() {
    let args = Args::parse();
    let ip = format!("0.0.0.0:{}", args.port.to_string());

    let data = Arc::new(Mutex::new([0. as f32, 0. as f32, 0. as f32]));
    let data_for_thread = data.clone();

    let _listener = std::thread::spawn(move || {
        let mut data = data_for_thread.lock().unwrap();
        data[0] = 1.;
        data[1] = 2.;
        data[2] = 3.;
        drop(data);
        verbose_print!(
            args,
            "{}: Starting server",
            &Local::now().format("%H:%M:%S").to_string()
        );
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

        // let mut writer = csv::Writer::from_writer(tempfile().expect("couldnt open tempfile"));
        let mut status = Status {
            next: None,
            position: 0,
        };

        'listener: while listen(&socket, &mut buf) != 0 {
            let deserialised: Telemetry = bincode::deserialize(&buf).expect("error parsing packet");
            if deserialised.is_race_on == 0 {
                continue 'listener;
            }

            if status.position != deserialised.race_position {
                status.position = deserialised.race_position;
                verbose_print!(args, "now position {}", status.position);
            }
            let mut data = data_for_thread.lock().unwrap();
            data[0] = deserialised.position_x;
            data[1] = deserialised.position_y;
            data[2] = deserialised.position_z;
            drop(data);

            // status.next.
            if deserialised.race_position == 0 {
                status.next.take().and_then(|next| -> Option<NextFile> {
                    verbose_print!(
                        args,
                        "{}: no longer in race",
                        &Local::now().format("%H:%M:%S").to_string()
                    );
                    finish_race(&args, next);
                    None
                });
            }
            match status.next {
                Some(ref mut next) => {
                    continue_race(&deserialised, &mut next.writer);
                }
                None => {
                    if deserialised.race_position > 0 {
                        verbose_print!(
                            args,
                            "{}: entering race\ncar class: {}",
                            &Local::now().format("%H:%M:%S").to_string(),
                            deserialised.car_performance_index
                        );
                        let mut new_next = begin_race(&deserialised);
                        continue_race(&deserialised, &mut new_next.writer);
                        status.next = Some(new_next);
                    }
                }
            }
        }
    });
    let _result = rocket::custom(rocket::Config::figment().merge(("port", 1111)))
        .mount("/", routes![index])
        .manage(DataStruct { data })
        .launch()
        .await;
}

#[get("/")]
fn index(data: &State<DataStruct>) -> String {
    let get = data.data.lock().unwrap();
    format!("x: {}, y: {}, z: {}", get[0], get[1], get[2])
}

fn begin_race(deserialised: &Telemetry) -> NextFile {
    NextFile {
        writer: csv::Writer::from_writer(vec![]),
        name: Filename::new_filename(deserialised.car_performance_index, deserialised.car_ordinal),
    }
}

fn finish_race(args: &Args, next: NextFile) {
    let mut options = OpenOptions::new().write(true).create_new(true).clone();
    // open file with wide open permissions on unix systems
    if cfg!(target_family = "unix") {
        options.mode(0o777);
    }
    // open file for this race
    let filename = format!("{}/{}.csv", args.folder, next.name.get_string());
    verbose_print!(args, "Opening file {}", filename);
    let mut filewriter = options.open(filename).expect("failed to open file");
    let a = next.writer.into_inner().unwrap();
    filewriter.write_all(&a).unwrap();
    filewriter.flush().expect("could not flush file");
}

fn continue_race(deserialised: &Telemetry, writer: &mut csv::Writer<Vec<u8>>) {
    writer.serialize(deserialised).expect("couldnt serialise");
}
