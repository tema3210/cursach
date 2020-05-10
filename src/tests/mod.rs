#[macro_use]
extern crate diesel_migrations;

#[cfg(test)]
static dbInitFlag: OnceCell<bool> = OnceCell::new();

#[cfg(test)]
pub static DBCONNPOOL: OnceCell<Pool<ConnectionManager<diesel::SqliteConnection>>> = OnceCell::new();

#[cfg(test)]
pub async fn transaction_inner<T: 'static + std::marker::Send, F>(f: F) -> std::result::Result<T, PoolError>
where
    F: 'static + FnOnce(&conn_t) -> QueryResult<T> + Send,
{
    let pool = DBCONNPOOL.get_or_init(||{
        let manager = ConnectionManager::<conn_t>::new(":memory:");
    	let pool = Pool::builder().build(manager).unwrap();
    	pool
    });


    if let None = dbInitFlag.get() {
        let r = {
            use diesel_migrations::embed_migrations;
            pool.transaction(|c|
                    use diesel_migrations::embed_migrations;
                    embed_migrations!();
                    embedded_migrations::run(c)?;
                Ok(())
            })
        };
        dbInitFlag.set(true);
    };

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
