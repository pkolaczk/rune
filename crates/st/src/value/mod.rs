mod slot;
mod value;
mod value_ptr;
mod value_ref;
mod value_type;
mod value_type_info;

pub use self::slot::Slot;
pub use self::value::Value;
pub use self::value_ptr::ValuePtr;
pub use self::value_ref::ValueRef;
pub use self::value_type::ValueType;
pub use self::value_type_info::ValueTypeInfo;

/// The type of an object.
pub type Object<T> = crate::collections::HashMap<String, T>;

/// The type of an array.
pub type Array<T> = Vec<T>;
