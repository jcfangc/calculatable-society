pub mod components_related {
    pub mod agent_related {
        pub mod resources_related {
            use sqlx::FromRow;
            use uuid::Uuid;

            #[derive(FromRow)]
            pub struct ResourcesModel {
                // 记录的唯一ID
                pub record_id: i64,
                // 代理的唯一标识
                pub agent_id: Uuid,
                // 分子
                pub numerator: i32,
                // 分母
                pub denominator: i32,
                // 可分配的资源值
                pub allocatable_value: i32,
                // 投资值
                pub investment_value: i32,
                // 债务值
                pub debt_value: i32,
            }
        }

        pub mod preferences_related {
            use sqlx::FromRow;
            use uuid::Uuid;

            #[derive(FromRow)]
            pub struct PreferencesModel {
                // 记录的唯一ID
                pub record_id: i64,
                // 代理的唯一标识
                pub agent_id: Uuid,
                // 分子
                pub numerator: i32,
                // 分母
                pub denominator: i32,
                // 偏好值
                pub preference_value: f64,
            }
        }
    }

    pub mod shared_related {
        pub mod property_related {
            use sqlx::FromRow;

            #[derive(FromRow)]
            pub struct PropertyModel {
                // 记录的唯一ID
                pub record_id: i64,
                // 资源的分子
                pub resource_numerator: i32,
                // 资源的分母
                pub resource_dominator: i32,
                // 频率常数
                pub frequency_constant: i32,
                // 相位常数
                pub phase_constant: i32,
                // 属性的值
                pub property_value: f64,
            }
        }
    }
}
