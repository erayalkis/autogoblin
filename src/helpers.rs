use once_cell::sync::Lazy;
use systemstat::{System, Platform};

static SYS: once_cell::sync::Lazy<systemstat::platform::linux::PlatformImpl> = Lazy::new(|| {System::new()});


pub fn get_vitals() -> String {

  let mem = SYS.memory().unwrap();

  let vitals_string = format!("Memory available: {}", mem.free);

  return vitals_string;
}