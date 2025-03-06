//! Body mapping functions

use crate::Person;

pub const CENTER_FACE: usize = 0;

pub const RIGHT_EYE: usize = 15;
pub const RIGHT_EAR: usize = 17;

pub const LEFT_EYE: usize = 16;
pub const LEFT_EAR: usize = 18;

pub const STERNUM: usize = 1;

pub const RIGHT_SHOULDER: usize = 2;
pub const RIGHT_ELBOW: usize = 3;
pub const RIGHT_WRIST: usize = 4;

pub const LEFT_SHOULDER: usize = 5;
pub const LEFT_ELBOW: usize = 6;
pub const LEFT_WRIST: usize = 7;

pub const PELVIS: usize = 8;

pub const LEFT_HIP: usize = 12;
pub const LEFT_KNEE: usize = 13;
pub const LEFT_ANKLE: usize = 14;
pub const LEFT_HEEL: usize = 21;
pub const LEFT_FOOT: usize = 19;
pub const LEFT_TOES: usize = 20;

pub const RIGHT_HIP: usize = 9;
pub const RIGHT_KNEE: usize = 10;
pub const RIGHT_ANKLE: usize = 11;
pub const RIGHT_HEEL: usize = 24;
pub const RIGHT_FOOT: usize = 22;
pub const RIGHT_TOES: usize = 23;

impl Person {
    /// Gets the person's center_face
    pub fn get_center_face(&self) -> f32 {
        self.pose_keypoints_2d[CENTER_FACE]
    }

    /// Gets the person's right_eye
    pub fn get_right_eye(&self) -> f32 {
        self.pose_keypoints_2d[RIGHT_EYE]
    }

    /// Gets the person's right_ear
    pub fn get_right_ear(&self) -> f32 {
        self.pose_keypoints_2d[RIGHT_EAR]
    }

    /// Gets the person's left_eye
    pub fn get_left_eye(&self) -> f32 {
        self.pose_keypoints_2d[LEFT_EYE]
    }

    /// Gets the person's left_ear
    pub fn get_left_ear(&self) -> f32 {
        self.pose_keypoints_2d[LEFT_EAR]
    }

    /// Gets the person's right_shoulder
    pub fn get_right_shoulder(&self) -> f32 {
        self.pose_keypoints_2d[RIGHT_SHOULDER]
    }

    /// Gets the person's right_elbow
    pub fn get_right_elbow(&self) -> f32 {
        self.pose_keypoints_2d[RIGHT_ELBOW]
    }

    /// Gets the person's right_wrist
    pub fn get_right_wrist(&self) -> f32 {
        self.pose_keypoints_2d[RIGHT_WRIST]
    }

    /// Gets the person's left_shoulder
    pub fn get_left_shoulder(&self) -> f32 {
        self.pose_keypoints_2d[LEFT_SHOULDER]
    }

    /// Gets the person's left_elbow
    pub fn get_left_elbow(&self) -> f32 {
        self.pose_keypoints_2d[LEFT_ELBOW]
    }

    /// Gets the person's left_wrist
    pub fn get_left_wrist(&self) -> f32 {
        self.pose_keypoints_2d[LEFT_WRIST]
    }

    /// Gets the person's left_hip
    pub fn get_left_hip(&self) -> f32 {
        self.pose_keypoints_2d[LEFT_HIP]
    }

    /// Gets the person's left_knee
    pub fn get_left_knee(&self) -> f32 {
        self.pose_keypoints_2d[LEFT_KNEE]
    }

    /// Gets the person's left_ankle
    pub fn get_left_ankle(&self) -> f32 {
        self.pose_keypoints_2d[LEFT_ANKLE]
    }

    /// Gets the person's left_heel
    pub fn get_left_heel(&self) -> f32 {
        self.pose_keypoints_2d[LEFT_HEEL]
    }

    /// Gets the person's left_foot
    pub fn get_left_foot(&self) -> f32 {
        self.pose_keypoints_2d[LEFT_FOOT]
    }

    /// Gets the person's left_toes
    pub fn get_left_toes(&self) -> f32 {
        self.pose_keypoints_2d[LEFT_TOES]
    }

    /// Gets the person's right_hip
    pub fn get_right_hip(&self) -> f32 {
        self.pose_keypoints_2d[RIGHT_HIP]
    }

    /// Gets the person's right_knee
    pub fn get_right_knee(&self) -> f32 {
        self.pose_keypoints_2d[RIGHT_KNEE]
    }

    /// Gets the person's right_ankle
    pub fn get_right_ankle(&self) -> f32 {
        self.pose_keypoints_2d[RIGHT_ANKLE]
    }

    /// Gets the person's right_heel
    pub fn get_right_heel(&self) -> f32 {
        self.pose_keypoints_2d[RIGHT_HEEL]
    }

    /// Gets the person's right_foot
    pub fn get_right_foot(&self) -> f32 {
        self.pose_keypoints_2d[RIGHT_FOOT]
    }

    /// Gets the person's right_toes
    pub fn get_right_toes(&self) -> f32 {
        self.pose_keypoints_2d[RIGHT_TOES]
    }

    /// Gets the person's pelvis
    pub fn get_pelvis(&self) -> f32 {
        self.pose_keypoints_2d[PELVIS]
    }

    /// Gets the person's sternum
    pub fn get_sternum(&self) -> f32 {
        self.pose_keypoints_2d[STERNUM]
    }
}
