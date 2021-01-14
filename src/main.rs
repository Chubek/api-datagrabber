
mod routes;
mod state;

use state::State;

#[async_std::main]
async fn main() -> tide::Result<()> {
    femme::start(log::LevelFilter::Debug)?;

    let db_uri = "mongodb://chubak:4d4m4k_Dummy@142.93.110.205:27017/";
    let state = State::new(db_uri).await?;
    let mut app = tide::with_state(state);

    app.at("/list").get(routes::list_dbs);
    app.at("/:db/list").get(routes::list_colls);
    app.at("/:db/:collection").post(routes::insert_doc);
    app.at("/:db/:collection").get(routes::find_doc);
    app.at("/:db/:collection/update").get(routes::update_doc);
    app.listen("localhost:8080").await?;

    Ok(())
}