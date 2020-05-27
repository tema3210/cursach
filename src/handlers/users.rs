use actix_web::{web, Responder};
use actix_web_codegen::{get,post};

use crate::{schema,lib};

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

#[get("/users/login")]
pub async fn usr_login(info: web::Json<lib::Protocol::UserLoginPayload>) -> impl Responder {
	println!("users/login handler called");
	use schema::UserData::dsl::*;
	use serde_json;

	let stmt = UserData.select(ID)
		.filter(Login.eq(Some(info.login.clone())))
		.filter(Passwh.eq(Some(info.passwh.clone())));

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
					serde_json::json!({
						"id": x
					}).to_string().with_status(http::status::StatusCode::from_u16(200).unwrap())
				},
				_ => {
					String::from("").with_status(http::status::StatusCode::from_u16(500).unwrap())
				}
			}

		},
		Err(_err) => {
			String::from("").with_status(http::status::StatusCode::from_u16(500).unwrap())
		},
	}
}

#[post("/users/register")]
pub async fn usr_reg(info: web::Json<lib::Protocol::UserRegisterPayload>) -> impl Responder {
	println!("users/reg handler called");
	"Unimplemented".with_status(http::status::StatusCode::from_u16(501).unwrap())
}

#[get("/users/about/{login}")]
pub async fn usr_about(info: web::Path<(String,)>) -> impl Responder {
	println!("users/about handler called");
	use serde_json;

	let stmt_usr = {
		use schema::UserData::dsl::*;
		UserData.filter(Login.eq(Some(info.0.clone())))
	};
	let (res,code) = {
		let res = lib::transaction(|conn|{
			stmt_usr.load::<lib::ORM::UserData>(conn)
		}).await;

		match res {
			Ok(vec) => {
				match vec.len() {
					0 => {
						(None,404)
					},
					1 => {
						(Some(vec[0].clone()),200)
					},
					_ => {
						(None,500)
					}
				}
			},
			Err(_) => {
				(None,500)
			}
		}

	};
	match code {
		404 => {
			String::from("").with_status(http::status::StatusCode::from_u16(404).unwrap())
		},
		500 => {
			String::from("").with_status(http::status::StatusCode::from_u16(500).unwrap())
		},
		200 => {
			let res = res.unwrap();//It's guranteed not to panic.
			let mut new_code = code;
			let t: String = {
				match res.PublicProfile {
					0 => {
						new_code = 403;
						String::from("")
					},
					1 => {
						serde_json::json!({
							"UserType": res.UserType,
							//"Credits": res.Credits,
							//"Balance": res.Balance,
							"About": res.AssocInf,
						}).to_string()
					},
					2 => {
						serde_json::json!({
							"UserType": res.UserType,
							"Credits": res.Credits,
							"Balance": res.Balance,
							"About": res.AssocInf,
						}).to_string()
					},
					_ => {
						new_code=500;
						String::from("")
					}
				}

			};
			t.with_status(http::status::StatusCode::from_u16(new_code).unwrap())
		},
		_ => unreachable!()
	}

}
