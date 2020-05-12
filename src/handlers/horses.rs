use actix_web::{web, Responder};
use actix_web_codegen::{get};
//use diesel::MysqlConnection;
use crate::{schema,lib};



#[get("/horses/info/{id}")]
pub async fn horse_info(_info: web::Path<(u64,)>) -> impl Responder{
	let prep_select = {
		use schema::Horses::dsl::*;
		Horses.filter(ID.eq(info.0))
	}
	let resp = lib::transaction(move |conn|{
		prep_select.load::<lib::ORM::Horses>(conn)
	}).await;

	match resp {
		Ok(horse) => {
			serde_json::ser::to_string(&horse).unwrap().with_status(200.try_into().unwrap())
		},
		Err(_) => {
			String::from("").with_status(500.try_into().unwrap())
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
