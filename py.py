import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

# 设定全局字体
plt.rcParams["font.sans-serif"] = ["SimHei"]
plt.rcParams["axes.unicode_minus"] = False

# 打开读取 "D:\系统默认\桌面\天河vs海珠.xlsx"
path = "D:\\系统默认\\桌面\\天河vs海珠.xlsx"
# 读取第一张表为tianhe，第二张表为haizhu
tianhe: pd.DataFrame = pd.read_excel(path, sheet_name=0)
haizhu: pd.DataFrame = pd.read_excel(path, sheet_name=1)

# 丢弃tianhe中haizhu没有的列数据
# 直接在drop方法中使用集合操作找出差异列并丢弃
tianhe = tianhe.drop(columns=set(tianhe.columns) - set(haizhu.columns))


def draw_about(tianhe: pd.DataFrame, haizhu: pd.DataFrame, column_name: str):
    plt.figure(figsize=(12, 6))

    # 设置条形图的位置和宽度
    n_years = len(tianhe["年份"])
    index = np.arange(n_years)
    bar_width = 0.1

    # 绘制原始数据的条形图
    plt.bar(index - bar_width, tianhe[column_name], bar_width, label="天河", alpha=0.7)
    plt.bar(index, haizhu[column_name], bar_width, label="海珠", alpha=0.7)

    # 计算差值并绘制
    diff_values = tianhe[column_name] - haizhu[column_name]
    plt.bar(
        index + bar_width,
        diff_values,
        bar_width,
        label="差值",
        color="gray",
        alpha=0.7,
    )

    # 绘制原始数据的折线图
    plt.plot(index, tianhe[column_name], "o-", label="天河折线")
    plt.plot(index, haizhu[column_name], "o-", label="海珠折线")

    # 绘制差值的折线图
    plt.plot(index, diff_values, "s--", label="差值折线", color="black")

    # 获取数据的最大值和最小值
    max_value = max(
        tianhe[column_name].max(), haizhu[column_name].max(), diff_values.max()
    )
    min_value = min(
        tianhe[column_name].min(), haizhu[column_name].min(), diff_values.min()
    )

    # 计算数据范围
    data_range = max_value - min_value
    # 设置y轴范围，稍微大于数据的最大值和小于最小值
    plt.ylim(min_value - data_range * 0.1, max_value + data_range * 0.4)

    # 设置图表标题和坐标轴标签
    plt.title(f"天河vs海珠{column_name}比较")
    plt.xlabel("年份")
    plt.ylabel(f"{column_name}（万元）")

    # 设置x轴刻度标签
    plt.xticks(index, tianhe["年份"])
    # 添加图例
    plt.legend(loc="upper left")
    # 显示网格
    plt.grid(True)
    # 显示图形
    plt.savefig(f"D:\\系统默认\\桌面\\{column_name}.png")


for column_name in tianhe.columns[1:]:
    draw_about(tianhe, haizhu, column_name)
