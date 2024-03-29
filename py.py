import numpy as np
import matplotlib.pyplot as plt
from scipy.stats import gaussian_kde
from scipy.signal import argrelextrema

# 生成100个个体的阶级数据（这里简化使用随机数模拟）
np.random.seed(0)  # 确保每次运行生成的数据一致
class_values = np.random.lognormal(mean=2.0, sigma=0.75, size=100)

# 构建直方图
plt.figure(figsize=(10, 6))
plt.hist(class_values, bins=20, color="skyblue", edgecolor="black", alpha=0.7)
plt.title("Class Distribution Histogram")
plt.xlabel("Class Value")
plt.ylabel("Number of Individuals")
plt.grid(axis="y", alpha=0.75)

# 函数拟合 - 使用核密度估计（KDE）拟合阶级分布
kde = gaussian_kde(class_values)
x_range = np.linspace(min(class_values), max(class_values), 1000)
kde_values = kde(x_range)

# 绘制拟合曲线
plt.plot(
    x_range,
    kde_values * len(class_values) * (max(class_values) - min(class_values)) / 20,
    color="red",
)  # 调整因子以匹配直方图的高度

# 寻找局部极大值点和局部极小值点
maxima_indices = argrelextrema(kde_values, np.greater)[0]
minima_indices = argrelextrema(kde_values, np.less)[0]
maxima_values = x_range[maxima_indices]
minima_values = x_range[minima_indices]

# 标记极大值点和极小值点
for maximum in maxima_values:
    plt.axvline(
        x=maximum,
        color="green",
        linestyle="--",
        label="Maximum (Class Domain Center)",
    )
for minimum in minima_values:
    plt.axvline(
        x=minimum,
        color="orange",
        linestyle="--",
        label="Minimum (Class Domain Boundary)",
    )

plt.annotate(
    "Class Domain Maximum",
    xy=(
        maxima_values[0],
        kde(maxima_values[0])
        * len(class_values)
        * (max(class_values) - min(class_values))
        / 20,
    ),
    xytext=(maxima_values[0] + 0.5, kde(maxima_values[0]) * len(class_values) * 1.5),
    arrowprops=dict(facecolor="black", arrowstyle="->"),
)

# 为避免重复的图例标签，进行去重处理
handles, labels = plt.gca().get_legend_handles_labels()
by_label = dict(zip(labels, handles))
plt.legend(by_label.values(), by_label.keys())

plt.tight_layout()
plt.show()
