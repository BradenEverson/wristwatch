//! Body mapping functions

use crate::{Person, Point};

pub const CENTER_FACE: usize = 0 * 3;
pub const RIGHT_EYE: usize = 15 * 3;
pub const RIGHT_EAR: usize = 17 * 3;
pub const LEFT_EYE: usize = 16 * 3;
pub const LEFT_EAR: usize = 18 * 3;
pub const STERNUM: usize = 1 * 3;
pub const RIGHT_SHOULDER: usize = 2 * 3;
pub const RIGHT_ELBOW: usize = 3 * 3;
pub const RIGHT_WRIST: usize = 4 * 3;
pub const LEFT_SHOULDER: usize = 5 * 3;
pub const LEFT_ELBOW: usize = 6 * 3;
pub const LEFT_WRIST: usize = 7 * 3;
pub const PELVIS: usize = 8 * 3;
pub const LEFT_HIP: usize = 12 * 3;
pub const LEFT_KNEE: usize = 13 * 3;
pub const LEFT_ANKLE: usize = 14 * 3;
pub const LEFT_HEEL: usize = 21 * 3;
pub const LEFT_FOOT: usize = 19 * 3;
pub const LEFT_TOES: usize = 20 * 3;
pub const RIGHT_HIP: usize = 9 * 3;
pub const RIGHT_KNEE: usize = 10 * 3;
pub const RIGHT_ANKLE: usize = 11 * 3;
pub const RIGHT_HEEL: usize = 24 * 3;
pub const RIGHT_FOOT: usize = 22 * 3;
pub const RIGHT_TOES: usize = 23 * 3;

impl Person {
    /// Gets the person's center_face
    pub fn get_center_face(&self) -> Point {
        Point(
            self.pose_keypoints_2d[CENTER_FACE],
            self.pose_keypoints_2d[CENTER_FACE + 1],
        )
    }

    /// Gets the person's right_eye
    pub fn get_right_eye(&self) -> Point {
        Point(
            self.pose_keypoints_2d[RIGHT_EYE],
            self.pose_keypoints_2d[RIGHT_EYE + 1],
        )
    }

    /// Gets the person's right_ear
    pub fn get_right_ear(&self) -> Point {
        Point(
            self.pose_keypoints_2d[RIGHT_EAR],
            self.pose_keypoints_2d[RIGHT_EAR + 1],
        )
    }

    /// Gets the person's left_eye
    pub fn get_left_eye(&self) -> Point {
        Point(
            self.pose_keypoints_2d[LEFT_EYE],
            self.pose_keypoints_2d[LEFT_EYE + 1],
        )
    }

    /// Gets the person's left_ear
    pub fn get_left_ear(&self) -> Point {
        Point(
            self.pose_keypoints_2d[LEFT_EAR],
            self.pose_keypoints_2d[LEFT_EAR + 1],
        )
    }

    /// Gets the person's right_shoulder
    pub fn get_right_shoulder(&self) -> Point {
        Point(
            self.pose_keypoints_2d[RIGHT_SHOULDER],
            self.pose_keypoints_2d[RIGHT_SHOULDER + 1],
        )
    }

    /// Gets the person's right_elbow
    pub fn get_right_elbow(&self) -> Point {
        Point(
            self.pose_keypoints_2d[RIGHT_ELBOW],
            self.pose_keypoints_2d[RIGHT_ELBOW + 1],
        )
    }

    /// Gets the person's right_wrist
    pub fn get_right_wrist(&self) -> Point {
        Point(
            self.pose_keypoints_2d[RIGHT_WRIST],
            self.pose_keypoints_2d[RIGHT_WRIST + 1],
        )
    }

    /// Gets the person's left_shoulder
    pub fn get_left_shoulder(&self) -> Point {
        Point(
            self.pose_keypoints_2d[LEFT_SHOULDER],
            self.pose_keypoints_2d[LEFT_SHOULDER + 1],
        )
    }

