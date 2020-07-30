//! The bytes package, providing access to the bytes type.

use crate::functions::{Module, RegisterError};
use std::fmt;
use std::ops;

/// A bytes container.
#[derive(Clone)]
pub struct Bytes {
    bytes: Vec<u8>,
}

impl ops::Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl fmt::Debug for Bytes {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.bytes, fmt)
    }
}

impl Bytes {
    /// Construct from a byte array.
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Construct a new bytes container.
    fn new() -> Self {
        Bytes { bytes: Vec::new() }
    }

    /// Construct a new bytes container with the specified capacity.
    fn with_capacity(cap: usize) -> Self {
        Bytes {
            bytes: Vec::with_capacity(cap),
        }
    }

    /// Do something with the bytes.
    fn extend(&mut self, other: &Self) {
        self.bytes.extend(other.bytes.iter().copied());
    }

    /// Do something with the bytes.
    fn push_str(&mut self, s: &str) {
        self.bytes.extend(s.as_bytes());
    }

    /// Get the length of the bytes collection.
    fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Get the capacity of the bytes collection.
    fn capacity(&self) -> usize {
        self.bytes.capacity()
    }

    /// Get the bytes collection.
    fn clear(&mut self) {
        self.bytes.clear();
    }

    fn reserve(&mut self, additional: usize) {
        self.bytes.reserve(additional);
    }

    fn reserve_exact(&mut self, additional: usize) {
        self.bytes.reserve_exact(additional);
    }

    fn shrink_to_fit(&mut self) {
        self.bytes.shrink_to_fit();
    }

    fn pop(&mut self) -> Option<u8> {
        self.bytes.pop()
    }

    fn last(&mut self) -> Option<u8> {
        self.bytes.last().copied()
    }
}

decl_external!(Bytes);

impl<'a> crate::UnsafeFromValue for &'a [u8] {
    unsafe fn unsafe_from_value(
        value: crate::ValuePtr,
        vm: &mut crate::Vm,
    ) -> Result<Self, crate::StackError> {
        let slot = value.into_external()?;
        let value = crate::Ref::unsafe_into_ref(vm.external_ref::<Bytes>(slot)?);
        Ok(value.bytes.as_slice())
    }
}

/// Get the module for the bytes package.
pub fn module() -> Result<Module, RegisterError> {
    let mut module = Module::new(&["bytes"]);
    module.global_fn("new", Bytes::new)?;
    module.global_fn("with_capacity", Bytes::with_capacity)?;

    module.instance_fn("extend", Bytes::extend)?;
    module.instance_fn("pop", Bytes::pop)?;
    module.instance_fn("last", Bytes::last)?;

    module.instance_fn("len", Bytes::len)?;
    module.instance_fn("capacity", Bytes::capacity)?;
    module.instance_fn("clear", Bytes::clear)?;
    module.instance_fn("push_str", Bytes::push_str)?;
    module.instance_fn("reserve", Bytes::reserve)?;
    module.instance_fn("reserve_exact", Bytes::reserve_exact)?;
    module.instance_fn("clone", Bytes::clone)?;
    module.instance_fn("shrink_to_fit", Bytes::shrink_to_fit)?;
    Ok(module)
}
