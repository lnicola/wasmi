use core::marker::PhantomData;
use wasmi_core::F32;

/// A typed 16-bit encoded constant value.
#[derive(Debug)]
pub struct Const16<T> {
    /// The underlying untyped value.
    inner: AnyConst16,
    /// The type marker to satisfy the Rust type system.
    marker: PhantomData<fn() -> T>,
}

impl<T> Const16<T> {
    /// Crete a new typed [`Const16`] value.
    pub fn new(inner: AnyConst16) -> Self {
        Self {
            inner,
            marker: PhantomData,
        }
    }
}

impl<T> Clone for Const16<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Const16<T> {}

impl<T> PartialEq for Const16<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T> Eq for Const16<T> {}

impl From<i16> for Const16<i32> {
    fn from(value: i16) -> Self {
        Self::new(AnyConst16::from_i16(value))
    }
}

impl From<u16> for Const16<u32> {
    fn from(value: u16) -> Self {
        Self::new(AnyConst16::from_u16(value))
    }
}

impl From<i16> for Const16<i64> {
    fn from(value: i16) -> Self {
        Self::new(AnyConst16::from_i16(value))
    }
}

impl From<u16> for Const16<u64> {
    fn from(value: u16) -> Self {
        Self::new(AnyConst16::from_u16(value))
    }
}

impl From<Const16<i32>> for i32 {
    fn from(value: Const16<i32>) -> Self {
        value.inner.to_i32()
    }
}

impl From<Const16<u32>> for u32 {
    fn from(value: Const16<u32>) -> Self {
        value.inner.to_u32()
    }
}

impl From<Const16<i64>> for i64 {
    fn from(value: Const16<i64>) -> Self {
        value.inner.to_i64()
    }
}

impl From<Const16<u64>> for u64 {
    fn from(value: Const16<u64>) -> Self {
        value.inner.to_u64()
    }
}

impl Const16<i32> {
    pub fn from_i32(value: i32) -> Option<Self> {
        i16::try_from(value).map(Self::from).ok()
    }
}

impl Const16<u32> {
    pub fn from_u32(value: u32) -> Option<Self> {
        u16::try_from(value).map(Self::from).ok()
    }
}

impl Const16<i64> {
    pub fn from_i64(value: i64) -> Option<Self> {
        i16::try_from(value).map(Self::from).ok()
    }
}

impl Const16<u64> {
    pub fn from_u64(value: u64) -> Option<Self> {
        u16::try_from(value).map(Self::from).ok()
    }
}

/// A 16-bit constant value of any type.
///
/// # Note
///
/// Can be used to store information about small integer values.
/// Upon use the small 16-bit value has to be sign-extended to
/// the actual integer type, e.g. `i32` or `i64`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AnyConst16(i16);

/// Error that may occur upon converting values to [`Const16`].
#[derive(Debug, Copy, Clone)]
pub struct OutOfBoundsConst16;

impl TryFrom<i32> for AnyConst16 {
    type Error = OutOfBoundsConst16;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_i32(value).ok_or(OutOfBoundsConst16)
    }
}

impl TryFrom<u32> for AnyConst16 {
    type Error = OutOfBoundsConst16;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_u32(value).ok_or(OutOfBoundsConst16)
    }
}

impl TryFrom<i64> for AnyConst16 {
    type Error = OutOfBoundsConst16;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Self::from_i64(value).ok_or(OutOfBoundsConst16)
    }
}

impl TryFrom<u64> for AnyConst16 {
    type Error = OutOfBoundsConst16;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::from_u64(value).ok_or(OutOfBoundsConst16)
    }
}

impl From<i16> for AnyConst16 {
    fn from(value: i16) -> Self {
        Self::from_i16(value)
    }
}

impl From<u16> for AnyConst16 {
    fn from(value: u16) -> Self {
        Self::from_u16(value)
    }
}

impl AnyConst16 {
    /// Creates an [`Const16`] from the given `i16` value.
    pub fn from_i16(value: i16) -> Self {
        Self(value)
    }

    /// Creates an [`Const16`] from the given `u16` value.
    pub fn from_u16(value: u16) -> Self {
        Self::from_i16(value as i16)
    }

    /// Creates an [`Const16`] from the given `i32` value if possible.
    pub fn from_i32(value: i32) -> Option<Self> {
        i16::try_from(value).ok().map(Self)
    }

    /// Creates an [`Const16`] from the given `u32` value if possible.
    pub fn from_u32(value: u32) -> Option<Self> {
        let value = u16::try_from(value).ok()? as i16;
        Some(Self(value))
    }

