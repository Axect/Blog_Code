from netCDF4 import Dataset
import matplotlib.pyplot as plt
import seaborn as sns

# Import netCDF file
ncfile = './data/prs.nc'
data = Dataset(ncfile)
var = data.variables

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(7,5), dpi=300)
plt.title(r"Histogram", fontsize=16)
plt.xlabel(r'$E$', fontsize=14)
plt.ylabel(r'Density', fontsize=14)

# Prepare Data to Plot
x = var['E'][:]  
#x_domain = var['x_domain'][:]
#pdf = var['pdf'][:]

# Plot with Legends
sns.distplot(x, kde=True, norm_hist=True, bins=100, label='Samples')
#plt.plot(x_domain, pdf, label=r'KDE')

# Other options
plt.legend(fontsize=12)
plt.xlim(-8, 208)
#plt.ylim(0, 0.2)
plt.savefig("hist.png", dpi=300, bbox_inches='tight')
