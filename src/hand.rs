//! Hand pose implementation

use crate::{Person, Point};

pub const HAND_WRIST: usize = 0 * 3;
pub const THUMB_BASE: usize = 1 * 3;
pub const THUMB_MIDDLE: usize = 2 * 3;
pub const THUMB_UPPER: usize = 3 * 3;
pub const THUMB_TIP: usize = 4 * 3;
pub const INDEX_BASE: usize = 5 * 3;
pub const INDEX_MIDDLE: usize = 6 * 3;
pub const INDEX_UPPER: usize = 7 * 3;
pub const INDEX_TIP: usize = 8 * 3;
pub const MIDDLE_BASE: usize = 9 * 3;
pub const MIDDLE_MIDDLE: usize = 10 * 3;
pub const MIDDLE_UPPER: usize = 11 * 3;
pub const MIDDLE_TIP: usize = 12 * 3;
pub const RING_BASE: usize = 13 * 3;
pub const RING_MIDDLE: usize = 14 * 3;
pub const RING_UPPER: usize = 15 * 3;
pub const RING_TIP: usize = 16 * 3;
pub const PINKY_BASE: usize = 17 * 3;
pub const PINKY_MIDDLE: usize = 18 * 3;
pub const PINKY_UPPER: usize = 19 * 3;
pub const PINKY_TIP: usize = 20 * 3;

