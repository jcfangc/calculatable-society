pub mod components_related {
    pub mod agent_related {
        pub mod resources_related {
            use crate::model::components_related::agent_related::resources_related::ResourcesModel;
            use components::agent::resource_amount::ResourceAmount;
            use components::shared::subtance_type::SubtanceType;
            use context::db::db_contextd::DatabaseContexted;
            use context::GLOBAL_APP_CONTEXT;
            use sqlx::Error;
            use uuid::Uuid;

            pub async fn get_resource_by_id_numerator_and_dominator(
                agent_id: Uuid,
                resource_numerator: i32,
                resource_dominator: i32,
            ) -> Result<(SubtanceType, ResourceAmount), Error> {
                if resource_dominator <= 0 || resource_numerator <= 0 {
                    return Err(sqlx::Error::Protocol(
                        "resource_numerator 和 resource_dominator 参数应该为正数".into(),
                    ));
                }

                let pool = &*GLOBAL_APP_CONTEXT.get().unwrap().db_pool().await;

                // 执行查询
                if let Some(resources_model) = sqlx::query_as::<_, ResourcesModel>(
                    r#"
                    SELECT 
                        agent_id,
                        numerator,
                        denominator,
                        allocatable,
                        investment,
                        debt
                    FROM 
                        resources
                    WHERE 
                        agent_id = $1 AND numerator = $2 AND denominator = $3
                    "#,
                )
                .bind(agent_id)
                .bind(resource_numerator)
                .bind(resource_dominator)
                .fetch_optional(pool)
                .await?
                {
                    if let Ok(resource_type_coefficient) = SubtanceType::try_new(
                        resources_model.numerator as usize,
                        resources_model.denominator as usize,
                    ) {
                        let resource_amount = ResourceAmount::new(
                            resources_model.allocatable as usize,
                            resources_model.investment as usize,
                            resources_model.debt as usize,
                        );

                        return Ok((resource_type_coefficient, resource_amount));
                    }
                }

                // 如果没有找到对应的记录，返回一个自定义错误或选择返回一个默认值
                Err(sqlx::Error::RowNotFound)
            }
        }

        pub mod preferences_related {
            use crate::model::components_related::agent_related::preferences_related::PreferencesModel;
            use components::agent::preference_value::PreferenceValue;
            use components::shared::subtance_type::SubtanceType;
            use context::db::db_contextd::DatabaseContexted;
            use context::GLOBAL_APP_CONTEXT;
            use sqlx::Error;
            use uuid::Uuid;

            /// 根据 `resource_numerator` 和 `resource_dominator` 从数据库获取 `PropertyModel`
            pub async fn get_properties_by_numerator_and_dominator(
                agent_id: Uuid,
                resource_numerator: i32,
                resource_dominator: i32,
            ) -> Result<(SubtanceType, PreferenceValue), Error> {
                let pool = &*GLOBAL_APP_CONTEXT.get().unwrap().db_pool().await;

                // 使用 sqlx::query_as 函数版本
                if let Some(preferences_model) = sqlx::query_as::<_, PreferencesModel>(
                    r#"
                    SELECT 
                        agent_id,
                        numerator,
                        denominator,
                        preference
                    FROM
                        preferences
                    WHERE 
                        agent_id = $1 AND numerator = $2 AND denominator = $32
                    "#,
                )
                .bind(agent_id)
                .bind(resource_numerator)
                .bind(resource_dominator)
                .fetch_optional(pool)
                .await?
                {
                    if let Ok(resource_type_coefficient) = SubtanceType::try_new(
                        preferences_model.numerator as usize,
                        preferences_model.denominator as usize,
                    ) {
                        if let Ok(preference_value) =
                            PreferenceValue::try_new(preferences_model.preference)
                        {
                            return Ok((resource_type_coefficient, preference_value));
                        }
                    }
                }

                Err(Error::RowNotFound)
            }
        }
    }
}
