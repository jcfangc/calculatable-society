import math

class Coordinate:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __repr__(self):
        return f"({self.x}, {self.y})"

    def z(self):
        """计算隐藏的第三轴坐标 z"""
        return -(self.x + self.y)

    def hex_distance(self, other):
        """计算当前坐标和另一个坐标之间的六边形距离"""
        dx = abs(self.x - other.x)
        dy = abs(self.y - other.y)
        dz = abs(self.z() - other.z())
        return (dx + dy + dz) // 2

def interpolate(start, end):
    """
    生成从起点到终点的平滑路径，步数等于三轴距离，每步使用线性插值。
    :param start: Coordinate, 起点坐标
    :param end: Coordinate, 终点坐标
    :return: list of Coordinate, 包含每一步的坐标
    """
    # 使用六边形距离计算步数
    steps = start.hex_distance(end)
    
    path = []
    for i in range(steps + 1):
        # 计算插值系数 t
        t = i / steps
        # 使用线性插值公式计算 x 和 y 坐标
        x = start.x * (1 - t) + end.x * t
        y = start.y * (1 - t) + end.y * t
        # 四舍五入到最近的整数坐标
        rounded_x = round(x)
        rounded_y = round(y)
        # 将四舍五入后的坐标添加到路径中
        path.append(Coordinate(rounded_x, rounded_y))
    return path

# 测试代码
if __name__ == "__main__":
    start = Coordinate(2, 5)
    end = Coordinate(5, 3)
    path = interpolate(start, end)
    
    print("Generated Path:")
    for step, coord in enumerate(path):
        print(f"Step {step}: {coord}")
