-- Add migration script here

-- 1. 删除主键约束
ALTER TABLE resource DROP CONSTRAINT resource_pkey;

-- 2. 删除 `id` 列
ALTER TABLE resource DROP COLUMN id;

-- 3. 添加新的主键（例如 `numerator` 和 `denominator`）
ALTER TABLE resource ADD PRIMARY KEY (numerator, denominator);
