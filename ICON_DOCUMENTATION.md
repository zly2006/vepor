# 窗口图标功能说明

## 图标设计

程序窗口现在有一个自定义的高分辨率图标（1024x1024），展示了两个相交的圆：

### 视觉元素
- 🔵 **蓝色圆**（左侧大圆）：代表第一个圆
- 🟢 **绿色圆**（右侧小圆）：代表第二个圆弧
- 🟡 **黄色交集**：两个圆的重叠部分
- ⚪ **白色边缘**：清晰的圆形轮廓
- 🌑 **深蓝背景**：提供对比度

### 图标参数
```rust
尺寸: 1024x1024 像素（高分辨率）
格式: RGBA
圆1: 中心(384, 512), 半径 320
圆2: 中心(640, 512), 半径 256
边缘宽度: 3 像素
```

## 实现细节

### src/icon.rs
生成图标的核心模块：
- `create_app_icon()` - 程序化生成 RGBA 像素数据
- `get_icon_data()` - 返回 egui 兼容的图标数据结构

### src/viewer.rs
在 `run_viewer()` 函数中设置图标：
```rust
let icon = crate::icon::get_icon_data();

let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
        .with_icon(Arc::new(icon)),
    // ...
};
```

## 为什么选择 1024x1024？

1. **高分辨率**: 在 Retina/HiDPI 显示器上显示清晰
2. **兼容性**: 大多数操作系统会自动缩放到需要的尺寸
3. **未来扩展**: 如果需要导出图标文件，可以生成多种尺寸

## 图标生成过程

对于每个像素 (x, y)：
1. 计算到两个圆心的距离
2. 判断是否在圆内/圆边缘
3. 根据位置分配颜色：
   - 圆边缘 → 白色
   - 交集区域 → 黄色
   - 圆1区域 → 蓝色
   - 圆2区域 → 绿色
   - 背景 → 深蓝色

## 性能考虑

- 图标生成在程序启动时执行一次
- 生成 1024x1024 的图标需要处理约 400 万个像素（1024 × 1024 × 4）
- 在现代硬件上几乎是瞬时完成的（< 100ms）

## 自定义图标

如果想修改图标设计，编辑 `src/icon.rs`：

### 改变圆的位置和大小
```rust
let circle1_center = (384.0, 512.0);
let circle1_radius = 320.0;

let circle2_center = (640.0, 512.0);
let circle2_radius = 256.0;
```

### 改变颜色
```rust
// 圆1 - 蓝色
pixels[idx] = 50;      // R
pixels[idx + 1] = 120; // G
pixels[idx + 2] = 255; // B
pixels[idx + 3] = 200; // A (透明度)
```

### 改变分辨率
```rust
let size = 1024;  // 可以改成 512, 2048 等
```

## 使用外部图标文件

如果想使用 PNG/ICO 文件而不是程序生成的图标：

1. 添加 `image` crate 到 Cargo.toml:
```toml
[dependencies]
image = "0.24"
```

2. 修改 `src/icon.rs`:
```rust
pub fn get_icon_data() -> egui::IconData {
    let img = image::open("icon.png").unwrap().to_rgba8();
    let (width, height) = img.dimensions();
    
    egui::IconData {
        rgba: img.into_raw(),
        width: width as u32,
        height: height as u32,
    }
}
```

## 平台兼容性

- ✅ **Windows**: 显示在窗口标题栏和任务栏
- ✅ **macOS**: 显示在 Dock（如果打包成 .app）
- ✅ **Linux**: 显示在窗口管理器和任务栏

## 内存占用

1024x1024 RGBA 图标：
- 1024 × 1024 × 4 bytes = 4,194,304 bytes ≈ 4 MB
- 这是可接受的，因为它只在内存中存储一次

## 运行查看

```bash
cargo build --release
./target/release/vepor
```

现在窗口标题栏应该显示两个相交的圆的图标！

## 故障排除

### 图标不显示
1. 检查是否正确设置了 `.with_icon()`
2. 某些平台可能需要窗口获得焦点后才显示图标
3. 在某些 Linux 窗口管理器中可能需要额外配置

### 图标模糊
- 当前是 1024x1024，应该足够清晰
- 如果仍然模糊，可能是系统缩放设置问题

### 编译慢
- 生成大图标会稍微增加编译时间
- 可以考虑使用更小的尺寸（如 512x512）作为折中

