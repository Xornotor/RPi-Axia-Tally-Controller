async function get_config() {
    var tally_config;
    const res = await fetch("http://127.0.0.1:9000/getconfig");
    tally_config = await res.json();
    console.log(tally_config);
    initialFill(tally_config);
}

function initialFill(cfg){
    document.getElementById("id_console_1").value = cfg.consoles[0].id_console;
    document.getElementById("id_console_2").value = cfg.consoles[1].id_console;
    document.getElementById("ip_addr_console_1").value = cfg.consoles[0].ip_addr;
    document.getElementById("ip_addr_console_2").value = cfg.consoles[1].ip_addr;

    document.getElementById("id_console_tally_1").value = cfg.tallys[0].id_console;
    document.getElementById("fader_tally_1").value = cfg.tallys[0].id_fader;
    document.getElementById("gpio_tally_1").value = cfg.tallys[0].gpio;
    document.getElementById("enable_tally_1").checked = cfg.tallys[0].enable;

    document.getElementById("id_console_tally_2").value = cfg.tallys[1].id_console;
    document.getElementById("fader_tally_2").value = cfg.tallys[1].id_fader;
    document.getElementById("gpio_tally_2").value = cfg.tallys[1].gpio;
    document.getElementById("enable_tally_2").checked = cfg.tallys[1].enable;

    document.getElementById("id_console_tally_3").value = cfg.tallys[2].id_console;
    document.getElementById("fader_tally_3").value = cfg.tallys[2].id_fader;
    document.getElementById("gpio_tally_3").value = cfg.tallys[2].gpio;
    document.getElementById("enable_tally_3").checked = cfg.tallys[2].enable;

    document.getElementById("id_console_tally_4").value = cfg.tallys[3].id_console;
    document.getElementById("fader_tally_4").value = cfg.tallys[3].id_fader;
    document.getElementById("gpio_tally_4").value = cfg.tallys[3].gpio;
    document.getElementById("enable_tally_4").checked = cfg.tallys[3].enable;

    document.getElementById("id_console_tally_5").value = cfg.tallys[4].id_console;
    document.getElementById("gpio_tally_5").value = cfg.tallys[4].gpio;
    document.getElementById("enable_tally_5").checked = cfg.tallys[4].enable;
}

function reconfig() {
    let tally_config = {
        "consoles": [
            {
              "id_console": Number(document.getElementById("id_console_1").value),
              "ip_addr": document.getElementById("ip_addr_console_1").value
            },
            {
              "id_console": Number(document.getElementById("id_console_2").value),
              "ip_addr": document.getElementById("ip_addr_console_2").value
            }
          ],
          "tallys": [
            {
              "id_console": Number(document.getElementById("id_console_tally_1").value),
              "id_fader": Number(document.getElementById("fader_tally_1").value),
              "gpio": Number(document.getElementById("gpio_tally_1").value),
              "enable": document.getElementById("enable_tally_1").checked
            },
            {
              "id_console": Number(document.getElementById("id_console_tally_2").value),
              "id_fader": Number(document.getElementById("fader_tally_2").value),
              "gpio": Number(document.getElementById("gpio_tally_2").value),
              "enable": document.getElementById("enable_tally_2").checked
            },
            {
              "id_console": Number(document.getElementById("id_console_tally_3").value),
              "id_fader": Number(document.getElementById("fader_tally_3").value),
              "gpio": Number(document.getElementById("gpio_tally_3").value),
              "enable": document.getElementById("enable_tally_3").checked
            },
            {
              "id_console": Number(document.getElementById("id_console_tally_4").value),
              "id_fader": Number(document.getElementById("fader_tally_4").value),
              "gpio": Number(document.getElementById("gpio_tally_4").value),
              "enable": document.getElementById("enable_tally_4").checked
            },
            {
              "id_console": Number(document.getElementById("id_console_tally_5").value),
              "id_fader": 255,
              "gpio": Number(document.getElementById("gpio_tally_5").value),
              "enable": document.getElementById("enable_tally_5").checked
            },
          ]
    };

    fetch("http://127.0.0.1:9000/reconfig", {
        method: "POST",
        body: JSON.stringify(tally_config),
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json'
        },
    });
}

get_config()