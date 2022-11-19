from netCDF4 import Dataset
import matplotlib.pyplot as plt
import seaborn as sns

# Import netCDF file
ncfile = './data/test.nc'
data = Dataset(ncfile)
var = data.variables

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(7,5), dpi=300)
plt.title(r"Histogram", fontsize=16)
plt.xlabel(r'$x$', fontsize=14)
plt.ylabel(r'Density', fontsize=14)

# Prepare Data to Plot
x = var['x'][:]  
x_domain = var['x_domain'][:]
pdf = var['pdf'][:]

# Plot with Legends
sns.distplot(x, kde=False, norm_hist=True, bins=100, label='Samples')
plt.plot(x_domain, pdf, label=r'KDE')

# Other options
plt.legend(fontsize=12)
plt.xlim(0, 10)
plt.ylim(0, 0.2)
plt.savefig("hist.png", dpi=300, bbox_inches='tight')
