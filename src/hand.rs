//! Hand pose implementation

use crate::Person;

pub const HAND_WRIST: usize = 0;

pub const THUMB_BASE: usize = 1;
pub const THUMB_MIDDLE: usize = 2;
pub const THUMB_UPPER: usize = 3;
pub const THUMB_TIP: usize = 4;

pub const INDEX_BASE: usize = 5;
pub const INDEX_MIDDLE: usize = 6;
pub const INDEX_UPPER: usize = 7;
pub const INDEX_TIP: usize = 8;

pub const MIDDLE_BASE: usize = 9;
pub const MIDDLE_MIDDLE: usize = 10;
pub const MIDDLE_UPPER: usize = 11;
pub const MIDDLE_TIP: usize = 12;

pub const RING_BASE: usize = 13;
pub const RING_MIDDLE: usize = 14;
pub const RING_UPPER: usize = 15;
pub const RING_TIP: usize = 16;

pub const PINKY_BASE: usize = 17;
pub const PINKY_MIDDLE: usize = 18;
pub const PINKY_UPPER: usize = 19;
pub const PINKY_TIP: usize = 20;

impl Person {
    /// Gets the person's left hand_wrist
    pub fn get_hand_wrist_left(&self) -> f32 {
        self.hand_left_keypoints_2d[HAND_WRIST]
    }
    /// Gets the person's left thumb_base
    pub fn get_thumb_base_left(&self) -> f32 {
        self.hand_left_keypoints_2d[THUMB_BASE]
    }
    /// Gets the person's left thumb_middle
    pub fn get_thumb_middle_left(&self) -> f32 {
        self.hand_left_keypoints_2d[THUMB_MIDDLE]
    }
    /// Gets the person's left thumb_upper
    pub fn get_thumb_upper_left(&self) -> f32 {
        self.hand_left_keypoints_2d[THUMB_UPPER]
    }
    /// Gets the person's left thumb_tip
    pub fn get_thumb_tip_left(&self) -> f32 {
        self.hand_left_keypoints_2d[THUMB_TIP]
    }
    /// Gets the person's left index_base
    pub fn get_index_base_left(&self) -> f32 {
        self.hand_left_keypoints_2d[INDEX_BASE]
    }
    /// Gets the person's left index_middle
    pub fn get_index_middle_left(&self) -> f32 {
        self.hand_left_keypoints_2d[INDEX_MIDDLE]
    }
    /// Gets the person's left index_upper
    pub fn get_index_upper_left(&self) -> f32 {
        self.hand_left_keypoints_2d[INDEX_UPPER]
    }
    /// Gets the person's left index_tip
    pub fn get_index_tip_left(&self) -> f32 {
        self.hand_left_keypoints_2d[INDEX_TIP]
    }
    /// Gets the person's left middle_base
    pub fn get_middle_base_left(&self) -> f32 {
        self.hand_left_keypoints_2d[MIDDLE_BASE]
    }
    /// Gets the person's left middle_middle
    pub fn get_middle_middle_left(&self) -> f32 {
        self.hand_left_keypoints_2d[MIDDLE_MIDDLE]
    }
    /// Gets the person's left middle_upper
    pub fn get_middle_upper_left(&self) -> f32 {
        self.hand_left_keypoints_2d[MIDDLE_UPPER]
    }
    /// Gets the person's left middle_tip
    pub fn get_middle_tip_left(&self) -> f32 {
        self.hand_left_keypoints_2d[MIDDLE_TIP]
    }
    /// Gets the person's left ring_base
    pub fn get_ring_base_left(&self) -> f32 {
        self.hand_left_keypoints_2d[RING_BASE]
    }
    /// Gets the person's left ring_middle
    pub fn get_ring_middle_left(&self) -> f32 {
        self.hand_left_keypoints_2d[RING_MIDDLE]
    }
    /// Gets the person's left ring_upper
    pub fn get_ring_upper_left(&self) -> f32 {
        self.hand_left_keypoints_2d[RING_UPPER]
    }
    /// Gets the person's left ring_tip
    pub fn get_ring_tip_left(&self) -> f32 {
        self.hand_left_keypoints_2d[RING_TIP]
    }
    /// Gets the person's left pinky_base
    pub fn get_pinky_base_left(&self) -> f32 {
        self.hand_left_keypoints_2d[PINKY_BASE]
    }
    /// Gets the person's left pinky_middle
    pub fn get_pinky_middle_left(&self) -> f32 {
        self.hand_left_keypoints_2d[PINKY_MIDDLE]
    }
    /// Gets the person's left pinky_upper
    pub fn get_pinky_upper_left(&self) -> f32 {
        self.hand_left_keypoints_2d[PINKY_UPPER]
    }
    /// Gets the person's left pinky_tip
    pub fn get_pinky_tip_left(&self) -> f32 {
        self.hand_left_keypoints_2d[PINKY_TIP]
    }

