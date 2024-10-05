pub trait PropertyRepository {
    fn get_property_by_id(&self, id: i32) -> Option<Property>;
    fn save_property(&self, property: &Property) -> Result<(), String>;
}

pub struct PgPropertyRepository {
    conn: PgConnection, // 使用 Diesel 进行数据库连接
}

impl PropertyRepository for PgPropertyRepository {
    fn get_property_by_id(&self, id: i32) -> Option<Property> {
        // 实现从 PostgreSQL 数据库中查找用户的逻辑
    }

    fn save_property(&self, property: &Property) -> Result<(), String> {
        // 实现将用户数据保存到 PostgreSQL 数据库的逻辑
    }
}
