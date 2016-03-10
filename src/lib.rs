//! A Demo library

/// a Foo struct
pub struct Foo{ name : String }

/// This function always returns true
pub fn get_true() -> bool { return true; }

/// The trait HasName
pub trait HasName { fn name(&self) -> String; }

/// The implementation of the HasName name for Foo
impl HasName for Foo {
    fn name(&self) -> String { return self.name.clone(); }
}