use utils::enum_map;

enum_map! {
    pub SQLQuery => &'static str {
        GetResourceByAgentIdAndResourceType => || {
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
            "#
        },
        GetPreferenceByAgentIdAndResourceType => || {
            r#"
            SELECT 
                agent_id,
                numerator,
                denominator,
                preference
            FROM
                preferences
            WHERE 
                agent_id = $1 AND numerator = $2 AND denominator = $3
            "#
        },
        GetPropertiesByResourceType => || {
            r#"
            SELECT 
                record_id,
                resource_numerator,
                resource_dominator,
                frequency_constant,
                phase_constant,
                property_value
            FROM 
                property
            WHERE 
                resource_numerator = $1
                AND resource_dominator = $2
                AND ($3::BIGINT IS NULL OR record_id > $3)
            ORDER BY record_id ASC
            LIMIT $4
            "#
        },
    }
}

pub mod components_related {
    pub mod agent_related {
        pub mod resources_related {
            use crate::dto::dto_resource_amount::DTOResourceAmount;
            use crate::dto::dto_subtance_type::DTOSubtanceType;
            use crate::model::components_related::agent_related::resources_related::ResourcesModel;
            use crate::repository::SQLQuery;
            use context::db::db_contextd::DatabaseContexted;
            use context::GLOBAL_APP_CONTEXT;
            use sqlx::Error;
            use uuid::Uuid;

            pub async fn get_resource_by_id_numerator_and_dominator(
                agent_id: Uuid,
                resource_numerator: i32,
                resource_dominator: i32,
            ) -> Result<(DTOSubtanceType, DTOResourceAmount), Error> {
                if resource_dominator <= 0 || resource_numerator <= 0 {
                    return Err(sqlx::Error::Protocol(
                        "resource_numerator 和 resource_dominator 参数应该为正数".into(),
                    ));
                }

                let pool = &*GLOBAL_APP_CONTEXT.get().unwrap().db_pool().await;

                // 执行查询
                if let Some(resources_model) = sqlx::query_as::<_, ResourcesModel>(
                    *SQLQuery::to_map()
                        .get(&SQLQuery::GetResourceByAgentIdAndResourceType)
                        .unwrap(),
                )
                .bind(agent_id)
                .bind(resource_numerator)
                .bind(resource_dominator)
                .fetch_optional(&**pool)
                .await?
                {
                    // 如果找到对应的记录，返回一个包含资源类型和资源数量的元组
                    let subtance_type = DTOSubtanceType {
                        numerator: resources_model.numerator as usize,
                        denominator: resources_model.denominator as usize,
                    };
                    let resource_amount = DTOResourceAmount {
                        allocatable: resources_model.allocatable_value as usize,
                        investment: resources_model.investment_value as usize,
                        debt: resources_model.debt_value as usize,
                    };
                    return Ok((subtance_type, resource_amount));
                }

                // 如果没有找到对应的记录，返回一个自定义错误或选择返回一个默认值
                Err(sqlx::Error::RowNotFound)
            }
        }

        pub mod preferences_related {
            use crate::dto::dto_preference_value::DTOPreferenceValue;
            use crate::dto::dto_subtance_type::DTOSubtanceType;
            use crate::model::components_related::agent_related::preferences_related::PreferencesModel;
            use crate::repository::SQLQuery;
            use context::db::db_contextd::DatabaseContexted;
            use context::GLOBAL_APP_CONTEXT;
            use sqlx::Error;
            use uuid::Uuid;

            /// 根据代理 ID、资源类型的分子和分母获取偏好值
            pub async fn get_preference_by_id_numerator_and_dominator(
                agent_id: Uuid,
                resource_numerator: i32,
                resource_dominator: i32,
            ) -> Result<(DTOSubtanceType, DTOPreferenceValue), Error> {
                let pool = &*GLOBAL_APP_CONTEXT.get().unwrap().db_pool().await;

                // 使用 sqlx::query_as 函数版本
                if let Some(preferences_model) = sqlx::query_as::<_, PreferencesModel>(
                    *SQLQuery::to_map()
                        .get(&SQLQuery::GetPreferenceByAgentIdAndResourceType)
                        .unwrap(),
                )
                .bind(agent_id)
                .bind(resource_numerator)
                .bind(resource_dominator)
                .fetch_optional(&**pool)
                .await?
                {
                    // 如果找到对应的记录，返回一个包含资源类型和偏好值的元组
                    let subtance_type = DTOSubtanceType {
                        numerator: preferences_model.numerator as usize,
                        denominator: preferences_model.denominator as usize,
                    };
                    let preference_value = DTOPreferenceValue {
                        value: preferences_model.preference_value,
                    };
                    return Ok((subtance_type, preference_value));
                }

                Err(Error::RowNotFound)
            }
        }
    }

    pub mod shared_related {
        pub mod property_related {
            use crate::dto::dto_property_params::DTOPropertyParams;
            use crate::model::components_related::shared_related::property_related::PropertyModel;
            use crate::repository::SQLQuery;
            use context::db::db_contextd::DatabaseContexted;
            use context::GLOBAL_APP_CONTEXT;
            use futures::stream::Stream;
            use sqlx::Error;
            use tokio::sync::mpsc;
            use tokio_stream::wrappers::ReceiverStream;

            pub async fn get_properties_by_numerator_and_dominator_stream(
                resource_numerator: i32,
                resource_dominator: i32,
                batch_size: Option<usize>,
            ) -> impl Stream<Item = Result<Vec<(DTOPropertyParams, f64)>, Error>> {
                // 获取持久的数据库池引用
                let pool = GLOBAL_APP_CONTEXT.get().unwrap().db_pool().await;

                let (tx, rx) = mpsc::channel(10); // 缓冲区大小可以根据需求调整

                // 启动异步任务以分批获取数据
                tokio::spawn(async move {
                    let batch_size = batch_size.unwrap_or(100) as i64;
                    let mut last_record_id: Option<i64> = None;

                    loop {
                        // 使用 `record_id` 实现基于非主键的分页
                        let rows = sqlx::query_as::<_, PropertyModel>(
                            *SQLQuery::to_map()
                                .get(&SQLQuery::GetPropertiesByResourceType)
                                .unwrap(),
                        )
                        .bind(resource_numerator)
                        .bind(resource_dominator)
                        .bind(last_record_id)
                        .bind(batch_size)
                        .fetch_all(&**pool)
                        .await;

                        match rows {
                            Ok(property_models) => {
                                if property_models.is_empty() {
                                    break; // 没有更多数据，退出循环
                                }

                                let batch: Vec<_> = property_models
                                    .iter()
                                    .map(|property_model| {
                                        let property_params = DTOPropertyParams {
                                            frequency_constant: property_model.frequency_constant
                                                as isize,
                                            phase_constant: property_model.phase_constant as isize,
                                        };
                                        let property_value = property_model.value;
                                        (property_params, property_value)
                                    })
                                    .collect();

                                if tx.is_closed() {
                                    break;
                                }

                                if tx.send(Ok(batch)).await.is_err() {
                                    break;
                                }

                                // 更新 `last_record_id` 为本批次的最后一个 `record_id`
                                last_record_id = property_models.last().map(|m| m.record_id);
                            }
                            Err(e) => {
                                let _ = tx.send(Err(e)).await;
                                break;
                            }
                        }
                    }
                });

                ReceiverStream::new(rx)
            }
        }
    }
}
