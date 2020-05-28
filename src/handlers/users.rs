use actix_web::{web, Responder};
use actix_web_codegen::{get,post};

use crate::{schema,lib};

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

#[get("/users/about/{login}")]
pub async fn usr_about(info: web::Path<(String,)>) -> impl Responder {
	use std::convert::TryInto;

	println!("users/about handler called");

	let stmt_usr = {
		use schema::UserData::dsl::*;
		UserData.filter(Login.eq(Some(info.0.clone())))
	};

	let res = lib::transaction(|conn|{
		stmt_usr.load::<lib::ORM::UserData>(conn)
	}).await;

	match res {
		Ok(vec) if vec.len() == 1 => {
			let usr = &vec[0];
			match usr.PublicProfile {
				0 => {
					String::from("").with_status(403.try_into().unwrap())
				},
				1..=2 => {
					let ut = match usr.UserType {
						Some(1) => "Admin",
						Some(2) => "User",
						Some(3) => "Guest",
						Some(_) => "No such kind",
						None => "NULL",
					};
					match usr.PublicProfile {
						1 => {
							use serde_json::json;
							json!({
								"User type" : ut,
								"About": usr.AssocInf,
								//"Credits": usr.Credits,
								//"Balance": usr.Balance,
							}).to_string().with_status(200u16.try_into().unwrap())
						},
						2 => {
							use serde_json::json;
							json!({
								"User type" : ut,
								"About": usr.AssocInf,
								"Credits": usr.Credits,
								"Balance": usr.Balance,
							}).to_string().with_status(200u16.try_into().unwrap())
						},
						_ => unreachable!(),
					}
				},
				_ => {
					String::from("").with_status(500u16.try_into().unwrap())
				}
			}
		},
		Ok(vec) if vec.len() == 0 => {
			String::from("").with_status(404u16.try_into().unwrap())
		},
		Ok(vec) if vec.len() > 1 => {
			String::from("").with_status(500u16.try_into().unwrap())
		},
		Ok(_) => {
			String::from("").with_status(500u16.try_into().unwrap())
		},
		Err(_) => {
			String::from("").with_status(500u16.try_into().unwrap())
		},
	}

}


#[post("/users/login")]
pub async fn usr_login(info: web::Json<lib::Protocol::UserLoginPayload>) -> impl Responder {
	println!("users/login handler called");
	use schema::UserData::dsl::*;
	use serde_json;

	let passwh = base64::decode(&info.passwh);
	if let Err(_) = passwh {
		use std::convert::TryInto;
		return String::from("").with_status(400u16.try_into().unwrap());
	};

	let stmt = UserData.select(ID)
		.filter(Login.eq(Some(info.login.clone())))
		.filter(Passwh.eq(Some(passwh.unwrap())));

	let res = lib::transaction(|conn| {
		stmt.load::<i32>(conn)
	}).await;

	match res {
		Ok(x) => {
			match x.len() {
				0 => {
					String::from("").with_status(http::status::StatusCode::from_u16(403).unwrap())
				},
				1 => {
					use serde_json::json;
					json!({
						"id": x
					}).to_string().with_status(http::status::StatusCode::from_u16(200).unwrap())
				},
				_ => {
					String::from("").with_status(http::status::StatusCode::from_u16(500).unwrap())
				}
			}

		},
		Err(_err) => {
			String::from("DB error").with_status(http::status::StatusCode::from_u16(500).unwrap())
		},
	}
}

#[post("/users/register")]
pub async fn usr_reg(info: web::Json<lib::Protocol::UserRegisterPayload>) -> impl Responder {
	println!("users/reg handler called");
	"Unimplemented".with_status(http::status::StatusCode::from_u16(501).unwrap())
}
