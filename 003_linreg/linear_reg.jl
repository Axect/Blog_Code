# Julia
using NCDataFrame, Statistics, DataFrames

# 데이터 불러오기
df = readnc("linear.nc")

# 표본평균 구하기
x_bar = mean(df[!,:x])
y_bar = mean(df[!,:y])
x²_bar = mean(df[!,:x] .^ 2)
xy_bar = mean(df[!,:x] .* df[!,:y])

# 최대가능도추정
a = (xy_bar - x_bar * y_bar) / (x²_bar - x_bar^2)
b = y_bar - a * x_bar

# a,b 출력
@show a
@show b

# 그림 그릴 준비
x_plot = -1.0:0.01:1.0
y_plot = a .* x_plot .+ b

# 데이터 쓰기
dg = DataFrame(
    x=x_plot, 
    y=y_plot, 
    a=fillmissing([a], length(x_plot)),
    b=fillmissing([b], length(x_plot))
)
writenc(dg, "linear_plot.nc")
