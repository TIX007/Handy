 前端测试 (Playwright)

  # 运行 Playwright 测试                                                                                                  bun run test:playwright
                                                                                                                          # 运行带 UI 的 Playwright 测试                            t
  bun run test:playwright:ui

  后端测试 (Rust)

  # 进入 src-tauri 目录
  cd src-tauri

  # 运行所有 Rust 测试
  cargo test

  # 运行特定模块的测试
  cargo test license::verify
  cargo test license::fingerprint
  cargo test license::counter
  cargo test portable

  # 运行带输出的测试（显示 println!）
  cargo test -- --nocapture

  代码检查和格式化

  # 前端 lint 检查
  bun run lint

  # 前端 lint 自动修复
  bun run lint:fix

  # 格式化检查（前端 + 后端）
  bun run format:check

  # 自动格式化（前端 + 后端）
  bun run format

  # 仅前端格式化
  bun run format:frontend

  # 仅后端格式化
  bun run format:backend

  翻译检查

  # 检查翻译完整性
  bun run check:translations

  开发模式运行

  # 启动开发服务器（前端 + 后端）
  bun run tauri dev

  # 仅前端开发
  bun run dev