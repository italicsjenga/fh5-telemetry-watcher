use std::net::UdpSocket;
use std::path::Path;
use std::{fs, net};

use bincode;
use chrono::Local;
use clap::Parser;
use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize, Debug)]
struct Telemetry {
    is_race_on: i32,

    time_stamp_ms: u32,

    engine_max_rpm: f32,
    engine_idle_rpm: f32,
    current_engine_rpm: f32,

    acceleration_x: f32, // local space: X = right, Y = up, Z = forward
    acceleration_y: f32,
    acceleration_z: f32,

    velocity_x: f32, // local space: X = right, Y = up, Z = forward
    velocity_y: f32,
    velocity_z: f32,

    angular_velocity_x: f32, // local space: X = pitch, Y = yaw, Z = roll
    angular_velocity_y: f32,
    angular_velocity_z: f32,

    yaw: f32,
    pitch: f32,
    roll: f32,

    normalized_suspension_travel_front_left: f32, // Suspension travel normalized: 0.0f = max stretch; 1.0 = max compression
    normalized_suspension_travel_front_right: f32,
    normalized_suspension_travel_rear_left: f32,
    normalized_suspension_travel_rear_right: f32,

    tire_slip_ratio_front_left: f32, // Tire normalized slip ratio, = 0 means 100% grip and |ratio| > 1.0 means loss of grip.
    tire_slip_ratio_front_right: f32,
    tire_slip_ratio_rear_left: f32,
    tire_slip_ratio_rear_right: f32,

    wheel_rotation_speed_front_left: f32, // Wheel rotation speed radians/sec.
    wheel_rotation_speed_front_right: f32,
    wheel_rotation_speed_rear_left: f32,
    wheel_rotation_speed_rear_right: f32,

    wheel_on_rumble_strip_front_left: i32, // = 1 when wheel is on rumble strip, = 0 when off.
    wheel_on_rumble_strip_front_right: i32,
    wheel_on_rumble_strip_rear_left: i32,
    wheel_on_rumble_strip_rear_right: i32,

    wheel_in_puddle_depth_front_left: f32, // = from 0 to 1, where 1 is the deepest puddle
    wheel_in_puddle_depth_front_right: f32,
    wheel_in_puddle_depth_rear_left: f32,
    wheel_in_puddle_depth_rear_right: f32,

    surface_rumble_front_left: f32, // Non-dimensional surface rumble values passed to controller force feedback
    surface_rumble_front_right: f32,
    surface_rumble_rear_left: f32,
    surface_rumble_rear_right: f32,

    tire_slip_angle_front_left: f32, // Tire normalized slip angle, = 0 means 100% grip and |angle| > 1.0 means loss of grip.
    tire_slip_angle_front_right: f32,
    tire_slip_angle_rear_left: f32,
    tire_slip_angle_rear_right: f32,

    tire_combined_slip_front_left: f32, // Tire normalized combined slip, = 0 means 100% grip and |slip| > 1.0 means loss of grip.
    tire_combined_slip_front_right: f32,
    tire_combined_slip_rear_left: f32,
    tire_combined_slip_rear_right: f32,

    suspension_travel_meters_front_left: f32, // Actual suspension travel in meters
    suspension_travel_meters_front_right: f32,
    suspension_travel_meters_rear_left: f32,
    suspension_travel_meters_rear_right: f32,

    car_ordinal: i32,
    car_class: i32,
    car_performance_index: i32,
    drivetraintype: i32, // 0 = FWD, 1 = RWD, 2 = AWD
    num_cylinders: i32,
    car_type: i32,

    unknown1: u8,
    unknown2: u8,
    unknown3: u8,
    unknown4: u8,
    unknown5: u8,
    unknown6: u8,
    unknown7: u8,
    unknown8: u8,

    position_x: f32, // meters
    position_y: f32,
    position_z: f32,

    speed: f32,  // meters per second
    power: f32,  // watts
    torque: f32, // newton meters

    tire_temp_front_left: f32,
    tire_temp_front_right: f32,
    tire_temp_rear_left: f32,
    tire_temp_rear_right: f32,

    boost: f32,
    fuel: f32,
    distance_traveled: f32,
    best_lap: f32,
    last_lap: f32,
    current_lap: f32,
    current_race_time: f32,

    lap_number: i16,
    race_position: u8,

    accel: u8,
    brake: u8,
    clutch: u8,
    handbrake: u8,
    gear: u8,
    steer: i8,

    normalized_driving_line: i8,
    normalized_ai_brake_difference: i8,
}

fn listen(socket: &net::UdpSocket, mut buffer: &mut [u8]) -> usize {
    let number_of_bytes = match socket.recv_from(&mut buffer) {
        Ok((num, _)) => num,
        // skip packets if they're too big
        Err(_) => 0,
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
    let mut buf = [0; 500];

    let mut writer = csv::Writer::from_writer(tempfile().expect("couldnt open tempfile"));

    let mut inrace = false;

    'listener: while listen(&socket, &mut buf) != 0 {
        let deserialised: Telemetry = bincode::deserialize(&buf).expect("error parsing packet");
        if deserialised.is_race_on == 0 {
            continue 'listener;
        }
        if args.verbose {}

        if inrace {
            if deserialised.race_position == 0 {
                // coming out of race
                inrace = false;
                writer.flush().expect("couldnt flush to file");
                println!(
                    "{}: no longer in race",
                    &Local::now().format("%H:%M:%S").to_string()
                );
                continue 'listener;
            } else {
                // still in race
            }
        } else {
            if deserialised.race_position > 0 {
                // getting into race
                inrace = true;
                println!(
                    "{}: entering race",
                    &Local::now().format("%H:%M:%S").to_string()
                );
                println!("car class: {}", deserialised.car_performance_index);
                // open file for this race
                // filename format: timestamp _ car class _ car ordinal
                writer = csv::Writer::from_writer(
                    fs::File::create(folder_path.join(format!(
                        "{}_{}_{}{}",
                        &Local::now().format("%Y-%m-%d_%H-%M").to_string(),
                        deserialised.car_performance_index,
                        deserialised.car_ordinal,
                        ".csv",
                    )))
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
