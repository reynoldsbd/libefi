use core::ffi::c_void;
use core::ptr;
use super::BootServices;

use types::{
    Status,
};

use core::mem;


/// UEFI Event
#[derive(Debug, Clone, Copy)]
pub struct Event(*const c_void);

unsafe impl Sync for Event { }

impl Default for Event {
    fn default() -> Event {
        Event(ptr::null())
    }
}


bitflags! {
    /// Specifies an Event's mode and attributes
    pub struct EventType: u32 {
        const TIMER = 0x8000_0000;
        const RUNTIME = 0x4000_0000;
        const NOTIFY_WAIT = 0x0000_0100;
        const NOTIFY_SIGNAL = 0x0000_0200;
        const SIGNAL_EXIT_BOOT_SERVICES = 0x0000_0201;
        const SIGNAL_VIRTUAL_ADDRESS_CHANGE = 0x6000_0202;
    }
}


/// Defines the type of a timer
#[repr(C)]
pub enum TimerDelay {
    Cancel,
    Periodic,
    Relative,
}


/// Task priority level
#[derive(Clone, Copy, Debug)]
#[repr(usize)]
pub enum TPL {
    Application = 4,
    Callback = 8,
    Notify = 16,
    HightLevel = 31,
}


impl BootServices {

    /// Creates an event
    pub fn create_event<T>(&self,
                           event_type: EventType,
                           notify_tpl: TPL,
                           notify_function: Option<extern "win64" fn(&Event, &T)>,
                           notify_context: &T)
        -> Result<&Event, Status> where T: ?Sized {

        // It's safe to cast notify_function to a different signature as long as the UEFI system
        // upholds its side of the spec and passes notify_context unmodified
        let notify_function: Option<extern "win64" fn(&Event, *const ())> =
            notify_function.map(|func| unsafe { mem::transmute(func) } );
        let notify_context = notify_context as *const T as *const ();

        let mut event = &Event(ptr::null());
        (self._create_event)(event_type, notify_tpl, notify_function, notify_context, &mut event)
            .as_result()
            .map(|_| event)
    }

    /// Closes the given event
    pub fn close_event(&self, event: &Event) -> Result<(), Status> {

        (self._close_event)(event)
            .as_result()
            .map(|_| ())
    }

    /// Signals the given event
    pub fn signal_event(&self, event: &Event) -> Result<(), Status> {

        (self._signal_event)(event)
            .as_result()
            .map(|_| ())
    }

    /// Stops execution until an event is signaled
    pub fn wait_for_event(&self, events: &[&Event]) -> Result<usize, Status> {

        let mut index: usize = 0;
        (self._wait_for_event)(events.len(), events.as_ptr(), &mut index)
            .as_result()
            .map(|_| index)
    }

    /// Checks whether an event is in the signaled state
    pub fn check_event(&self, event: &Event) -> Result<(), Status> {

        (self._check_event)(event)
            .as_result()
            .map(|_| ())
    }

    /// Sets the type of timer and the trigger time for a timer event.
    pub fn set_timer(&self, event: &Event, timer_type: TimerDelay, trigger_time: u64)
        -> Result<(), Status> {

        (self._set_timer)(event, timer_type, trigger_time)
            .as_result()
            .map(|_| ())
    }

    /// Raises a task’s priority level and returns its previous level.
    pub fn raise_tpl(&self, new_tpl: TPL) -> Result<(), Status> {

        (self._raise_tpl)(new_tpl)
            .as_result()
            .map(|_| ())
    }

    /// Restores a task’s priority level to its previous value.
    pub fn restore_tpl(&self, old_tpl: TPL) -> Result<(), Status> {

        (self._restore_tpl)(old_tpl)
            .as_result()
            .map(|_| ())
    }
}
