pub mod components_related {
    pub mod agent_related {
        pub mod resources_related {
            use sqlx::FromRow;
            use uuid::Uuid;

            #[derive(FromRow)]
            pub struct ResourcesModel {
                pub record_id: i64,
                pub agent_id: Uuid,
                pub numerator: i32,
                pub denominator: i32,
                pub allocatable_value: i32,
                pub investment_value: i32,
                pub debt_value: i32,
            }
        }

        pub mod preferences_related {
            use sqlx::FromRow;
            use uuid::Uuid;

            #[derive(FromRow)]
            pub struct PreferencesModel {
                pub record_id: i64,
                pub agent_id: Uuid,
                pub numerator: i32,
                pub denominator: i32,
                pub preference_value: f64,
            }
        }
    }

    pub mod shared_related {
        pub mod property_related {
            use sqlx::FromRow;

            #[derive(FromRow)]
            pub struct PropertyModel {
                pub record_id: i64,
                pub resource_numerator: i32,
                pub resource_dominator: i32,
                pub frequency_constant: i32,
                pub phase_constant: i32,
                pub value: f64,
            }
        }
    }
}
