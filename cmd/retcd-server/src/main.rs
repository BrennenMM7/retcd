use jammdb::{DB, Data, Error};

fn main() -> Result<(), Error> {
{
    // open a new database file
    let db = DB::open("my-database.db")?;

    // open a writable transaction so we can make changes
    let tx = db.tx(true)?;

    // create a bucket to store a map of first names to last names
    let names_bucket = tx.create_bucket("names")?;
    names_bucket.put("Kanan", "Jarrus")?;
    names_bucket.put("Ezra", "Bridger")?;

    // commit the changes so they are saved to disk
    tx.commit()?;
}
{
    // open the existing database file
    let db = DB::open("my-database.db")?;
    // open a read-only transaction to get the data
    let tx = db.tx(false)?;
    // get the bucket we created in the last transaction
    let names_bucket = tx.get_bucket("names")?;
    // get the key/ value pair we inserted into the bucket
    if let Some(data) = names_bucket.get("Kanan") {
        assert!(data.is_kv());
        assert_eq!(data.kv().value(), b"Jarrus");
    }
}
    Ok(())
}
