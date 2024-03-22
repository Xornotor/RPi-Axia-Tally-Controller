mod event_parser;
mod gpio_driver;
mod json_handler;
mod lwcp_handler;
mod api_handler;
mod ctrl_thread;

use crate::api_handler::*;
use crate::ctrl_thread::*;

use actix_web::*;
use actix_cors::Cors;
//use std::sync::{Arc, mpsc::{Sender}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

	let (ctrl_handle, tx_ctrl_kill) = start_ctrl_thread();
	let data = AppData::new(ctrl_handle, tx_ctrl_kill);
    let actix_data = web::Data::new(data);
    
    HttpServer::new(move || {
    	let cors = Cors::permissive();
		App::new()
			.wrap(cors)
			.app_data(web::Data::clone(&actix_data))
			.service(restart)
			.service(reconfig)
			.service(getconfig)
	}).bind(("127.0.0.1", 9000))?.run().await
}