    /// Gets the person's right hand_wrist
    pub fn get_hand_wrist_right(&self) -> f32 {
        self.hand_right_keypoints_2d[HAND_WRIST]
    }
    /// Gets the person's right thumb_base
    pub fn get_thumb_base_right(&self) -> f32 {
        self.hand_right_keypoints_2d[THUMB_BASE]
    }
    /// Gets the person's right thumb_middle
    pub fn get_thumb_middle_right(&self) -> f32 {
        self.hand_right_keypoints_2d[THUMB_MIDDLE]
    }
    /// Gets the person's right thumb_upper
    pub fn get_thumb_upper_right(&self) -> f32 {
        self.hand_right_keypoints_2d[THUMB_UPPER]
    }
    /// Gets the person's right thumb_tip
    pub fn get_thumb_tip_right(&self) -> f32 {
        self.hand_right_keypoints_2d[THUMB_TIP]
    }
    /// Gets the person's right index_base
    pub fn get_index_base_right(&self) -> f32 {
        self.hand_right_keypoints_2d[INDEX_BASE]
    }
    /// Gets the person's right index_middle
    pub fn get_index_middle_right(&self) -> f32 {
        self.hand_right_keypoints_2d[INDEX_MIDDLE]
    }
    /// Gets the person's right index_upper
    pub fn get_index_upper_right(&self) -> f32 {
        self.hand_right_keypoints_2d[INDEX_UPPER]
    }
    /// Gets the person's right index_tip
    pub fn get_index_tip_right(&self) -> f32 {
        self.hand_right_keypoints_2d[INDEX_TIP]
    }
    /// Gets the person's right middle_base
    pub fn get_middle_base_right(&self) -> f32 {
        self.hand_right_keypoints_2d[MIDDLE_BASE]
    }
    /// Gets the person's right middle_middle
    pub fn get_middle_middle_right(&self) -> f32 {
        self.hand_right_keypoints_2d[MIDDLE_MIDDLE]
    }
    /// Gets the person's right middle_upper
    pub fn get_middle_upper_right(&self) -> f32 {
        self.hand_right_keypoints_2d[MIDDLE_UPPER]
    }
    /// Gets the person's right middle_tip
    pub fn get_middle_tip_right(&self) -> f32 {
        self.hand_right_keypoints_2d[MIDDLE_TIP]
    }
    /// Gets the person's right ring_base
    pub fn get_ring_base_right(&self) -> f32 {
        self.hand_right_keypoints_2d[RING_BASE]
    }
    /// Gets the person's right ring_middle
    pub fn get_ring_middle_right(&self) -> f32 {
        self.hand_right_keypoints_2d[RING_MIDDLE]
    }
    /// Gets the person's right ring_upper
    pub fn get_ring_upper_right(&self) -> f32 {
        self.hand_right_keypoints_2d[RING_UPPER]
    }
    /// Gets the person's right ring_tip
    pub fn get_ring_tip_right(&self) -> f32 {
        self.hand_right_keypoints_2d[RING_TIP]
    }
    /// Gets the person's right pinky_base
    pub fn get_pinky_base_right(&self) -> f32 {
        self.hand_right_keypoints_2d[PINKY_BASE]
    }
    /// Gets the person's right pinky_middle
    pub fn get_pinky_middle_right(&self) -> f32 {
        self.hand_right_keypoints_2d[PINKY_MIDDLE]
    }
    /// Gets the person's right pinky_upper
    pub fn get_pinky_upper_right(&self) -> f32 {
        self.hand_right_keypoints_2d[PINKY_UPPER]
    }
    /// Gets the person's right pinky_tip
    pub fn get_pinky_tip_right(&self) -> f32 {
        self.hand_right_keypoints_2d[PINKY_TIP]
    }
}
