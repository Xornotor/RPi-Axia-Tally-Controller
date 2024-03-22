use actix_web::*;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::thread::JoinHandle;
use crate::ctrl_thread::*;

pub struct AppData {
	handle: Mutex<Box<JoinHandle<()>>>,
	tx: Mutex<Sender<String>>,
}

impl AppData {
	pub fn new(handle: Box<JoinHandle<()>>, tx: Sender<String>) -> AppData {
		AppData { 
			handle: Mutex::new(handle),
			tx: Mutex::new(tx)
		}
	}
	
	pub fn update(&self, handle: Box<JoinHandle<()>>, tx: Sender<String>) {
		let mut handle_mutex = self.handle.lock().unwrap();
		let mut tx_mutex = self.tx.lock().unwrap();
		*handle_mutex = handle;
		*tx_mutex = tx;
	}
	
	pub fn kill_thread(&self) {
		let _ = self.tx.lock().unwrap().send(String::from("KILL"));
		while !self.handle.lock().unwrap().is_finished() {}
	}
}

#[get("/stop")]
pub async fn stop(req: HttpRequest) -> impl Responder {
	let appdata = req.app_data::<web::Data<AppData>>().unwrap();
	appdata.kill_thread();
	HttpResponse::Ok().body("Tally Controller Service Finished.\n")
}
	
#[get("/restart")]
pub async fn restart(req: HttpRequest) -> impl Responder {
	let appdata = req.app_data::<web::Data<AppData>>().unwrap();
	appdata.kill_thread();
	println!("Restarting service...");
	let (ctrl_handle, tx_ctrl_kill) = start_ctrl_thread();
	appdata.update(ctrl_handle, tx_ctrl_kill);
	HttpResponse::Ok().body("Tally Controller Service Restarted.\n")
}
