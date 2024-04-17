use alloc::sync::Arc;

#[macro_export(local_inner_macros)]
/// Create a new storage ID type.
macro_rules! storage_id_type {
    ($name:ident) => {
        /// Storage ID.
        #[derive(Clone, Hash, PartialEq, Eq)]
        pub struct $name {
            value: u64,
        }

        impl $name {
            /// Create a new ID.
            pub fn new() -> Self {
                use core::sync::atomic::{AtomicU64, Ordering};

                static COUNTER: AtomicU64 = AtomicU64::new(0);

                let value = COUNTER.fetch_add(1, Ordering::Relaxed);
                if value == u64::MAX {
                    core::panic!("Memory ID overflowed");
                }
                Self { value }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

/// Reference to a buffer handle.
#[derive(Clone, Debug)]
pub struct HandleRef<Id> {
    id: Arc<Id>,
    all: Arc<()>,
}

/// Reference to buffer binding.
#[derive(Clone, Debug)]
pub struct BindingRef<Id> {
    id: Id,
    _all: Arc<()>,
}

impl<Id> BindingRef<Id>
where
    Id: Clone + core::fmt::Debug,
{
    /// The id associated to the buffer.
    pub(crate) fn id(&self) -> &Id {
        &self.id
    }
}

impl<Id> HandleRef<Id>
where
    Id: Clone + core::fmt::Debug,
{
    /// Create a new handle.
    pub(crate) fn new(id: Id) -> Self {
        Self {
            id: Arc::new(id),
            all: Arc::new(()),
        }
    }
    /// Derive the handle from another one, increasing its total ref count.
    pub(crate) fn derive_from<T>(id: Id, other: &HandleRef<T>) -> Self {
        Self {
            id: Arc::new(id),
            all: other.all.clone(),
        }
    }

    /// The id associated to the handle.
    pub(crate) fn id(&self) -> &Id {
        &self.id
    }

    /// Get the binding.
    pub(crate) fn binding(&self) -> BindingRef<Id> {
        BindingRef {
            id: self.id.as_ref().clone(),
            _all: self.all.clone(),
        }
    }

    /// If the handle can be mut.
    pub(crate) fn can_mut(&self) -> bool {
        // 1 memory management reference with 1 tensor reference.
        Arc::strong_count(&self.id) <= 2
    }

    /// If the resource can be reused by another tensor.
    pub(crate) fn is_free(&self) -> bool {
        // 1 memory management reference with 0 tensor reference.
        Arc::strong_count(&self.id) <= 1
    }

    /// If the resource can be dealloc.
    pub(crate) fn can_be_dealloc(&self) -> bool {
        Arc::strong_count(&self.all) <= 1
    }
}

#[macro_export(local_inner_macros)]
/// Create new memory ID types.
macro_rules! memory_id_type {
    ($id:ident, $handle:ident, $binding:ident) => {
        /// Memory Handle.
        #[derive(Clone, Debug)]
        pub struct $handle {
            value: $crate::id::HandleRef<$id>,
        }

        /// Binding of a memory handle.
        #[derive(Clone, Debug)]
        pub struct $binding {
            value: $crate::id::BindingRef<$id>,
        }

        /// Memory ID.
        #[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
        pub struct $id {
            value: u64,
        }

        impl $handle {
            /// Create a new ID.
            pub(crate) fn new() -> Self {
                let value = Self::gen_id();
                Self {
                    value: $crate::id::HandleRef::new($id { value }),
                }
            }

            /// Derive the handle from another one, increasing its total ref count.
            #[allow(unused)]
            pub(crate) fn derive_from<Id>(handle: &$crate::id::HandleRef<Id>) -> Self {
                let value = Self::gen_id();
                Self {
                    value: $crate::id::HandleRef::derive_from($id { value }, handle),
                }
            }

            pub(crate) fn binding(&self) -> $binding {
                $binding {
                    value: self.value.binding(),
                }
            }

            fn gen_id() -> u64 {
                static COUNTER: core::sync::atomic::AtomicU64 =
                    core::sync::atomic::AtomicU64::new(0);

                let value = COUNTER.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
                if value == u64::MAX {
                    core::panic!("Memory ID overflowed");
                }

                value
            }
        }

        impl core::ops::Deref for $handle {
            type Target = $crate::id::HandleRef<$id>;

            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }

        impl core::ops::Deref for $binding {
            type Target = $crate::id::BindingRef<$id>;

            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }

        impl Default for $handle {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}
