#[cfg(target_os = "macos")]
pub(crate) fn set_main_window_visible(visible: bool) {
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy};

    let Some(main_thread) = MainThreadMarker::new() else {
        log::warn!("Unable to update macOS activation policy off the main thread");
        return;
    };

    let app = NSApplication::sharedApplication(main_thread);
    let policy = if visible {
        NSApplicationActivationPolicy::Regular
    } else {
        NSApplicationActivationPolicy::Accessory
    };

    if !app.setActivationPolicy(policy) {
        log::warn!("Failed to switch macOS activation policy");
    }

    if visible {
        app.activate();
    } else {
        app.deactivate();
    }
}

#[cfg(not(target_os = "macos"))]
pub(crate) fn set_main_window_visible(_visible: bool) {}
