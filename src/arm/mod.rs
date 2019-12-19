// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// Export 4.14 bindings when the feature kvm-v4_20_0 is not specified.
#[cfg(all(feature = "kvm-v4_14_0", not(feature = "kvm-v4_20_0")))]
#[allow(clippy::all)]
mod bindings_v4_14_0;

// Export 4.20 bindings when kvm-v4_20_0 is specified or no kernel version
// related features are specified.
#[cfg(any(
    feature = "kvm-v4_20_0",
    all(not(feature = "kvm-v4_14_0"), not(feature = "kvm-v4_20_0"))
))]
#[allow(clippy::all)]
mod bindings_v4_20_0;

pub mod bindings {
    #[cfg(all(feature = "kvm-v4_14_0", not(feature = "kvm-v4_20_0")))]
    pub use super::bindings_v4_14_0::*;

    #[cfg(any(
        feature = "kvm-v4_20_0",
        all(not(feature = "kvm-v4_14_0"), not(feature = "kvm-v4_20_0"))
    ))]
    pub use super::bindings_v4_20_0::*;
}

#[cfg_attr(feature = "with-serde", derive(Deserialize, Serialize))]
#[cfg_attr(test, derive(Debug, PartialEq))]
/// Composite version of the autogenerated bindings.
pub struct Version {
    /// Architecture.
    pub arch: &'static str,
    /// Kernel version.
    pub kernel_ver: &'static str,
    /// Crate version.
    pub crate_ver: &'static str,
}

#[allow(unused)]
static VERSION: Version = Version {
    arch: "aarch",

    #[cfg(feature = "kvm-v4_14_0")]
    kernel_ver: "v4.14.0",
    #[cfg(feature = "kvm-v4_20_0")]
    kernel_ver: "v4.20.0",
    #[cfg(all(not(feature = "kvm-v4_14_0"), not(feature = "kvm-v4_20_0")))]
    kernel_ver: "v4.20.0",

    crate_ver: env!("CARGO_PKG_VERSION"),
};

#[cfg(test)]
mod tests {
    #[cfg(feature = "with-serde")]
    extern crate serde_json;

    use super::{Version, VERSION};

    #[test]
    fn test_version() {
        assert_eq!(VERSION.arch, "aarch");

        #[cfg(feature = "kvm-v4_14_0")]
        assert_eq!(VERSION.kernel_ver, "v4.14.0");
        #[cfg(feature = "kvm-v4_20_0")]
        assert_eq!(VERSION.kernel_ver, "v4.20.0");
        #[cfg(all(not(feature = "kvm-v4_14_0"), not(feature = "kvm-v4_20_0")))]
        assert_eq!(VERSION.kernel_ver, "v4.20.0");

        assert_eq!(VERSION.crate_ver, env!("CARGO_PKG_VERSION"));
    }
}
