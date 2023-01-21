//! Register structures.

macro_rules! impl_boilerplate_for {
    ($REG:ident) => {
        impl From<u32> for $REG {
            fn from(val: u32) -> Self {
                Self(val)
            }
        }

        impl From<$REG> for u32 {
            fn from(val: $REG) -> u32 {
                val.0
            }
        }

        impl Default for $REG {
            fn default() -> Self {
                Self::DEFAULT
            }
        }
    };
}

/// Chip Address register (CHIPADDR).
///
/// Used to know chip information.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ChipAddress(u32);
impl_boilerplate_for!(ChipAddress);

impl ChipAddress {
    /// Chip Address register reset value.
    pub const RESET: u32 = 0x1397_1800;

    /// Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// # Example
    ///
    /// ```
    /// use w5500_ll::ChipAddress;
    ///
    /// assert_eq!(ChipAddress::DEFAULT, ChipAddress::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// Bit offset for the `CHIP_ID` field.
    pub const CHIP_ID_OFFSET: u32 = 16;
    /// Bit offset for the `CORE_NUM` field.
    pub const CORE_NUM_OFFSET: u32 = 8;
    /// Bit offset for the `ADDR` field.
    pub const ADDR_OFFSET: u32 = 0;

    /// Bit mask for the `CHIP_ID` field.
    pub const CHIP_ID_MASK: u32 = 0xffff << Self::CHIP_ID_OFFSET;
    /// Bit mask for the `CORE_NUM` field.
    pub const CORE_NUM_MASK: u32 = 0xff << Self::CORE_NUM_OFFSET;
    /// Bit mask for the `ADDR` field.
    pub const ADDR_MASK: u32 = 0xff << Self::ADDR_OFFSET;

    /// Get the chip id.
    ///
    /// This returns an `u16` with the chip_id value.
    ///
    /// # Example
    ///
    /// ```
    /// use w5500_ll::{ChipAddress};
    ///
    /// assert_eq!(ChipAddress::DEFAULT.chip_id(), 0x1397);
    /// ```
    pub const fn chip_id(&self) -> u16 {
        (self.0 >> Self::CHIP_ID_OFFSET) as u16
    }

    /// Get the core num.
    ///
    /// This returns an `u8` with the core_num value.
    ///
    /// # Example
    ///
    /// ```
    /// use w5500_ll::{ChipAddress};
    ///
    /// assert_eq!(ChipAddress::DEFAULT.core_num(), 0x18);
    /// ```
    pub const fn core_num(&self) -> u8 {
        (self.0 >> Self::CORE_NUM_OFFSET) as u8
    }

    /// Get the address.
    ///
    /// This returns an `u8` with the address value.
    ///
    /// # Example
    ///
    /// ```
    /// use w5500_ll::{ChipAddress};
    ///
    /// assert_eq!(ChipAddress::DEFAULT.addr(), 0x00);
    /// ```
    pub const fn addr(&self) -> u8 {
        (self.0 >> Self::ADDR_OFFSET) as u8
    }
}

impl ::core::fmt::Display for ChipAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ChipAddress")
            .field("chip_id", &self.chip_id())
            .field("core_num", &self.core_num())
            .field("addr", &self.addr())
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChipAddress {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ChipAddress {{ chip_id: {}, core_num: {}, addr: {} }}",
            self.chip_id(),
            self.core_num(),
            self.addr(),
        );
    }
}
