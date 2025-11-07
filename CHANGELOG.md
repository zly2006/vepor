# Changelog

## 2025-11-07 - ConnectedArc 优化

### 改进
- **ConnectedArc 结构优化**：添加 `end_angle` 字段
  - 之前：`ConnectedArc(Point, f64, f64, Point, Point)` - center, radius, start_angle, start_point, end_point
  - 现在：`ConnectedArc(Point, f64, f64, f64, Point, Point)` - center, radius, start_angle, **end_angle**, start_point, end_point

### 简化的算法
以下函数不再需要从端点计算角度，直接使用 `end_angle` 字段：

1. **`find_shape_intersections`**
   - 之前：`line_arc_intersection(*s, *e, *c, *r, *start_angle, *start_angle + 360.0)` (临时假设为完整圆)
   - 现在：`line_arc_intersection(*s, *e, *c, *r, *start_angle, *end_angle)` (精确的角度范围)

2. **`point_inside_shape`**
   - 之前：需要计算 `let end_angle = (end_pt.y - center.y).atan2(end_pt.x - center.x).to_degrees();`
   - 现在：直接使用 `*end_angle`

3. **`get_segment_midpoint`**
   - 之前：使用端点坐标平均值作为近似中点
   - 现在：使用角度中点 `(start_angle + end_angle) / 2.0` 计算精确中点

### 优势
- ✅ **更精确**：不需要从浮点坐标反算角度（避免精度损失）
- ✅ **更简洁**：减少重复计算，代码更清晰
- ✅ **更高效**：避免 `atan2` 等三角函数重复调用
- ✅ **更一致**：Arc 和 ConnectedArc 使用相同的角度表示方式

### 测试验证
所有测试用例通过：
- ✅ Circle-Rectangle 交集：3个交点
- ✅ Circle-Circle 相交：2个交点
- ✅ Circle-Circle 外切：1个交点
- ✅ Circle-Circle 分离：0个交点
- ✅ Line-Line 各种情况：正确处理

