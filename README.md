# 依赖

npm install
pip install -r requirements.txt

# 启动

py server.py
npm run tauri dev

# 打包相关

```bash
# 打包python程序(将python打包成一个.exe)
pyinstaller --onefile --noconsole server.py

# python代码后重新编译后端
npm run build:python

# 开发测试 (Tauri 会自动运行打包好的 exe)
npm run tauri dev

# 最终发布应用
npm run tauri build


# 日常开发 (一键搞定)
npm run tauri dev

# 准备发布 (上线打包)：
npm run build:python  # 第一步：把最终代码打包成 exe (放进 bin 目录)
npm run tauri build   # 第二步：Tauri 自动将其合成为一个最终版程序

```

