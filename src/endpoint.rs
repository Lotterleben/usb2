//! Endpoint descriptors

use crate::{desc, Endpoint};

/// Endpoint descriptor
pub struct Descriptor {
    // pub bLength: u8,
    // pub bDescriptorType: u8,
    /// Endpoint address
    pub bEndpointAddress: Endpoint,
    /// Attributes
    pub bmAttributes: bmAttributes,
    /// Maximum packet size
    pub wMaxPacketSize: wMaxPacketSize,
    /// Polling interval
    pub bInterval: u8,
}

/// Endpoint attributes
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum bmAttributes {
    /// Bulk endpoint
    Bulk,
    /// Control endpoint
    Control,
    /// Interrupt endpoint
    Interrupt,
    /// Isochronous endpoint
    Isochronous {
        /// Synchronization type
        synchronization_type: SynchronizationType,
        /// Usage type
        usage_type: UsageType,
    },
}

/// Synchronization type
#[derive(Clone, Copy)]
pub enum SynchronizationType {
    /// No synchronization
    NoSynchronization = 0b00,
    /// Asynchronous
    Asynchronous = 0b01,
    /// Adaptive
    Adaptive = 0b10,
    /// Synchronous
    Synchronous = 0b11,
}

/// Usage type
#[derive(Clone, Copy)]
pub enum UsageType {
    /// Data endpoint
    DataEndpoint = 0b00,
    /// Feedback endpoint
    FeedbackEndpoint = 0b01,
    /// Implicit feedback data endpoint
    ImplicitFeedbackDataEndpoint = 0b10,
}

impl bmAttributes {
    fn byte(&self) -> u8 {
        match self {
            bmAttributes::Bulk => 0b10,
            bmAttributes::Control => 0b00,
            bmAttributes::Interrupt => 0b11,
            bmAttributes::Isochronous {
                synchronization_type,
                usage_type,
            } => 0b01 | (*synchronization_type as u8) << 2 | (*usage_type as u8) << 4,
        }
    }
}

/// Maximum packet size
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum wMaxPacketSize {
    /// Bulk or control endpoint
    BulkControl {
        /// Must be less than `1 << 11`
        size: u16,
    },

    /// Isochronous or interrupt endpoint
    IsochronousInterrupt {
        /// Must be less than `1 << 11`
        size: u16,
        /// Transactions per microframe
        transactions_per_microframe: Transactions,
    },
}

/// Transactions per microframe
#[derive(Clone, Copy)]
pub enum Transactions {
    /// 1 transaction per microframe
    _1 = 0b00,
    /// 2 transactions per microframe
    _2 = 0b01,
    /// 3 transactions per microframe
    _3 = 0b10,
}

impl wMaxPacketSize {
    fn word(&self) -> u16 {
        match self {
            wMaxPacketSize::BulkControl { size } => *size & ((1 << 11) - 1),

            wMaxPacketSize::IsochronousInterrupt {
                size,
                transactions_per_microframe,
            } => (*size & ((1 << 11) - 1)) | ((*transactions_per_microframe as u16) << 11),
        }
    }
}
impl Descriptor {
    /// The size of this descriptor on the wire
    pub const SIZE: u8 = 7;

    /// Returns the wire representation of this descriptor
    pub fn bytes(&self) -> [u8; Self::SIZE as usize] {
        let word = self.wMaxPacketSize.word();
        [
            Self::SIZE,
            desc::Type::Endpoint as u8,
            self.bEndpointAddress.byte(),
            self.bmAttributes.byte(),
            word as u8,
            (word >> 8) as u8,
            self.bInterval,
        ]
    }
}
