# 使用 Node.js lts-slim 作为基础镜像
FROM node:lts-slim

# 设置环境变量
ENV PNPM_HOME=/usr/local/share/pnpm
ENV PATH=$PNPM_HOME:$PATH

# 安装工具和依赖
RUN apt update && apt install -y net-tools curl lsof && \
    npm install -g pnpm && \
    pnpm config set global-bin-dir $PNPM_HOME && \
    pnpm add -g vite @vue/cli typescript sass && \
    apt-get clean && rm -rf /var/lib/apt/lists/*


# 设置工作目录
WORKDIR /frontend_app

# 将项目代码复制到容器中
COPY . .

# 暴露开发端口
EXPOSE 5173 5174

# 启动开发服务器
CMD ["/bin/bash"]