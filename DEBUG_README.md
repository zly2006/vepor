# 中文字体调试工具集

## 快速开始

如果程序无法显示中文，请按顺序使用以下工具进行调试：

### 1. 查找系统字体
```bash
chmod +x find_fonts.sh
./find_fonts.sh
```
这会列出系统中所有可用的中文字体文件及其路径。

### 2. 测试字体加载
```bash
chmod +x test_fonts.sh
./test_fonts.sh
```
这会尝试读取各个字体文件，确认哪些可以正常访问。

### 3. 运行主程序（带调试信息）
```bash
chmod +x run.sh
./run.sh
```
这会编译并运行程序，输出详细的字体加载信息。

## 输出解读

### 成功的输出应该类似：
```
=== 两圆交点计算程序 ===

圆1: 中心 (0, 0), 半径 5
圆2: 中心 (6, 0), 半径 4

两圆交点数量: 2
交点 1: (3.2000, 3.2000)
交点 2: (3.2000, -3.2000)

正在启动可视化窗口...
开始加载中文字体...
尝试加载字体: /System/Library/Fonts/PingFang.ttc
成功读取字体文件: /System/Library/Fonts/PingFang.ttc (12345678 bytes)
✓ 成功加载字体: /System/Library/Fonts/PingFang.ttc
字体设置完成
```

### 如果看到字体加载失败：
```
开始加载中文字体...
尝试加载字体: /System/Library/Fonts/PingFang.ttc
✗ 无法读取字体: /System/Library/Fonts/PingFang.ttc
...
警告: 未能加载任何中文字体，中文可能无法正常显示
```

说明系统字体路径不对，需要：
1. 运行 `./find_fonts.sh` 找到正确路径
2. 编辑 `src/viewer.rs` 更新字体路径

## 修改字体路径

如果需要修改字体路径，编辑 `src/viewer.rs` 文件中的 `setup_chinese_fonts` 函数：

```rust
fn setup_chinese_fonts(ctx: &egui::Context) {
    // ...
    let font_paths = vec![
        "/你/的/字体/路径.ttc",  // <- 在这里添加正确的路径
        "/System/Library/Fonts/PingFang.ttc",
        // ...
    ];
    // ...
}
```

然后重新编译运行。

## 文件说明

- `find_fonts.sh` - 查找系统中的中文字体
- `test_fonts.sh` - 测试字体文件是否可读
- `run.sh` - 编译并运行主程序
- `font_test.rs` - 独立的字体测试程序
- `FONT_DEBUG_GUIDE.md` - 详细的调试指南

## 常见问题

### Q: 为什么我看到 "✓ 成功加载字体" 但中文还是方框？

A: 可能的原因：
1. TTC 字体文件包含多个字体，egui 可能选择了错误的索引
2. 字体文件损坏或不完整
3. egui 版本问题

解决方法：尝试其他字体文件，或使用 TTF 格式而不是 TTC。

### Q: 所有字体路径都无法访问怎么办？

A: 可以：
1. 下载开源中文字体（如思源黑体）放入项目
2. 使用 `include_bytes!` 宏嵌入字体
3. 参考 `FONT_DEBUG_GUIDE.md` 中的 "备选方案：嵌入字体" 部分

### Q: 程序崩溃或无法启动？

A: 检查：
1. 是否正确安装了所有依赖：`cargo build`
2. 是否有图形界面环境（SSH 连接可能无法运行 GUI 程序）
3. 系统是否支持 OpenGL

## 获取更多帮助

如果以上工具都无法解决问题，请提供：
1. `./find_fonts.sh` 的输出
2. `./test_fonts.sh` 的输出
3. `./run.sh` 的完整输出
4. 你的 macOS 版本：`sw_vers`