impl Person {
    /// Gets the person's left hand_wrist
    pub fn get_hand_wrist_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[HAND_WRIST],
            self.hand_left_keypoints_2d[HAND_WRIST + 1],
        )
    }
    /// Gets the person's left thumb_base
    pub fn get_thumb_base_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[THUMB_BASE],
            self.hand_left_keypoints_2d[THUMB_BASE + 1],
        )
    }
    /// Gets the person's left thumb_middle
    pub fn get_thumb_middle_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[THUMB_MIDDLE],
            self.hand_left_keypoints_2d[THUMB_MIDDLE + 1],
        )
    }
    /// Gets the person's left thumb_upper
    pub fn get_thumb_upper_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[THUMB_UPPER],
            self.hand_left_keypoints_2d[THUMB_UPPER + 1],
        )
    }
    /// Gets the person's left thumb_tip
    pub fn get_thumb_tip_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[THUMB_TIP],
            self.hand_left_keypoints_2d[THUMB_TIP + 1],
        )
    }
    /// Gets the person's left index_base
    pub fn get_index_base_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[INDEX_BASE],
            self.hand_left_keypoints_2d[INDEX_BASE + 1],
        )
    }
    /// Gets the person's left index_middle
    pub fn get_index_middle_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[INDEX_MIDDLE],
            self.hand_left_keypoints_2d[INDEX_MIDDLE + 1],
        )
    }
    /// Gets the person's left index_upper
    pub fn get_index_upper_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[INDEX_UPPER],
            self.hand_left_keypoints_2d[INDEX_UPPER + 1],
        )
    }
    /// Gets the person's left index_tip
    pub fn get_index_tip_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[INDEX_TIP],
            self.hand_left_keypoints_2d[INDEX_TIP + 1],
        )
    }
    /// Gets the person's left middle_base
    pub fn get_middle_base_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[MIDDLE_BASE],
            self.hand_left_keypoints_2d[MIDDLE_BASE + 1],
        )
    }
    /// Gets the person's left middle_middle
    pub fn get_middle_middle_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[MIDDLE_MIDDLE],
            self.hand_left_keypoints_2d[MIDDLE_MIDDLE + 1],
        )
    }
    /// Gets the person's left middle_upper
    pub fn get_middle_upper_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[MIDDLE_UPPER],
            self.hand_left_keypoints_2d[MIDDLE_UPPER + 1],
        )
    }
    /// Gets the person's left middle_tip
    pub fn get_middle_tip_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[MIDDLE_TIP],
            self.hand_left_keypoints_2d[MIDDLE_TIP + 1],
        )
    }
    /// Gets the person's left ring_base
    pub fn get_ring_base_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[RING_BASE],
            self.hand_left_keypoints_2d[RING_BASE + 1],
        )
    }
    /// Gets the person's left ring_middle
    pub fn get_ring_middle_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[RING_MIDDLE],
            self.hand_left_keypoints_2d[RING_MIDDLE + 1],
        )
    }
    /// Gets the person's left ring_upper
    pub fn get_ring_upper_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[RING_UPPER],
            self.hand_left_keypoints_2d[RING_UPPER + 1],
        )
    }
    /// Gets the person's left ring_tip
    pub fn get_ring_tip_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[RING_TIP],
            self.hand_left_keypoints_2d[RING_TIP + 1],
        )
    }
    /// Gets the person's left pinky_base
    pub fn get_pinky_base_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[PINKY_BASE],
            self.hand_left_keypoints_2d[PINKY_BASE + 1],
        )
    }
    /// Gets the person's left pinky_middle
    pub fn get_pinky_middle_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[PINKY_MIDDLE],
            self.hand_left_keypoints_2d[PINKY_MIDDLE + 1],
        )
    }
    /// Gets the person's left pinky_upper
    pub fn get_pinky_upper_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[PINKY_UPPER],
            self.hand_left_keypoints_2d[PINKY_UPPER + 1],
        )
    }
    /// Gets the person's left pinky_tip
    pub fn get_pinky_tip_left(&self) -> Point {
        Point(
            self.hand_left_keypoints_2d[PINKY_TIP],
            self.hand_left_keypoints_2d[PINKY_TIP + 1],
        )
    }

    /// Gets the person's right hand_wrist
    pub fn get_hand_wrist_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[HAND_WRIST],
            self.hand_right_keypoints_2d[HAND_WRIST + 1],
        )
    }
    /// Gets the person's right thumb_base
    pub fn get_thumb_base_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[THUMB_BASE],
            self.hand_right_keypoints_2d[THUMB_BASE + 1],
        )
    }
    /// Gets the person's right thumb_middle
    pub fn get_thumb_middle_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[THUMB_MIDDLE],
            self.hand_right_keypoints_2d[THUMB_MIDDLE + 1],
        )
    }
    /// Gets the person's right thumb_upper
    pub fn get_thumb_upper_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[THUMB_UPPER],
            self.hand_right_keypoints_2d[THUMB_UPPER + 1],
        )
    }
    /// Gets the person's right thumb_tip
    pub fn get_thumb_tip_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[THUMB_TIP],
            self.hand_right_keypoints_2d[THUMB_TIP + 1],
        )
    }
    /// Gets the person's right index_base
    pub fn get_index_base_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[INDEX_BASE],
            self.hand_right_keypoints_2d[INDEX_BASE + 1],
        )
    }
    /// Gets the person's right index_middle
    pub fn get_index_middle_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[INDEX_MIDDLE],
            self.hand_right_keypoints_2d[INDEX_MIDDLE + 1],
        )
    }
    /// Gets the person's right index_upper
    pub fn get_index_upper_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[INDEX_UPPER],
            self.hand_right_keypoints_2d[INDEX_UPPER + 1],
        )
    }
    /// Gets the person's right index_tip
    pub fn get_index_tip_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[INDEX_TIP],
            self.hand_right_keypoints_2d[INDEX_TIP + 1],
        )
    }
    /// Gets the person's right middle_base
    pub fn get_middle_base_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[MIDDLE_BASE],
            self.hand_right_keypoints_2d[MIDDLE_BASE + 1],
        )
    }
    /// Gets the person's right middle_middle
    pub fn get_middle_middle_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[MIDDLE_MIDDLE],
            self.hand_right_keypoints_2d[MIDDLE_MIDDLE + 1],
        )
    }
    /// Gets the person's right middle_upper
    pub fn get_middle_upper_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[MIDDLE_UPPER],
            self.hand_right_keypoints_2d[MIDDLE_UPPER + 1],
        )
    }
    /// Gets the person's right middle_tip
    pub fn get_middle_tip_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[MIDDLE_TIP],
            self.hand_right_keypoints_2d[MIDDLE_TIP + 1],
        )
    }
    /// Gets the person's right ring_base
    pub fn get_ring_base_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[RING_BASE],
            self.hand_right_keypoints_2d[RING_BASE + 1],
        )
    }
    /// Gets the person's right ring_middle
    pub fn get_ring_middle_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[RING_MIDDLE],
            self.hand_right_keypoints_2d[RING_MIDDLE + 1],
        )
    }
    /// Gets the person's right ring_upper
    pub fn get_ring_upper_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[RING_UPPER],
            self.hand_right_keypoints_2d[RING_UPPER + 1],
        )
    }
    /// Gets the person's right ring_tip
    pub fn get_ring_tip_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[RING_TIP],
            self.hand_right_keypoints_2d[RING_TIP + 1],
        )
    }
    /// Gets the person's right pinky_base
    pub fn get_pinky_base_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[PINKY_BASE],
            self.hand_right_keypoints_2d[PINKY_BASE + 1],
        )
    }
    /// Gets the person's right pinky_middle
    pub fn get_pinky_middle_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[PINKY_MIDDLE],
            self.hand_right_keypoints_2d[PINKY_MIDDLE + 1],
        )
    }
    /// Gets the person's right pinky_upper
    pub fn get_pinky_upper_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[PINKY_UPPER],
            self.hand_right_keypoints_2d[PINKY_UPPER + 1],
        )
    }
    /// Gets the person's right pinky_tip
    pub fn get_pinky_tip_right(&self) -> Point {
        Point(
            self.hand_right_keypoints_2d[PINKY_TIP],
            self.hand_right_keypoints_2d[PINKY_TIP + 1],
        )
    }
}
