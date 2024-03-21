use actix_web::*;
use std::sync::mpsc::Sender;

#[get("/stop")]
pub async fn stop(req: HttpRequest) -> impl Responder {
	let ctrl_kill = req.app_data::<web::Data<Sender<String>>>().unwrap();
	let _ = ctrl_kill.send(String::from("KILL"));
	HttpResponse::Ok().body("Tally Controller Service Finished.\n")
}
	
#[get("/restart")]
pub async fn restart(req: HttpRequest) -> impl Responder {
	let ctrl_kill = req.app_data::<web::Data<Sender<String>>>().unwrap();
	let _ = ctrl_kill.send(String::from("KILL"));
	HttpResponse::Ok().body("Tally Controller Service Restarted.\n")
}