    /// Creates an [`Const16`] from the given `i64` value if possible.
    pub fn from_i64(value: i64) -> Option<Self> {
        i16::try_from(value).ok().map(Self)
    }

    /// Creates an [`Const16`] from the given `u64` value if possible.
    pub fn from_u64(value: u64) -> Option<Self> {
        let value = u16::try_from(value).ok()? as i16;
        Some(Self(value))
    }

    /// Returns an `i32` value from `self`.
    pub fn to_i32(self) -> i32 {
        i32::from(self.0)
    }

    /// Returns an `i64` value from `self`.
    pub fn to_i64(self) -> i64 {
        i64::from(self.0)
    }

    /// Returns an `u32` value from `self`.
    pub fn to_u32(self) -> u32 {
        u32::from(self.0 as u16)
    }

    /// Returns an `u64` value from `self`.
    pub fn to_u64(self) -> u64 {
        u64::from(self.0 as u16)
    }
}

/// A 32-bit constant value of any type.
///
/// # Note
///
/// Can be used to store information about small integer values.
/// Upon use the small 32-bit value has to be sign-extended to
/// the actual integer type, e.g. `i32` or `i64`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(align(2))] // 2-byte alignment is sufficient for `wasmi` bytecode
pub struct AnyConst32([u8; 4]);

/// Error that may occur upon converting values to [`AnyConst32`].
#[derive(Debug, Copy, Clone)]
pub struct OutOfBoundsConst32;

impl TryFrom<i64> for AnyConst32 {
    type Error = OutOfBoundsConst32;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Self::from_i64(value).ok_or(OutOfBoundsConst32)
    }
}

impl From<bool> for AnyConst32 {
    fn from(value: bool) -> Self {
        Self::from_bool(value)
    }
}

impl From<i8> for AnyConst32 {
    fn from(value: i8) -> Self {
        Self::from_u32(value as u32)
    }
}

impl From<i16> for AnyConst32 {
    fn from(value: i16) -> Self {
        Self::from_u32(value as u32)
    }
}

impl From<i32> for AnyConst32 {
    fn from(value: i32) -> Self {
        Self::from_i32(value)
    }
}

impl From<u32> for AnyConst32 {
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

impl From<f32> for AnyConst32 {
    fn from(value: f32) -> Self {
        Self::from(F32::from(value))
    }
}

impl From<F32> for AnyConst32 {
    fn from(value: F32) -> Self {
        Self::from_f32(value)
    }
}

impl AnyConst32 {
    /// Creates a [`AnyConst32`] from the given `bool` value.
    pub fn from_bool(value: bool) -> Self {
        Self::from_u32(u32::from(value))
    }

    /// Creates an [`AnyConst32`] from the given `u32` value.
    pub fn from_u32(value: u32) -> Self {
        Self(value.to_ne_bytes())
    }

    /// Creates an [`AnyConst32`] from the given `i32` value.
    pub fn from_i32(value: i32) -> Self {
        Self::from_u32(value as u32)
    }

    /// Creates an [`AnyConst32`] from the given [`F32`] value.
    pub fn from_f32(value: F32) -> Self {
        Self::from_u32(value.to_bits())
    }

    /// Creates an [`AnyConst32`] from the given `i64` value if possible.
    pub fn from_i64(value: i64) -> Option<Self> {
        i32::try_from(value).ok().map(Self::from_i32)
    }

    /// Returns an `u32` value from `self`.
    ///
    /// # Note
    ///
    /// It is the responsibility of the user to validate type safety
    /// since access via this method is not type checked.
    pub fn to_u32(self) -> u32 {
        u32::from_ne_bytes(self.0)
    }

    /// Returns an `i32` value from `self`.
    ///
    /// # Note
    ///
    /// It is the responsibility of the user to validate type safety
    /// since access via this method is not type checked.
    pub fn to_i32(self) -> i32 {
        self.to_u32() as i32
    }

    /// Returns an `f32` value from `self`.
    ///
    /// # Note
    ///
    /// It is the responsibility of the user to validate type safety
    /// since access via this method is not type checked.
    pub fn to_f32(self) -> F32 {
        F32::from(f32::from_bits(self.to_u32()))
    }

    /// Returns an `i64` value from `self`.
    ///
    /// # Note
    ///
    /// It is the responsibility of the user to validate type safety
    /// since access via this method is not type checked.
    pub fn to_i64(self) -> i64 {
        i64::from(self.to_i32())
    }
}
