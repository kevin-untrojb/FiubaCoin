use rocket::fairing::AdHoc;
use rocket::Rocket;
use controller::blockchain_controllers::*;


pub fn rocket() -> Rocket {
    let rocket = rocket::ignite()
        .mount(
            "/",
            routes![new_transaction, mine_block, change_reward, change_mines_amount, get_all_blocks],
        );
    rocket
}