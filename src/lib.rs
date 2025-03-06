use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoseData {
    version: f32,
    people: Vec<Person>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    person_id: Vec<i32>,
    pose_keypoints_2d: Vec<f32>,
    face_keypoints_2d: Vec<f32>,
    hand_left_keypoints_2d: Vec<f32>,
    hand_right_keypoints_2d: Vec<f32>,
    pose_keypoints_3d: Vec<f32>,
    face_keypoints_3d: Vec<f32>,
    hand_left_keypoints_3d: Vec<f32>,
    hand_right_keypoints_3d: Vec<f32>,
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use crate::PoseData;

    #[test]
    fn loading_soldering_example() {
        let mut buf = String::new();
        let mut file = File::open("files/soldering.json").expect("Load Soldering");
        file.read_to_string(&mut buf).expect("Read to buf");

        let pose: PoseData = serde_json::from_str(&buf).expect("Failed to parse");
        panic!("{pose:?}")
    }
}
