use capabilities::capability;
use single_api::model::BowlWaterlevel;
use async_trait::async_trait;

#[capability(Create, BowlWaterlevel)]
pub fn create_waterbowl(bowl: BowlWaterlevel) -> Result<BowlWaterlevel, CapServiceError> {
    sqlx::query!(
        "INSERT INTO bowls (date, waterlevel) VALUES ($1, $2)",
        bowl.date,
        bowl.waterlevel
    )
    .execute(&self.db)
    .await
    .expect("Failed to insert to database");
    Ok(bowl)
}

#[capability(Read, BowlWaterlevel, id = "i64")]
pub fn get_by_id(bowl_id: i64) -> Result<BowlWaterlevel, CapServiceError> {
    let bowl = sqlx::query_as!(
        BowlWaterlevel,
        r#"SELECT * FROM bowls WHERE id = $1"#,
        bowl_id
    )
    .fetch_one(&self.db)
    .await
    .expect(format!("Failed to fetch bowl with id: {}", bowl_id).as_str());
    Ok(bowl)
}

#[capability(Read, BowlWaterlevel)]
pub fn get(bowl: BowlWaterlevel) -> Result<BowlWaterlevel, CapServiceError> {
    let bowl = sqlx::query_as!(
        BowlWaterlevel,
        r#"SELECT * FROM bowls WHERE id = $1"#,
        bowl.id
    )
    .fetch_one(&self.db)
    .await
    .expect(format!("Failed to fetch bowl with id: {}", bowl.id).as_str());
    Ok(bowl)
}

#[capability(ReadAll, BowlWaterlevel)]
pub fn get_all() -> Result<Vec<BowlWaterlevel>, CapServiceError> {
    let bowls: Vec<BowlWaterlevel> = sqlx::query_as!(BowlWaterlevel, r#"SELECT * FROM bowls"#)
        .fetch_all(&self.db)
        .await
        .expect("Failed to query database for all bowls");

    Ok(bowls)
}


#[capability(Delete, BowlWaterlevel, id = "i64")]
pub fn delete_bowl_by_id(bowl_id: i64) -> Result<(), CapServiceError> {
    let b = sqlx::query_as!(BowlWaterlevel,r#"SELECT * FROM bowls WHERE id = $1"#,
        bowl_id)
        .fetch_one(&self.db)
        .await
        .expect("Failed to find bowl");

    let _res = sqlx::query!(r#"DELETE FROM bowls WHERE id = $1"#,
        bowl_id)
        .execute(&self.db)
        .await
        .expect("Failed to delete");
    Ok(b)
}


#[capability(Delete, BowlWaterlevel)]
pub fn delete_bowl(bowl: BowlWaterlevel) -> Result<BowlWaterlevel, CapServiceError> {
    let _res = sqlx::query!(r#"DELETE FROM bowls WHERE id = $1"#,
            bowl.id)
        .execute(&self.db)
        .await
        .expect("Failed to delete");
    Ok(BowlWaterlevel { id: bowl.id, date: bowl.date , waterlevel: bowl.waterlevel })
}