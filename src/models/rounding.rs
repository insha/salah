// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Debug, Copy, Clone)]
/// Policy used when converting calculated instants to minute precision.
///
/// # Examples
///
/// ```
/// use salah::Rounding;
///
/// let policy = Rounding::Nearest;
/// assert_eq!(policy, Rounding::Nearest);
/// ```
pub enum Rounding {
    /// Round to the nearest minute.
    Nearest,
    /// Round upward to the next minute.
    Up,
    /// Preserve the calculated precision.
    None,
}
