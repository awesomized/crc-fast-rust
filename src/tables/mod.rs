#[cfg(any(feature = "lazy-tables", all(target_arch = "wasm32", feature = "std")))]
mod lazy;
#[cfg(any(feature = "lazy-tables", all(target_arch = "wasm32", feature = "std")))]
pub(crate) use lazy::*;

#[cfg(any(
    test,
    not(any(feature = "lazy-tables", all(target_arch = "wasm32", feature = "std")))
))]
mod prebuilt;
#[cfg(not(any(feature = "lazy-tables", all(target_arch = "wasm32", feature = "std"))))]
pub(crate) use prebuilt::*;
