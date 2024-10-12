use crate::shared::property::Property;
use std::collections::HashMap;
use std::fmt;
use types::components_types::shared::resource::{
    resource_amount::{AllocatableOperation, DebtOperation, InvestmentOperation, ResourceAmount},
    resource_type_coefficient::ResourceTypeCoefficient,
};

/// `Resource` 结构体，表示单个资源类型及其数量
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Resource {
    pub resource_type: ResourceTypeCoefficient,
    pub amount: ResourceAmount,
}

impl Resource {
    /// 构造函数，创建一个新的 `Resource` 实例
    ///
    /// # 参数
    /// - `resource_type`: 资源类型系数
    /// - `amount`: 资源数量
    ///
    /// # 返回值
    /// 返回一个新的 `Resource` 实例
    pub fn new(resource_type: ResourceTypeCoefficient, amount: ResourceAmount) -> Self {
        Resource {
            resource_type,
            amount,
        }
    }

    /// 获取资源的种类
    pub fn get_resource_type(&self) -> &ResourceTypeCoefficient {
        &self.resource_type
    }

    /// 获取资源的数量
    pub fn get_amount(&self) -> &ResourceAmount {
        &self.amount
    }

    /// 设置资源的数量
    pub fn set_amount(&mut self, new_allocatable: u64, new_investment: u64, new_debt: u64) {
        self.amount
            .set_allocatable(new_allocatable)
            .set_investment(new_investment)
            .set_debt(new_debt)
            .finalize();
    }

    /// 计算资源属性
    pub fn properties(&self) -> HashMap<Property, f64> {
        // 检查数据库中是否有记载本资源的属性
        // 如果有，返回属性
        // 如果没有，进行计算并存入数据库
        let property_value_entries = Property::hash_map()
            .iter()
            .map(|(property, property_const)| {
                (*property, property_const.calculate(&self.resource_type))
            })
            .collect();

        // 返回属性
        property_value_entries
    }
}

impl fmt::Display for Resource {
    /// 格式化 `Resource` 为字符串
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.resource_type, self.amount)
    }
}
