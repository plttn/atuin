//! The hostname that macOS keeps for this computer across networks.
#![allow(
    unsafe_code,
    reason = "Core Foundation dictionaries need a cast to give their values a type."
)]

use objc2_core_foundation::{CFDictionary, CFPropertyList, CFString, CFType};
use objc2_system_configuration::SCDynamicStore;

/// This computer's hostname, which stays the same when the computer joins another network.
///
/// The name is the `HostName` that `scutil --set HostName` sets. When `HostName` isn't set, the
/// name is the local hostname that System Settings shows under General > Sharing, without the
/// `.local` suffix. For example, the name is `My-MacBook-Pro`. The result is `None` when
/// neither name is set.
#[must_use]
pub fn current() -> Option<String> {
    configured().or_else(local)
}

/// The `HostName` that the user set. macOS also gives this name to `gethostname()`.
fn configured() -> Option<String> {
    let key = SCDynamicStore::key_create_computer_name(None);
    let system = SCDynamicStore::value(None, &key)?;
    host_name_from(&system)
}

/// The `LocalHostName`, which macOS derives from the computer name in System Settings.
fn local() -> Option<String> {
    let name = SCDynamicStore::local_host_name(None)?;
    nonempty(&name)
}

/// Reads `HostName` from the `Setup:/System` dictionary of the dynamic store.
fn host_name_from(system: &CFPropertyList) -> Option<String> {
    let system = system.downcast_ref::<CFDictionary>()?;
    // SAFETY: Every key and value of a CFDictionary is a CFType. The CFString key type only
    // types the lookup key, and lookups compare keys with CFEqual.
    let system = unsafe { system.cast_unchecked::<CFString, CFType>() };
    let name = system.get(&CFString::from_static_str("HostName"))?;
    nonempty(name.downcast_ref::<CFString>()?)
}

fn nonempty(name: &CFString) -> Option<String> {
    let name = name.to_string();
    (!name.is_empty()).then_some(name)
}

#[cfg(test)]
mod tests {
    use objc2_core_foundation::{CFRetained, Type as _};
    use rstest::rstest;

    use super::*;

    fn system(host_name: Option<&CFType>) -> CFRetained<CFType> {
        let key = CFString::from_static_str("HostName");
        let dictionary = host_name.map_or_else(CFDictionary::<CFString, CFType>::empty, |name| {
            CFDictionary::from_slices(&[&*key], &[name])
        });
        let dictionary: &CFType = dictionary.as_ref();
        dictionary.retain()
    }

    #[rstest]
    #[case::set(system(Some(&CFString::from_static_str("My-Mac.example.com"))), Some("My-Mac.example.com"))]
    #[case::empty(system(Some(&CFString::from_static_str(""))), None)]
    #[case::missing(system(None), None)]
    #[case::not_a_string(system(Some(&system(None))), None)]
    #[case::not_a_dictionary(CFString::from_static_str("My-Mac").into(), None)]
    fn reads_host_name(#[case] system: CFRetained<CFType>, #[case] expected: Option<&str>) {
        assert_eq!(host_name_from(&system).as_deref(), expected);
    }

    #[rstest]
    fn current_has_a_name() {
        // Every Mac has a LocalHostName, so a lookup that finds no name is broken.
        assert!(current().is_some_and(|name| !name.is_empty()));
    }
}
