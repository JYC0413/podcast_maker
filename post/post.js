const express = require('express');
const { createProxyMiddleware } = require('http-proxy-middleware');

const app = express();

// 配置代理规则
app.use('/tasks/:task_id', createProxyMiddleware({
    target: 'http://159.138.158.109:8005', // 目标地址
    changeOrigin: true,
    pathRewrite: (path, req) => {
        // 动态替换路径中的 task_id
        const taskId = req.params.task_id;
        return `/tasks/${taskId}`;  // 转发到 http://159.138.158.109:8005/tasks/{task_id}
    }
}));

app.use('/record_article', createProxyMiddleware({
    target: 'http://159.138.158.109:8005', // 目标地址
    changeOrigin: true,
    pathRewrite: {
        '^/record_article': '/record_article', // 保持路径不变
    },
}));

// 启动服务
const PORT = 3000;
app.listen(PORT, () => {
    console.log(`代理服务器已启动，监听端口 ${PORT}`);
});