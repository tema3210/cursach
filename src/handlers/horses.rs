use actix_web::{web, Responder};
use actix_web_codegen::{get};
//use diesel::MysqlConnection;
use crate::{schema,lib};

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

#[get("/horses/info/{id}")]
pub async fn horse_info(info: web::Path<(i32,)>) -> impl Responder{
	let prep_select = {
		use schema::Horses::dsl::*;
		Horses.filter(ID.eq(info.0))
	};
	let resp = lib::transaction(move |conn|{
		prep_select.execute(conn)
	}).await;

	use std::convert::TryInto;
	match resp {
		Ok(horse) => {
			serde_json::ser::to_string(&horse).unwrap().with_status(200u16.try_into().unwrap())
		},
		Err(_) => {
			String::from("").with_status(500u16.try_into().unwrap())
		}
	}
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
