use actix_web::{Responder, HttpRequest};
use actix_web_codegen::{post};
//use diesel::MysqlConnection;
//use crate::{schema,lib};



#[post("/paym/in")]
pub async fn in_paym(req: HttpRequest) -> impl Responder {
	println!("paym/in handler called");
	"Unimplemented".with_status(http::status::StatusCode::from_u16(501).unwrap())
}

#[post("/paym/out")]
pub async fn out_paym(req: HttpRequest) -> impl Responder {
	println!("paym/out handler called");
	"Unimplemented".with_status(http::status::StatusCode::from_u16(501).unwrap())
}
