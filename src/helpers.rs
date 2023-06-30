use once_cell::sync::Lazy;
use sysinfo::{System, SystemExt, CpuExt};
use std::sync::Mutex;
use serde_yaml;
use reqwest;

static SYS: once_cell::sync::Lazy<Mutex<sysinfo::System>> = Lazy::new(|| {Mutex::new(System::new())});


pub struct MachineVitals {
  pub mem_free: u64,
  pub mem_used: u64,
  pub cpu_usage: f32
}

pub struct Server {
  pub image: String,
  pub port: i64
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

pub fn get_servers() -> Vec<Server> {
  let yaml_data = std::fs::File::open("containers.yml").unwrap();
  let data: serde_yaml::Value = serde_yaml::from_reader(yaml_data).unwrap();

  let servers = data.get("containers").unwrap().as_sequence().unwrap();
  let mut servers_vec: Vec<Server> = Vec::new();

  for server in servers {
    let server_struct = Server {
      image: server.get("image").unwrap().as_str().unwrap().to_string(),
      port: server.get("port").unwrap().as_i64().unwrap()
    };

    servers_vec.push(server_struct);
  }

  return servers_vec;
}

pub fn probe_port(port: i64) -> bool {
  let ip = format!("localhost:{}", port);
  let result = reqwest::blocking::get(ip).unwrap();

  result.status() == 200
}