use actix_web::{post, web};
use sea_orm::DbConn;




// #[post("/todos")]
// pub async fn create_todo(
//     db: web::Data<DbConn>,
//     create_todo_dto: web::Json<CreateTodoDto>,
// ) -> Result<CustomResponse, CustomError> {
    // if let Err(e) = create_todo_dto.validate() {
    //     println!("DOES IT WORK?");
    //     return Err(CustomError::ValidationError { e });
    // }
    // let title = create_todo_dto.title.clone();
    // let description = create_todo_dto.description.clone();
    // let done = create_todo_dto.done;
    // let resp = insert_todo(&conn, title.as_str(), description.as_str(), done).await?;
    // Ok(resp)
//     Ok(())
// }