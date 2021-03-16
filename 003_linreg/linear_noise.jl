using NCDataFrame, DataFrames;

function f(x::S) where {T <: Number, S <: AbstractVector{T}}
	2x .+ 1
end

x = -1.0:0.01:1.0;
ϵ = randn(length(x));
y = f(x) + ϵ;

df = DataFrame(x=x, y=y);
writenc(df, "linear.nc")