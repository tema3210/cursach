use actix_web::{web, Responder};
use actix_web_codegen::{get,post};
use crate::{schema,lib};

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;


#[get("/run/about/{id}")]
pub async fn run_about(info: web::Path<(i32,)>) -> impl Responder{
	use std::convert::TryInto;
	use serde_json;
	//TODO
	let sq = {
		use schema::Run::dsl::*;
		Run.filter(ID.eq(info.0))
	};
	let res = lib::transaction(move |conn|{
		sq.load::<lib::ORM::Run>(conn)
	}).await;

	match res {
		Ok(x) => {
			match x.len() {
				0 => {
					String::from("").with_status(404u16.try_into().unwrap())
				},
				1 => {
					let resp = {
						serde_json::ser::to_string(&x[0]).unwrap()
					};
					resp.with_status(200u16.try_into().unwrap())
				},
				_ => {
					String::from("").with_status(500u16.try_into().unwrap())
				}
			}
		},
		Err(_) => {
			String::from("").with_status(500u16.try_into().unwrap())
		}
	}
}


#[get("/run/of/owner/{id}")]
pub async fn run_of_owner(_info: web::Path<(u64,)>) -> impl Responder{
	//TODO
	"Unimplemented".with_status(http::status::StatusCode::from_u16(501).unwrap())
}

#[get("/run/pending")]
pub async fn runs_pending() -> impl Responder{
	let stmt = {
		use schema::Run::dsl::*;
		Run.filter(Winner.eq(None))
	};
	let res = lib::transaction(move |conn|{
		stmt.load::<lib::ORM::Run>(conn)
	}).await;
	match res {
		Ok(vec) => {
			serde_json::ser::to_string(&vec).with_status(200.try_into().unwrap())
		},
		Err(_) => {
			String::from("").with_status(500.try_into().unwrap())
		}
	}
}

#[get("/run/pending/of/{id}")]
pub async fn runs_pending_of(info: web::Path<(i32,)>) -> impl Responder{
	let stmt_sr = |ids| {
		use schema::Run::dsl::*;
		Run.filter(Winner.eq(None)).filter(ID.eq_any(ids))
	};
	let bet_stmt = {
		use schema::Bet::dsl::*;
		Bet.select(ID).filter(Who.eq(Some(info.0)))
	}

	let res = lib::transaction(move |conn|{
		let res = bet_stmt.execute(conn);
		match res{
			Ok(vec) if vec.len() > 0 => {
				Ok((stmt_sr(vec).execute(conn)?,200u16))
			},
			Ok(vec) if vec.len() == 0 => {
				Ok((Vec::new(),404))
			},
			Err(_e) {
				Err(())
			}
		}
	}).await;
	match res {
		Ok((vec,code)) => {
			serde_json::ser::to_string(&vec).with_status(code.try_into().unwrap())
		},
		Err(_) => {
			String::from("").with_status(500u16.try_into().unwrap())
		}
	}
}


#[get("/run/of/horse/{id}")]
pub async fn run_of_horse(_info: web::Path<(u64,)>) -> impl Responder{
	//TODO
	"Unimplemented".with_status(http::status::StatusCode::from_u16(501).unwrap())
}


#[post("/run/register")]
pub async fn run_register(info: web::Json<lib::Protocol::RunRegisterPayload>) -> impl Responder{
	use std::convert::TryInto;
	let usq = {
		use schema::UserData::dsl::*;
		UserData.filter(Login.eq(Some(info.login.clone())))
				.filter(Passwh.eq(Some(info.passwh.clone())))
	};
	let res = lib::transaction(move |conn|{
		usq.load::<lib::ORM::UserData>(conn)
	}).await;

	match res {
		Ok(v) => {
			match v.len() {
				0 => {
					String::from("No such user").with_status(404u16.try_into().unwrap())
				},
				1 => {
					let usr = &v[0];
					match usr.UserType {
						Some(0) => { //guest
							String::from("").with_status(403u16.try_into().unwrap())
						},
						Some(1) => { //user
							String::from("").with_status(403u16.try_into().unwrap())
						}
						Some(2) => { //admin
							let horses_check = {
								if let Some(_it) = info.competitors.iter().find(|x| **x == info.winner) {
									Some({
										use schema::Horses::dsl::*;
										let stmt = Horses.select(ID).filter(ID.eq_any(info.competitors.clone())).count();
										stmt
									})
								} else {
									None
								}
							};
							let run_transaction_p1 = {
								use schema::Run::dsl::*;
								diesel::insert_into(Run)
									.values(
										(DateOf.eq(info.date),Place.eq(info.place.clone()),Winner.eq(Some(info.winner)))
									)
							};
							let run_transaction_p2 = {
								use schema::Run::dsl::*;
								Run.select(ID)
									.filter(Winner.eq(Some(info.winner)))
									.filter(Place.eq(info.place.clone()))
									.filter(DateOf.eq(info.date))
							};
							let run_transaction_p3 = {
								let ic = info.competitors.clone();
								move |id: i32| {
									use schema::CompetList::dsl::*;
									diesel::insert_into(CompetList)
										.values({
											ic.into_iter().map(|item| (Run_compet.eq(id),HorseID.eq(item))).collect::<Vec<(_,_)>>()
										})
								}
							};
							if let Some(stmt) = horses_check {
								let resp: Result<u16,lib::PoolError> = lib::transaction(move |conn| {
									let hc: usize = stmt.execute(conn)?;
									if hc != info.competitors.len() {
										Ok(500u16)
									} else {
										run_transaction_p1.execute(conn)?;
										let id = run_transaction_p2.execute(conn)?;
										run_transaction_p3(id as i32).execute(conn)?;
										Ok(200u16)
									}
								}).await;
								match resp {
									Ok(num) => {
										String::from("").with_status(num.try_into().unwrap())
									},
									Err(e) => {
										String::from(e.msg).with_status(500u16.try_into().unwrap())
									},
								}
							} else {
								String::from("").with_status(400u16.try_into().unwrap())
							}
						},
						Some(_x) => {
							String::from("").with_status(500u16.try_into().unwrap())
						},
						None => {
							String::from("").with_status(500u16.try_into().unwrap())
						}
					}
				},
				_ => {
					String::from("").with_status(500u16.try_into().unwrap())
				}
			}
		},
		Err(_) => {
			String::from("").with_status(http::status::StatusCode::from_u16(500).unwrap())
		}
	}
}
