# Bug 修复：缩放时保持视觉位置

## 问题描述

**Bug**: 当用户调整 Zoom 滑块改变缩放比例时，视图的偏移量（offset）没有相应调整，导致视觉上图形的位置发生跳变。

## 问题原因

在原始代码中：
- `scale` 控制缩放比例
- `offset` 控制视图偏移
- 当 `scale` 改变时，`offset` 保持不变

这导致世界坐标到屏幕坐标的转换公式中：
```rust
screen_x = center_x + (world_x * scale) + offset_x
```

当 `scale` 改变但 `offset` 不变时，图形会"跳动"到不同的位置。

## 解决方案

### 1. 添加 `previous_scale` 字段

在 `ShapeViewer` 结构中跟踪上一次的缩放值：

```rust
pub struct ShapeViewer {
    shapes: Vec<(ResolvedShape, egui::Color32, String)>,
    scale: f32,
    offset: egui::Vec2,
    show_grid: bool,
    previous_scale: f32,  // 新增：跟踪上一次的 scale
}
```

### 2. 缩放时同步调整 offset

在 `update` 方法中，当检测到 scale 变化时，按比例调整 offset：

```rust
let old_scale = self.scale;
ui.add(egui::Slider::new(&mut self.scale, 1.0..=50.0).text("scale"));

// 当 scale 改变时，调整 offset 以保持视觉中心不变
if (self.scale - old_scale).abs() > 0.001 {
    let scale_ratio = self.scale / old_scale;
    self.offset = self.offset * scale_ratio;
    self.previous_scale = self.scale;
}
```

### 3. 重置时也更新 previous_scale

在 Reset View 按钮点击时：

```rust
if ui.button("Reset View").clicked() {
    self.offset = egui::Vec2::new(0.0, 0.0);
    self.scale = 10.0;
    self.previous_scale = 10.0;  // 同步更新
}
```

## 数学原理

假设世界坐标点 P(x, y)，在不同缩放下保持相同的屏幕位置：

**旧缩放**：
```
screen_pos = center + (world_pos * old_scale) + old_offset
```

**新缩放**（保持屏幕位置不变）：
```
screen_pos = center + (world_pos * new_scale) + new_offset
```

因此：
```
center + (world_pos * old_scale) + old_offset = center + (world_pos * new_scale) + new_offset
world_pos * old_scale + old_offset = world_pos * new_scale + new_offset
new_offset = old_offset + world_pos * (old_scale - new_scale)
```

但由于 offset 应该与所有点一致，我们使用比例缩放：
```
new_offset = old_offset * (new_scale / old_scale)
```

## 效果

修复后的行为：
- ✅ 放大时：图形保持在视觉中心，只是变大
- ✅ 缩小时：图形保持在视觉中心，只是变小
- ✅ 拖拽后缩放：图形在新位置保持相对位置不变
- ✅ Reset View：正确重置所有参数

## 测试方法

1. 运行程序
2. 拖拽视图移动图形到某个位置
3. 调整 Zoom 滑块
4. 观察图形应该在原地放大/缩小，而不是跳到其他位置

## 代码变更

- ✅ `ShapeViewer` 结构添加 `previous_scale` 字段
- ✅ `Default::default()` 初始化 `previous_scale: 10.0`
- ✅ `update()` 方法中添加 scale 变化检测和 offset 调整
- ✅ Reset View 按钮同步重置 `previous_scale`

## 编译和运行

```bash
cd /Users/zhaoliyan/IdeaProjects/vepor
cargo build --release
./target/release/vepor
```

现在缩放功能应该工作正常，视图会保持稳定！

