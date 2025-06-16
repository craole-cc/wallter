use crate::config::monitor::{Size, Position};
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
  /// The size of the monitor
  pub size: Size,
  /// Height of the monitor's screen space (in pixels).
  pub height: u32,
  /// Width of the monitor's screen space (in pixels).
  pub width: u32,
  /// Ratio of the monitor's screen space (width / height).
  pub ratio: f32,
  /// The monitor's scale factor (DPI scaling, e.g., 1.0 for 100%).
  pub scale: f32,
  /// The monitor's position in the virtual screen space (x, y).
  pub position: Position,
  /// Whether the monitor is the primary monitor.
  pub primary: bool
}

impl Config {
  pub fn from_monitor_handle(handle: &MonitorHandle, id: u32) -> Self {
    let PhysicalSize { width, height } = handle.size();
    let PhysicalPosition { x, y } = handle.position();
    let ratio = if height > 0 {
      width as f32 / height as f32
    } else {
      0.0 //? Default ratio if height is 0, though unlikely for a monitor
    };

    let position = Point { x, y };
    let name = handle.name().unwrap_or_else(|| format!("Monitor {id}"));
    let scale = handle.scale_factor() as f32;

    Self {
      id,
      name,
      height,
      width,
      ratio,
      scale,
      position,
      primary: false
    }
  }

  /// Returns the monitor's resolution.
  pub fn resolution(&self) -> Resolution {
    Resolution {
      width: self.width,
      height: self.height
    }
  }

  /// Returns the monitor's orientation.
  pub fn orientation(&self) -> Orientation {
    Orientation::from_resolution(&self.resolution())
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
      fn window_event(
        &mut self,
        _: &ActiveEventLoop,
        _: WindowId,
        _: WindowEvent
      ) {
      }
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
    printf!(f, "Height", self.height, PAD)?;
    printf!(f, "Width", self.width, PAD)?;
    printf!(f, "Resolution", &self.resolution(), PAD)?;
    printf!(f, "Ratio", format!("{:.2}", self.ratio), PAD)?;
    printf!(f, "Orientation", &self.orientation(), PAD)?;
    printf!(f, "Scale", format!("{:.1}x", self.scale), PAD)?;
    printf!(f, "Position", &self.position, PAD)?;
    printf!(f, "Primary", self.primary, PAD)?;

    Ok(())
  }
}

 pub fn from_monitor_handle(handle: &MonitorHandle, id: u32) -> Self {
    let PhysicalSize { width, height } = handle.size();
    let PhysicalPosition { x, y } = handle.position();

    Self::new(handle, id, width, height, x, y)
  }
