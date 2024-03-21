mod event_parser;
mod gpio_driver;
mod json_handler;
mod lwcp_handler;
mod api_handler;
mod ctrl_thread;

use crate::api_handler::*;
use crate::ctrl_thread::*;

use actix_web::*;
use std::sync::{Arc, mpsc::{Sender}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

	let (ctrl_handle, mut tx_ctrl_kill) = start_ctrl_thread();
	let mut arc_tx_ctrl_kill = Arc::new(tx_ctrl_kill);
	let mut api_tx_ctrl_kill = web::Data::from(arc_tx_ctrl_kill);
    
    HttpServer::new(move || {
		App::new()
			.app_data(web::Data::clone(&api_tx_ctrl_kill))
			.service(stop)
			//.service(restart)
	}).bind(("127.0.0.1", 9000))?.run().await
}
