rust写的消息服务器
用websocket与客户端产生长连接
类似于在线会议室
目前没有实现像qq一样的消息缓存功能
每次login会发送一个token和im-server的cookie，token放在redis,以后的所有请求需要在header里添加login的cookie，不需要添加token，token用于解析用户基本信息，不用于验证
