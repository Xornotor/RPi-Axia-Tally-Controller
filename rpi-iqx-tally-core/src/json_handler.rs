use std::io::{ErrorKind, Read};
use std::net::Ipv4Addr;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::error::Error;

// Tally struct model
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Tally {
    pub id_console: u8,
    pub id_fader: u8,
    pub gpio: u8,
    pub enable: bool,
}

// Axia Console struct model
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Console {
    pub id_console: u8,
    pub ip_addr: Ipv4Addr,
}

// Tally Config struct model
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TallyConfig {
    pub consoles: Vec<Console>,
    pub tallys: Vec<Tally>,
}

// Methods for TallyConfig struct
impl TallyConfig {
    pub fn standard() -> TallyConfig {
        TallyConfig{
            consoles: vec![
                Console {
                    id_console: 1,
                    ip_addr: Ipv4Addr::new(10, 216, 1, 131),
                },
                Console {
                    id_console: 2,
                    ip_addr: Ipv4Addr::new(10, 216, 1, 132),
                },
            ],
            tallys: vec![
                Tally {
                    id_console: 1,
                    id_fader: 1,
                    gpio: 14,
                    enable: true,
                },
                Tally {
                    id_console: 1,
                    id_fader: 2,
                    gpio: 15,
                    enable: true,
                },
                Tally {
                    id_console: 1,
                    id_fader: 3,
                    gpio: 16,
                    enable: true,
                },
                Tally {
                    id_console: 1,
                    id_fader: 4,
                    gpio: 17,
                    enable: true,
                },
            ],
        }
    }

    pub fn write_to_json(&self, path: &str) -> Result<File, Box<dyn Error>> {
        let serialized = serde_json::to_string_pretty(&self)?.into_bytes();
        let mut file = File::create(path)?;
        file.write_all(&serialized)?;
        Ok(file)
    }
}

pub fn init_tally_config() -> Result<TallyConfig, Box<dyn Error>> {
    let path = "tally_config.json";
    match File::open(path){
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let tally_config: TallyConfig = serde_json::from_str(&contents)?;
            return Ok(tally_config)
        },
        Err(err) => {
            if let ErrorKind::NotFound = err.kind() {
                let standard = TallyConfig::standard();
                standard.write_to_json(path)?;
                return Ok(standard)            
            } else {
                return Err(Box::new(err))
            }
        }
    };
}
