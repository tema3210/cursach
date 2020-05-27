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
					use serde_json::builder::ObjectBuilder;

					let ut = match usr.UserType {
						Some(1) => "Admin",
						Some(2) => "User",
						Some(3) => "Guest",
						Some(_) => unreachable!(),
						None => "NULL",
					};
					let mut builder = ObjectBuilder::new();
					//Insert allowed on lvl 1 data
					builder = builder.insert_into("UserType",ut).insert_into("About",usr.AssocInf);
					if usr.PublicProfile == 2 {
						//Insert allowed on lvl 2 data
						builder = builder.insert_into("Credits",usr.Credits).insert_into("Balance",usr.Balance);
					};
					builder.build().to_string().with_status(200u16.try_into().unwrap())
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
		Err(_) => {
			String::from("").with_status(500u16.try_into().unwrap())
		},
	}

}

// #[get("/users/about/{login}")]
// pub async fn usr_about(info: web::Path<(String,)>) -> impl Responder {
// 	println!("users/about handler called");
// 	use serde_json;
//
// 	let stmt_usr = {
// 		use schema::UserData::dsl::*;
// 		UserData.filter(Login.eq(Some(info.0.clone())))
// 	};
// 	let (res,code) = {
// 		let res = lib::transaction(|conn|{
// 			stmt_usr.load::<lib::ORM::UserData>(conn)
// 		}).await;
//
// 		match res {
// 			Ok(vec) => {
// 				match vec.len() {
// 					0 => {
// 						(None,404)
// 					},
// 					1 => {
// 						(Some(vec[0].clone()),200)
// 					},
// 					_ => {
// 						(None,500)
// 					}
// 				}
// 			},
// 			Err(_) => {
// 				(None,500)
// 			}
// 		}
//
// 	};
// 	match code {
// 		404 => {
// 			String::from("").with_status(http::status::StatusCode::from_u16(404).unwrap())
// 		},
// 		500 => {
// 			String::from("").with_status(http::status::StatusCode::from_u16(500).unwrap())
// 		},
// 		200 => {
// 			let res = res.unwrap();//It's guranteed not to panic.
// 			let mut new_code = code;
// 			let t: String = {
// 				match res.PublicProfile {
// 					0 => {
// 						new_code = 403;
// 						String::from("")
// 					},
// 					1 => {
// 						serde_json::json!({
// 							"UserType": res.UserType,
// 							//"Credits": res.Credits,
// 							//"Balance": res.Balance,
// 							"About": res.AssocInf,
// 						}).to_string()
// 					},
// 					2 => {
// 						let ut = match res.UserType {
// 								Some(1) => "Admin",
// 								Some(2) => "User",
// 								Some(3) => "Guest",
// 								Some(_) => unreachable!(),
// 								None => "NULL",
// 						};
// 						serde_json::json!({
// 							"UserType": ut,
// 							"Credits": res.Credits,
// 							"Balance": res.Balance,
// 							"About": res.AssocInf,
// 						}).to_string()
// 					},
// 					_ => {
// 						new_code=500;
// 						String::from("")
// 					}
// 				}
//
// 			};
// 			t.with_status(http::status::StatusCode::from_u16(new_code).unwrap())
// 		},
// 		_ => unreachable!()
// 	}
//
// }

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
			String::from("DB error").with_status(http::status::StatusCode::from_u16(500).unwrap())
		},
	}
}

#[post("/users/register")]
pub async fn usr_reg(info: web::Json<lib::Protocol::UserRegisterPayload>) -> impl Responder {
	println!("users/reg handler called");
	"Unimplemented".with_status(http::status::StatusCode::from_u16(501).unwrap())
}
