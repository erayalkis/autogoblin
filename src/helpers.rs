use local_ip_address::local_ip;
use once_cell::sync::Lazy;
use rand::Rng;
use reqwest::{self, Error, Response};
use serde_yaml;
use std::{env, sync::Mutex};
use sysinfo::{CpuExt, System, SystemExt};

pub static SYS: once_cell::sync::Lazy<Mutex<sysinfo::System>> =
    Lazy::new(|| Mutex::new(System::new()));

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
    pub endpoint: Option<String>,
    pub ip: Option<u32>,
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
        uptime,
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
                Some(val) => Some(val.as_str().unwrap().to_string()),

                None => None,
            },
            ip: match server.get("ip") {
                Some(val) => Some(val.as_u64().unwrap() as u32),
                None => None,
            },
        };

        servers_vec.push(server_struct);
    }

    return servers_vec;
}

pub async fn probe_port(port: &i64, endpoint: &Option<String>, ip: &Option<u32>) -> bool {
    let local = local_ip().unwrap();

    let ip = if endpoint.is_none() {
        if ip.is_none() {
            format!("http://{}:{}", local, port)
        } else {
            format!("http://{}:{}", ip.unwrap(), port)
        }
    } else {
        let endp = endpoint.clone().unwrap();
        if !endp.starts_with("/") {
            println!(
                "Warning! Endpoint does not start with /, this will result in an invalid request!"
            );
        }

        if ip.is_none() {
            format!("http://{}:{}{}", local, port, endp)
        } else {
            format!("http://{}:{}{}", ip.unwrap(), port, endp)
        }
    };

    match reqwest::get(ip).await {
        Ok(res) => {
            println!("{:?}", res);
            return true;
        }
        Err(err) => {
            println!("{}", err);
            return false;
        }
    };
}

pub async fn generate_random_number(start: u64, end: u64) -> u64 {
    rand::thread_rng().gen_range(start, end)
}

fn get_beholder_url(path: String) -> String {
    let beholder_url = env::var("BEHOLDER_URL").unwrap();

    if path.len() > 0 {
        let full_url = format!("{}/{}", beholder_url, path);
        return full_url;
    }

    return beholder_url;
}

pub async fn up_server(server_name: &str) -> Result<Response, Error> {
    let path = format!("up/{}", server_name);
    let url = get_beholder_url(path);

    reqwest::get(url).await
}

pub async fn down_server(server_name: &str) -> Result<Response, Error> {
    let path = format!("down/{}", server_name);
    let url = get_beholder_url(path);

    reqwest::get(url).await
}

pub fn get_argument_from_command(command_content: &String) -> &str {
    let split: Vec<&str> = command_content.split(" ").collect();
    split[1]
}
