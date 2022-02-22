use std::net;
use std::net::UdpSocket;

use bincode;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "")]
    folder: String,

    #[clap(short, long, default_value_t = 9999)]
    port: i32,

    #[clap(short, long)]
    verbose: bool,

    #[clap(short, long)]
    errors: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Telemetry {
    // is_race_on: i32,
    // timestamp_ms: u32,
    // engine_max_rpm: f32,
    // engine_idle_rpm: f32,
    // current_engine_rpm: f32,
    // acceleration_x: f32,
    // acceleration_y: f32,
    // acceleration_z: f32,
    // velocity_x: f32,
    // velocity_y: f32,
    // velocity_z: f32,
    // angular_velocity_x: f32,
    // angular_velocity_y: f32,
    // angular_velocity_z: f32,
    // yaw: f32,
    // pitch: f32,
    // roll: f32,
    // normalized_suspension_travel_front_left: f32,
    // normalized_suspension_travel_front_right: f32,
    // normalized_suspension_travel_rear_left: f32,
    // normalized_suspension_travel_rear_right: f32,
    // tire_slip_ratio_front_left: f32,
    // tire_slip_ratio_front_right: f32,
    // tire_slip_ratio_rear_left: f32,
    // tire_slip_ratio_rear_right: f32,
    // wheel_rotation_speed_front_left: f32,
    // wheel_rotation_speed_front_right: f32,
    // wheel_rotation_speed_rear_left: f32,
    // wheel_rotation_speed_rear_right: f32,
    // wheel_on_rumble_strip_front_left: i32,
    // wheel_on_rumble_strip_front_right: i32,
    // wheel_on_rumble_strip_rear_left: i32,
    // wheel_on_rumble_strip_rear_right: i32,
    // wheel_in_puddle_depth_front_left: f32,
    // wheel_in_puddle_depth_front_right: f32,
    // wheel_in_puddle_depth_rear_left: f32,
    // wheel_in_puddle_depth_rear_right: f32,
    // surface_rumble_front_left: f32,
    // surface_rumble_front_right: f32,
    // surface_rumble_rear_left: f32,
    // surface_rumble_rear_right: f32,
    // tire_slip_angle_front_left: f32,
    // tire_slip_angle_front_right: f32,
    // tire_slip_angle_rear_left: f32,
    // tire_slip_angle_rear_right: f32,
    // tire_combined_slip_front_left: f32,
    // tire_combined_slip_front_right: f32,
    // tire_combined_slip_rear_left: f32,
    // tire_combined_slip_rear_right: f32,
    // suspension_travel_meters_front_left: f32,
    // suspension_travel_meters_front_right: f32,
    // suspension_travel_meters_rear_left: f32,
    // suspension_travel_meters_rear_right: f32,
    // car_ordinal: i32,
    // car_class: i32,
    // car_performance_index: i32,
    // drivetrain_type: i32,
    // num_cylinders: i32,
    // horizon_placeholder1: u32,
    // horizon_placeholder2: u32,
    // horizon_placeholder3: u32,
    // position_x: f32,
    // position_y: f32,
    // position_z: f32,
    // speed: f32,
    // power: f32,
    // torque: f32,
    // tire_temp_front_left: f32,
    // tire_temp_front_right: f32,
    // tire_temp_rear_left: f32,
    // tire_temp_rear_right: f32,
    // boost: f32,
    // fuel: f32,
    // distance_traveled: f32,
    // best_lap: f32,
    // last_lap: f32,
    // current_lap: f32,
    // current_race_time: f32,
    // lap_number: u16,
    // race_position: u8,
    // accel: u8,
    // brake: u8,
    // clutch: u8,
    // hand_brake: u8,
    // gear: u8,
    // steer: i8,
    // normalized_driving_line: i8,
    // normalized_ai_brake_difference: i8,

    // BORDER
    is_race_on: i32,
    time_stamp_ms: u32,
    engine_max_rpm: f32,
    engine_idle_rpm: f32,
    current_engine_rpm: f32,
    acceleration_x: f32,
    acceleration_y: f32,
    acceleration_z: f32,
    velocity_x: f32,
    velocity_y: f32,
    velocity_z: f32,
    angular_velocity_x: f32,
    angular_velocity_y: f32,
    angular_velocity_z: f32,
    yaw: f32,
    pitch: f32,
    roll: f32,
    normalized_suspension_travel_front_left: f32,
    normalized_suspension_travel_front_right: f32,
    normalized_suspension_travel_rear_left: f32,
    normalized_suspension_travel_rear_right: f32,
    tire_slip_ratio_front_left: f32,
    tire_slip_ratio_front_right: f32,
    tire_slip_ratio_rear_left: f32,
    tire_slip_ratio_rear_right: f32,
    wheel_rotation_speed_front_left: f32,
    wheel_rotation_speed_front_right: f32,
    wheel_rotation_speed_rear_left: f32,
    wheel_rotation_speed_rear_right: f32,
    wheel_on_rumble_strip_front_left: i32,
    wheel_on_rumble_strip_front_right: i32,
    wheel_on_rumble_strip_rear_left: i32,
    wheel_on_rumble_strip_rear_right: i32,
    wheel_in_puddle_depth_front_left: f32,
    wheel_in_puddle_depth_front_right: f32,
    wheel_in_puddle_depth_rear_left: f32,
    wheel_in_puddle_depth_rear_right: f32,
    surface_rumble_front_left: f32,
    surface_rumble_front_right: f32,
    surface_rumble_rear_left: f32,
    surface_rumble_rear_right: f32,
    tire_slip_angle_front_left: f32,
    tire_slip_angle_front_right: f32,
    tire_slip_angle_rear_left: f32,
    tire_slip_angle_rear_right: f32,
    tire_combined_slip_front_left: f32,
    tire_combined_slip_front_right: f32,
    tire_combined_slip_rear_left: f32,
    tire_combined_slip_rear_right: f32,
    suspension_travel_meters_front_left: f32,
    suspension_travel_meters_front_right: f32,
    suspension_travel_meters_rear_left: f32,
    suspension_travel_meters_rear_right: f32,
    car_class: i32,
    car_performance_index: i32,
    drivetrai32ype: i32,
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
    car_ordinal: i32,
    position_x: f32,
    position_y: f32,
    position_z: f32,
    speed: f32,
    power: f32,
    torque: f32,
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
    let (number_of_bytes, _) = socket.recv_from(&mut buffer).expect("no data received");
    number_of_bytes
}
fn main() {
    let args = Args::parse();
    let ip = format!("0.0.0.0:{}", args.port.to_string());

    if args.verbose {
        if args.folder != "" {
            println!("Folder: {}", args.folder);
        } else {
            println!("No folder specified - not saving logs");
        }
    }

    let socket = UdpSocket::bind(ip).expect("couldnt bind");
    let mut buf = [0; 500];

    while listen(&socket, &mut buf) != 0 {
        let deserialised: Telemetry = bincode::deserialize(&buf).expect("couldnt deser");
        if deserialised.current_engine_rpm == 0.0 {
            continue;
        }
        if args.verbose {
            println!(
                "Gear: {}, RPM: {}",
                deserialised.gear, deserialised.current_engine_rpm
            );
        }
    }
}
