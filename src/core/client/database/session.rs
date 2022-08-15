use sea_orm::ConnectionTrait;
use crate::core::error::InternalResult;

trait SessionTrail {
    type Conn: ConnectionTrait;
    fn get_trail(&self) -> &[String];

    // fn find_by_id<T, R>(&self, entity: &T, id: i32) -> InternalResult<R>
    // where
    //     T: sea_orm::EntityTrait,
    //     R: sea_orm::ActiveModelTrait,
    // {
    //     // self.Conn
    //     entity::find_by_id(id).await
    // }
}