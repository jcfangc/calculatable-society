#[derive(Debug)]
pub struct DTOResourceAmount {
    pub allocatable: usize, // 表示当前可分配的资源量
    pub investment: usize,  // 累积投资总量
    pub debt: usize,        // 累积债务总量
}
