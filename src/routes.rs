use rocket::http::RawStr;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use controller::blockchain_controllers::*;
use rocket::Rocket;

embed_migrations!();

pub fn rocket() -> Rocket {
    let rocket = rocket::ignite()
        .mount(
            "/",
            routes![new_transaction, mine_block, change_reward, change_mines_amount, get_all_blocks],
        );
    rocket
}