//! Testing basic functionality with `efm32pg23` chip.
//!
//! Operations used here are based on
//!
//! - [reference manual](https://www.silabs.com/documents/public/reference-manuals/efm32pg23-rm.pdf)
//! - [devboard user guide](https://www.silabs.com/documents/public/user-guides/ug515-efm32pg23-brd2504a-user-guide.pdf)
//! - [official API docs](https://docs.silabs.com/gecko-platform/latest/emlib/api/efm32xg23/modules)
//! - [published official open source SDK in C](https://github.com/SiliconLabs/gecko_sdk/tree/gsdk_4.2/platform)

#![no_std]

extern crate alloc;

pub mod init;
mod peripherals;
pub mod devices;
pub mod draw;

pub use peripherals::ldma::{BUF_QUARTER, LINK_1, LINK_2, LINK_DESCRIPTORS, TIMER0_CC0_ICF, NfcXfer, NfcXferBlock};
