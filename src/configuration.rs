//! Configuration descriptors

use core::num::NonZeroU8;

use crate::desc;

/// Configuration Descriptor
///
/// See section 9.6.3 of (USB2)
#[allow(non_snake_case)]
#[derive(Clone, Copy)]
pub struct Descriptor {
    // pub blength: u8,
    // pub bDescriptorType: u8,
    /// The total length of this configuration descriptor plus the descriptors (interface, endpoint,
    /// etc.) below it
    pub wTotalLength: u16,
    /// Number of interfaces associated to this configuration
    pub bNumInterfaces: NonZeroU8,
    /// Configuration value
    pub bConfigurationValue: NonZeroU8,
    /// Configuration string index
    pub iConfiguration: Option<NonZeroU8>,
    /// Attributes
    pub bmAttributes: bmAttributes,
    /// Maximum power (1 ULP = 2 mA)
    pub bMaxPower: u8,
}

impl Descriptor {
    /// The size of this descriptor on the wire
    pub const SIZE: u8 = 9;

    /// Returns the wire representation of this descriptor
    pub fn bytes(&self) -> [u8; Self::SIZE as usize] {
        [
            Self::SIZE,
            desc::Type::Configuration as u8,
            self.wTotalLength as u8,
            (self.wTotalLength >> 8) as u8,
            self.bNumInterfaces.get(),
            self.bConfigurationValue.get(),
            self.iConfiguration.map(|nz| nz.get()).unwrap_or(0),
            (1 << 7)
                | if self.bmAttributes.self_powered {
                    1 << 6
                } else {
                    0
                }
                | if self.bmAttributes.remote_wakeup {
                    1 << 5
                } else {
                    0
                },
            self.bMaxPower,
        ]
    }
}

/// Attributes
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct bmAttributes {
    /// Self-powered?
    pub self_powered: bool,
    /// Remote wakeup
    pub remote_wakeup: bool,
}
