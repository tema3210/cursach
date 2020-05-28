use actix_web::{web, Responder};
use actix_web_codegen::{get,post};
use crate::{schema,lib};

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

#[get("/bet/about/{id}")]
pub async fn bet_about(info: web::Path<(i32,)>) -> impl Responder{
	println!("bet/about handler called");
	use schema::Bet::dsl::*;
	use serde_json;
	let stmt = Bet.filter(ID.eq(info.0));
	let res = lib::transaction(move |conn|{
		stmt.load::<lib::ORM::Bet>(conn)
	}).await.unwrap_or(Vec::new());

	serde_json::ser::to_string(&res).with_status(http::status::StatusCode::from_u16(200).unwrap())

}

#[get("/bet/of/{id_usr}/")]
pub async fn bet_of(info: web::Path<(i32,)>) -> impl Responder{
	println!("bet/of handler called");
	use schema::Bet::dsl::*;
	use serde_json;

	let stmt = Bet.select(ID).filter(Who.eq(Some(info.0)));
	let res = lib::transaction(move |conn|{
		stmt.load::<i32>(conn)
	}).await.unwrap_or(Vec::new());


	match res.len() {
		0 => String::from("").with_status(http::status::StatusCode::from_u16(404).unwrap()),
		//---------------------------------- ||||||||| -?
		_ => serde_json::ser::to_string(&res).unwrap().with_status(http::status::StatusCode::from_u16(200).unwrap())
	}
}

#[post("/bet/make")]
pub async fn bet_make(req: web::Json<lib::Protocol::BetMakePayload>) -> impl Responder {
	println!("bet/make handler called");

	let passwh = base64::decode(&req.passwh);
	if let Err(_) =  passwh {
		use std::convert::TryInto;
		return String::from("").with_status(400u16.try_into().unwrap());
	};

	let (id,code) = {
		use schema::UserData::dsl::*;

		let stmt = UserData.select(ID)
			.filter(Login.eq(Some(req.login.clone())))
			.filter(Passwh.eq(Some(passwh.unwrap())));
		let res = lib::transaction(move |conn|{
			stmt.load::<i32>(conn)
		}).await.unwrap_or(Vec::new());

		match res.len() {
			0 => {(0,404)},
			1 => { (res[0],200) },
			_ => {(0,500)},
		}
	};

	match code {
		404 => {
			String::from("").with_status(http::status::StatusCode::from_u16(code).unwrap())
		},
		500 => {
			String::from("").with_status(http::status::StatusCode::from_u16(code).unwrap())
		},
		200 => {
			let mut new_code = code;
			let res: String = {
				//prepared insert
				let stmt_in_bet = {
					use schema::Bet::dsl::*;
					let row = (Who.eq(id),Value.eq(Some(req.money as f64)),on_run.eq(req.on_id_run),on_winner.eq(req.on_id_horse),win_rate.eq(Some(2.0)));
					diesel::insert_into(Bet)
						.values(row)
				};
				//Balance extractor
				let stmt_on_usr = {
					use schema::UserData::dsl::*;
					UserData.select(Balance).filter(ID.eq(id))
				};
				//upd closure
				let stmt_usr_upd = {
					use schema::UserData::dsl::*;
					move |trg: f64| {
						diesel::update(UserData)
							.set(Balance.eq(trg))
							.filter(ID.eq(id))
					}
				};

				//Do transaction
				let res = lib::transaction(move |conn|{
					let bal: Vec<f64> = stmt_on_usr.get_results(conn)?;
					match bal.len() {
						//Ok branch
						1 =>{
							if bal[0] < req.money {
								new_code = 403;
								//Err(lib::Errors::bet_make_error{msg: "ill-formed request: not enough money",code: 403})
								Ok(())
							} else {
								let f = stmt_usr_upd(req.money - bal[0]);
								f.execute(conn)?;
								stmt_in_bet.execute(conn)?;
								Ok(())
							}
						},
						//Err branch
						_ => {
							new_code = 500;
							//Err(lib::Errors::bet_make_error{msg: "Insane balance result",code: 500})
							Ok(())
						}
					}
				}).await;
				//Extract message
				let res = match res {
					Ok(()) => {
						match &new_code {
							403 => "ill-formed request: not enough money",
							500 => "Insane balance result",
							200 => "",
							_ => unreachable!(),
						}
					},
					Err(_) => {
						"ISE"
					}
				};
				//Convert message
				String::from(res)
			};
			res.with_status(http::status::StatusCode::from_u16(new_code).unwrap())
		},
		_ => unreachable!(),
	}
}
