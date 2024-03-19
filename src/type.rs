use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign};

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Hash, Eq)]
pub struct Size {
    byte: u64,
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
        *self = Self::from_byte(self.as_byte() + other.as_byte());
    }
}

impl Deref for Size {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.byte
    }
}

impl DerefMut for Size {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.byte
    }
}

impl Add for Size {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::from_byte(self.as_byte() + other.as_byte())
    }
}

impl Sub for Size {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::from_byte(if self.as_byte() > other.as_byte() {
            self.as_byte() - other.as_byte()
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
        Self { byte: value.into() }
    }
}

impl Size {
    pub fn set_byte(&mut self, byte: impl Into<u64>) {
        self.byte = byte.into();
    }
    pub fn reset(&mut self) {
        self.byte = 0;
    }
    pub fn from_byte(byte: impl Into<u64>) -> Self {
        Self { byte: byte.into() }
    }
    pub fn from_kb(byte: impl Into<u64>) -> Self {
        Self {
            byte: byte.into() * 1024,
        }
    }
    pub fn from_mb(byte: impl Into<u64>) -> Self {
        Self {
            byte: byte.into() * 1024,
        }
    }
    pub fn from_gb(byte: impl Into<u64>) -> Self {
        Self {
            byte: byte.into() * 1024 * 1024,
        }
    }
    pub fn from_pb(byte: impl Into<u64>) -> Self {
        Self {
            byte: byte.into() * 1024 * 1024 * 1024,
        }
    }
    pub fn from_eb(byte: impl Into<u64>) -> Self {
        Self {
            byte: byte.into() * 1024 * 1024 * 1024 * 1024,
        }
    }
    pub fn as_byte(&self) -> u64 {
        self.byte
    }
    pub fn as_kb_f64(&self) -> f64 {
        self.byte as f64 / 1024f64
    }
    pub fn as_kb(&self) -> u64 {
        self.byte / 1024
    }
    pub fn as_mb_f64(&self) -> f64 {
        self.as_kb_f64() / 1024f64
    }
    pub fn as_mb(&self) -> u64 {
        self.byte / 1024 / 1024
    }
    pub fn as_gb_f64(&self) -> f64 {
        self.as_mb_f64() / 1024f64
    }
    pub fn as_gb(&self) -> u64 {
        self.byte / 1024 / 1024 / 1024
    }
    pub fn as_pb_f64(&self) -> f64 {
        self.as_gb_f64() / 1024f64
    }
    pub fn as_pb(&self) -> u64 {
        self.byte / 1024 / 1024 / 1024 / 1024
    }
    pub fn as_eb_f64(&self) -> f64 {
        self.as_gb_f64() / 1024f64
    }
    pub fn as_eb(&self) -> u64 {
        self.byte / 1024 / 1024 / 1024 / 1024 / 1024
    }
}
