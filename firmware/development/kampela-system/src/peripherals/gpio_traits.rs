//! Typesafety for GPIO

use core::marker::PhantomData;

mod sealed {
    /// Seal GPIO type-objects
    pub trait Sealed {}
}

/// State of the pin
pub trait PinState: sealed::Sealed {}

/// Default disabled pin state
pub trait DisabledState: sealed::Sealed {}

/// Input pin states
pub trait InputState: sealed::Sealed {}

/// Output pin states
pub trait OutputState: sealed::Sealed {}

/// Struct to store input state of pin
pub struct GPInput<S: InputState> {
    _p: PhantomData<S>,
}

impl<S: InputState> PinState for GPInput<S> {}
impl<S: InputState> sealed::Sealed for GPInput<S> {}

/// Input enabled. Filter if DOUT is set.
pub struct Input;

/// Input enabled. DOUT determines pull direction.
pub struct InputPull;

/// Input enabled with filter. DOUT determines pull direction.
pub struct InputPullFilter;


impl InputState for Input {}
impl InputState for InputPull {}
impl InputState for InputPullFilter {}
impl sealed::Sealed for Input {}
impl sealed::Sealed for InputPull {}
impl sealed::Sealed for InputPullFilter {}

/// Struct to store output state of pin
pub struct GPOutput<S: OutputState> {
    _p: PhantomData<S>,
}

impl<S: OutputState> PinState for GPOutput<S> {}
impl<S: OutputState> sealed::Sealed for GPOutput<S> {}

/// Push-pull output.
pub struct PushPull;

/// Open-drain output with pullup.
pub struct WiredAndPullUp;

impl OutputState for PushPull {}
impl OutputState for WiredAndPullUp {}
impl sealed::Sealed for PushPull {}
impl sealed::Sealed for WiredAndPullUp {}

pub struct PA0;
pub struct PA1;
pub struct PA2;
pub struct PA3<S: PinState> {
    _p: PhantomData<S>,
};
pub struct PA4;
pub struct PA5;
pub struct PA6;
pub struct PA7;
pub struct PA8;
pub struct PA9;
pub struct PA10;
pub struct PA11;

pub struct PB0;
pub struct PB1;
pub struct PB2;
pub struct PB3;
pub struct PB4;
pub struct PB5;
pub struct PB6;

pub struct PC0;
pub struct PC1;
pub struct PC2;
pub struct PC3;
pub struct PC4;
pub struct PC5;
pub struct PC6;
pub struct PC7;
pub struct PC8;
pub struct PC9;

pub struct PD0;
pub struct PD1;
pub struct PD2;
pub struct PD3;
pub struct PD4;
pub struct PD5;

pub struct PortA;
pub struct PortB;
pub struct PortC;
pub struct PortD;

pub struct PortAPins {
    pub pa0: PA0,
    pub pa1: PA1,
    pub pa2: PA2,
    pub pa3: PA3,
    pub pa4: PA4,
    pub pa5: PA5,
    pub pa6: PA6,
    pub pa7: PA7,
    pub pa8: PA8,
    pub pa9: PA9,
    pub pa10: PA10,
    pub pa11: PA11,
}

pub struct PortBPins {
    pub pb0: PB0,
    pub pb1: PB1,
    pub pb2: PB2,
    pub pb3: PB3,
    pub pb4: PB4,
    pub pb5: PB5,
    pub pb6: PB6,
}

pub struct PortCPins {
    pub pc0: PC0,
    pub pc1: PC1,
    pub pc2: PC2,
    pub pc3: PC3,
    pub pc4: PC4,
    pub pc5: PC5,
    pub pc6: PC6,
    pub pc7: PC7,
    pub pc8: PC8,
    pub pc9: PC9,
}

pub struct PortDPins {
    pub pd0: PD0,
    pub pd1: PD1,
    pub pd2: PD2,
    pub pd3: PD3,
    pub pd4: PD4,
    pub pd5: PD5,
}

impl PortA {
    pub fn split(self) -> PortAPins {
        PortAPins {
            pa0: PA0,
            pa1: PA1,
            pa2: PA2,
            pa3: PA3,
            pa4: PA4,
            pa5: PA5,
            pa6: PA6,
            pa7: PA7,
            pa8: PA8,
            pa9: PA9,
            pa10: PA10,
            pa11: PA11,
        }
    }
}

impl PortB {
    pub fn split(self) -> PortBPins {
        PortBPins {
            pb0: PB0,
            pb1: PB1,
            pb2: PB2,
            pb3: PB3,
            pb4: PB4,
            pb5: PB5,
            pb6: PB6,
        }
    }
}

impl PortC {
    pub fn split(self) -> PortCPins {
        PortCPins {
            pc0: PC0,
            pc1: PC1,
            pc2: PC2,
            pc3: PC3,
            pc4: PC4,
            pc5: PC5,
            pc6: PC6,
            pc7: PC7,
            pc8: PC8,
            pc9: PC9,
        }
    }
}

impl PortD {
    pub fn split(self) -> PortDPins {
        PortDPins {
            pd0: PD0,
            pd1: PD1,
            pd2: PD2,
            pd3: PD3,
            pd4: PD4,
            pd5: PD5,
        }
    }
}


