# 使用 rust slim-bullseye 作为基础镜像
FROM rust:slim-bullseye

# 设置环境变量
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH

# 创建应用程序目录
WORKDIR /backend_app

# 暴露端口
EXPOSE 8000

# 将当前代码复制到容器中
COPY . .