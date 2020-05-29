#[macro_use]
extern crate diesel;
//#[macro_use]
//extern crate diesel_migrations;

#[allow(nonstandard_style)]

use actix_web::{web, App, HttpServer, Responder};
use actix_web_codegen::{get};
use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;

use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig};



mod lib;
pub mod schema;


#[get("/static/{file}")]
async fn static_srv(info: web::Path<(String,)>) -> impl Responder {
    let mut path = PathBuf::new();
	path.push(".");
	path.push("static"); // Now path point to dir static in folder of executable.

	path.push(info.0.clone());

	let file = tokio::fs::read(path).await;
	use std::io::ErrorKind;
	use http;
	match file {
		Ok(cont) => {String::from_utf8_lossy(&cont).to_string().with_status(http::status::StatusCode::from_u16(200).unwrap())}, //OK
		Err(x) if x.kind() == ErrorKind::NotFound => {String::from("").with_status(http::status::StatusCode::from_u16(404).unwrap())}, //Not Found
		Err(_) => { String::from("").with_status(http::status::StatusCode::from_u16(500).unwrap())} //ISE
	}
}

mod handlers;
use handlers::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	if cfg!(release) {
		println!("Running in release mode!");
	} else {
		println!("Running in debug mode!");
		//return Ok(());
	};

    {
        use dotenv::dotenv;
        use std::env;
        dotenv().ok();

    	let database_url = env::var("DATABASE_URL")
        	.expect("DATABASE_URL must be set");
        crate::lib::initConnPool(database_url);
    }

    let srv = HttpServer::new(|| App::new() //ALL ENDPOINTS
    		.service(static_srv) //+
    		.service(run::run_about) //+
    		.service(run::run_of_owner) //-
    		.service(run::run_of_horse) //-
    		.service(run::run_register) //+
            .service(run::runs_pending) //+
            .service(run::runs_pending_of) //+
    		.service(horses::horse_info) //+
    		.service(horses::horse_info_many) //-
    		.service(horses::horse_of) //-
    		.service(users::usr_login) //+
    		.service(users::usr_reg) //-
    		.service(users::usr_about) //+
    		.service(owners::owner_about) //-
    		.service(owners::owner_of) //-
    		.service(paym::in_paym) //-
    		.service(paym::out_paym) //-
    		.service(bet::bet_about) //+
    		.service(bet::bet_of) //+
    		.service(bet::bet_make) //+
    	);

    if cfg!(release) {
		let mut config = ServerConfig::new(NoClientAuth::new());
	    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
	    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
	    let cert_chain = certs(cert_file).unwrap();
	    let mut keys = rsa_private_keys(key_file).unwrap();
	    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

        println!("Running on port 8443");
    	srv.bind_rustls("127.0.0.1:8443",config)?.run().await
    } else {
        println!("Running on port 8080");
    	srv.bind("127.0.0.1:8080").unwrap().run().await
    }
}
