
<p align="center">
  Converge 消息推送工具
</p>

# 简介

移动端通过自动化功能，配置收到短信时将消息推送到服务端，服务端将短信下发到PC客户端，PC客户端即可弹出短信通知（如图）。让在PC端登录各种账号时，不再需要每次都要找手机接收验证码


# 截图
图中自动化配置为IOS系统，安卓类似
![image-mark_tie_ex](https://res.ztion.cn/imgs/fixed/converge_demo.png)

# 技术
Rust+Tauri+Vue3
<br/>
移动端使用Http接口推送消息，客户端通过SSE链接客户端接受消息
HTTP接口及SSE接口见：cn.ztion.converge.server.controller.MessageController

# 功能

- `[已完成]`**服务端**：消息接受并转发
- 
- `[已完成]`**客户端**：实时接受消息
- 
- `[已完成]`**客户端**：推送系统通知

# 声明

- 软件涉及到的引用的图片、影音、数据均来自网络，侵删，仅限学习使用！禁止任何商业用途


# 版权

[GPL 3.0](https://www.gnu.org/licenses/gpl-3.0.html)
