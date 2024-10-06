use std::fmt;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use tracing::error;

// 自定义 "资源数量" 类型并进行验证
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceAmount {
    allocatable: u64, // 表示当前可分配的资源量
    investment: u64,  // 累积投资总量
    debt: u64,        // 累积债务总量
}

impl ResourceAmount {
    /// 构造函数，创建 `ResourceAmount` 实例
    pub fn new(allocatable: u64, investment: u64, debt: u64) -> Self {
        ResourceAmount {
            allocatable,
            investment,
            debt,
        }
    }

    /// 初始化 `ResourceAmount` 实例
    ///
    /// 创建一个初始的 `ResourceAmount` 实例，所有字段都为 0。
    pub fn init() -> Self {
        ResourceAmount {
            allocatable: 0,
            investment: 0,
            debt: 0,
        }
    }

    /// 最终设置完成的方法，用于结束链式调用
    pub fn finalize(&mut self) {
        // 这里可以执行一些最终的操作，比如打印日志等
    }
}

impl AddAssign for ResourceAmount {
    fn add_assign(&mut self, other: Self) {
        // 将所有字段都进行累加操作
        self.allocatable += other.allocatable;
        self.investment += other.investment;
        self.debt += other.debt;
    }
}

impl SubAssign for ResourceAmount {
    /// 重载 `-=` 运算符，实现资源数量的减法
    fn sub_assign(&mut self, other: Self) {
        self.allocatable = self.allocatable.saturating_sub(other.allocatable);
        self.investment = self.investment.saturating_sub(other.investment);
        self.debt = self.debt.saturating_sub(other.debt);
    }
}

impl MulAssign for ResourceAmount {
    /// 重载 `*=` 运算符，实现资源数量的乘法
    fn mul_assign(&mut self, other: Self) {
        self.allocatable *= other.allocatable;
        self.investment *= other.investment;
        self.debt *= other.debt;
    }
}

impl DivAssign for ResourceAmount {
    /// 重载 `/=` 运算符，实现资源数量的除法
    fn div_assign(&mut self, other: Self) {
        if other.allocatable == 0 || other.investment == 0 || other.debt == 0 {
            error!("除数不能为 0！");
            return;
        }
        self.allocatable /= other.allocatable;
        self.investment /= other.investment;
        self.debt /= other.debt;
    }
}

// 定义 allocatable 字段的相关行为
pub trait AllocatableOperation {
    fn get_allocatable(&self) -> u64;
    fn set_allocatable(&mut self, new_value: u64) -> &mut Self;
}

impl AllocatableOperation for ResourceAmount {
    fn get_allocatable(&self) -> u64 {
        self.allocatable
    }

    fn set_allocatable(&mut self, new_value: u64) -> &mut Self {
        self.allocatable = new_value;
        self
    }
}

// 定义 investment 字段的相关行为
pub trait InvestmentOperation {
    fn get_investment(&self) -> u64;
    fn set_investment(&mut self, new_value: u64) -> &mut Self;
}

impl InvestmentOperation for ResourceAmount {
    fn get_investment(&self) -> u64 {
        self.investment
    }

    fn set_investment(&mut self, new_value: u64) -> &mut Self {
        self.investment = new_value;
        self
    }
}

// 定义 debt 字段的相关行为
pub trait DebtOperation {
    fn get_debt(&self) -> u64;
    fn set_debt(&mut self, new_value: u64) -> &mut Self;
}

impl DebtOperation for ResourceAmount {
    fn get_debt(&self) -> u64 {
        self.debt
    }

    fn set_debt(&mut self, new_value: u64) -> &mut Self {
        self.debt = new_value;
        self
    }
}

impl fmt::Display for ResourceAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ResourceAmount {{ allocatable: {}, investment: {}, debt: {} }}",
            self.allocatable, self.investment, self.debt
        )
    }
}
