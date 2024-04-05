# Raspberry Pi Axia Tally Controller

Application which connects to Axia consoles and subscribes for ON/OFF updates in order to light up the tally indicators (Raspberry Pi GPIO's).

## Overview

This application is composed by two parts: 
*  A multithreaded backend application to manage connections, GPIO's and configuration changes;
*  A frontend interface for easy configuration of the consoles' IP addresses and fader-tally association.

### Frontend application

The frontend interface is a page that allows the user to associate faders to tallys.
It consists of a simple HTML/CSS/JS webpage deployed in nginx, which interacts with the API of the backend application.
The webpage can be accessed in any browser by the IP Address of the Raspberry Pi (default `10.216.1.80`).


![rpi-axia-webpage](https://github.com/Xornotor/RPi-Axia-Tally-Controller/assets/26725302/d1a02c55-5dcb-43da-a2af-404e32bea9a7)

### Backend application

The backend was developed in Rust, and consists of a multithreaded application that:
* Manage GPIO's (turn on/off the tally lights);
* Manage websocket connections with the Axia Consoles (LWCP, port 9010);
* Manage fader-tally association;
* Serves an API for reading/writing fader-tally association configuration.

## Pin Planning

| **Tally**    | **RPi GPIO** |
|--------------|--------------|
| Tally 1      | 19           |
| Tally 2      | 13           |
| Tally 3      | 6            |
| Tally 4      | 5            |
| Control Room | 26           |
