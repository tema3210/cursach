use actix_web::{web, Responder};
use actix_web_codegen::{get};
//use diesel::MysqlConnection;
//use crate::{schema,lib};



#[get("/owners/about/{id}")]
pub async fn owner_about(info: web::Path<(u64,)>) -> impl Responder {
	println!("owners/about handler called");
	"Unimplemented".with_status(http::status::StatusCode::from_u16(501).unwrap())
}

#[get("/owners/of/{h_id}")]
pub async fn owner_of(info: web::Path<(u64,)>) -> impl Responder {
	println!("owners/of handler called");
	"Unimplemented".with_status(http::status::StatusCode::from_u16(501).unwrap())
}
