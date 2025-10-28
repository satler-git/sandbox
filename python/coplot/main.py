import numpy as np
import matplotlib.pyplot as plt
import matplotlib.ticker as ticker

rng = np.random.default_rng(42)
n = 100
x = rng.standard_normal(n)
noise = rng.standard_normal(n)
r = -0.3

# --- Method 3: サンプル相関を厳密に作る ---
# 中心化
x_c = x - x.mean()
noise_c = noise - noise.mean()

# noise を x に直交化
proj = (np.dot(noise_c, x_c) / np.dot(x_c, x_c)) * x_c
z = noise_c - proj  # z は x_c に直交

# L2ノルムで正規化（ゼロ割りに注意）
xn = x_c / np.linalg.norm(x_c)
zn = z   / np.linalg.norm(z)

y = r * xn + np.sqrt(max(0.0, 1 - r**2)) * zn

# サンプル相関の確認
actual_r = np.corrcoef(x, y)[0, 1]
print("sample r =", actual_r)

# --- 描画 ---
fig, ax = plt.subplots(figsize=(6, 4))
ax.scatter(x, y, alpha=0.7)

# 回帰直線（オプション）
m, b = np.polyfit(x, y, 1)
xx = np.array([x.min(), x.max()])
ax.plot(xx, m*xx + b, linestyle='--', linewidth=1)

# 目盛を揃える設定
x_major = ticker.MultipleLocator(1.0)    # x軸：1刻み
y_major = ticker.MultipleLocator(0.1)    # y軸：0.1刻み
ax.xaxis.set_major_locator(x_major)
ax.yaxis.set_major_locator(y_major)

# 目盛ラベルのフォーマット（yは小数1桁）
ax.yaxis.set_major_formatter(ticker.FormatStrFormatter('%.1f'))

# 軸範囲を目盛に合わせて丸める（見た目が揃う）
# x範囲を整数境界に拡張
x_min = np.floor(x.min()) - 1
x_max = np.ceil(x.max()) + 1
ax.set_xlim(x_min, x_max)

# y範囲を0.1刻みに丸めて少し余白を入れる
ymin_raw, ymax_raw = y.min(), y.max()
y_min = np.floor(ymin_raw * 10) / 10 - 0.1
y_max = np.ceil (ymax_raw * 10) / 10 + 0.1
ax.set_ylim(y_min, y_max)

ax.grid(which='major', linestyle='-', linewidth=0.5)
ax.set_xlabel("x")
ax.set_ylabel("y")
ax.set_title(f"Exact-sample r (target r={r}, sample r={actual_r:.4f})")
plt.tight_layout()
plt.show()
