use once_cell::sync::Lazy;
use systemstat::{System, Platform, ByteSize, Duration};

static SYS: once_cell::sync::Lazy<systemstat::platform::linux::PlatformImpl> = Lazy::new(|| {System::new()});

pub struct MachineVitals {
  pub mem_free: ByteSize,
  pub mem_total: ByteSize,
  pub cpu_temp: f32,
  pub uptime: Duration
}

pub fn get_vitals() -> MachineVitals {

  let mem = SYS.memory().unwrap();
  let cpu_temp = SYS.cpu_temp().unwrap();
  let uptime = SYS.uptime().unwrap();


  let vitals = MachineVitals  {
    mem_free: mem.free,
    mem_total: mem.total,
    cpu_temp,
    uptime
  };

  return vitals;
}