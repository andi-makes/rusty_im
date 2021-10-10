use sea_orm::{Database, DatabaseConnection, DbErr, EntityTrait};

mod entity;

pub async fn get_database() -> Result<DatabaseConnection, DbErr> {
    Database::connect("postgres://postgres:postgres@localhost/rim").await
}

pub enum Table {
    Batch,
    Item,
    Model,
    Property,
}

pub async fn list(
    db: &DatabaseConnection,
) -> Result<Vec<(entity::item::Model, Option<entity::property::Model>)>, sea_orm::DbErr> {
    use entity::batch::Entity as Batch;
    use entity::item::Entity as Item;
    use entity::model::Entity as Model;
    use entity::property::Entity as Property;

    Item::find().find_also_related(Property).all(&db).await
}

pub async fn list_batch(
    db: &DatabaseConnection,
) -> Result<Vec<entity::batch::Model>, sea_orm::DbErr> {
    use entity::batch as e;

    e::Entity::find().all(&db).await
}

pub async fn list_item(db: &DatabaseConnection) -> Result<Vec<entity::item::Model>, DbErr> {
    use entity::item as e;

    e::Entity::find().all(&db).await
}

pub async fn list_model(db: &DatabaseConnection) -> Result<Vec<entity::model::Model>, DbErr> {
    use entity::model as e;

    e::Entity::find().all(&db).await
}

pub async fn list_property(db: &DatabaseConnection) -> Result<Vec<entity::property::Model>, DbErr> {
    use entity::property as e;

    e::Entity::find().all(&db).await
}

// pub async fn database_test() {
//     let db = &Database::connect("postgres://postgres:postgres@localhost/rim").await.unwrap();

//     // Insert a Batch
//     // Craft the data to be inserted
//     // let batch = entity::batch::ActiveModel {
//     //     name: Set("Electric Components".to_string()),
//     //     description: Set("A collection of all Electric Components stored @ home".to_string()),
//     //     ..Default::default()
//     // };

//     // Insert the data
//     // let res = batch.insert(db).await.unwrap();
//     // println!("{:?}", res);

//     // Get the data that was inserted by the comment above:
//     // let electric_components = entity::batch::Entity::find_by_id(1).one(db).await.unwrap().unwrap();

//     // // Now, attatch a subbatch to the electric_components batch
//     // let resistors = entity::batch::ActiveModel {
//     //     name: Set("Resistors".into()),
//     //     description: Set("Resistors of all kinds".into()),
//     //     id_batch: Set(Some(electric_components.id)),
//     //     ..Default::default()
//     // };

//     // // Insert the data
//     // let resistors = resistors.insert(db).await.unwrap();

//     // Get the resistors
//     // let resistors = entity::batch::Entity::find_by_id(2).one(db).await.unwrap().unwrap();

//     // println!("{:?}", resistors);

//     // Now, try deleting the Electric Components batch
//     let electric_components: entity::batch::ActiveModel = entity::batch::Entity::find_by_id(1).one(db).await.unwrap().unwrap().into();

//     let res: DeleteResult = electric_components.delete(db).await.unwrap();

//     println!("Rows affected: {}", res.rows_affected);
// }
