import sys
import matplotlib.pyplot as plt
from matplotlib.dates import (YEARLY, DateFormatter,
                              rrulewrapper, RRuleLocator, drange)
import numpy as np
import datetime

if len(sys.argv) != 2:
    print("mettre le nom du fichier en argument")
    sys.exit()

difficulty = []
dates = []
fd = open(sys.argv[1])
for line in fd:
    nb = int(line[0:3])
    difficulty.append(nb)

    date = line[5:-1]
    date = datetime.datetime.strptime(date, "%d %b %y (%H:%M)")
    dates.append(date)

difficulty.reverse()
dates.reverse()

plt.plot_date(dates, difficulty, linestyle='solid')
plt.xlabel("date")
plt.ylabel("difficulty")
plt.draw()
plt.show()
