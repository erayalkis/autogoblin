use once_cell::sync::Lazy;
use sysinfo::{System, SystemExt};
use std::sync::Mutex;

static SYS: once_cell::sync::Lazy<Mutex<sysinfo::System>> = Lazy::new(|| {Mutex::new(System::new())});


pub struct MachineVitals {
  pub mem_free: u64,
  pub mem_used: u64
}

pub fn get_vitals() -> MachineVitals {
  let mut sys = SYS.lock().unwrap();

  sys.refresh_all();

  let mem_free = sys.available_memory();
  let mem_used = sys.used_memory();

  let vitals = MachineVitals {
    mem_free,
    mem_used
  };

  return vitals;
}