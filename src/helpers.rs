use once_cell::sync::Lazy;
use sysinfo::{System, SystemExt, CpuExt};
use std::sync::Mutex;

static SYS: once_cell::sync::Lazy<Mutex<sysinfo::System>> = Lazy::new(|| {Mutex::new(System::new())});


pub struct MachineVitals {
  pub mem_free: u64,
  pub mem_used: u64,
  pub cpu_usage: f32
}

pub fn get_vitals() -> MachineVitals {
  let mut sys = SYS.lock().unwrap();

  sys.refresh_all();

  let mem_free = sys.available_memory();
  let mem_used = sys.used_memory();

  sys.refresh_cpu();
  let cpu_usage = sys.global_cpu_info().cpu_usage();


  let vitals = MachineVitals {
    mem_free,
    mem_used,
    cpu_usage
  };

  return vitals;
}
