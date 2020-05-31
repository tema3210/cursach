use actix_web::{web, Responder};
use actix_web_codegen::{get};
//use diesel::MysqlConnection;
use crate::{schema,lib};

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

#[get("/horses/info/{id}")]
pub async fn horse_info(info: web::Path<(i32,)>) -> impl Responder{
	println!("horses/info/id handler called");
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

// #[get("horses/of/run/{id}")]
pub async fn horse_coef(which: i32) -> Result<f64,&'static str> {
	println!("horses/of/run handler called");

	let prep_select = {
		use schema::Horses::dsl::*;
		Horses.select(WinRate).filter(ID.eq(which))
	};

	let res = lib::transaction(move |conn| {
		prep_select.load::<Option<f64>>(conn)
	}).await;
	match res {
		Ok(v) if v.len() == 1 => {
			if let Some(rate) = v[0] {
				Ok(1.0-(1.0-rate))
			} else {
				Err("winrate not set")
			}
		},
		Ok(v) => {
			Err("more than one horse_infose for given ID")
		},
		Err(_) => {
			Err("Internal error")
		}
	}

}

#[get("/horses/info/{idl}-{idh}")]
pub async fn horse_info_many(_info: web::Path<(u64,u64)>) -> impl Responder{
	println!("horses/info/l-h handler called");
	//TODO
	"Unimplemented".with_status(http::status::StatusCode::from_u16(501).unwrap())
}
#[get("/horses/of/owner/{own_id}")]
pub async fn horse_of(_info: web::Path<(u64,)>) -> impl Responder {
	println!("horses/of/ handler called");
	//TODO
	"Unimplemented".with_status(http::status::StatusCode::from_u16(501).unwrap())
}