    /// Gets the person's left_elbow
    pub fn get_left_elbow(&self) -> Point {
        Point(
            self.pose_keypoints_2d[LEFT_ELBOW],
            self.pose_keypoints_2d[LEFT_ELBOW + 1],
        )
    }

    /// Gets the person's left_wrist
    pub fn get_left_wrist(&self) -> Point {
        Point(
            self.pose_keypoints_2d[LEFT_WRIST],
            self.pose_keypoints_2d[LEFT_WRIST + 1],
        )
    }

    /// Gets the person's left_hip
    pub fn get_left_hip(&self) -> Point {
        Point(
            self.pose_keypoints_2d[LEFT_HIP],
            self.pose_keypoints_2d[LEFT_HIP + 1],
        )
    }

    /// Gets the person's left_knee
    pub fn get_left_knee(&self) -> Point {
        Point(
            self.pose_keypoints_2d[LEFT_KNEE],
            self.pose_keypoints_2d[LEFT_KNEE + 1],
        )
    }

    /// Gets the person's left_ankle
    pub fn get_left_ankle(&self) -> Point {
        Point(
            self.pose_keypoints_2d[LEFT_ANKLE],
            self.pose_keypoints_2d[LEFT_ANKLE + 1],
        )
    }

    /// Gets the person's left_heel
    pub fn get_left_heel(&self) -> Point {
        Point(
            self.pose_keypoints_2d[LEFT_HEEL],
            self.pose_keypoints_2d[LEFT_HEEL + 1],
        )
    }

    /// Gets the person's left_foot
    pub fn get_left_foot(&self) -> Point {
        Point(
            self.pose_keypoints_2d[LEFT_FOOT],
            self.pose_keypoints_2d[LEFT_FOOT + 1],
        )
    }

    /// Gets the person's left_toes
    pub fn get_left_toes(&self) -> Point {
        Point(
            self.pose_keypoints_2d[LEFT_TOES],
            self.pose_keypoints_2d[LEFT_TOES + 1],
        )
    }

    /// Gets the person's right_hip
    pub fn get_right_hip(&self) -> Point {
        Point(
            self.pose_keypoints_2d[RIGHT_HIP],
            self.pose_keypoints_2d[RIGHT_HIP + 1],
        )
    }

    /// Gets the person's right_knee
    pub fn get_right_knee(&self) -> Point {
        Point(
            self.pose_keypoints_2d[RIGHT_KNEE],
            self.pose_keypoints_2d[RIGHT_KNEE + 1],
        )
    }

    /// Gets the person's right_ankle
    pub fn get_right_ankle(&self) -> Point {
        Point(
            self.pose_keypoints_2d[RIGHT_ANKLE],
            self.pose_keypoints_2d[RIGHT_ANKLE + 1],
        )
    }

    /// Gets the person's right_heel
    pub fn get_right_heel(&self) -> Point {
        Point(
            self.pose_keypoints_2d[RIGHT_HEEL],
            self.pose_keypoints_2d[RIGHT_HEEL + 1],
        )
    }

    /// Gets the person's right_foot
    pub fn get_right_foot(&self) -> Point {
        Point(
            self.pose_keypoints_2d[RIGHT_FOOT],
            self.pose_keypoints_2d[RIGHT_FOOT + 1],
        )
    }

    /// Gets the person's right_toes
    pub fn get_right_toes(&self) -> Point {
        Point(
            self.pose_keypoints_2d[RIGHT_TOES],
            self.pose_keypoints_2d[RIGHT_TOES + 1],
        )
    }

    /// Gets the person's pelvis
    pub fn get_pelvis(&self) -> Point {
        Point(
            self.pose_keypoints_2d[PELVIS],
            self.pose_keypoints_2d[PELVIS + 1],
        )
    }

    /// Gets the person's sternum
    pub fn get_sternum(&self) -> Point {
        Point(
            self.pose_keypoints_2d[STERNUM],
            self.pose_keypoints_2d[STERNUM + 1],
        )
    }
}
