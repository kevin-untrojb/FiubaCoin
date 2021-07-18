
use rocket::http::RawStr;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use blockchin;



#[post("/new_transaction", format = "json", data = "<Transaction>")]
pub fn new_transaction()



#[post("/mine_block", format = "json", data = "<Transaction>")]
pub fn mine_block()


#[post("/change_reward", format = "json", data = "<Transaction>")]
pub fn change_reward()

#[post("/change_miners_amount", format = "json", data = "<Transaction>")]
pub fn change_miners_amount()

#[get("/all_blocks")]
pub fn get_all_blocks()