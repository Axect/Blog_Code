from netCDF4 import Dataset
import matplotlib.pyplot as plt

# Import netCDF file
ncfile = './linear.nc'
data = Dataset(ncfile)
var = data.variables

# Prepare Data to Plot
x = var['x'][:]
y = var['y'][:]

# Import netCDF file
ncfile = './linear_plot.nc'
data = Dataset(ncfile)
var = data.variables

# Prepare Data to Plot
x_reg = var['x'][:]
y_reg = var['y'][:]
a = var['a'][:][0]
b = var['b'][:][0]

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Linear Regression", fontsize=16)
plt.xlabel(r'$x$', fontsize=14)
plt.ylabel(r'$y$', fontsize=14)


# Plot with Legends
plt.scatter(x, y, label=r'$y=2x+1+\epsilon$', alpha=0.7)
plt.plot(x_reg, y_reg, label=r'$y={:.2f}x+{:.2f}$'.format(a, b), color='red', alpha=0.8)

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("linear_reg.png", dpi=300)
