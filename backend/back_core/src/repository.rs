use utils::enum_map;
enum_map! {
    pub SQLQuery => &'static str {
        // 查询指定代理ID和资源类型的资源信息
        GetResourceByAgentIdAndSubtanceType => || {
            r#"
            SELECT 
                agent_id,                   -- 代理的唯一标识
                subtance_denominator,       -- 物质的分子
                subtance_denominator,       -- 物质的分母
                allocatable,                -- 可分配的资源量
                investment,                 -- 投资的资源量
                debt                        -- 债务的资源量
            FROM 
                resources
            WHERE 
                agent_id = $1               -- 代理ID
                AND numerator = $2          -- 物质的分子值
                AND denominator = $3        -- 物质的分母值
            "#
        },

        // 查询指定代理ID和资源类型的偏好信息
        GetPreferenceByAgentIdAndSubtanceType => || {
            r#"
            SELECT 
                agent_id,                   -- 代理的唯一标识
                subtance_denominator,       -- 物质的分子
                subtance_denominator,       -- 物质的分母
                preference                  -- 偏好的值
            FROM
                preferences
            WHERE 
                agent_id = $1               -- 代理ID
                AND numerator = $2          -- 物质的分子值
                AND denominator = $3        -- 物质的分母值
            "#
        },

        // 查询指定资源类型的属性信息
        GetPropertiesBySubtanceType => || {
            r#"
            SELECT 
                record_id,                     -- 记录的唯一标识
                subtance_numerator,            -- 物质的分子值
                subtance_denominator,          -- 物质的分母值
                frequency_constant,            -- 频率常数
                phase_constant,                -- 相位常数
                property_value                 -- 属性的值
            FROM 
                properties
            WHERE 
                subtance_numerator = $1               -- 物质的分子值
                AND subtance_denominator = $2         -- 物质的分母值
                AND ($3::BIGINT IS NULL OR record_id > $3)         -- 可选的记录ID过滤条件
            ORDER BY record_id ASC                    -- 按记录ID升序排列
            LIMIT $4                                  -- 返回的记录条数限制
            "#
        },
    }
}

pub mod game_related {
    pub mod agent_related {
        pub mod resources_related {
            use crate::context::core_context::GLOBAL_APP_CONTEXT;
            use crate::context::db::db_contexted::DatabaseContexted;
            use crate::dto::dto_resource_amount::DTOResourceAmount;
            use crate::dto::dto_subtance_type::DTOSubtanceType;
            use crate::model::components_related::agent_related::resources_related::ResourcesModel;
            use crate::repository::SQLQuery;
            use sqlx::Error;
            use uuid::Uuid;

            pub async fn get_resource_by_id_numerator_and_denominator(
                agent_id: Uuid,
                subtance_numerator: i32,
                subtance_denominator: i32,
            ) -> Result<(DTOSubtanceType, DTOResourceAmount), Error> {
                if subtance_denominator <= 0 || subtance_numerator <= 0 {
                    // 使用 error 记录无效的参数
                    tracing::error!(
                        "无效的参数: subtance_numerator: {}, subtance_denominator: {}",
                        subtance_numerator,
                        subtance_denominator
                    );
                    return Err(sqlx::Error::Protocol(
                        "subtance_numerator 和 subtance_denominator 参数应该为正数".into(),
                    ));
                }

                let pool = &*GLOBAL_APP_CONTEXT.get().unwrap().db_pool().await;
                tracing::trace!("获取数据库连接池");

                // 执行查询
                tracing::debug!(
                    "执行查询: agent_id: {}, subtance_numerator: {}, subtance_denominator: {}",
                    agent_id,
                    subtance_numerator,
                    subtance_denominator
                );

                if let Some(resources_model) = sqlx::query_as::<_, ResourcesModel>(
                    *SQLQuery::to_map()
                        .get(&SQLQuery::GetResourceByAgentIdAndSubtanceType)
                        .unwrap(),
                )
                .bind(agent_id)
                .bind(subtance_numerator)
                .bind(subtance_denominator)
                .fetch_optional(&**pool)
                .await?
                {
                    // 记录查询成功，找到资源记录
                    tracing::debug!(
                        "找到资源记录: agent_id: {}, subtance_numerator: {}, subtance_denominator: {}",
                        agent_id,
                        subtance_numerator,
                        subtance_denominator
                    );

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

                    tracing::trace!(
                        "返回资源数据: agent_id: {} -> subtance_type: {:?}, resource_amount: {:?}",
                        agent_id,
                        subtance_type,
                        resource_amount
                    );

                    return Ok((subtance_type, resource_amount));
                }

                // 记录查询失败，没有找到对应的资源记录
                tracing::error!(
                    "未找到资源记录: agent_id: {}, subtance_numerator: {}, subtance_denominator: {}",
                    agent_id,
                    subtance_numerator,
                    subtance_denominator
                );

                // 如果没有找到对应的记录，返回一个自定义错误或选择返回一个默认值
                Err(sqlx::Error::RowNotFound)
            }
        }

        pub mod preferences_related {
            use crate::context::core_context::GLOBAL_APP_CONTEXT;
            use crate::context::db::db_contexted::DatabaseContexted;
            use crate::dto::dto_preference_value::DTOPreferenceValue;
            use crate::dto::dto_subtance_type::DTOSubtanceType;
            use crate::model::components_related::agent_related::preferences_related::PreferencesModel;
            use crate::repository::SQLQuery;
            use sqlx::Error;
            use uuid::Uuid;

