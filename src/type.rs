use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign};

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Hash, Eq)]
pub struct Size {
    _byte: u64,
}

impl Serialize for Size {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.as_byte())
    }
}

impl<'de> Deserialize<'de> for Size {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_byte(u64::deserialize(deserializer)?))
    }
}

impl AddAssign for Size {
    fn add_assign(&mut self, other: Self) {
        self._byte += other._byte;
    }
}

impl Add for Size {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::from_byte(self._byte + other._byte)
    }
}

impl Sub for Size {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::from_byte(if self._byte > other._byte {
            self._byte - other._byte
        } else {
            0
        })
    }
}

impl SubAssign for Size {
    fn sub_assign(&mut self, other: Self) {
        *self = Self::from_byte(if self.as_byte() > other.as_byte() {
            self.as_byte() - other.as_byte()
        } else {
            0
        });
    }
}

impl<T> From<T> for Size
where
    T: Into<u64>,
{
    fn from(value: T) -> Self {
        Self { _byte: value.into() }
    }
}

impl Size {
    pub fn set_byte(&mut self, value: u64) {
        self._byte = value;
    }
    pub fn bzero(&mut self) {
        self._byte = 0;
    }
    pub const fn from_byte(value: u64) -> Self {
        Self { _byte: value }
    }
    pub const fn from_kb(value: u64) -> Self {
        Self {
            _byte: value * 1024,
        }
    }
    pub const fn from_mb(value: u64) -> Self {
        Self {
            _byte: value * 1024,
        }
    }
    pub const fn from_gb(value: u64) -> Self {
        Self {
            _byte: value * 1024 * 1024,
        }
    }
    pub const fn from_pb(value: u64) -> Self {
        Self {
            _byte: value * 1024 * 1024 * 1024,
        }
    }
    pub const fn from_eb(value: u64) -> Self {
        Self {
            _byte: value * 1024 * 1024 * 1024 * 1024,
        }
    }
    pub const fn as_byte(&self) -> u64 {
        self._byte
    }
    pub const fn as_byte_f64(&self) -> f64 {
        self._byte as f64
    }
    pub fn as_kb_f64(&self) -> f64 {
        self.as_byte_f64() / 1024f64
    }
    pub const fn as_kb(&self) -> u64 {
        self.as_byte() / 1024
    }
    pub fn as_mb_f64(&self) -> f64 {
        self.as_kb_f64() / 1024f64
    }
    pub const fn as_mb(&self) -> u64 {
        self.as_kb() / 1024
    }
    pub fn as_gb_f64(&self) -> f64 {
        self.as_mb_f64() / 1024f64
    }
    pub const fn as_gb(&self) -> u64 {
        self.as_mb() / 1024
    }
    pub fn as_pb_f64(&self) -> f64 {
        self.as_gb_f64() / 1024f64
    }
    pub const fn as_pb(&self) -> u64 {
        self.as_gb() / 1024
    }
    pub fn as_eb_f64(&self) -> f64 {
        self.as_gb_f64() / 1024f64
    }
    pub const fn as_eb(&self) -> u64 {
        self.as_pb() / 1024
    }
}
