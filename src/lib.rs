// #[allow(dead_code)]
// #[allow(non_snake_case)]


use once_cell::sync::{OnceCell};
extern crate dotenv;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use tokio_diesel::*;

pub fn getB64Config() -> base64::Config {
    base64::Config::new(base64::CharacterSet::Bcrypt,true)
}

//Connection pool
static DBCONNPOOL: OnceCell<Pool<ConnectionManager<diesel::MysqlConnection>>> = OnceCell::new();

#[derive(Debug)]
pub struct PoolError{pub msg: &'static str}

#[inline(always)]
async fn transaction_inner<T: 'static + std::marker::Send, F>(f: F) -> std::result::Result<T, PoolError>
where
    F: 'static + FnOnce(&diesel::MysqlConnection) -> QueryResult<T> + Send,
{
	let pool = DBCONNPOOL.get().expect("Pool uninitialized");
	let res = pool.transaction(f).await;
	match res {
		Ok(data) => {
			Ok(data)
		},
		Err(AsyncError::Checkout(_)) => {
			Err(PoolError{msg: "Pool error"})
		},
		Err(AsyncError::Error(_)) => {
			Err(PoolError{msg: "Diesel error"})
		},
	}
}

type conn_t = diesel::MysqlConnection;


#[inline(always)]
pub async fn transaction<T: 'static + std::marker::Send, F>(f: F) -> std::result::Result<T, PoolError>
where
    F: 'static + FnOnce(&conn_t) -> QueryResult<T> + Send,
{
    return transaction_inner(f).await
}

pub fn initConnPool(url: String){
    println!("DB url: {:?}",url);
	DBCONNPOOL.set({
		let manager = ConnectionManager::<diesel::MysqlConnection>::new(url);
    	let pool = Pool::builder().build(manager).unwrap();
    	pool
	}).unwrap_or_else(|_err|{panic!("DB connection pool init failed")});
    println!("Conn pool set up done");
}


pub mod ORM {
	use diesel::backend::Backend;
	use diesel::deserialize::*;
	use diesel::sql_types::*;
	use diesel_derives::*;

	use serde::Serialize;

	#[derive(Queryable,Serialize,Clone)]
	pub struct Run {
		pub ID: i32,
		pub Date: Option<chrono::NaiveDate>,
		pub Place: Option<String>,
		pub Winner: Option<i32>, //Loshad chto pobedila
		pub CompetLFK: Option<i32> //
	}

	//Public profile opts: 0 - not; 1 - without balance and co; 2 - with.
	#[derive(Queryable,Clone)]
	pub struct UserData {
		pub ID: i32,
		pub Login: Option<String>,
		pub Passwh: Option<Vec<u8>>,
        // 3 - guest; 2 - user; 1 - admin
		pub UserType: Option<i32>,
		pub Credits: Option<i32>,
		pub Balance: f64,
		pub AssocInf: Option<String>,
		pub PublicProfile: i32,
	}

	#[derive(Queryable)]
	pub struct Owners {
		pub ID: i32,
		pub Name: Option<String>,
		pub Surname: Option<String>,
		pub Age: Option<i32>,
		pub UUID: Option<i32>,
	}

	#[derive(Queryable,Serialize)]
	pub struct Horses {
		pub ID: i32,
		pub Name: Option<String>,
		pub Owner: Option<i32>,
		pub Age: Option<i32>,
		pub WinRate: Option<f64>,
		pub RunsDone: i32, //Skoko zabegov begala
	}

	#[derive(Queryable,Serialize)]
	pub struct Bet {
		pub ID: i32,
		pub Who: Option<i32>, // Kto?
		pub Value: Option<f64>, // Skoko?
		pub on_run: i32, // Na zabeg: -
		pub on_winner: Option<i32>,
		pub win_rate: Option<f64>,
	}

	#[derive(Queryable)]
	pub struct CompetList {
		pub Run_compet: i32,
		pub HorseID: i32,
	}

	pub enum PaymentState{
		Pending=1,
		Rejected=2,
		Done=3,
	}

	impl<DB> FromSql<Integer, DB> for PaymentState
	where
	    DB: Backend,
	    i32: FromSql<Integer, DB>,
	{
	    fn from_sql(bytes: Option<&<DB as Backend>::RawValue>) -> diesel::deserialize::Result<Self> {
	        match i32::from_sql(bytes)? {
	            1 => Ok(PaymentState::Pending),
	            2 => Ok(PaymentState::Rejected),
	            3 => Ok(PaymentState::Done),
	            x => Err(format!("Unrecognized variant {}", x).into()),
	        }
	    }
	}

	#[derive(Queryable)]
	pub struct Payments {
		ID: i32,
		Other_side: Option<i32>,
		Value: Option<i32>,
		Outcoming: Option<bool>,
		State: Option<PaymentState>,
	}
}



//PROTOCOL STRUCTS
pub mod Protocol {
	use serde::Deserialize;

	#[derive(Deserialize)]
	pub struct BetMakePayload {
		pub login: String,
		pub passwh: String,
		pub on_id_run: i32,
		pub money: f64,
		pub on_id_horse: i32,
        pub expected_win_rate: f64,
	}

	#[derive(Deserialize,Clone)]
	pub struct RunRegisterPayload {
		pub login: String,
		pub passwh: Vec<u8>,
		pub date: chrono::NaiveDate,
		pub place: String,
		pub winner: i32,
		pub competitors: Vec<i32>,
	}

	#[derive(Deserialize)]
	pub struct UserRegisterPayload {
		pub login: String,
		pub passwh: String,
		pub credits: u64,
		pub about: String,
		pub public: bool,
	}

	#[derive(Deserialize)]
	pub struct UserLoginPayload {
		pub login: String,
		pub passwh: String,
	}

}

pub mod Errors {
    #[derive(serde::Deserialize)]
	pub struct bet_make_error {
		pub msg: &'static str,
		pub code: u16,
	}
	impl From<diesel::result::Error> for bet_make_error {
		fn from(trg: diesel::result::Error)-> Self {
			use diesel::result::Error::*;
			match trg {
				InvalidCString(_) => {
					Self{msg: "InvalidCString",code: 500}
				},
				DatabaseError(_kind,_) => {
					Self{msg: "InternalServerError(DB)",code: 500}
				},
				NotFound => {
					Self{msg: "NotFound",code: 404}
				},
				RollbackTransaction => {
					Self{msg: "TransactionErr",code: 500}
				},
				AlreadyInTransaction => {
					Self{msg: "TransactionErr",code: 500}
				},
				_ => {
					Self{msg: "SerDe error",code: 500}
				}
			}
		}
	}

}