            /// 根据代理 ID、资源类型的分子和分母获取偏好值
            pub async fn get_preference_by_id_numerator_and_denominator(
                agent_id: Uuid,
                subtance_numerator: i32,
                subtance_denominator: i32,
            ) -> Result<(DTOSubtanceType, DTOPreferenceValue), Error> {
                tracing::trace!(
                    "接收到获取偏好值请求 - agent_id: {}, subtance_numerator: {}, subtance_denominator: {}",
                    agent_id,
                    subtance_numerator,
                    subtance_denominator
                );

                let pool = &*GLOBAL_APP_CONTEXT.get().unwrap().db_pool().await;
                tracing::trace!("成功获取数据库连接池");

                // 使用 sqlx::query_as 函数版本
                if let Some(preferences_model) = sqlx::query_as::<_, PreferencesModel>(
                    *SQLQuery::to_map()
                        .get(&SQLQuery::GetPreferenceByAgentIdAndSubtanceType)
                        .unwrap(),
                )
                .bind(agent_id)
                .bind(subtance_numerator)
                .bind(subtance_denominator)
                .fetch_optional(&**pool)
                .await?
                {
                    tracing::debug!(
                        "找到对应的偏好记录 - agent_id: {}, subtance_numerator: {}, subtance_denominator: {}",
                        agent_id,
                        subtance_numerator,
                        subtance_denominator
                    );

                    // 如果找到对应的记录，返回一个包含资源类型和偏好值的元组
                    let subtance_type = DTOSubtanceType {
                        numerator: preferences_model.numerator as usize,
                        denominator: preferences_model.denominator as usize,
                    };
                    let preference_value = DTOPreferenceValue {
                        value: preferences_model.preference_value,
                    };

                    tracing::trace!(
                        "返回偏好数据 - agent_id: {} -> subtance_type: {:?}, preference_value: {:?}",
                        agent_id,
                        subtance_type,
                        preference_value
                    );

                    return Ok((subtance_type, preference_value));
                }

                tracing::error!(
                    "未找到偏好记录 - agent_id: {}, subtance_numerator: {}, subtance_denominator: {}",
                    agent_id,
                    subtance_numerator,
                    subtance_denominator
                );
                Err(Error::RowNotFound)
            }
        }
    }

    pub mod shared_related {
        pub mod property_related {
            use crate::context::core_context::GLOBAL_APP_CONTEXT;
            use crate::context::db::db_contexted::DatabaseContexted;
            use crate::dto::dto_property_params::DTOPropertyParams;
            use crate::model::components_related::shared_related::property_related::PropertyModel;
            use crate::repository::SQLQuery;
            use futures::stream::Stream;
            use sqlx::Error;
            use tokio::sync::mpsc;
            use tokio_stream::wrappers::ReceiverStream;

            pub async fn get_properties_by_numerator_and_denominator_stream(
                subtance_numerator: i32,
                subtance_denominator: i32,
                batch_size: Option<usize>,
            ) -> impl Stream<Item = Result<Vec<(DTOPropertyParams, f64)>, Error>> {
                tracing::trace!(
                    "启动获取属性流 - subtance_numerator: {}, subtance_denominator: {}, batch_size: {:?}",
                    subtance_numerator,
                    subtance_denominator,
                    batch_size
                );

                // 获取持久的数据库池引用
                let pool = GLOBAL_APP_CONTEXT.get().unwrap().db_pool().await;
                tracing::trace!("成功获取数据库连接池");

                let (tx, rx) = mpsc::channel(10); // 缓冲区大小可以根据需求调整

                // 启动异步任务以分批获取数据
                tokio::spawn(async move {
                    let batch_size = batch_size.unwrap_or(100) as i64;
                    let mut last_record_id: Option<i64> = None;

                    loop {
                        tracing::trace!(
                            "正在获取下一批属性数据 - subtance_numerator: {}, subtance_denominator: {}, last_record_id: {:?}, batch_size: {}",
                            subtance_numerator,
                            subtance_denominator,
                            last_record_id,
                            batch_size
                        );

                        // 使用 `record_id` 实现基于非主键的分页
                        let rows = sqlx::query_as::<_, PropertyModel>(
                            *SQLQuery::to_map()
                                .get(&SQLQuery::GetPropertiesBySubtanceType)
                                .unwrap(),
                        )
                        .bind(subtance_numerator)
                        .bind(subtance_denominator)
                        .bind(last_record_id)
                        .bind(batch_size)
                        .fetch_all(&**pool)
                        .await;

                        match rows {
                            Ok(property_models) => {
                                if property_models.is_empty() {
                                    tracing::debug!("没有更多属性数据，退出流循环");
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
                                        let property_value = property_model.property_value;
                                        (property_params, property_value)
                                    })
                                    .collect();

                                tracing::trace!("发送一批属性数据，batch_size: {}", batch.len());

                                if tx.is_closed() {
                                    tracing::error!("通道已关闭，退出流循环");
                                    break;
                                }

                                if tx.send(Ok(batch)).await.is_err() {
                                    tracing::error!("发送数据失败，退出流循环");
                                    break;
                                }

                                // 更新 `last_record_id` 为本批次的最后一个 `record_id`
                                last_record_id = property_models.last().map(|m| m.record_id);
                                tracing::trace!("更新 last_record_id 为: {:?}", last_record_id);
                            }
                            Err(e) => {
                                tracing::error!("数据库查询错误: {:?}", e);
                                let _ = tx.send(Err(e)).await;
                            }
                        }
                    }
                });

                ReceiverStream::new(rx)
            }
        }
    }
}
