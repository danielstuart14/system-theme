use core::{ffi::c_void, ptr};
use objc2::{
    available, define_class, msg_send,
    rc::Retained,
    runtime::{AnyObject, Sel},
    DefinedClass, MainThreadMarker, MainThreadOnly,
};
use objc2_app_kit::{
    NSAppearance, NSAppearanceNameAqua, NSAppearanceNameDarkAqua, NSApplication, NSColor,
    NSColorSpace, NSSystemColorsDidChangeNotification, NSWorkspace,
    NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification,
};
use objc2_foundation::{
    ns_string, NSArray, NSDictionary, NSKeyValueChangeKey, NSKeyValueObservingOptions,
    NSNotification, NSNotificationCenter, NSObject, NSObjectNSKeyValueObserverRegistration,
    NSString,
};
use std::sync::Arc;
use tokio::sync::Notify;

use crate::{error::Error, ThemeColor, ThemeContrast, ThemeKind, ThemeScheme};

struct Ivars {
    application: Retained<NSApplication>,
    default_notification: Retained<NSNotificationCenter>,
    workspace_notification: Retained<NSNotificationCenter>,
    notify: Arc<Notify>,
}

define_class!(
    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - ThemeObserver implements `Drop` and ensures that:
    //   - It does not call an overridden method.
    //   - It does not `retain` itself.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = Ivars]
    struct ThemeObserver;

    impl ThemeObserver {
        #[unsafe(method(observeValueForKeyPath:ofObject:change:context:))]
        fn value_change_callback(
            &self,
            _key_path: Option<&NSString>,
            _object: Option<&AnyObject>,
            _change: Option<&NSDictionary<NSKeyValueChangeKey, AnyObject>>,
            _context: *mut c_void,
        ) {
            self.ivars().notify.notify_waiters();
        }

        #[unsafe(method(notificationCallback:))]
        fn notification_callback(
            &self, _notification: &NSNotification
        ) {
            self.ivars().notify.notify_waiters();
        }
    }
);

impl ThemeObserver {
    pub fn new(
        mtm: MainThreadMarker,
        application: Retained<NSApplication>,
        workspace: &NSWorkspace,
        notify: Arc<Notify>,
    ) -> Retained<Self> {
        let observer = ThemeObserver::alloc(mtm).set_ivars(Ivars {
            application,
            default_notification: NSNotificationCenter::defaultCenter(),
            workspace_notification: workspace.notificationCenter(),
            notify,
        });
        // SAFETY: Object is allocated and ivars are set.
        let observer: Retained<Self> = unsafe { msg_send![super(observer), init] };

        // Create theme scheme observer (dark / light)
        // SAFETY: We make sure to un-register the observer before it's deallocated.
        unsafe {
            observer
                .ivars()
                .application
                .addObserver_forKeyPath_options_context(
                    &observer,
                    ns_string!("effectiveAppearance"),
                    NSKeyValueObservingOptions::New | NSKeyValueObservingOptions::Old,
                    ptr::null_mut(),
                );
        }

        // Create theme accent observer
        // SAFETY: We make sure to un-register the observer before it's deallocated.
        unsafe {
            observer
                .ivars()
                .default_notification
                .addObserver_selector_name_object(
                    &observer,
                    Sel::register(c"notificationCallback:"),
                    Some(NSSystemColorsDidChangeNotification),
                    None,
                );
        }

        // Create theme contrast observer (high / normal)
        // SAFETY: We make sure to un-register the observer before it's deallocated.
        unsafe {
            observer
                .ivars()
                .workspace_notification
                .addObserver_selector_name_object(
                    &observer,
                    Sel::register(c"notificationCallback:"),
                    Some(NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification),
                    None,
                );
        }

        observer
    }
}

impl Drop for ThemeObserver {
    fn drop(&mut self) {
        // SAFETY: Object is of the correct type
        unsafe {
            self.ivars()
                .application
                .removeObserver_forKeyPath(self, ns_string!("effectiveAppearance"));
            self.ivars().default_notification.removeObserver(self);
            self.ivars().workspace_notification.removeObserver(self);
        }
    }
}

/// Check if the given NSAppearance is dark.
fn is_appearance_dark(appearance: Retained<NSAppearance>) -> bool {
    // SAFETY: Valid extern static
    let aqua_name = unsafe { NSAppearanceNameAqua };
    // SAFETY: Valid extern static
    let dark_aqua_name = unsafe { NSAppearanceNameDarkAqua };

    // Check which appearance is best matched (dark vs normal aqua)
    match appearance
        .bestMatchFromAppearancesWithNames(&NSArray::from_slice(&[aqua_name, dark_aqua_name]))
    {
        Some(appearance_name) => *appearance_name == *dark_aqua_name,
        None => false,
    }
}

pub struct Platform {
    workspace: Retained<NSWorkspace>,
    application: Retained<NSApplication>,
    _observer: Retained<ThemeObserver>,
    notify: Arc<Notify>,
}

impl Platform {
    pub fn new() -> Result<Self, Error> {
        let notify = Arc::new(Notify::new());

        let mtm = match MainThreadMarker::new() {
            Some(mtm) => mtm,
            None => return Err(Error::MainThreadRequired),
        };

        let application = NSApplication::sharedApplication(mtm);
        let workspace = NSWorkspace::sharedWorkspace();

        // Theme change observer
        let _observer = ThemeObserver::new(mtm, application.clone(), &workspace, notify.clone());

        Ok(Platform {
            workspace,
            application,
            _observer,
            notify,
        })
    }

    pub fn theme_kind(&self) -> Result<ThemeKind, Error> {
        Ok(ThemeKind::MacOS)
    }

    pub fn theme_scheme(&self) -> Result<ThemeScheme, Error> {
        // Method used is supported since 10.14
        if !available!(macos = 10.14) {
            return Err(Error::Unsupported);
        }

        if is_appearance_dark(self.application.effectiveAppearance()) {
            Ok(ThemeScheme::Dark)
        } else {
            Ok(ThemeScheme::Light)
        }
    }

    pub fn theme_contrast(&self) -> Result<ThemeContrast, Error> {
        // Method used is supported since 10.10
        if !available!(macos = 10.10) {
            return Err(Error::Unsupported);
        }

        let contrast = if self.workspace.accessibilityDisplayShouldIncreaseContrast() {
            ThemeContrast::High
        } else {
            ThemeContrast::Normal
        };

        Ok(contrast)
    }

    pub fn theme_accent(&self) -> Result<ThemeColor, Error> {
        // Method used is supported since 10.14
        if !available!(macos = 10.14) {
            return Err(Error::Unsupported);
        }

        match NSColor::controlAccentColor()
            .colorUsingColorSpace(&NSColorSpace::genericRGBColorSpace())
        {
            Some(color) => Ok(ThemeColor {
                red: color.redComponent() as f32,
                green: color.greenComponent() as f32,
                blue: color.blueComponent() as f32,
            }),
            None => Err(Error::Unavailable),
        }
    }

    pub fn get_notify(&self) -> Arc<Notify> {
        self.notify.clone()
    }
}
