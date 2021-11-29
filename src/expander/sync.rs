use core::marker::PhantomData;

use super::{Register, SyncExpander};

use super::{mutex::ExpanderMutex, Expander};

/// A wrapper struct to use an Expander as multiple instances of [`ExpanderInputPin`] or [`ExpanderOutputPin`] making the pins sync.
pub struct IoExpander<Em, Ex>
where
    Em: ExpanderMutex<Ex>,
{
    expander_mutex: Em,
    phantom_data: PhantomData<Ex>,
}

impl<Em: ExpanderMutex<Ex>, Ex: Expander> IoExpander<Em, Ex> {
    ///Creates a new IoExpander instance out of a ExpanderMutex
    pub fn new(expander_mutex: Em) -> IoExpander<Em, Ex> {
        IoExpander {
            expander_mutex,
            phantom_data: PhantomData,
        }
    }
}

impl<Em: ExpanderMutex<Ex>, Ex: Expander> SyncExpander for IoExpander<Em, Ex> {
    type Error = <Ex as Expander>::Error;

    fn write_byte(&self, register: Register, data: u8) -> Result<(), Self::Error> {
        self.expander_mutex.lock(|ex| ex.write_byte(register, data))
    }
    fn read_byte(&self, register: Register, buffer: &mut u8) -> Result<(), Self::Error> {
        self.expander_mutex
            .lock(|ex| ex.read_byte(register, buffer))
    }
    fn write_halfword(&self, register: Register, data: u16) -> Result<(), Self::Error> {
        self.expander_mutex
            .lock(|ex| ex.write_halfword(register, data))
    }
    fn read_halfword(&self, register: Register, buffer: &mut u16) -> Result<(), Self::Error> {
        self.expander_mutex
            .lock(|ex| ex.read_halfword(register, buffer))
    }
}
