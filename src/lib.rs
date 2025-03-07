use math::{LineSegment, Point};
use serde::{Deserialize, Serialize};

pub mod body;
pub mod hand;
pub mod math;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoseData {
    pub version: f32,
    pub people: Vec<Person>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub person_id: Vec<i32>,
    pub pose_keypoints_2d: Vec<f32>,
    pub face_keypoints_2d: Vec<f32>,
    pub hand_left_keypoints_2d: Vec<f32>,
    pub hand_right_keypoints_2d: Vec<f32>,
    pub pose_keypoints_3d: Vec<f32>,
    pub face_keypoints_3d: Vec<f32>,
    pub hand_left_keypoints_3d: Vec<f32>,
    pub hand_right_keypoints_3d: Vec<f32>,
}

impl Person {
    pub fn angle_left_wrist(&self) -> f32 {
        let elbow = self.get_left_elbow();
        let wrist = self.get_left_wrist();

        let wrist_base = {
            let Point(tx, ty) = self.get_thumb_base_left();
            let Point(ix, iy) = self.get_index_base_left();
            let Point(mx, my) = self.get_middle_base_left();
            let Point(rx, ry) = self.get_ring_base_left();
            let Point(px, py) = self.get_pinky_base_left();

            Point(
                (tx + ix + mx + rx + px) / 5.0,
                (ty + iy + my + ry + py) / 5.0,
            )
        };

        let origin = LineSegment(elbow, wrist);
        let delta = LineSegment(wrist, wrist_base);

        let origin_v = origin.vector();
        let delta_v = delta.vector();

        origin_v.angle_between(&delta_v)
    }

    pub fn angle_right_wrist(&self) -> f32 {
        let elbow = self.get_right_elbow();
        let wrist = self.get_right_wrist();

        let wrist_base = {
            let Point(tx, ty) = self.get_thumb_base_right();
            let Point(ix, iy) = self.get_index_base_right();
            let Point(mx, my) = self.get_middle_base_right();
            let Point(rx, ry) = self.get_ring_base_right();
            let Point(px, py) = self.get_pinky_base_right();

            Point(
                (tx + ix + mx + rx + px) / 5.0,
                (ty + iy + my + ry + py) / 5.0,
            )
        };

        let origin = LineSegment(elbow, wrist);
        let delta = LineSegment(wrist, wrist_base);

        let origin_v = origin.vector();
        let delta_v = delta.vector();

        origin_v.angle_between(&delta_v)
    }
}
