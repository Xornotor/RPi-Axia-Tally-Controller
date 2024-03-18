use std::net::Ipv4Addr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tally {
    pub id_mesa: u8,
    pub id_canal: u8,
    pub gpio: u8,
    pub enable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mesa {
    pub id_mesa: u8,
    pub ip_addr: Ipv4Addr,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TallyConfig {
    pub mesas: Vec<Mesa>,
    pub tallys: Vec<Tally>,
}

impl TallyConfig {

    pub fn standard() -> TallyConfig {
        TallyConfig{
            mesas: vec![
                Mesa {
                    id_mesa: 1,
                    ip_addr: Ipv4Addr::new(10, 216, 1, 133),
                },
                Mesa {
                    id_mesa: 2,
                    ip_addr: Ipv4Addr::new(10, 216, 1, 134),
                },
            ],
            tallys: vec![
                Tally {
                    id_mesa: 1,
                    id_canal: 1,
                    gpio: 14,
                    enable: true,
                },
                Tally {
                    id_mesa: 1,
                    id_canal: 2,
                    gpio: 15,
                    enable: true,
                },
                Tally {
                    id_mesa: 1,
                    id_canal: 3,
                    gpio: 16,
                    enable: true,
                },
                Tally {
                    id_mesa: 1,
                    id_canal: 4,
                    gpio: 17,
                    enable: true,
                },
            ],
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self)
    }
}
