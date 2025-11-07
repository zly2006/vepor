# PathSegment 快速参考

## 结构定义

```rust
enum PathSegment {
    Line(Point, Point),
    Arc(Point, f64, f64, f64),
    ConnectedArc(Point, f64, f64, f64, Point, Point),
    ClosePath,
}
```

## 各类型说明

### 1. Line(start, end)
- **start**: Point - 起点
- **end**: Point - 终点

### 2. Arc(center, radius, start_angle, end_angle)
- **center**: Point - 圆心
- **radius**: f64 - 半径
- **start_angle**: f64 - 起始角度（度）
- **end_angle**: f64 - 结束角度（度）

### 3. ConnectedArc(center, radius, start_angle, end_angle, start_point, end_point)
- **center**: Point - 圆心
- **radius**: f64 - 半径
- **start_angle**: f64 - 起始角度（度）
- **end_angle**: f64 - 结束角度（度）
- **start_point**: Point - 起点坐标（用于连接）
- **end_point**: Point - 终点坐标（用于连接）

### 4. ClosePath
- 闭合路径到起点

## 用法示例

### 创建一个圆弧
```rust
// 从0度到90度的圆弧
PathSegment::Arc(
    Point { x: 10.0, y: 10.0 },  // 圆心
    5.0,                          // 半径
    0.0,                          // 起始角度
    90.0,                         // 结束角度
)
```

### 创建连接弧
```rust
// 用于布尔运算结果中的交点连接
PathSegment::ConnectedArc(
    Point { x: 10.0, y: 10.0 },   // 圆心
    5.0,                           // 半径
    45.0,                          // 起始角度
    135.0,                         // 结束角度
    Point { x: 13.5, y: 13.5 },   // 起点
    Point { x: 6.5, y: 13.5 },    // 终点
)
```

## 关键算法

### 获取中点
- **Line**: 两点坐标平均值
- **Arc/ConnectedArc**: 使用角度中点 `(start_angle + end_angle) / 2.0` 计算精确位置

### 交点计算
- 所有弧相关的交点计算都使用 `start_angle` 和 `end_angle` 进行精确验证
- 确保交点在弧的角度范围内

