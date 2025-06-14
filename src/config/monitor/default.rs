use crate::config::monitor::{Orientation, Resolution};
use serde::{Deserialize, Serialize};
use std::{
  cell::RefCell,
  fmt::{self, Display, Formatter}
};
use winit::{
  application::ApplicationHandler,
  dpi::{PhysicalPosition, PhysicalSize},
  event::WindowEvent,
  event_loop::{ActiveEventLoop, EventLoop},
  monitor::MonitorHandle,
  window::WindowId
};

/// Represents a physical monitor and its properties.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
  /// Unique identifier for the monitor (based on enumeration order).
  pub id: u32,
  /// Human-readable monitor name (e.g., "DP-1", "HDMI-1").
  pub name: String,
  /// The monitor's pixel resolution.
  pub resolution: Resolution,
  /// The orientation of the monitor (landscape, portrait, or square).
  pub orientation: Orientation,
  /// The monitor's scale factor (DPI scaling, e.g., 1.0 for 100%).
  pub scale: f32,
  /// The monitor's position in the virtual screen space (x, y).
  pub position: (i32, i32),
  /// Whether the monitor is the primary monitor.
  pub primary: bool
}

impl Config {
  pub fn from_monitor_handle(handle: &MonitorHandle, id: u32) -> Self {
    let PhysicalSize { width, height } = handle.size();
    let PhysicalPosition { x, y } = handle.position();
    let resolution = Resolution { width, height };
    let orientation = Orientation::from_resolution(&resolution);

    Self {
      id,
      name: handle.name().unwrap_or_else(|| format!("Monitor {id}")),
      resolution,
      orientation,
      scale: handle.scale_factor() as f32,
      position: (x, y),
      primary: false
    }
  }

  /// Enumerate all monitors and return their information.
  pub fn enumerate() -> Vec<Self> {
    let result = RefCell::new(Vec::new());

    struct Handler<'a> {
      result: &'a RefCell<Vec<Config>>
    }

    impl ApplicationHandler for Handler<'_> {
      fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        //{ Get the primary monitor handle }
        let primary_monitor = event_loop.primary_monitor();

        //{ Enumerate all monitors and store their info in the result }
        let monitors = event_loop
          .available_monitors()
          .enumerate()
          .map(|(i, handle)| {
            let mut monitor = Config::from_monitor_handle(&handle, i as u32);

            //{ Determine if this is the primary monitor }
            monitor.primary = match &primary_monitor {
              Some(primary) => primary == &handle,
              None => false
            };
            monitor
          })
          .collect();

        //{ Set the result and exit the event loop }
        *self.result.borrow_mut() = monitors;
        event_loop.exit();
      }

      //{ Implement the other event handlers as no-ops }
      fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, _: WindowEvent) {}
      fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        _device_event: winit::event::DeviceEvent
      ) {
        // No-op implementation
      }
      fn suspended(&mut self, _: &ActiveEventLoop) {}
      fn memory_warning(&mut self, _: &ActiveEventLoop) {}
    }

    let event_loop = EventLoop::new().unwrap();
    let mut handler = Handler { result: &result };
    let _ = event_loop.run_app(&mut handler);

    result.into_inner()
  }
}

impl Display for Config {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    //{ Set the padding width for alignment }
    const PAD: usize = 16;

    //{ Strip the leading "\\.\" from the monitor name }
    let display_name = self.name.strip_prefix(r"\\.\").unwrap_or(&self.name);

    //{ Print each field with uniform style }
    printf!(f, "Id", self.id, PAD)?;
    printf!(f, "Name", display_name, PAD)?;
    printf!(f, "Resolution", &self.resolution, PAD)?;
    printf!(f, "Orientation", &self.orientation, PAD)?;
    printf!(f, "Scale", format!("{:.1}x", self.scale), PAD)?;
    printf!(
      f,
      "Position",
      format!("({}, {})", self.position.0, self.position.1),
      PAD
    )?;
    printf!(f, "Primary", self.primary, PAD)?;

    Ok(())
  }
}
