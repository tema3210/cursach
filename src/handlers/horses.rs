use actix_web::{web, Responder};
use actix_web_codegen::{get};
//use diesel::MysqlConnection;
//use crate::{schema,lib};



#[get("/horses/info/{id}")]
pub async fn horse_info(_info: web::Path<(u64,)>) -> impl Responder{
	//TODO
	"Unimplemented".with_status(http::status::StatusCode::from_u16(501).unwrap())
}

#[get("/horses/info/{idl}-{idh}")]
pub async fn horse_info_many(_info: web::Path<(u64,u64)>) -> impl Responder{
	//TODO
	"Unimplemented".with_status(http::status::StatusCode::from_u16(501).unwrap())
}
#[get("/horses/of/{own_id}")]
pub async fn horse_of(_info: web::Path<(u64,)>) -> impl Responder {
	//TODO
	"Unimplemented".with_status(http::status::StatusCode::from_u16(501).unwrap())
}
