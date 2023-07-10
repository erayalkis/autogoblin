use once_cell::sync::Lazy;
use sysinfo::{System, SystemExt, CpuExt};
use std::sync::Mutex;
use serde_yaml;
use reqwest::{self, Response, Error};
use rand::Rng;

pub static SYS: once_cell::sync::Lazy<Mutex<sysinfo::System>> = Lazy::new(|| {Mutex::new(System::new())});

pub struct MachineVitals {
  pub mem_free: u64,
  pub mem_used: u64,
  pub cpu_usage: f32,
  pub uptime: u64,
}

pub struct Server {
  pub image: String,
  pub name: String,
  pub port: i64,
  pub endpoint: Option<String>
}

pub fn get_vitals() -> MachineVitals {
  let mut sys = SYS.lock().unwrap();

  sys.refresh_all();

  let mem_free = sys.available_memory();
  let mem_used = sys.used_memory();

  sys.refresh_cpu();
  let cpu_usage = sys.global_cpu_info().cpu_usage();

  let uptime = sys.uptime();


  let vitals = MachineVitals {
    mem_free,
    mem_used,
    cpu_usage,
    uptime
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
      name: server.get("name").unwrap().as_str().unwrap().to_string(),
      port: server.get("port").unwrap().as_i64().unwrap(),
      endpoint: match server.get("endpoint") {
        Some(val) => {
          Some(val.as_str().unwrap().to_string())
        }

        None => {
          None
        }
      }
    };

    servers_vec.push(server_struct);
  }

  return servers_vec;
}

pub async fn probe_port(port: &i64, name: &String, endpoint: &Option<String>) -> bool {

  let ip = if endpoint.is_none() {
    format!("http://{}:{}", name, port)
  } else {
    let endp = endpoint.clone().unwrap();
    if !endp.starts_with("/") {
      println!("Warning! Endpoint does not start with /, this will result in an invalid request!");
    }

    format!("http://{}:{}{}", name, port, endp)
  };

  match reqwest::get(ip).await {
    Ok(res) => {
      println!("{:?}", res);
      return true
    }
    Err(err) => {
      println!("{}", err);
      return false
    }
  };
}

pub async fn generate_random_number(start: u64, end: u64) -> u64 {
  rand::thread_rng().gen_range(start, end)
}

pub async fn up_server(server_name: &String) -> Result<Response, Error> {
  let url = format!("host.docker.internal:8000/up/{}", server_name);

  reqwest::get(url).await
}

pub async fn down_server(server_name: &String) -> Result<Response, Error> {
  let url = format!("host.docker.internal:8000/down/{}", server_name);

  reqwest::get(url).await 
}

pub fn get_argument_from_command(command_content: &String) -> String {
  let split: Vec<String> = command_content.split(" ").collect();
  split[1]
}