# 使用 rust slim-bullseye 作为基础镜像
FROM rust:slim-bullseye

# 设置环境变量
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH
ENV LANG=zh_CN.UTF-8
ENV LANGUAGE=zh_CN.UTF-8
ENV LC_ALL=zh_CN.UTF-8

# 创建应用程序目录
WORKDIR /backend_app

# 安装必要工具、PostgreSQL 客户端、Redis CLI 和本地化支持
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    gnupg \
    locales \
    redis-tools \
    iputils-ping && \
    mkdir -p /etc/apt/keyrings && \
    curl -sSL https://www.postgresql.org/media/keys/ACCC4CF8.asc | gpg --dearmor -o /etc/apt/keyrings/postgresql.gpg && \
    echo "deb [signed-by=/etc/apt/keyrings/postgresql.gpg] http://apt.postgresql.org/pub/repos/apt bullseye-pgdg main" > /etc/apt/sources.list.d/pgdg.list && \
    apt-get update && \
    apt-get install -y --no-install-recommends postgresql-client-17 && \
    echo "zh_CN.UTF-8 UTF-8" > /etc/locale.gen && \
    locale-gen zh_CN.UTF-8 && \
    update-locale LANG=zh_CN.UTF-8 LANGUAGE=zh_CN.UTF-8 LC_ALL=zh_CN.UTF-8 && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rustup component add rustfmt


# 暴露端口
EXPOSE 8000

# 将当前代码复制到容器中
COPY . .