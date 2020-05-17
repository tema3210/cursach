use once_cell::sync::{OnceCell};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
    //result::Result as DResult,
};
use tokio_diesel::*;
//use crate::{lib};

mod tests;

#[cfg(test)]
static dbInitFlag: OnceCell<bool> = OnceCell::new();

#[cfg(test)]
pub static DBCONNPOOL: OnceCell<Pool<ConnectionManager<diesel::SqliteConnection>>> = OnceCell::new();

#[cfg(test)]
pub async fn transaction_inner<T: 'static + std::marker::Send, F>(f: F) -> std::result::Result<T, lib::PoolError>
where
    F: 'static + FnOnce(&diesel::SqliteConnection) -> QueryResult<T> + Send,
{
    let pool = DBCONNPOOL.get_or_init(||{
        let manager = ConnectionManager::<diesel::SqliteConnection>::new(":memory:");
    	let pool = Pool::builder().build(manager).unwrap();
    	pool
    });


    if true != *dbInitFlag.get_or_init(||{
        let r = {
            pool.transaction(|c|{
                // use diesel_migrations::embed_migrations;
                // diesel_migrations::embed_migrations!();
                // embedded_migrations::run(c).unwrap();
                Ok(())
            })
        };
        true
    }) {
        panic!("Impossible happened")
    };

    let res = pool.transaction(f).await;
	match res {
		Ok(data) => {
			Ok(data)
		},
		Err(AsyncError::Checkout(_)) => {
			Err(lib::PoolError{msg: "Pool error"})
		},
		Err(AsyncError::Error(_)) => {
			Err(lib::PoolError{msg: "Diesel error"})
		},
	}
}
