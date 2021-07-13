use schema::messages;

#[#[derive(Queryable, Serialize, Debug)]]
pub struct Message {
  pub id: i32,
  pub username: String,
  pub message: String,
  pub timestamp: i64,
}

#[derive(Insertable, Debug)]
#[table_name = "messages"]

struct NewMessage {
  username: String,
  message: String,
}
