pub mod components_related {
    pub mod agent_related {
        pub mod resources_related {
            use sqlx::FromRow;
            use uuid::Uuid;

            #[derive(FromRow)]
            pub struct ResourcesModel {
                pub agent_id: Uuid,
                pub numerator: i32,
                pub denominator: i32,
                pub allocatable: i32,
                pub investment: i32,
                pub debt: i32,
            }
        }

        pub mod preferences_related {
            use sqlx::FromRow;
            use uuid::Uuid;

            #[derive(FromRow)]
            pub struct PreferencesModel {
                pub agent_id: Uuid,
                pub numerator: i32,
                pub denominator: i32,
                pub preference: f64,
            }
        }
    }
}
