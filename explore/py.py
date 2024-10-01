from fractions import Fraction
import random
import matplotlib.pyplot as plt

# 基础利益的分式列表
base_fractions = [
    Fraction(1, 9),
    Fraction(1, 8),
    Fraction(1, 7),
    Fraction(1, 6),
    Fraction(1, 5),
    Fraction(2, 9),
    Fraction(1, 4),
    Fraction(2, 7),
    Fraction(1, 3),
    Fraction(3, 8),
    Fraction(2, 5),
    Fraction(3, 7),
    Fraction(4, 9),
    Fraction(1, 2),
    Fraction(5, 9),
    Fraction(4, 7),
    Fraction(3, 5),
    Fraction(5, 8),
    Fraction(2, 3),
    Fraction(5, 7),
    Fraction(3, 4),
    Fraction(7, 9),
    Fraction(4, 5),
    Fraction(5, 6),
    Fraction(6, 7),
    Fraction(7, 8),
    Fraction(8, 9),
    Fraction(1, 1),
    Fraction(10, 9),
    Fraction(9, 8),
    Fraction(8, 7),
    Fraction(7, 6),
    Fraction(6, 5),
    Fraction(11, 9),
    Fraction(5, 4),
    Fraction(9, 7),
    Fraction(4, 3),
    Fraction(11, 8),
    Fraction(7, 5),
    Fraction(10, 7),
    Fraction(13, 9),
    Fraction(3, 2),
    Fraction(14, 9),
    Fraction(11, 7),
    Fraction(8, 5),
    Fraction(13, 8),
    Fraction(5, 3),
    Fraction(12, 7),
    Fraction(7, 4),
    Fraction(16, 9),
    Fraction(9, 5),
    Fraction(11, 6),
    Fraction(13, 7),
    Fraction(15, 8),
    Fraction(17, 9),
    Fraction(2, 1),
]

# 线性组合的次数
n_combinations = 10000

# 存储结果分母的位数
combinations = []
denominator_lengths = []
N = 2

for _ in range(n_combinations):
    # 随机选择N个分式进行线性组合
    fractions_chosen = random.sample(base_fractions, N)
    combination = sum(fractions_chosen) / N

    # 约分
    if combination > 2:
        combination -= 2
    combination = combination.limit_denominator()
    combinations.append(combination)

    # 获取并存储分母位数
    denominator_length = len(str(combination.denominator))
    denominator_lengths.append(denominator_length)

# 绘制直方图
plt.hist(
    denominator_lengths,
    bins=range(1, max(denominator_lengths) + 2),
    edgecolor="black",
    align="left",
)
plt.xlabel("Denominator Length")
plt.ylabel("Frequency")
plt.title("Denominator Length Distribution in Linear Combinations")
plt.show()

print(combinations[:10])
