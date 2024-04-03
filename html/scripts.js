async function get_config() {
    var tally_config;
    const res = await fetch("http://10.216.1.80:9000/getconfig")
      .catch((error) => {
        alert("ERRO: Falha na conexão com a API.");
      });
    tally_config = await res.json();
    initialFill(tally_config);
}

function initialFill(cfg){
    document.getElementById("ip_addr_console_1").value = cfg.consoles[0].ip_addr;
    document.getElementById("ip_addr_console_2").value = cfg.consoles[1].ip_addr;

    document.getElementById("id_console_tally_1").value = cfg.tallys[0].id_console;
    document.getElementById("fader_tally_1").value = cfg.tallys[0].id_fader;
    document.getElementById("gpio_tally_1").value = cfg.tallys[0].gpio_relay;

    document.getElementById("id_console_tally_2").value = cfg.tallys[1].id_console;
    document.getElementById("fader_tally_2").value = cfg.tallys[1].id_fader;
    document.getElementById("gpio_tally_2").value = cfg.tallys[1].gpio_relay;

    document.getElementById("id_console_tally_3").value = cfg.tallys[2].id_console;
    document.getElementById("fader_tally_3").value = cfg.tallys[2].id_fader;
    document.getElementById("gpio_tally_3").value = cfg.tallys[2].gpio_relay;

    document.getElementById("id_console_tally_4").value = cfg.tallys[3].id_console;
    document.getElementById("fader_tally_4").value = cfg.tallys[3].id_fader;
    document.getElementById("gpio_tally_4").value = cfg.tallys[3].gpio_relay;

    document.getElementById("id_console_tally_5").value = cfg.tallys[4].id_console;
    document.getElementById("gpio_tally_5").value = cfg.tallys[4].gpio_relay;
}

function validateIPaddress(ipaddress) {  
  if (/^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/.test(ipaddress)) {  
    return (true)  
  } 
  return (false)  
} 

async function reconfig() {
    let ip_console_1 = document.getElementById("ip_addr_console_1").value;
    let ip_console_2 = document.getElementById("ip_addr_console_2").value;
    
    let console_tally_cr = Number(document.getElementById("id_console_tally_5").value);

    let console_tally_1 = Number(document.getElementById("id_console_tally_1").value);
    let console_tally_2 = Number(document.getElementById("id_console_tally_2").value);
    let console_tally_3 = Number(document.getElementById("id_console_tally_3").value);
    let console_tally_4 = Number(document.getElementById("id_console_tally_4").value);
    let fader_tally_1 = Number(document.getElementById("fader_tally_1").value);
    let fader_tally_2 = Number(document.getElementById("fader_tally_2").value);
    let fader_tally_3 = Number(document.getElementById("fader_tally_3").value);
    let fader_tally_4 = Number(document.getElementById("fader_tally_4").value);

    if(!validateIPaddress(ip_console_1)){
      alert("ERRO: IP do Console Principal Inválido.");
      return;
    }

    if(!validateIPaddress(ip_console_2)){
      alert("ERRO: IP do Console Reserva Inválido.");
      return;
    }

    if( (console_tally_1 == console_tally_2 && fader_tally_1 == fader_tally_2) ||
        (console_tally_1 == console_tally_3 && fader_tally_1 == fader_tally_3) ||
        (console_tally_1 == console_tally_4 && fader_tally_1 == fader_tally_4) ||
        (console_tally_2 == console_tally_3 && fader_tally_2 == fader_tally_3) ||
        (console_tally_2 == console_tally_4 && fader_tally_2 == fader_tally_4) ||
        (console_tally_3 == console_tally_4 && fader_tally_3 == fader_tally_4)
      ) {
      alert("ERRO: Dois tallys não podem ser associados ao mesmo fader.");
      return;
    }

    let tally_config = {
        "consoles": [
            {
              "id_console": 1,
              "ip_addr": ip_console_1
            },
            {
              "id_console": 2,
              "ip_addr": ip_console_2
            }
          ],
          "tallys": [
            {
              "id_console": console_tally_1,
              "id_fader": fader_tally_1,
              "gpio_relay": 25,
              "enable": true
            },
            {
              "id_console": console_tally_2,
              "id_fader": fader_tally_2,
              "gpio_relay": 8,
              "enable": true
            },
            {
              "id_console": console_tally_3,
              "id_fader": fader_tally_3,
              "gpio_relay": 7,
              "enable": true
            },
            {
              "id_console": console_tally_4,
              "id_fader": fader_tally_4,
              "gpio_relay": 1,
              "enable": true
            },
            {
              "id_console": console_tally_cr,
              "id_fader": 255,
              "gpio_relay": 12,
              "enable": true
            },
          ]
    };

    const send_req = await fetch("http://10.216.1.80:9000/reconfig", {
        method: "POST",
        body: JSON.stringify(tally_config),
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json'
        },
    }).catch((error) => {
      alert("ERRO: Falha na conexão com a API.");
    });

    if(send_req.ok){
      alert("Configuração aplicada com sucesso.");
    }
}

get_config()
