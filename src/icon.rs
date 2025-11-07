// 生成程序图标
// 创建一个简单的几何图标：两个相交的圆

pub fn create_app_icon() -> Vec<u8> {
    // 创建 1024x1024 的 RGBA 图像（高分辨率）
    let size = 1024;
    let mut pixels = vec![0u8; size * size * 4];

    // 定义两个圆（按比例放大）
    let circle1_center = (384.0, 512.0); // 12 * 32 = 384
    let circle1_radius = 320.0; // 10 * 32 = 320

    let circle2_center = (640.0, 512.0); // 20 * 32 = 640
    let circle2_radius = 256.0; // 8 * 32 = 256

    // 绘制像素
    for y in 0..size {
        for x in 0..size {
            let idx = (y * size + x) * 4;
            let fx = x as f32;
            let fy = y as f32;

            // 计算到两个圆心的距离
            let dist1 = ((fx - circle1_center.0).powi(2) + (fy - circle1_center.1).powi(2)).sqrt();
            let dist2 = ((fx - circle2_center.0).powi(2) + (fy - circle2_center.1).powi(2)).sqrt();

            // 判断点是否在圆内或边缘（边缘宽度也按比例放大）
            let in_circle1 = dist1 <= circle1_radius;
            let on_edge1 = (dist1 - circle1_radius).abs() < 3.0; // 边缘宽度从1增加到3

            let in_circle2 = dist2 <= circle2_radius;
            let on_edge2 = (dist2 - circle2_radius).abs() < 3.0;

            // 绘制颜色
            if on_edge1 || on_edge2 {
                // 边缘 - 白色
                pixels[idx] = 255; // R
                pixels[idx + 1] = 255; // G
                pixels[idx + 2] = 255; // B
                pixels[idx + 3] = 255; // A
            } else if in_circle1 && in_circle2 {
                // 交集 - 黄色
                pixels[idx] = 255; // R
                pixels[idx + 1] = 255; // G
                pixels[idx + 2] = 0; // B
                pixels[idx + 3] = 200; // A
            } else if in_circle1 {
                // 圆1 - 蓝色
                pixels[idx] = 50; // R
                pixels[idx + 1] = 120; // G
                pixels[idx + 2] = 255; // B
                pixels[idx + 3] = 200; // A
            } else if in_circle2 {
                // 圆2 - 绿色
                pixels[idx] = 50; // R
                pixels[idx + 1] = 200; // G
                pixels[idx + 2] = 100; // B
                pixels[idx + 3] = 200; // A
            } else {
                // 背景 - 深蓝色
                pixels[idx] = 20; // R
                pixels[idx + 1] = 30; // G
                pixels[idx + 2] = 50; // B
                pixels[idx + 3] = 255; // A
            }
        }
    }

    pixels
}

pub fn get_icon_data() -> egui::IconData {
    let pixels = create_app_icon();

    egui::IconData {
        rgba: pixels,
        width: 1024,
        height: 1024,
    }
}
