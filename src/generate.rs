use uuid::Uuid;

pub async fn gen_uuid() -> Uuid {
    Uuid::new_v4()
}