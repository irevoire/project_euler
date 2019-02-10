import sys
import matplotlib.pyplot as plt
from matplotlib.dates import (YEARLY, DateFormatter,
                              rrulewrapper, RRuleLocator, drange)
import numpy as np

if len(sys.argv) != 2:
    print("mettre le nom du fichier en argument")
    sys.exit()

difficulty = []
fd = open(sys.argv[1])
for line in fd:
    nb = int(line[0:3])
    difficulty.append(nb)

difficulty.reverse()

axis = [i for i in range(0, len(difficulty))];

plt.plot(axis, difficulty, linestyle='solid')
plt.xlabel("index")
plt.ylabel("difficulty")
plt.draw()
plt.show()
