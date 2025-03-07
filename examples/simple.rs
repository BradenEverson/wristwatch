//! Simple Example for loading json and parsing wrist angles

use std::f32::consts::PI;
use std::fs::File;
use std::io::Read;
use wristwatch::PoseData;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("files/soldering.json").expect("Load Soldering");
    file.read_to_string(&mut buf).expect("Read to buf");

    let pose: PoseData = serde_json::from_str(&buf).expect("Failed to parse");

    let right_rad = pose.people[0].angle_right_wrist();
    let right_deg = right_rad * (180.0 / PI);

    println!(
        "Right Wrist Angle in\n\tRadians: {:0.4} rad\n\tDegrees: {:0.2}°",
        right_rad, right_deg
    );

    let left_rad = pose.people[0].angle_left_wrist();
    let left_deg = left_rad * (180.0 / PI);

    println!(
        "Left Wrist Angle in\n\tRadians: {:0.4} rad\n\tDegrees: {:0.2}°",
        left_rad, left_deg
    );
}
